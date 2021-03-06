use std::error::Error;
use std::fs::File;
use std::io::Read;

pub fn read_file(path: &str) -> Result<String, Box<dyn Error>> {
    let mut file = File::open(&path)?;
    let file_len = file.metadata()?.len();
    let mut file_contents = Vec::with_capacity(file_len as usize + 1);
    file.read_to_end(&mut file_contents)?;

    Ok(String::from_utf8(file_contents)?)
}

#[cfg(test)]
mod tests {
    use std::fs::remove_file;
    use std::io::Write;

    use super::*;

    fn create_and_populate_file(text: &str, path: &str) -> Result<(), std::io::Error> {
        let mut buffer = File::create(path)?;
        buffer.write_all(text.as_bytes())?;

        Ok(())
    }

    #[test]
    fn read_file_works_with_correct_path() {
        let path = "test_file.txt";
        let text = "Hello world!";

        create_and_populate_file(text, path).unwrap();

        let file_text = read_file(path).unwrap();

        remove_file(path).unwrap();

        assert_eq!(file_text, text);
    }

    #[test]
    fn read_file_error_with_invalid_path() {
        let path = "test_file_invalid.txt";

        let result = read_file(path);

        assert!(result.is_err());
    }
}
