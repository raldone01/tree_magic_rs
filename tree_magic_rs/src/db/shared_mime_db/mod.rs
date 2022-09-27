mod mime_db;

mod shared_mime_db_types;
use shared_mime_db_types::SharedMimeMagicRule;

mod parse_magic_rule;
use parse_magic_rule::{parse_magic_file, MagicRuleParseError};

mod shared_mime_db_provider;
pub use shared_mime_db_provider::{SharedMimeDbProviderError, SharedMimeDbProviderExt};
