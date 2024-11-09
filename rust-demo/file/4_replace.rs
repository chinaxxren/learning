use std::fs::{self, File};
use std::io::{self, Write};

struct SafeFileReplacer {
    target_path: String,
}

impl SafeFileReplacer {
    fn new(path: &str) -> Self {
        Self {
            target_path: path.to_string(),
        }
    }

    fn replace_atomic(&self, content: &[u8]) -> io::Result<()> {
        // 创建临时文件
        let temp_path = format!("{}.{}.tmp", self.target_path, "abc");

        // 写入临时文件
        {
            let mut file = File::create(&temp_path)?;
            file.write_all(content)?;
            file.flush()?;
        }

        // 原子替换
        fs::rename(temp_path, &self.target_path)?;
        
        Ok(())
    }
}

fn main() -> io::Result<()> {
    let path = "test.txt";
    let content = b"This is a test content.";

    let replacer = SafeFileReplacer::new(path);
    replacer.replace_atomic(content)?;

    println!("File replaced successfully.");
    Ok(())
}