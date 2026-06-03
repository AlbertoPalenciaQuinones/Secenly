use std::process::Command;

use crate::exceptions::LicenseError;

pub struct HardwareManager {
    pub hardware_id: String,
}

// Obtiene el identificador de hardware asignado al equipo
impl HardwareManager {
    pub fn new() -> Result<Self, LicenseError> {
        // Objeto que se devolverá como resultado al llamar al constructor
        let mut obj = Self {
            hardware_id : String::new(),
        };

        obj.hardware_id = Self::obtain_hwid()?;

        Ok(obj)
    }

    /**
     * Genera el identificador del hardware.
     *
     * El comportamiento actual es la ejecución de un comando.
     *
     * Este comportamiento puede ser modificado por cualquiera que utilice la 
     * herramienta con el fin de que sea ajustable a las necesidades de cada usuario.
     *
     * La función cumple con las siguientes necesidades:
     *     - Generación de identificador único
     *     - Distinto comando para Linux y Windows
     * 
     * Debe saber que puede añadir distintos procedimientos escribiendo nuevas
     * funciones, todas ellas deben funcionar de forma correcta para no
     * romper el mecanismo de obtención del identificador.
     * 
     * Otro aspecto a tener en cuenta es que la modificación en la herramineta 
     * Secenly de la generación del identificador de hardware crea la necesidad 
     * de su modificación en la biblioteca de Secenly, siempre y cuando se haya 
     * optado utilizarla a la hora de generar licencias de software.
     */
    fn obtain_hwid() -> Result<String, LicenseError> {
        let output = if cfg!(target_os = "linux") {
            Command::new("cat")
                .arg("/etc/machine-id")
                .output()?
        } else if cfg!(target_os = "windows") {
            Command::new("powershell")
                .args([
                    "-Command",
                    "(Get-CimInstance Win32_ComputerSystemProduct).UUID",
                ])
                .output()?
        } else {
            return Err(LicenseError::HardwareError {
                msg: "Unsupported operating system".into(),
            });
        };

        // Comprobar si el comando falló
        if !output.status.success() {
            return Err(LicenseError::HardwareError {
                msg : String::from_utf8_lossy(&output.stderr).to_string(),
            });
        }

        // Extraer resultado
        let hwid = String::from_utf8_lossy(&output.stdout)
            .trim()
            .to_string();

        if hwid.is_empty() {
            return Err(LicenseError::HardwareError {
                msg : "Empty hardware ID".into()
            });
        }

        Ok(hwid)

    }

    pub fn get_hardware_id(&self) -> String {
        self.hardware_id.clone()
    }
}

