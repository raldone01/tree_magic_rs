use crate::db::{Alias, DbProvider, MagicRule, Subclass};
use std::collections::VecDeque;

type DynAlias<'a> = dyn Alias + 'a;
type DynSubclass<'a> = dyn Subclass + 'a;
type DynMagicRule<'a> = dyn MagicRule + 'a;
type DynDbProvider<'a> = Box<
  dyn DbProvider<
    'a,
    Alias = DynAlias<'a>,
    Subclass = DynSubclass<'a>,
    MagicRule = DynMagicRule<'a>,
  >,
>;

pub struct StackedDbProvider<'a> {
  dbs: VecDeque<DynDbProvider<'a>>,
}
impl<'a> StackedDbProvider<'a> {
  #[must_use]
  pub fn new() -> Self {
    Self {
      dbs: VecDeque::new(),
    }
  }
  pub fn prepend_db(&mut self, db: DynDbProvider<'a>) {
    self.dbs.push_front(db)
  }
  pub fn append_db(&mut self, db: DynDbProvider<'a>) {
    self.dbs.push_back(db)
  }
}
impl<'a> DbProvider<'a> for StackedDbProvider<'a> {
  type MagicRule = DynMagicRule<'a>;
  fn iter_magic_rules(&'a self) -> Box<dyn Iterator<Item = &Self::MagicRule> + 'a> {
    Box::new(self.dbs.iter().flat_map(|db| db.iter_magic_rules()))
  }

  type Alias = DynAlias<'a>;
  fn iter_aliases(&'a self) -> Box<dyn Iterator<Item = &Self::Alias> + 'a> {
    Box::new(self.dbs.iter().flat_map(|db| db.iter_aliases()))
  }

  type Subclass = DynSubclass<'a>;
  fn iter_subclasses(&'a self) -> Box<dyn Iterator<Item = &Self::Subclass> + 'a> {
    Box::new(self.dbs.iter().flat_map(|db| db.iter_subclasses()))
  }
}
