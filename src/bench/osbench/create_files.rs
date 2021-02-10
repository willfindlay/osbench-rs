use std::fs::{remove_file, File};
use std::io::Write;
use std::path::Path;

use crate::{Error, Result};

/// Creates 65534 files (the maximum allowed in a FAT32 directory) in the
/// directory `root_dir`, populates them with bytes, and then deletes them.
pub fn create_files(root_dir: &Path) -> Result<()> {
    // The maximum number of files in a FAT32 dir
    const MAX_FILES: u32 = 65534;

    // 32 different bytes
    const BUF: &[u8] = &[
        0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24,
        25, 26, 27, 28, 29, 30, 31,
    ];

    // Create and write to files
    for i in 0..MAX_FILES {
        let path = root_dir.join(&format!("{:x}", i));
        // Create file
        let mut f = match File::create(path) {
            Ok(f) => f,
            Err(e) => return Err(Error::FileCreationError(format!("{}", e))),
        };
        // Write to file
        if let Err(e) = f.write(BUF) {
            return Err(Error::FileCreationError(format!("{}", e)));
        }
    }

    // Delete files
    for i in 0..MAX_FILES {
        let path = root_dir.join(&format!("{:x}", i));
        if let Err(e) = remove_file(path) {
            return Err(Error::FileDeletionError(format!("{}", e)));
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_files_test() {
        create_files(Path::new("/tmp")).expect("Failed to create files");
    }
}
