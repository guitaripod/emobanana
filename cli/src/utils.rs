use base64::Engine;
use std::fs;
use std::path::Path;

use crate::error::{EmobananaError, Result};

pub fn load_image_as_base64(image_path: &str) -> Result<String> {
    if !Path::new(image_path).exists() {
        return Err(EmobananaError::FileNotFound(image_path.to_string()));
    }

    let image_data = fs::read(image_path)?;
    let base64_image = base64::engine::general_purpose::STANDARD.encode(image_data);
    Ok(format!("data:image/png;base64,{}", base64_image))
}

pub fn save_base64_image(base64_data: &str, output_path: &str) -> Result<()> {
    let image_data = decode_base64_image(base64_data)?;
    fs::write(output_path, image_data)?;
    Ok(())
}

pub fn decode_base64_image(base64_data: &str) -> Result<Vec<u8>> {
    let base64_data = if base64_data.starts_with("data:") {
        let parts: Vec<&str> = base64_data.split(',').collect();
        if parts.len() == 2 {
            parts[1]
        } else {
            base64_data
        }
    } else {
        base64_data
    };

    Ok(base64::engine::general_purpose::STANDARD.decode(base64_data)?)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::NamedTempFile;

    #[test]
    fn test_decode_base64_image_without_data_prefix() {
        let test_data = b"Hello, World!";
        let base64_string = base64::engine::general_purpose::STANDARD.encode(test_data);
        let result = decode_base64_image(&base64_string).unwrap();
        assert_eq!(result, test_data);
    }

    #[test]
    fn test_decode_base64_image_with_data_prefix() {
        let test_data = b"Hello, World!";
        let base64_string = base64::engine::general_purpose::STANDARD.encode(test_data);
        let data_url = format!("data:image/png;base64,{}", base64_string);
        let result = decode_base64_image(&data_url).unwrap();
        assert_eq!(result, test_data);
    }

    #[test]
    fn test_load_image_as_base64_file_not_found() {
        let result = load_image_as_base64("nonexistent_file.png");
        assert!(matches!(result, Err(EmobananaError::FileNotFound(_))));
    }

    #[test]
    fn test_save_and_load_base64_image() {
        let test_data = b"Test image data";
        let base64_string = base64::engine::general_purpose::STANDARD.encode(test_data);

        let temp_file = NamedTempFile::new().unwrap();
        let temp_path = temp_file.path().to_str().unwrap();

        save_base64_image(&base64_string, temp_path).unwrap();
        let loaded_data = fs::read(temp_path).unwrap();
        assert_eq!(loaded_data, test_data);
    }
}