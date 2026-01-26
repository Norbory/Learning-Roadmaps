use oracle::{Connection, Error};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // ⚠️ CONFIGURA ESTOS VALORES SEGÚN TU BASE DE DATOS
    let username = "tu_usuario";           // Cambia por tu usuario de Oracle
    let password = "tu_contraseña";        // Cambia por tu contraseña
    let connect_string = "localhost:1521/XE"; // Cambia por tu string de conexión

    println!("🔄 Intentando conectar a Oracle...");
    
    // Crear conexión
    match Connection::connect(username, password, connect_string) {
        Ok(conn) => {
            println!("✅ ¡Conexión exitosa a Oracle!");
            
            // Ejemplo de consultas
            ejecutar_ejemplos(&conn)?;
            
            println!("✅ Todas las operaciones completadas correctamente");
        }
        Err(e) => {
            eprintln!("❌ Error al conectar a Oracle:");
            eprintln!("   Motivo: {:?}", e);
            eprintln!();
            eprintln!("💡 Verifica:");
            eprintln!("   • Que Oracle esté ejecutándose");
            eprintln!("   • Usuario y contraseña correctos");
            eprintln!("   • String de conexión correcto");
            eprintln!("   • Oracle Instant Client instalado");
            return Err(Box::new(e));
        }
    }
    
    Ok(())
}

fn ejecutar_ejemplos(conn: &Connection) -> Result<(), Error> {
    println!("\n🔍 --- Ejecutando consultas de ejemplo ---");
    
    // 1. Consulta de fecha actual
    consultar_fecha_actual(conn)?;
    
    // 2. Consulta de versión (si es posible)
    consultar_version_oracle(conn)?;
    
    // 3. Ejemplo de consulta personalizada
    // Descomenta la siguiente línea y cambia "EMPLOYEES" por una tabla que exista
    // consultar_tabla(conn, "EMPLOYEES")?;
    
    Ok(())
}

fn consultar_fecha_actual(conn: &Connection) -> Result<(), Error> {
    println!("\n📅 Consultando fecha actual del servidor...");
    
    let sql = "SELECT TO_CHAR(SYSDATE, 'DD/MM/YYYY HH24:MI:SS') FROM DUAL";
    
    let rows = conn.query(sql, &[])?;
    for row_result in rows {
        let row = row_result?;
        let fecha: String = row.get(0)?;
        println!("   Fecha y hora del servidor: {}", fecha);
    }
    
    Ok(())
}

fn consultar_version_oracle(conn: &Connection) -> Result<(), Error> {
    println!("\n🔧 Consultando versión de Oracle...");
    
    let sql = "SELECT banner FROM v$version WHERE rownum = 1";
    
    match conn.query(sql, &[]) {
        Ok(rows) => {
            for row_result in rows {
                let row = row_result?;
                let version: String = row.get(0)?;
                println!("   Versión: {}", version);
            }
        }
        Err(e) => {
            println!("   ⚠️  No se pudo consultar v$version: {:?}", e);
            println!("   (Es normal si no tienes permisos de sistema)");
        }
    }
    
    Ok(())
}

#[allow(dead_code)]
fn consultar_tabla(conn: &Connection, nombre_tabla: &str) -> Result<(), Error> {
    println!("\n📊 Consultando tabla '{}'...", nombre_tabla);
    
    let sql = format!("SELECT COUNT(*) FROM {}", nombre_tabla);
    
    match conn.query(&sql, &[]) {
        Ok(rows) => {
            for row_result in rows {
                let row = row_result?;
                let count: i64 = row.get(0)?;
                println!("   Número de registros: {}", count);
            }
        }
        Err(e) => {
            println!("   ❌ Error consultando tabla '{}': {:?}", nombre_tabla, e);
        }
    }
    
    Ok(())
}

#[allow(dead_code)]
fn ejemplo_consulta_avanzada(conn: &Connection) -> Result<(), Error> {
    println!("\n🚀 Ejemplo de consulta con parámetros...");
    
    let sql = "SELECT table_name FROM user_tables WHERE rownum <= :1";
    
    match conn.query(sql, &[&5]) { // Parámetro: máximo 5 tablas
        Ok(rows) => {
            println!("   Primeras 5 tablas del usuario:");
            for row_result in rows {
                let row = row_result?;
                let table_name: String = row.get(0)?;
                println!("   • {}", table_name);
            }
        }
        Err(e) => {
            println!("   ⚠️  Error en consulta avanzada: {:?}", e);
        }
    }
    
    Ok(())
}