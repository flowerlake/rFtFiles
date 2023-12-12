pub fn read_files(){
    let file_path = "C:\\Users\\Snkpty\\Downloads\\nmaps\\2023-12-03.txt";
    match get_file_content(file_path) {
        Ok(result) => {
            println!("File content in binary: {:?}", result);
        }
        Err(err) => {
            eprintln!("Error reading file: {}", err);
        }
    }

}