use std::fs;
use std::fs::File;
use std::io::Write;

pub struct ProductIdentifier {
    pub product_id: String,
    seed: Vec<u8>,
    path: std::path::PathBuf,
}

impl ProductIdentifier {
    pub fn new(path: std::path::PathBuf, hwid: String) -> Self {
        let mut obj = Self {
            product_id: String::new(),
            path: path.join("seed"),
            seed: Vec::new(),
        };

        if !obj.has_seed() {
            obj.create_seed();
        }

        obj.seed = obj.read_seed_bytes();

        if !obj.seed.is_empty() {
            obj.set_product_id(hwid);
        }

        obj
    }

    pub fn has_seed(&self) -> bool {
        if !self.path.exists() || !self.path.is_dir() {
            let _ = fs::create_dir(&self.path);
        }

        if let Ok(entries) = fs::read_dir(&self.path) {
            for entry in entries.flatten() {
                let entry_path = entry.path();
                if entry_path.is_file() {
                    return true;
                }
            }
        }
        false
    }

    pub fn create_seed(&self) -> bool {
        if self.has_seed() {
            return true
        }

        let seed_file = self.path.join("seed.dat");
        let mut file = File::create(seed_file).expect("Error creating seed file");
    
        // 64 valores de 4 bytes = 256 bytes
        for _ in 0..64 {
            let value = rand::random::<u32>();
            if file.write_all(&value.to_le_bytes()).is_err() {
                return false;
            }
        }
        true
    }

    pub fn read_seed_bytes(&self) -> Vec<u8> {
        let dir = &self.path;

        // Si no existe o no es directorio → devolver vacío
        if !dir.exists() || !dir.is_dir() {
            return vec![];
        }

        // Buscar archivos dentro
        if let Ok(entries) = fs::read_dir(dir) {
            for entry in entries.flatten() {
                let path = entry.path();

                if path.is_file() {
                    // Leer todo el archivo como bytes
                    return fs::read(path).unwrap_or_else(|_| vec![]);
                }
            }
        }
        // No había archivo
        vec![]
    }

    pub fn set_product_id(&mut self, hwid: String) {
        let mut seed_string = hwid;

        for byte in &self.seed {
            seed_string += &format!("{}", *byte as u32);
        }

        self.product_id = seed_string;
    }

    pub fn get_product_id(&self) -> String {
        self.product_id.clone()
    }
}