use std::fs::File;
use std::io::{Read, Write};
use std::path::Path;
use std::result;

pub type Result<T> = result::Result<T, std::io::Error>;

pub fn read_text(file_path: &str) {
    match get_file_content(file_path) {
        Ok(result) => {
            println!("File content in binary: {:?}", result);
        }
        Err(err) => {
            eprintln!("Error reading file: {}", err);
        }
    }
}

fn get_file_content(filepath: &str) -> Result<Vec<u8>> {
    let mut file = File::open(filepath)?;

    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)?;

    Ok(buffer)
}

pub fn upload_file(
    store_name: &str,
    store_dir: &str,
    content: Vec<u8>,
    overwrite: bool,
) -> Result<()> {
    let path = Path::new(store_dir).join(store_name);
    if path.exists() && !overwrite {
        // If the file already exists and overwrite is false, return early with Ok(())
        return Ok(());
    }
    let file = File::create(path);

    match file {
        Ok(mut fi) => {
            fi.write_all(content.as_slice()).unwrap();
            println!("upload file success");
            Ok(())
        }
        Err(e) => {
            return Err(e);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let re = get_file_content("E:\\rust_projects\\r_fet\\CONTRIBUTE.md").unwrap();
        let _upload_flag =
            upload_file("contribute_test.txt", "E:\\rust_projects\\r_fet", re, true).unwrap();
    }
}
