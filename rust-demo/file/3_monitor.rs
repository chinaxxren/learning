use std::collections::HashMap;
use std::fs::{self, FileType};
use std::time::SystemTime;
use std::io;

struct FileMonitor {
    files: HashMap<String, FileInfo>,
}

struct FileInfo {
    size: u64,
    modified: SystemTime,
    file_type: FileType,
}

impl FileMonitor {
    fn new() -> Self {
        Self {
            files: HashMap::new(),
        }
    }

    fn add_file(&mut self, path: &str) -> io::Result<()> {
        let metadata = fs::metadata(path)?;
        self.files.insert(
            path.to_string(),
            FileInfo {
                size: metadata.len(),
                modified: metadata.modified()?,
                file_type: metadata.file_type(),
            },
        );
        Ok(())
    }

    fn check_changes(&self, path: &str) -> io::Result<Vec<String>> {
        let mut changes = Vec::new();
        if let Some(old_info) = self.files.get(path) {
            let metadata = fs::metadata(path)?;

            if metadata.len() != old_info.size {
                changes.push("文件大小已更改".to_string());
            }

            if metadata.modified()? != old_info.modified {
                changes.push("文件被修改".to_string());
            }

            if metadata.file_type() != old_info.file_type {
                changes.push("文件类型已更改".to_string());
            }
        }
        Ok(changes)
    }
}

fn main() -> io::Result<()> {
    let mut monitor = FileMonitor::new();
    monitor.add_file("test.txt")?;

    let changes = monitor.check_changes("test.txt")?;
    for change in changes {
        println!("{}", change);
    }

    Ok(())
}