use anyhow::{Context, Result};
use std::fs;
use std::path::{Path, PathBuf};

use crate::crypto::encryption::CocoonEncryption;

/// File-based encrypted storage for Osnova
///
/// Provides encrypted file storage for:
/// - Application cache data
/// - Device keys
/// - Configuration files
/// - Other sensitive data that needs to be persisted to disk
///
/// All data is encrypted at rest using cocoon encryption.
///
/// # Example
///
/// ```no_run
/// use osnova_lib::storage::FileStorage;
///
/// # fn main() -> anyhow::Result<()> {
/// let storage = FileStorage::new("/path/to/storage")?;
/// let encryption_key = [0u8; 32];
///
/// // Write encrypted data
/// storage.write("cache/app-001", b"cached data", &encryption_key)?;
///
/// // Read encrypted data
/// let data = storage.read("cache/app-001", &encryption_key)?;
/// # Ok(())
/// # }
/// ```
pub struct FileStorage {
    base_path: PathBuf,
}

impl FileStorage {
    /// Create a new file storage instance
    ///
    /// # Arguments
    ///
    /// * `base_path` - Base directory for file storage
    ///
    /// # Errors
    ///
    /// Returns an error if the base directory cannot be created
    pub fn new<P: AsRef<Path>>(base_path: P) -> Result<Self> {
        let base_path = base_path.as_ref().to_path_buf();
        fs::create_dir_all(&base_path)
            .context("Failed to create storage directory")?;

        Ok(Self { base_path })
    }

    /// Write encrypted data to a file
    ///
    /// Creates parent directories as needed. The file is encrypted using
    /// the provided encryption key.
    ///
    /// # Arguments
    ///
    /// * `relative_path` - Path relative to base directory
    /// * `data` - Data to write
    /// * `encryption_key` - 256-bit encryption key
    ///
    /// # Errors
    ///
    /// Returns an error if encryption fails or file cannot be written
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use osnova_lib::storage::FileStorage;
    /// # fn main() -> anyhow::Result<()> {
    /// let storage = FileStorage::new("/tmp/storage")?;
    /// let key = [42u8; 32];
    /// storage.write("config/app.json", b"{\"theme\":\"dark\"}", &key)?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn write<P: AsRef<Path>>(
        &self,
        relative_path: P,
        data: &[u8],
        encryption_key: &[u8; 32],
    ) -> Result<()> {
        let full_path = self.base_path.join(relative_path.as_ref());

        // Create parent directories
        if let Some(parent) = full_path.parent() {
            fs::create_dir_all(parent)
                .context("Failed to create parent directories")?;
        }

        // Encrypt data
        let encryption = CocoonEncryption::new(encryption_key);
        let encrypted = encryption.encrypt(data)
            .context("Failed to encrypt data")?;

        // Write to file
        fs::write(&full_path, encrypted)
            .with_context(|| format!("Failed to write file: {}", full_path.display()))?;

        Ok(())
    }

    /// Read and decrypt data from a file
    ///
    /// # Arguments
    ///
    /// * `relative_path` - Path relative to base directory
    /// * `encryption_key` - 256-bit encryption key
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - File does not exist
    /// - File cannot be read
    /// - Decryption fails (wrong key or corrupted data)
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use osnova_lib::storage::FileStorage;
    /// # fn main() -> anyhow::Result<()> {
    /// let storage = FileStorage::new("/tmp/storage")?;
    /// let key = [42u8; 32];
    /// let data = storage.read("config/app.json", &key)?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn read<P: AsRef<Path>>(
        &self,
        relative_path: P,
        encryption_key: &[u8; 32],
    ) -> Result<Vec<u8>> {
        let full_path = self.base_path.join(relative_path.as_ref());

        // Read encrypted file
        let encrypted = fs::read(&full_path)
            .with_context(|| format!("Failed to read file: {}", full_path.display()))?;

        // Decrypt data
        let encryption = CocoonEncryption::new(encryption_key);
        let decrypted = encryption.decrypt(&encrypted)
            .context("Failed to decrypt data")?;

        Ok(decrypted)
    }

    /// Check if a file exists
    ///
    /// # Arguments
    ///
    /// * `relative_path` - Path relative to base directory
    pub fn exists<P: AsRef<Path>>(&self, relative_path: P) -> bool {
        let full_path = self.base_path.join(relative_path.as_ref());
        full_path.exists()
    }

    /// Delete a file
    ///
    /// # Arguments
    ///
    /// * `relative_path` - Path relative to base directory
    ///
    /// # Errors
    ///
    /// Returns an error if the file cannot be deleted
    pub fn delete<P: AsRef<Path>>(&self, relative_path: P) -> Result<bool> {
        let full_path = self.base_path.join(relative_path.as_ref());

        if !full_path.exists() {
            return Ok(false);
        }

        fs::remove_file(&full_path)
            .with_context(|| format!("Failed to delete file: {}", full_path.display()))?;

        Ok(true)
    }

    /// List all files in a directory
    ///
    /// Returns relative paths of all files (recursively) under the given directory.
    ///
    /// # Arguments
    ///
    /// * `relative_path` - Directory path relative to base directory
    ///
    /// # Errors
    ///
    /// Returns an error if the directory cannot be read
    pub fn list_files<P: AsRef<Path>>(&self, relative_path: P) -> Result<Vec<PathBuf>> {
        let full_path = self.base_path.join(relative_path.as_ref());

        if !full_path.exists() {
            return Ok(Vec::new());
        }

        let mut files = Vec::new();
        self.collect_files(&full_path, &self.base_path, &mut files)?;
        Ok(files)
    }

    /// Recursively collect files
    fn collect_files(
        &self,
        dir: &Path,
        base: &Path,
        files: &mut Vec<PathBuf>,
    ) -> Result<()> {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();

            if path.is_dir() {
                self.collect_files(&path, base, files)?;
            } else {
                // Store relative path
                if let Ok(relative) = path.strip_prefix(base) {
                    files.push(relative.to_path_buf());
                }
            }
        }
        Ok(())
    }

    /// Clear all files in a directory
    ///
    /// Recursively removes all files and subdirectories under the given path.
    ///
    /// # Arguments
    ///
    /// * `relative_path` - Directory path relative to base directory
    ///
    /// # Errors
    ///
    /// Returns an error if the directory cannot be removed
    pub fn clear_directory<P: AsRef<Path>>(&self, relative_path: P) -> Result<()> {
        let full_path = self.base_path.join(relative_path.as_ref());

        if full_path.exists() {
            fs::remove_dir_all(&full_path)
                .with_context(|| format!("Failed to clear directory: {}", full_path.display()))?;
        }

        Ok(())
    }

    /// Get the full path for a relative path
    ///
    /// Useful for debugging or integration with other file APIs.
    pub fn full_path<P: AsRef<Path>>(&self, relative_path: P) -> PathBuf {
        self.base_path.join(relative_path.as_ref())
    }

    /// Get the base path
    pub fn base_path(&self) -> &Path {
        &self.base_path
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    fn create_temp_storage() -> Result<(FileStorage, TempDir)> {
        let temp_dir = TempDir::new()?;
        let storage = FileStorage::new(temp_dir.path())?;
        Ok((storage, temp_dir))
    }

    #[test]
    fn test_write_and_read() -> Result<()> {
        let (storage, _temp) = create_temp_storage()?;
        let key = [42u8; 32];
        let data = b"Hello, encrypted world!";

        storage.write("test.dat", data, &key)?;
        let retrieved = storage.read("test.dat", &key)?;

        assert_eq!(retrieved, data);
        Ok(())
    }

    #[test]
    fn test_write_with_subdirectories() -> Result<()> {
        let (storage, _temp) = create_temp_storage()?;
        let key = [99u8; 32];
        let data = b"nested data";

        storage.write("cache/app-001/config.json", data, &key)?;
        let retrieved = storage.read("cache/app-001/config.json", &key)?;

        assert_eq!(retrieved, data);
        Ok(())
    }

    #[test]
    fn test_exists() -> Result<()> {
        let (storage, _temp) = create_temp_storage()?;
        let key = [1u8; 32];

        assert!(!storage.exists("nonexistent.dat"));

        storage.write("existing.dat", b"data", &key)?;
        assert!(storage.exists("existing.dat"));

        Ok(())
    }

    #[test]
    fn test_delete() -> Result<()> {
        let (storage, _temp) = create_temp_storage()?;
        let key = [2u8; 32];

        storage.write("to-delete.dat", b"data", &key)?;
        assert!(storage.exists("to-delete.dat"));

        assert!(storage.delete("to-delete.dat")?);
        assert!(!storage.exists("to-delete.dat"));

        // Deleting again should return false
        assert!(!storage.delete("to-delete.dat")?);

        Ok(())
    }

    #[test]
    fn test_list_files() -> Result<()> {
        let (storage, _temp) = create_temp_storage()?;
        let key = [3u8; 32];

        storage.write("file1.dat", b"data1", &key)?;
        storage.write("subdir/file2.dat", b"data2", &key)?;
        storage.write("subdir/nested/file3.dat", b"data3", &key)?;

        let files = storage.list_files("")?;
        assert_eq!(files.len(), 3);

        // Check that all files are present (order may vary)
        let file_names: Vec<String> = files
            .iter()
            .map(|p| p.to_string_lossy().to_string())
            .collect();

        assert!(file_names.iter().any(|f| f.contains("file1.dat")));
        assert!(file_names.iter().any(|f| f.contains("file2.dat")));
        assert!(file_names.iter().any(|f| f.contains("file3.dat")));

        Ok(())
    }

    #[test]
    fn test_clear_directory() -> Result<()> {
        let (storage, _temp) = create_temp_storage()?;
        let key = [4u8; 32];

        storage.write("cache/file1.dat", b"data1", &key)?;
        storage.write("cache/file2.dat", b"data2", &key)?;
        storage.write("cache/subdir/file3.dat", b"data3", &key)?;

        let files = storage.list_files("cache")?;
        assert_eq!(files.len(), 3);

        storage.clear_directory("cache")?;

        let files = storage.list_files("cache")?;
        assert_eq!(files.len(), 0);

        Ok(())
    }

    #[test]
    fn test_wrong_key_fails() -> Result<()> {
        let (storage, _temp) = create_temp_storage()?;
        let key1 = [5u8; 32];
        let key2 = [6u8; 32];

        storage.write("encrypted.dat", b"secret", &key1)?;

        // Reading with wrong key should fail
        let result = storage.read("encrypted.dat", &key2);
        assert!(result.is_err());

        Ok(())
    }

    #[test]
    fn test_overwrite_file() -> Result<()> {
        let (storage, _temp) = create_temp_storage()?;
        let key = [7u8; 32];

        storage.write("overwrite.dat", b"original", &key)?;
        let data1 = storage.read("overwrite.dat", &key)?;
        assert_eq!(data1, b"original");

        storage.write("overwrite.dat", b"updated", &key)?;
        let data2 = storage.read("overwrite.dat", &key)?;
        assert_eq!(data2, b"updated");

        Ok(())
    }

    #[test]
    fn test_binary_data() -> Result<()> {
        let (storage, _temp) = create_temp_storage()?;
        let key = [8u8; 32];
        let binary_data: Vec<u8> = (0..=255).collect();

        storage.write("binary.dat", &binary_data, &key)?;
        let retrieved = storage.read("binary.dat", &key)?;

        assert_eq!(retrieved, binary_data);
        Ok(())
    }

    #[test]
    fn test_empty_data() -> Result<()> {
        let (storage, _temp) = create_temp_storage()?;
        let key = [9u8; 32];

        storage.write("empty.dat", b"", &key)?;
        let retrieved = storage.read("empty.dat", &key)?;

        assert_eq!(retrieved, b"");
        Ok(())
    }

    #[test]
    fn test_large_data() -> Result<()> {
        let (storage, _temp) = create_temp_storage()?;
        let key = [10u8; 32];
        let large_data = vec![42u8; 1024 * 1024]; // 1 MB

        storage.write("large.dat", &large_data, &key)?;
        let retrieved = storage.read("large.dat", &key)?;

        assert_eq!(retrieved, large_data);
        Ok(())
    }
}
