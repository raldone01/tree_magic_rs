use crate::db::MagicRule;

#[derive(Clone)]
pub struct SharedMimeMagicRule<'a> {
  priority: u32,
  mime_type: &'a str,
  indent_level: u32,
  start_off: u32,
  val: Box<[u8]>,
  mask: Box<[u8]>,
  word_len: u32,
  region_len: u32,
}
impl<'a> SharedMimeMagicRule<'a> {
  #[must_use]
  pub fn new(
    priority: u32,
    mime_type: &'a str,
    indent_level: u32,
    start_off: u32,
    val: Box<[u8]>,
    mask: Box<[u8]>,
    word_len: u32,
    region_len: u32,
  ) -> Self {
    Self {
      priority,
      mime_type,
      indent_level,
      start_off,
      val,
      mask,
      word_len,
      region_len,
    }
  }
}
impl<'a> MagicRule for SharedMimeMagicRule<'a> {
  fn priority(&self) -> u32 {
    self.priority
  }

  fn mime_type(&self) -> &str {
    self.mime_type
  }

  fn indent_level(&self) -> u32 {
    self.indent_level
  }

  fn start_off(&self) -> u32 {
    self.start_off
  }

  fn val(&self) -> &[u8] {
    self.val.as_ref()
  }

  fn mask(&self) -> &[u8] {
    self.mask.as_ref()
  }

  fn word_len(&self) -> u32 {
    self.word_len
  }

  fn region_len(&self) -> u32 {
    self.region_len
  }
}
