use oracle::{Connection, Error};
use dotenv::dotenv;
use std::env;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Cargar variables desde archivo .env
    dotenv().ok();
    
    println!("🏛️  Script de Conexión a Oracle Database");
    println!("==========================================");
    
    // Obtener credenciales (primero desde .env, luego desde variables de entorno)
    let username = env::var("ORACLE_USER")
        .map_err(|_| "❌ Variable ORACLE_USER no encontrada en .env o variables de entorno")?;
    let password = env::var("ORACLE_PASSWORD")
        .map_err(|_| "❌ Variable ORACLE_PASSWORD no encontrada en .env o variables de entorno")?;
    let connect_string = env::var("ORACLE_CONNECT_STRING")
        .map_err(|_| "❌ Variable ORACLE_CONNECT_STRING no encontrada en .env o variables de entorno")?;

    println!("📋 Configuración cargada desde archivo .env");
    println!("👤 Usuario: {}", username);
    println!("🌐 Servidor: {}", connect_string);
    println!();
    
    println!("🔄 Conectando a Oracle...");
    
    // Crear conexión
    match Connection::connect(&username, &password, &connect_string) {
        Ok(conn) => {
            println!("✅ ¡Conexión exitosa a Oracle!");
            
            // Ejecutar consultas de ejemplo
            ejecutar_consultas_ejemplo(&conn)?;
            
            println!("✅ Operaciones completadas correctamente");
        }
        Err(e) => {
            eprintln!("❌ Error al conectar: {:?}", e);
            mostrar_ayuda_configuracion();
            return Err(Box::new(e));
        }
    }
    
    Ok(())
}

fn ejecutar_consultas_ejemplo(conn: &Connection) -> Result<(), Error> {
    println!("\n🔍 --- Ejecutando consultas de ejemplo ---");
    
    // 1. Fecha actual
    consultar_fecha_actual(conn)?;
    
    // 2. Versión de Oracle
    consultar_version_oracle(conn)?;
    
    Ok(())
}

fn consultar_fecha_actual(conn: &Connection) -> Result<(), Error> {
    println!("\n📅 Consultando fecha actual del servidor...");
    
    let sql = "SELECT TO_CHAR(SYSDATE, 'DD/MM/YYYY HH24:MI:SS') FROM DUAL";
    
    let rows = conn.query(sql, &[])?;
    for row_result in rows {
        let row = row_result?;
        let fecha: String = row.get(0)?;
        println!("   ⏰ {}", fecha);
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
                println!("   📋 {}", version);
            }
        }
        Err(e) => {
            println!("   ⚠️  No se pudo consultar v$version: {:?}", e);
            println!("   💡 Es normal si no tienes permisos de sistema");
        }
    }
    
    Ok(())
}

fn mostrar_ayuda_configuracion() {
    println!("\n🆘 Ayuda de Configuración:");
    println!("==========================");
    println!("1. ✅ Crea un archivo .env en la carpeta del proyecto");
    println!("2. ✅ Agrega las siguientes líneas al archivo .env:");
    println!("   ORACLE_USER=tu_usuario");
    println!("   ORACLE_PASSWORD=tu_contraseña");
    println!("   ORACLE_CONNECT_STRING=localhost:1521/XE");
    println!("3. ✅ Guarda el archivo y vuelve a ejecutar");
    println!();
    println!("💡 También puedes usar variables de entorno del sistema");
    println!("🔒 El archivo .env está en .gitignore para mayor seguridad");
}