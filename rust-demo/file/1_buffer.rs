use std::fs::{self, File};
use std::io::{self, BufReader, BufWriter, Read, Write};

struct SmartFileHandler {
    path: String,
}

impl SmartFileHandler {
    fn new(path: &str) -> Self {
        Self {
            path: path.to_string(),
        }
    }

    fn read_to_string(&self) -> io::Result<String> {
        let file = File::open(&self.path)?;
        let mut reader = BufReader::new(file);
        let mut content = String::new();
        reader.read_to_string(&mut content)?;
        Ok(content)
    }

    fn write_string(&self, content: &str) -> io::Result<()> {
        let file = File::create(&self.path)?;
        let mut writer = BufWriter::new(file);
        writer.write_all(content.as_bytes())?;
        writer.flush()?;
        Ok(())
    }

    fn append_string(&self, content: &str) -> io::Result<()> {
        let file = fs::OpenOptions::new()
            .append(true)
            .create(true)
            .open(&self.path)?;
        let mut writer = BufWriter::new(file);
        writer.write_all(content.as_bytes())?;
        writer.flush()?;
        Ok(())
    }
}

fn main() {
    let handler = SmartFileHandler::new("test.txt");
    handler.write_string("Hello, world!").unwrap();
    handler.append_string("\nHello, world!").unwrap();
    let content = handler.read_to_string().unwrap();
    println!("{}", content);
}
