use std::{
  alloc::{GlobalAlloc, Layout},
  cmp::max,
  mem::{self, align_of, size_of, transmute, MaybeUninit},
  ptr::{self, NonNull},
  sync::atomic::{AtomicPtr, AtomicUsize, Ordering},
  thread::current,
};

// fetch_add init_count
// READER: fetch
// init -> value release

/// After header is a `MaybeUninit<T>` slice.
struct Header<T: Send + Sync> {
  active_write_cnt: AtomicUsize,
  definitely_released_cnt: AtomicUsize,
  init_cnt: AtomicUsize,
  next_block: AtomicPtr<Header<T>>,
  block_size: usize,
}
impl<T: Send + Sync> Header<T> {
  /// Returns `(alloc_layout, data_offset)`.
  fn gen_layout(block_size: usize) -> (Layout, usize) {
    let header_layout = Layout::new::<Self>();
    let single_data_layout = Layout::new::<T>();

    let data_layout = single_data_layout.repeat(block_size).unwrap().0;
    header_layout.extend(data_layout).unwrap()
  }

  fn new(block_size: usize) -> NonNull<Self> {
    let (alloc_layout, _) = Self::gen_layout(block_size);

    let lselv = Self {
      active_write_cnt: AtomicUsize::new(0),
      definitely_released_cnt: AtomicUsize::new(0),
      init_cnt: AtomicUsize::new(0),
      next_block: AtomicPtr::new(std::ptr::null_mut()),
      block_size,
    };

    unsafe {
      let selv_ptr = transmute::<_, NonNull<Self>>(std::alloc::alloc(alloc_layout));
      selv_ptr.as_ptr().write(lselv);
      selv_ptr
    }
  }

  fn update_definitely_released(&self) -> Option<usize> {
    // Ordering: Relaxed no accesses or changes.
    let last_init_slot = self.init_cnt.load(Ordering::Relaxed);

    // Ordering: Acquire because the init_slot increment must happen before.
    let real_write_cnt = self.active_write_cnt.fetch_add(1, Ordering::Acquire);
    if real_write_cnt == 0 {
      // Ordering: Relaxed because the previous acquire already gets us the data.
      self
        .definitely_released_cnt
        .store(last_init_slot, Ordering::Relaxed);
      return Some(last_init_slot);
    }
    // Ordering: Relaxed no accesses or changes.
    self.active_write_cnt.fetch_sub(1, Ordering::Relaxed);

    None
  }

  fn push(&self, item: T) -> (&T, NonNull<Header<T>>) {
    let selv_ptr = unsafe { NonNull::<Self>::new_unchecked(self as *const Self as *mut Self) };

    // Ordering: Relaxed no accesses or changes.
    let init_slot = self.init_cnt.fetch_add(1, Ordering::Relaxed);

    // Ordering: Acquire because the init_slot increment must happen before.
    let old_write_cnt = self.active_write_cnt.fetch_add(1, Ordering::Acquire); // Returns the previous value so we later have to check with 0
    let active_write_guard = FnOnDrop {
      f: || {
        // Ordering: Release to make the data visible.
        self.active_write_cnt.fetch_sub(1, Ordering::Release);
      },
    };

    // If we are the only writer we know that the every up to us has been `definitely_released_cnt`.
    if old_write_cnt == 0 {
      // Ordering: Relaxed the writers have already release the data.
      self
        .definitely_released_cnt
        .store(init_slot - 1, Ordering::Relaxed);
    }

    if init_slot >= self.block_size {
      // We did not get a slot in the current block.
      // First try to fetch the next block it may already exist.

      // TODO: maybe unroll catchup to last block
      // Ordering: Acquire to access the block contents if the pointer is not null.
      let maybe_new_block = self.next_block.load(Ordering::Acquire);
      if !maybe_new_block.is_null() {
        // Someone else has already allocated a block. Lets try to use that block.
        mem::drop(active_write_guard);
        return unsafe { &*maybe_new_block }.push(item);
      }

      // No one has allocated a next block yet. So we allocate one.
      let new_block = Self::new(self.block_size);

      let mut result = Err(selv_ptr.as_ptr());

      // This is the next block in the chain.
      let mut first_new_block: *const Header<T> = ptr::null_mut();

      // We have allocated a block because there was no next block.
      // However while we were allocating another thread may have done the same.
      // If that is the case we add our already allocated block behind the block another thread has allocated.
      while let Err(other_allocated) = result {
        result = unsafe { &*other_allocated }.next_block.compare_exchange(
          ptr::null_mut(),
          new_block.as_ptr(),
          // Ordering: Release to make our newly allocate block available.
          Ordering::Release,
          // Ordering: Acquire to access the `next_block` field of the newly gotten block.
          // Also if it is the first block we will call `push`.
          Ordering::Acquire,
        );

        // Remember the first block with possibly empty slots
        if first_new_block.is_null() {
          first_new_block = match result {
            Ok(first_new_block) => first_new_block,
            Err(first_new_block) => first_new_block,
          }
        }
      }

      mem::drop(active_write_guard);
      return unsafe { &*first_new_block }.push(item);
    }

    // Guaranteed slot

    // Write data
    let (_, data_offset) = Self::gen_layout(self.block_size);
    let data_ref = unsafe {
      let data_slot =
        transmute::<_, *mut T>(selv_ptr.as_ptr().byte_add(data_offset)).add(init_slot);
      data_slot.write(item);
      &*data_slot
    };

    // Note: active_write_guard dropped here

    (data_ref, selv_ptr)
  }

  /// SAFETY: The caller must make sure that the index is not out bounds and that the data has been initialized.
  unsafe fn get_unchecked(&self, index: usize) -> &T {
    let selv_ptr = unsafe { NonNull::<Self>::new_unchecked(self as *const Self as *mut Self) };

    // Read data
    let (_, data_offset) = Self::gen_layout(self.block_size);
    let data_ref = unsafe {
      let data_slot = transmute::<_, *mut T>(selv_ptr.as_ptr().byte_add(data_offset)).add(index);
      &*data_slot
    };
    data_ref
  }
}

/// May skip some newly written elements.
/// Note: This is not a fused iterator if another block is added or new stuff is written it may start returning `&T`s again.
struct BlockIter<'a, T: Send + Sync> {
  header: &'a Header<T>,
  definitely_released_cnt: usize,
  index: usize,
}
impl<'a, T: Send + Sync> BlockIter<'a, T> {
  fn new(header: &'a Header<T>) -> Self {
    Self {
      header,
      definitely_released_cnt: header.definitely_released_cnt.load(Ordering::Acquire),
      index: 0,
    }
  }
}
impl<'a, T: Send + Sync> Iterator for BlockIter<'a, T> {
  type Item = &'a T;

  fn next(&mut self) -> Option<Self::Item> {
    if self.index >= self.definitely_released_cnt {
      // reload definitely_released
      self.definitely_released_cnt = self.header.definitely_released_cnt.load(Ordering::Acquire);
    } else if self.index >= self.definitely_released_cnt {
      // trigger a more expensive reload on the block level
      if let Some(new_definitely_released_cnt) = self.header.update_definitely_released() {
        self.definitely_released_cnt = new_definitely_released_cnt;
      }
    } else if self.index >= self.definitely_released_cnt {
      // reloading definitely_released did not help -> move on to next block
      let Some(next_block) = (unsafe { self.header.next_block.load(Ordering::Relaxed).as_ref() }) else {
        return None
      };
      *self = Self::new(next_block);
      return self.next();
    }

    // index < definitely_released_cnt -- Safe to get ref

    todo!()
  }
}

pub struct FnOnDrop<F: FnMut()> {
  f: F,
}
impl<F: FnMut()> Drop for FnOnDrop<F> {
  fn drop(&mut self) {
    (self.f)()
  }
}
