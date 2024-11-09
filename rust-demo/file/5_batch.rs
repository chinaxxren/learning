use rayon::prelude::*;
use std::fs::{self, DirBuilder};
use std::path::Path;
use std::io;

struct BatchFileProcessor {
    source_dir: String,
    target_dir: String,
}

impl BatchFileProcessor {
    fn new(source: &str, target: &str) -> Self {
        Self {
            source_dir: source.to_string(),
            target_dir: target.to_string(),
        }
    }

    // 定义一个异步函数，用于处理文件
    fn process_files<F>(&self, processor: F) -> io::Result<()>
    // 定义一个泛型类型 F，它必须实现 Fn(&[u8]) -> Vec<u8> + Send + Sync 这三个 trait
    where
        F: Fn(&[u8]) -> Vec<u8> + Send + Sync,
    {
        // 确保目标目录存在，如果不存在则创建
        DirBuilder::new().recursive(true).create(&self.target_dir)?;

        // 获取源目录下的所有文件和目录
        let entries: Vec<_> = fs::read_dir(&self.source_dir)?
            // 过滤掉读取目录时可能出现的错误
            .filter_map(Result::ok)
            // 将结果收集到一个向量中
            .collect();

        println!("Processing {} files...", entries.len());

        // 使用 rayon 库的 parallel iterator 并行处理文件
        entries.par_iter().try_for_each(|entry| {
            // 获取文件的路径
            let path = entry.path();
            // 如果路径指向的是一个文件
            if path.is_file() {
                // 读取文件的内容
                let content = fs::read(&path)?;
                // 使用传入的处理器函数处理文件内容
                let processed = processor(&content);

                // 构建目标文件的路径
                let target_path = Path::new(&self.target_dir).join(path.file_name().unwrap());
                // 将处理后的内容写入到目标文件中
                fs::write(target_path, processed)?;
            }
            // 返回一个 Result 类型，表示操作是否成功
            Ok(())
        })
    }
}

fn main() {
    let processor = BatchFileProcessor::new("tests", "./rust-demo/file");
    processor.process_files(|content| {
            // 假设我们要对文件内容进行加密处理
            let mut result = Vec::new();
            for byte in content {
                result.push(byte ^ 0x55);
            }
            result
        })
        .unwrap();
}
