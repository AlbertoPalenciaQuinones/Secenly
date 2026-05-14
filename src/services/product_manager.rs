use std::fs::{File, self};
use std::io::Write;
use std::path::PathBuf;

use crate::domain::LicenseError;

pub struct ProductManager {
    pub product_id: String,
    seed: Vec<u8>,
    path: PathBuf,
}

// Clase para manejar la semilla y generar el identificador de producto
impl ProductManager {
    /**
     * Inicializador de la semilla y el identificador de producto.
     *
     * Genera un archivo de semilla aleatorio si no existe en la ruta oportuna. El
     * identificador de producto es una concatenación de hwid + semilla.
     *
     * Este comportamiento puede ser modificado por cualquiera que utilice la 
     * herramienta con el fin de que sea ajustable a las necesidades de cada usuario.
     *
     * La función cumple con las siguientes necesidades:
     *     - Verificar que existe archivo de semilla
     *     - Generarlo si no existe (aleatoriamente)
     *     - Generar identificador de producto
     * 
     * Debe saber que puede añadir distintos procedimientos escribiendo nuevas
     * funciones, pero todas ellas deben funcionar de forma correcta para no
     * romper el mecanismo de generación de identificador de producto.
     * 
     * Otro aspecto a tener en cuenta es que la modificación en Secenly de la
     * generación del identificador de producto crea la necesidad de su
     * modificación en la biblioteca de Secenly siempre y cuando se opte por
     * utilizarla a la hora de validar licencias en el software propietario.
     */
    pub fn new(path: &PathBuf, hwid: String) -> Result<Self, LicenseError> {
        // Objeto que se devolverá como resultado al llamar al constructor
        let mut obj = Self {
            product_id: String::new(),
            path: path.clone(),
            seed: Vec::new(),
        };

        // Si no hay semilla, se genera
        if !obj.has_seed() {
            obj.create_seed()?;
        }

        
        obj.seed = obj.read_seed_bytes()?;

        // Si se ha rellenado correctamente la semilla, se genera el id de producto
        if !obj.seed.is_empty() {
            obj.set_product_id(hwid);
        }

        Ok(obj)
    }

    // Comprobación de que existe la semilla (ya se ha generado)
    pub fn has_seed(&self) -> bool {
        self.path.is_file()
    }

    // Creación del archivo de semilla aleatorio
    pub fn create_seed(&self) -> Result<(), LicenseError> {
        // Creación del archivo de semilla
        let mut file = File::create(&self.path.join("seed.dat"))
            .map_err(|e| LicenseError::IoWithContext {
                source: e,
                path: self.path.display().to_string()
            })?;

        // 64 valores de 4 bytes = 256 bytes
        for _ in 0..64 {
            let value = rand::random::<u32>();
            file.write_all(&value.to_le_bytes())?;
        }

        Ok(())
    }

    // Lectura del archivo de semilla
    pub fn read_seed_bytes(&self) -> Result<Vec<u8>, LicenseError> {
        let seed_file = fs::read(&self.path)
            .map_err(|e| LicenseError::IoWithContext {
                source: e,
                path: self.path.display().to_string()
            })?;

        return Ok(seed_file);
    }


    pub fn set_product_id(&mut self, hwid: String) {
        let mut id = hwid;

        // Por cada byte de la semilla, se concatena al identificador de hardware
        for byte in &self.seed {
            id += &format!("{}", *byte as u32);
        }

        self.product_id = id;
    }

    pub fn get_product_id(&self) -> &String {
        &self.product_id
    }

    pub fn get_seed(&self) -> &[u8] {
        &self.seed
    }
}