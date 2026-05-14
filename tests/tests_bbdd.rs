#[cfg(test)]

use secenly::application::{obtain_licenses};

use rusqlite::{Connection};

mod tests_it5 {
    use super::*;

    /* Test para verificar que la inserción de una licencia en la base de datos
       se realiza correctamente. Se crea una base de datos en memoria (exactamente
       igual que la utilizada por la herramienta), se inserta una licencia simulada 
       y se comprueba que el número de registros es el esperado. */
    #[test]
    fn test_insert_license() {
        let conn = Connection::open_in_memory().unwrap();

        // Crear tabla
        conn.execute(
            "CREATE TABLE licenses (
                id INTEGER PRIMARY KEY, 
                hwid TEXT, 
                seed BLOB, 
                license_der BLOB
            )",
            [],
        ).unwrap();

        // Simular datos
        let hwid = "test_hwid";
        let seed: Vec<u8> = vec![1, 2, 3];
        let der: Vec<u8> = vec![4, 5, 6];

        conn.execute(
            "INSERT INTO licenses (hwid, seed, license_der) VALUES (?1, ?2, ?3)",
            rusqlite::params![hwid, seed, der],
        ).unwrap();

        let count: i32 = conn.query_row("SELECT COUNT(*) FROM licenses", [], |r| r.get(0)).unwrap();

        assert_eq!(count, 1);
    }

    
    /* Test para verificar que la eliminación de una licencia en la base de datos
       funciona correctamente. Se inserta previamente una licencia, se elimina
       utilizando su identificador y se comprueba que la base de datos queda vacía. 
       Al igual que en el test anterior, se crea una tabla en memoria. */
    #[test]
    fn test_delete_license() {
        let conn = Connection::open_in_memory().unwrap();

        conn.execute(
            "CREATE TABLE licenses (
                id INTEGER PRIMARY KEY,
                hwid TEXT,
                seed BLOB,
                license_der BLOB
            )",
            [],
        ).unwrap();

        let seed: Vec<u8> = vec![1, 2, 3];
        let der: Vec<u8> = vec![4, 5, 6];

        // Insertar licencia
        conn.execute(
            "INSERT INTO licenses (hwid, seed, license_der) VALUES (?1, ?2, ?3)",
            rusqlite::params!["hwid_test", &seed, &der],
        ).unwrap();

        // Obtener id
        let id: i32 = conn.query_row(
            "SELECT id FROM licenses LIMIT 1",
            [],
            |row| row.get(0),
        ).unwrap();

        // Borrar licencia
        conn.execute("DELETE FROM licenses WHERE id = ?1", [id]).unwrap();

        // Comprobar que no queda ninguna
        let count: i32 = conn.query_row(
            "SELECT COUNT(*) FROM licenses",
            [],
            |row| row.get(0),
        ).unwrap();

        assert_eq!(count, 0);
    }

    
    /* Test para verificar que la actualización de una licencia en la base de datos
       modifica correctamente el contenido asociado sin alterar su identificador.
       Se inserta una licencia inicial, se actualiza su campo binario y se comprueba
       que el identificador permanece constante mientras que el contenido cambia. */
    #[test]
    fn test_update_license() {
        let conn = Connection::open_in_memory().unwrap();

        conn.execute(
            "CREATE TABLE licenses (
                id INTEGER PRIMARY KEY,
                hwid TEXT,
                seed BLOB,
                license_der BLOB
            )",
            [],
        ).unwrap();

        let seed: Vec<u8> = vec![1, 2, 3];
        let der1: Vec<u8> = vec![4, 5, 6];
        let der2: Vec<u8> = vec![7, 8, 9]; 

        // Insertar licencia
        conn.execute(
            "INSERT INTO licenses (hwid, seed, license_der) VALUES (?1, ?2, ?3)",
            rusqlite::params!["hwid_test", &seed, &der1],
        ).unwrap();

        // Obtener id y el primer DER
        let (id, old_der): (i32, Vec<u8>) = conn.query_row(
            "SELECT id, license_der FROM licenses LIMIT 1",
            [],
            |row| Ok((row.get(0)?, row.get(1)?)),
        ).unwrap();

        // Actualizar la licencia con el nuevo DER
        conn.execute(
            "UPDATE licenses SET license_der = ?1 WHERE id = ?2",
            rusqlite::params![&der2, id],
        ).unwrap();

        // Obtener el id y el nuevo DER 
        let (id_after, new_der): (i32, Vec<u8>) = conn.query_row(
            "SELECT id, license_der FROM licenses LIMIT 1",
            [],
            |row| Ok((row.get(0)?, row.get(1)?)),
        ).unwrap();

        // Verificaciones
        assert_eq!(id, id_after);           // ID no cambia
        assert_ne!(old_der, new_der);       // contenido cambia
        assert_eq!(new_der, der2);          // contenido correcto
    }
    
    
    /* Test para verificar que el filtrado de licencias por identificador de hardware
       (HWID) funciona correctamente. Se insertan varias licencias con distintos
       identificadores y se realiza una consulta filtrando por uno de ellos,
       comprobando que únicamente se recuperan las licencias asociadas a dicho HWID. */
    #[test]
    fn test_filter_by_hwid() {
        let conn = Connection::open_in_memory().unwrap();

        conn.execute(
            "CREATE TABLE licenses (
                id INTEGER PRIMARY KEY,
                hwid TEXT,
                seed BLOB,
                license_der BLOB
            )",
            [],
        ).unwrap();

        let seed: Vec<u8> = vec![1, 2, 3];
        let der: Vec<u8> = vec![4, 5, 6];

        // Insertar dos licencias con distinto HWID
        conn.execute(
            "INSERT INTO licenses (hwid, seed, license_der) VALUES (?1, ?2, ?3)",
            rusqlite::params!["hwid_1", &seed, &der],
        ).unwrap();

        conn.execute(
            "INSERT INTO licenses (hwid, seed, license_der) VALUES (?1, ?2, ?3)",
            rusqlite::params!["hwid_2", &seed, &der],
        ).unwrap();

        // Filtrar por hwid
        let licenses = obtain_licenses(&conn, Some(&"hwid_1".to_string())).unwrap();

        assert_eq!(licenses.len(), 1);
    }
}