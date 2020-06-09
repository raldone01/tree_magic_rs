mod from_filepath {
    use std::path::Path;

    #[test]
    fn nonexistent_file_returns_none() {
        assert_eq!(
            tree_magic::from_filepath(Path::new("this/file/does/not/exist")),
            None
        );
    }
}
