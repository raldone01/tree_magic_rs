mod db_traits;
pub use db_traits::{Alias, DbProvider, MagicRule, Subclass};

mod borrowed_db_types;
pub use borrowed_db_types::{
  BorrowedAlias, BorrowedBuildableDb, BorrowedMagicRule, BorrowedSubclass,
};

mod owned_db_types;
pub use owned_db_types::{OwnedAlias, OwnedBuildableDb, OwnedMagicRule, OwnedSubclass};

mod buildable_db_provider;
pub use buildable_db_provider::BuildeableDbProvider;

mod stacked_db_provider;
pub use stacked_db_provider::StackedDbProvider;

mod shared_mime_db;
pub use shared_mime_db::SharedMimeDbProviderExt;
