mod from_filepath {
    use std::path::Path;
    use tree_magic_mini as tree_magic;

    #[test]
    fn nonexistent_file_returns_none() {
        assert_eq!(
            tree_magic::from_filepath(Path::new("this/file/does/not/exist")),
            None
        );
    }
}
