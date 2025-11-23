use std::fs;
use std::io;

/// File I/O operations for the text editor
pub struct FileIO;

impl FileIO {
    /// Load content from a file
    pub fn load_file(filename: &str) -> io::Result<Vec<String>> {
        let content = fs::read_to_string(filename)?;
        let lines = if content.is_empty() {
            vec!["".to_string()]
        } else {
            content.lines().map(|s| s.to_string()).collect()
        };
        Ok(lines)
    }

    /// Save content to a file
    pub fn save_file(filename: &str, lines: &[String]) -> io::Result<()> {
        let content = lines.join("\n");
        fs::write(filename, content)?;
        Ok(())
    }

    /// Check if a file exists
    pub fn file_exists(filename: &str) -> bool {
        std::path::Path::new(filename).exists()
    }

    /// Get file metadata (size, last modified, etc.)
    pub fn get_file_info(filename: &str) -> io::Result<FileInfo> {
        let metadata = fs::metadata(filename)?;
        Ok(FileInfo {
            size: metadata.len(),
            is_readonly: metadata.permissions().readonly(),
        })
    }
}

/// Information about a file
#[derive(Debug)]
pub struct FileInfo {
    pub size: u64,
    pub is_readonly: bool,
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_load_and_save_file() {
        let test_filename = "test_file.txt";
        let test_content = vec!["Hello".to_string(), "World".to_string()];

        // Save test content
        FileIO::save_file(test_filename, &test_content).unwrap();

        // Load and verify
        let loaded_content = FileIO::load_file(test_filename).unwrap();
        assert_eq!(loaded_content, test_content);

        // Cleanup
        let _ = fs::remove_file(test_filename);
    }

    #[test]
    fn test_file_exists() {
        let test_filename = "test_exists.txt";
        
        // File should not exist initially
        assert!(!FileIO::file_exists(test_filename));

        // Create file
        FileIO::save_file(test_filename, &["test".to_string()]).unwrap();
        
        // File should now exist
        assert!(FileIO::file_exists(test_filename));

        // Cleanup
        let _ = fs::remove_file(test_filename);
    }
}
