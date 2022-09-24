/// Struct that contains the raw database as stored on disk.
pub struct DbGuts {
  /// The files loaded into memory
  /// TODO: flatten outer vec
  runtime_rules: Vec<Vec<u8>>,
  /// The alias file
  alias_string: String,
  /// The subclass file
  subclass_string: String,
}
