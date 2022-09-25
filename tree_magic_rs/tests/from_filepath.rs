mod from_filepath {
  use std::path::Path;
  use tree_magic_rs as tree_magic;

  #[test]
  fn nonexistent_file_returns_none() {
    assert_eq!(
      tree_magic::MimeDatabase::new().from_filepath(Path::new("this/file/does/not/exist")),
      None
    );
  }
}
