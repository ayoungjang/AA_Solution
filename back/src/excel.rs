use openxlsx::reader::ExcelReader;
use std::fs::File;

pub fn read_excel(file_path: &str) -> Result<Vec<Vec<String>>, String> {
    let file = File::open(file_path).map_err(|e| e.to_string())?;
    let mut reader = ExcelReader::new(file);
    let mut data = Vec::new();

    while let Some(row) = reader.next_row() {
        data.push(row);
    }

    Ok(data)
}
