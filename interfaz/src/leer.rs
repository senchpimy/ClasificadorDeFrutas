use serde::Deserialize;
use std::fs;

#[derive(Debug, Deserialize, Clone)]
pub struct CsvRow {
    pub R: f64,
    pub G: f64,
    pub B: f64,
    cebolla: f64,
    manzana: f64,
    zanahoria: f64,
    limon: f64,
}

#[derive(Debug, Clone)]
pub struct CsvData {
    pub filename: String,
    pub rows: Vec<CsvRow>,
}

pub fn read_csv_files_from_directory(dir: &str) -> Vec<CsvData> {
    let mut csv_data_collection = Vec::new();

    // Leer todos los archivos del directorio
    let entries = fs::read_dir(dir).expect("No se pudo leer el directorio");

    for entry in entries {
        let entry = entry.expect("No se pudo leer la entrada del directorio");
        let path = entry.path();

        // Verificar si es un archivo .csv
        if path.is_file() && path.extension().and_then(|s| s.to_str()) == Some("csv") {
            let filename = path.file_name().unwrap().to_string_lossy().to_string();
            let mut rdr = csv::Reader::from_path(&path).expect("No se pudo abrir el archivo CSV");

            let rows: Vec<CsvRow> = rdr
                .deserialize()
                .map(|result| result.expect("Error al deserializar una fila"))
                .collect();

            csv_data_collection.push(CsvData { filename, rows });
        }
    }

    csv_data_collection
}
