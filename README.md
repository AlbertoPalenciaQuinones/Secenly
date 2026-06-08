# Secenly
Esta herramienta forma parte del sistema **Secenly** y está diseñada para la gestión completa de licencias de software. A diferencia de la biblioteca, que se centra en la **validación**, la herramienta CLI permite realizar todas las operaciones necesarias sobre las licencias: creación, eliminación, modificación, consulta y exportación.

Además, la herramienta es responsable de generar la estructura de la licencia segura, creando el **ContentInfo**, que contiene el **SignedData**, donde se encuentra la licencia encapsulada. Este proceso se lleva a cabo mediante serialización en formato *ASN.1* y codificación en *DER*, garantizando la integridad y autenticidad de los datos.

# Ejecución
El funcionamiento de la herramienta se realiza mediante la utilidad de *cargo* propia de Rust. Para ejecutar la herramienta, los pasos son los siguientes:

1. Posicionarse en la raíz del proyecto (donde se encuentra el archivo **Cargo.toml**)
2. Ejecutar por la línea de comandos el siguiente comando: **cargo run**

Al ejecutar la herramienta, se mostrará un menú interactivo que permite seleccionar las distintas funcionalidades disponibles, como la generación o lectura de licencias.

Para compilar la herramienta sin ejecutarla, se puede utilizar **cargo build**, mientras que para eliminar los archivos generados durante la compilación se puede ejecutar **cargo clean**.

Para ejecutar los tests, el comando es **cargo test**, el cual ejecuta todas las pruebas unitarias y de integración definidas en el proyecto sin generar un ejecutable final.

# Estructura
La herramienta Secenly está organizada siguiendo una **arquitectura modular**, dividiendo sus componentes en distintos módulos que permiten separar responsabilidades y facilitar el mantenimiento del código.

Los principales módulos del proyecto son:

- **builder** → contiene la implementación del patrón de diseño Builder para la construcción de licencias.
- **domain** → define las estructuras principales del sistema, como la licencia y su representación en ASN.1.
- **services** → incluye la lógica de negocio del sistema, como la generación de identificadores, encapsulación y gestión de hardware.
- **application** → aunque no constituye un módulo como tal, actúa como el núcleo de ejecución del proyecto. Contiene la lógica principal de las distintas funcionalidades de la herramienta y coordina las operaciones entre los distintos componentes del sistema.

Además, existen dos componentes adicionales fuera de los módulos principales del proyecto:
- **input** → gestiona la interacción con el usuario mediante la línea de comandos.
- **exceptions** → define el sistema de gestión de errores centralizado.

El archivo principal (*main.rs*) actúa como **punto de entrada** del programa, encargándose de coordinar las acciones del usuario con los distintos componentes del sistema. Este solo contiene una llamada a la clase application, desde donde se ejecuta todo el flujo de la herramienta.

# Funcionamiento
La herramienta Secenly se encarga de gestionar el ciclo completo de vida de una licencia, desde su generación hasta su almacenamiento y recuperación. El proceso de generación de una licencia sigue los siguientes pasos:

1. Obtención del identificador de hardware (HWID)
2. Generación o lectura de una semilla asociada
3. Creación del identificador de producto
4. Generación del identificador de licencia
5. Construcción de la licencia mediante el patrón Builder
6. Serialización a ASN.1 y codificación en DER
7. Encapsulación y firma mediante CMS
8. Almacenamiento en la base de datos

Además, la herramienta permite:

- Eliminar licencias almacenadas
- Modificar licencias existentes (fecha, heartbeat o notas)
- Consultar licencias asociadas a un hardware
- Exportar licencias y semillas a archivos

De esta forma, la herramienta se encarga de preparar y gestionar las licencias que posteriormente serán utilizadas por la biblioteca para su validación.

# Modificación y personalización
La herramienta está diseñada para ser flexible y adaptable a distintos entornos. Aunque se trata de una herramienta de uso interno dentro del sistema Secenly, su código puede ser modificado para ajustarse a distintas necesidades.

Algunas partes del sistema han sido implementadas de forma configurable, permitiendo modificar aspectos como:

- Obtención del identificador de hardware
- Lógica de generación de identificadores
- Interacción con el usuario

Si se quieren llevar a cabo modificaciones en la herramienta, se recomienda que antes se comprenda el flujo general de ella, leer los comentarios del código fuente e identificar las partes críticas del código. Muchas funciones incluyen comentarios detallados explicando su funcionamiento y sugiriendo posibles modificaciones.

# Componentes personalizables
Las siguientes funciones han sido diseñadas para poder modificarse según las necesidades del usuario:

- `src/builder/director.rs` → `construct_license`
- `src/builder/builder.rs` → `Builder` (Interface)
- `src/builder/license_builder.rs` → `LicenseBuilder` (struct), `set` (añadir o eliminar setter) y `build` 
- `src/domain/license_asn1.rs` → `LicenseAsn1` (struct) y `from` (parser) 
- `src/domain/license.rs` → `new` (constructor) y `from` (parser)
- `src/services/hardware_manager.rs` → `obtain_hwid`
- `src/services/product_manager.rs` → `new` (constructor)
- `src/services/license_manager.rs` → `new` (constructor)
- `src/exceptions.rs` → `LicenseError` (enum), `from` (parser) y `Display`

Otra clase que no se incluye en la lista de componentes personalizables es **application.rs**. En ella se encuentra la lógica principal de la herramienta, coordinando el flujo de ejecución y las distintas funcionalidades disponibles. Aunque no está diseñada específicamente como un componente configurable, puede ser modificada para adaptar el comportamiento de la herramienta a las necesidades del usuario, añadiendo o eliminando funcionalidades existentes.

En cuanto al patrón **Builder** para la creación de licencias, si se quisiera añadir un nuevo campo a la licencia o eliminar uno, habría que modificar tanto las clases del patrón como la clase de la propia licencia y la del parser a ASN.1.

Estas funciones contienen comentarios explicativos directamente en el código, por lo que no se detalla aquí su funcionamiento. Por motivos de seguridad e integridad, se recomienda **no modificar**: procesos de validación criptográfica (CMS / firma), funcionamiento de las principales funcionalidades desarrolladas en la clase principal de la herramienta y estructuras críticas del sistema sin comprender su impacto. 

Además, al modificar la herramienta se debe tener en cuenta que puede romperse la compatibilidad con licencias existentes y que cambios en la generación del ID afectan a todo el sistema. Las modificaciones que más afectarían la compatibilidad con la biblioteca son en la estructura de las licencia, por lo tanto, si se modifica la estructura de las licencias en la herramienta, es de gran necesidad realizar ese cambio en la biblioteca para que maneje todos sus campos nuevos al leerlas.
