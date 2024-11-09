use std::fs::{self, DirEntry};
use std::io;
use std::path::Path;

fn visit_dirs(dir: &Path, cb: &dyn Fn(&DirEntry)) -> io::Result<()> {
    if dir.is_dir() {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                visit_dirs(&path, cb)?;
            } else {
                cb(&entry);
            }
        }
    }
    Ok(())
}

// 使用示例
fn main() -> io::Result<()> {
    let path = Path::new("./rust-demo");
    visit_dirs(&path, &|entry| {
        println!("Found file: {}", entry.path().display());
    })?;
    
    Ok(())
}
