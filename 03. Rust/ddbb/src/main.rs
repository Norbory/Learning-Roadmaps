use oracle::{Connection, Error};
use std::env;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Configuración de conexión
    let username = env::var("ORACLE_USER").unwrap_or_else(|_| "tu_usuario".to_string());
    let password = env::var("ORACLE_PASSWORD").unwrap_or_else(|_| "tu_password".to_string());
    let connect_string = env::var("ORACLE_CONNECT_STRING")
        .unwrap_or_else(|_| "localhost:1521/XE".to_string());

    println!("Intentando conectar a Oracle...");
    
    // Crear conexión
    match Connection::connect(&username, &password, &connect_string) {
        Ok(conn) => {
            println!("✅ Conexión exitosa a Oracle!");
            
            // Ejemplo de consulta simple
            ejecutar_consulta_ejemplo(&conn)?;
            
            println!("✅ Operaciones completadas correctamente");
        }
        Err(e) => {
            eprintln!("❌ Error al conectar: {:?}", e);
            return Err(Box::new(e));
        }
    }
    
    Ok(())
}

fn ejecutar_consulta_ejemplo(conn: &Connection) -> Result<(), Error> {
    println!("\n--- Ejecutando consulta de ejemplo ---");
    
    // Consulta para obtener la versión de Oracle
    let sql = "SELECT banner FROM v$version WHERE rownum = 1";
    
    match conn.query(sql, &[]) {
        Ok(rows) => {
            for row_result in rows {
                let row = row_result?;
                let version: String = row.get(0)?;
                println!("Versión de Oracle: {}", version);
            }
        }
        Err(e) => {
            println!("⚠️  No se pudo ejecutar la consulta de versión: {:?}", e);
            
            // Consulta alternativa más básica
            let sql_alt = "SELECT SYSDATE FROM DUAL";
            println!("Intentando consulta alternativa...");
            
            let rows = conn.query(sql_alt, &[])?;
            for row_result in rows {
                let row = row_result?;
                let fecha: String = row.get(0)?;
                println!("Fecha actual del servidor: {}", fecha);
            }
        }
    }
    
    Ok(())
}

#[allow(dead_code)]
fn ejemplo_consulta_personalizada(conn: &Connection, tabla: &str) -> Result<(), Error> {
    println!("\n--- Consultando tabla: {} ---", tabla);
    
    let sql = format!("SELECT COUNT(*) FROM {}", tabla);
    
    let rows = conn.query(&sql, &[])?;
    for row_result in rows {
        let row = row_result?;
        let count: i64 = row.get(0)?;
        println!("Número de registros en {}: {}", tabla, count);
    }
    
    Ok(())
}
