#[cfg(test)]
mod test {
    use fake_file::{Strategy, Structure};
    use fs_extra::dir::get_size;
    use std::{fs, path::Path};

    const TEST_SCRATCH_SPACE: &str = "test";
    const TEST_SIZE: usize = 1024 * 1024;
    const TEST_WIDTH: usize = 2;
    const TEST_DEPTH: usize = 2;

    /// Create a balanced file structure, 1 KB in size
    #[test]
    fn test_simple() {
        let test_scratch_space = Path::new(TEST_SCRATCH_SPACE);
        // Remove the scratch space and recreate it
        fs::remove_dir_all(&test_scratch_space).unwrap_or(());
        fs::create_dir_all(&test_scratch_space).unwrap();
        // Create a file structure
        let structure = Structure::new(TEST_WIDTH, TEST_DEPTH, TEST_SIZE, Strategy::Simple);
        // Push another path onto the scratch space
        let test_scratch_space = test_scratch_space.join(structure.to_path_string());
        // Generate the file structure
        structure
            .generate(&test_scratch_space)
            .map_err(|e| {
                println!("Error Generating FS: {}", e);
            })
            .unwrap();
        // Check that the file structure was created
        assert!(test_scratch_space.exists());
        // Check the the file structure is around the right size
        let structure_size = get_size(&test_scratch_space).unwrap();

        assert_eq!(structure_size, TEST_SIZE as u64);
    }
}
