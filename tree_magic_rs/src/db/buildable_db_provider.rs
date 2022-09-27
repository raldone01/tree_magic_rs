use super::DbProvider;

pub struct BuildeableDbProvider<MagicRule, Alias, Subclass>
where
  MagicRule: crate::db::MagicRule + Clone,
  Alias: crate::db::Alias + Clone,
  Subclass: crate::db::Subclass + Clone,
{
  magic_rules: Vec<MagicRule>,
  aliases: Vec<Alias>,
  subclasses: Vec<Subclass>,
}
impl<MagicRule, Alias, Subclass> BuildeableDbProvider<MagicRule, Alias, Subclass>
where
  MagicRule: crate::db::MagicRule + Clone,
  Alias: crate::db::Alias + Clone,
  Subclass: crate::db::Subclass + Clone,
{
  #[must_use]
  pub fn new() -> Self {
    Self {
      magic_rules: Vec::new(),
      aliases: Vec::new(),
      subclasses: Vec::new(),
    }
  }
  #[must_use]
  pub fn magic_rules_mut(&mut self) -> &mut Vec<MagicRule> {
    &mut self.magic_rules
  }
  #[must_use]
  pub fn aliases_mut(&mut self) -> &mut Vec<Alias> {
    &mut self.aliases
  }
  #[must_use]
  pub fn subclasses_mut(&mut self) -> &mut Vec<Subclass> {
    &mut self.subclasses
  }
  pub fn clear(&mut self) {
    self.magic_rules.clear();
    self.aliases.clear();
    self.subclasses.clear();
  }
}
impl<'a, MagicRule, Alias, Subclass> DbProvider<'a>
  for BuildeableDbProvider<MagicRule, Alias, Subclass>
where
  MagicRule: crate::db::MagicRule + Clone,
  Alias: crate::db::Alias + Clone,
  Subclass: crate::db::Subclass + Clone,
{
  type MagicRule = MagicRule;

  fn iter_magic_rules(&'a self) -> Box<dyn Iterator<Item = &Self::MagicRule> + 'a> {
    Box::new(self.magic_rules.iter())
  }

  type Alias = Alias;

  fn iter_aliases(&'a self) -> Box<dyn Iterator<Item = &Self::Alias> + 'a> {
    Box::new(self.aliases.iter())
  }

  type Subclass = Subclass;

  fn iter_subclasses(&'a self) -> Box<dyn Iterator<Item = &Self::Subclass> + 'a> {
    Box::new(self.subclasses.iter())
  }
}

// impl DbProvider for BuildeableDb {
//   fn magic_rules(&self) -> &[u8] {
//     self.magic_rules.as_bytes()
//   }

//   fn aliases<'a>(&'a self) -> impl Iterator<Item = impl Alias + 'a> {
//     self.alias_string.split('\n').map(|str| {
//       // TODO: Fix
//       let mut splt = str.split(' ');
//       BorrowedAlias {
//         alias: splt.next().unwrap(),
//         name: splt.next().unwrap(),
//       }
//     })
//   }

//   fn subclasses(&self) -> String {
//     todo!()
//   }
// }
