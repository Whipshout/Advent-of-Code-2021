use std::fs::File;
use std::io::Read;

pub fn read_file(path: &str) -> Result<String, std::io::Error> {
    let mut file = File::open(&path)?;
    let file_len = file.metadata()?.len();
    let mut file_contents = Vec::with_capacity(file_len as usize + 1);
    file.read_to_end(&mut file_contents)?;

    Ok(String::from_utf8(file_contents).expect("Error parsing file content"))
}
