use std::process::Command;

use crate::domain::LicenseError;

pub struct Hwid;

// Obtiene el identificador de hardware asignado al equipo
impl Hwid {
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
     * funciones, pero todas ellas deben funcionar de forma correcta para no
     * romper el mecanismo de obtención del identificador.
     * 
     * Otro aspecto a tener en cuenta es que la modificación en la herramineta 
     * Secenly de la generación del identificador de hardware crea la necesidad 
     * de su modificación en la biblioteca de Secenly, siempre y cuando se haya 
     * optado utilizarla a la hora de generar licencias de software.
     */
    pub fn obtain_hwid() -> Result<String, LicenseError> {
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
            panic!("[ERROR] SO not soported.");
        };


        Ok(String::from_utf8_lossy(&output.stdout).trim().to_string())
    }
}

