use crate::domain::LicenseError;

use openssl::hash::{hash, MessageDigest};

pub struct LicenseManager {
    pub license_id: String,
}

// Lógica utilizada para generar el identificador de la licencia
impl LicenseManager {
    /**
     * Genera el identificador de la licencia a partir del identificador de producto.
     *
     * El comportamiento actual es una sucesión de hashes + operación XOR.
     *
     * Este comportamiento puede ser modificado por cualquiera que utilice la 
     * herramienta con el fin de que sea ajustable a las necesidades de cada usuario.
     *
     * La función cumple con las siguientes necesidades:
     *     - Aportar integridad
     *     - Ofuscar la integridad
     *     - Validar  el identificador licencia
     * 
     * Debe saber que puede añadir distintos procedimientos escribiendo nuevas
     * funciones, pero todas ellas deben funcionar de forma correcta para no
     * romper el mecanismo de validación de identificador.
     * 
     * Otro aspecto a tener en cuenta es que la modificación en Secenly de la
     * generación del identificador de licencia crea la necesidad de su
     * modificación en la biblioteca de Secenly siempre y cuando se opte por
     * utilizarla a la hora de validar licencias en el software propietario.
     */
    pub fn new(product_id: &String) -> Result<Self, LicenseError> {
        // Objeto que se devolverá como resultado al llamar al constructor
        let mut obj = Self {
            license_id : String::new(),
        };

        let bytes = product_id.as_bytes();
        let length = bytes.len();
        // Vector para almacenar los hashes por fragmentos
        let mut hashes: Vec<Vec<u8>> = Vec::new();

        // Se generan 4 hashes de fragmentos del identificador de producto
        for i in 0..4 {
            let start = i * length / 4;
            let end = (i + 1) * length / 4;
            
            let fragment = &bytes[start..end];

            // Generación y almacenamiento del hash
            let hash = Self::hash(fragment)?;
            hashes.push(hash);
        }

        let mut mixed = hashes[0].clone();

        // Recorrer los hashes almacenados saltando el primero y realizar XOR
        // de forma iterativa
        for h in hashes.iter().skip(1) {
            mixed = Self::xor_buffers(&mixed, h);
        }

        // Convertir a hexadecimal para almacenarlo en license_id
        obj.license_id = hex::encode(mixed);

        Ok(obj)
    }

    // Generación del hash de los fragmentos del identificador de producto
    fn hash(fragment: &[u8]) -> Result<Vec<u8>, LicenseError> {
        let digest = hash(MessageDigest::sha512(), fragment).map_err(|e| LicenseError::InvalidPrivateKey {
            msg: "Something wrong ocurred calculating hash".into(),
            source: Some(e),
        })?;

        Ok(digest.to_vec())
    }

    // Cálculo de operación XOR con 2 fragmentos del identificador de producto
    fn xor_buffers(a: &[u8], b: &[u8]) -> Vec<u8> {
        a.iter()
            .zip(b.iter())
            .map(|(x, y)| x ^ y)
            .collect()
    }

    pub fn get_license_id(&self) -> &String {
        &self.license_id
    }
}