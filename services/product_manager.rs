use std::fs;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;

use crate::domain::LicenseError;

pub struct ProductIdentifier {
    pub product_id: String,
    seed: Vec<u8>,
    path: std::path::PathBuf,
}

// Clase para manejar la semilla y generar el identificador de producto
impl ProductIdentifier {
    /**
     * Inicializador de la semilla y el identificador de producto.
     *
     * Genera un archivo de semilla aleatorio si no existe en la ruta oportuna. El
     * identificador de producto es una concatenación de hwid + semilla. No se hace 
     * dentro del constructor ya que pueda fallar debido a la gran carga de trabajo.
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
    pub fn initialize(path: &PathBuf, hwid: String) -> Result<Self, LicenseError> {
        // Objeto que se devolverá como resultado al llamar al constructor
        let mut obj = Self {
            product_id: String::new(),
            path: path.join("seed"),
            seed: Vec::new(),
        };

        // Si no hay semilla, se genera
        if !obj.has_seed()? {
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
    pub fn has_seed(&self) -> Result<bool, LicenseError> {
        // Comprobación de que se encuentra el archivo en la ruta de la semilla
        if !self.path.exists() || !self.path.is_dir() {
            return Ok(false);
        }

        for entry in fs::read_dir(&self.path)? {
            let entry = entry?;
            let path = entry.path();

            // Si se ha encontrado el archivo de semilla, este existe
            if path.is_file() {
                return Ok(true);
            }
        }

        Ok(false)
    }

    // Creación del archivo de semilla aleatorio
    pub fn create_seed(&self) -> Result<(), LicenseError> {
        // Creación del archivo de semilla
        let mut file = File::create(self.path.join("seed.dat"))?;

        fs::create_dir_all(&self.path)?;
    
        // 64 valores de 4 bytes = 256 bytes
        for _ in 0..64 {
            let value = rand::random::<u32>();
            file.write_all(&value.to_le_bytes())?;
        }

        Ok(())
    }

    // Lectura del archivo de semilla
    pub fn read_seed_bytes(&self) -> Result<Vec<u8>, LicenseError> {
        // Buscar archivos dentro de la ruta
        for entry in fs::read_dir(&self.path)? {
            let path = entry?.path();

            if path.is_file() {
                // Leer todo el archivo como bytes
                return Ok(fs::read(path)?)
            }
        }
        
        // No había archivo
        Ok(vec![])
    }

    pub fn set_product_id(&mut self, hwid: String) {
        let mut id = hwid;

        // Por cada byte de la semilla, se concatena al identificador de hardware
        for byte in &self.seed {
            id += &format!("{}", *byte as u32);
        }

        self.product_id = id;
    }

    pub fn get_product_id(&self) -> String {
        self.product_id.clone()
    }
}