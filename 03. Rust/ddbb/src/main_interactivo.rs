use oracle::{Connection, Error};
use std::env;
use std::io::{self, Write};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🏛️  Script de Conexión a Oracle Database");
    println!("==========================================");
    
    // Obtener credenciales
    let (username, password, connect_string) = obtener_credenciales()?;
    
    println!("\n🔄 Conectando a Oracle...");
    
    // Crear conexión
    match Connection::connect(&username, &password, &connect_string) {
        Ok(conn) => {
            println!("✅ ¡Conexión exitosa!");
            mostrar_info_conexion(&username, &connect_string);
            
            // Ejecutar ejemplos
            ejecutar_menu_consultas(&conn)?;
        }
        Err(e) => {
            eprintln!("\n❌ Error de conexión: {:?}", e);
            mostrar_ayuda_troubleshooting();
            return Err(Box::new(e));
        }
    }
    
    Ok(())
}

fn obtener_credenciales() -> Result<(String, String, String), Box<dyn std::error::Error>> {
    // Intentar obtener desde variables de entorno
    if let (Ok(user), Ok(pass), Ok(conn_str)) = (
        env::var("ORACLE_USER"),
        env::var("ORACLE_PASSWORD"),
        env::var("ORACLE_CONNECT_STRING")
    ) {
        println!("📋 Usando credenciales desde variables de entorno");
        return Ok((user, pass, conn_str));
    }
    
    // Si no hay variables de entorno, solicitar al usuario
    println!("📝 Introduce los datos de conexión:");
    
    print!("Usuario: ");
    io::stdout().flush()?;
    let mut username = String::new();
    io::stdin().read_line(&mut username)?;
    let username = username.trim().to_string();
    
    print!("Contraseña: ");
    io::stdout().flush()?;
    let mut password = String::new();
    io::stdin().read_line(&mut password)?;
    let password = password.trim().to_string();
    
    print!("String de conexión [localhost:1521/XE]: ");
    io::stdout().flush()?;
    let mut connect_string = String::new();
    io::stdin().read_line(&mut connect_string)?;
    let connect_string = connect_string.trim();
    let connect_string = if connect_string.is_empty() {
        "localhost:1521/XE".to_string()
    } else {
        connect_string.to_string()
    };
    
    Ok((username, password, connect_string))
}

fn mostrar_info_conexion(username: &str, connect_string: &str) {
    println!("👤 Usuario: {}", username);
    println!("🌐 Servidor: {}", connect_string);
}

fn ejecutar_menu_consultas(conn: &Connection) -> Result<(), Error> {
    loop {
        println!("\n🔍 ¿Qué consulta quieres ejecutar?");
        println!("1. Fecha y hora del servidor");
        println!("2. Versión de Oracle");
        println!("3. Listar tablas del usuario");
        println!("4. Consulta personalizada");
        println!("5. Salir");
        
        print!("\nElige una opción [1-5]: ");
        io::stdout().flush().unwrap();
        
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let choice = input.trim();
        
        match choice {
            "1" => consultar_fecha_hora(conn)?,
            "2" => consultar_version(conn)?,
            "3" => listar_tablas_usuario(conn)?,
            "4" => consulta_personalizada(conn)?,
            "5" => {
                println!("👋 ¡Hasta luego!");
                break;
            },
            _ => println!("❌ Opción inválida. Elige 1-5."),
        }
    }
    
    Ok(())
}

fn consultar_fecha_hora(conn: &Connection) -> Result<(), Error> {
    println!("\n📅 Consultando fecha y hora del servidor...");
    
    let sql = "SELECT TO_CHAR(SYSDATE, 'DD/MM/YYYY HH24:MI:SS') AS fecha FROM DUAL";
    
    let rows = conn.query(sql, &[])?;
    for row_result in rows {
        let row = row_result?;
        let fecha: String = row.get(0)?;
        println!("⏰ {}", fecha);
    }
    
    Ok(())
}

fn consultar_version(conn: &Connection) -> Result<(), Error> {
    println!("\n🔧 Consultando versión de Oracle...");
    
    let sql = "SELECT banner FROM v$version WHERE rownum = 1";
    
    match conn.query(sql, &[]) {
        Ok(rows) => {
            for row_result in rows {
                let row = row_result?;
                let version: String = row.get(0)?;
                println!("📋 {}", version);
            }
        }
        Err(e) => {
            println!("⚠️  No se pudo consultar la versión: {:?}", e);
            println!("💡 Puede que no tengas permisos para acceder a v$version");
        }
    }
    
    Ok(())
}

fn listar_tablas_usuario(conn: &Connection) -> Result<(), Error> {
    println!("\n📊 Listando tablas del usuario...");
    
    let sql = "SELECT table_name FROM user_tables ORDER BY table_name";
    
    match conn.query(sql, &[]) {
        Ok(rows) => {
            let mut count = 0;
            for row_result in rows {
                let row = row_result?;
                let table_name: String = row.get(0)?;
                count += 1;
                println!("📋 {}", table_name);
            }
            
            if count == 0 {
                println!("⚠️  No se encontraron tablas para este usuario");
            } else {
                println!("✅ Total de tablas: {}", count);
            }
        }
        Err(e) => {
            println!("❌ Error listando tablas: {:?}", e);
        }
    }
    
    Ok(())
}

fn consulta_personalizada(conn: &Connection) -> Result<(), Error> {
    println!("\n🚀 Consulta personalizada");
    print!("Introduce tu consulta SQL: ");
    io::stdout().flush().unwrap();
    
    let mut sql = String::new();
    io::stdin().read_line(&mut sql).unwrap();
    let sql = sql.trim();
    
    if sql.is_empty() {
        println!("❌ No se introdujo ninguna consulta");
        return Ok(());
    }
    
    println!("🔄 Ejecutando: {}", sql);
    
    match conn.query(sql, &[]) {
        Ok(rows) => {
            let mut count = 0;
            for row_result in rows {
                match row_result {
                    Ok(row) => {
                        count += 1;
                        // Intentar mostrar la primera columna
                        if let Ok(value) = row.get::<usize, String>(0) {
                            println!("📋 {}", value);
                        } else if let Ok(value) = row.get::<usize, i64>(0) {
                            println!("📋 {}", value);
                        } else {
                            println!("📋 [Fila {}]", count);
                        }
                    }
                    Err(e) => {
                        println!("❌ Error en fila {}: {:?}", count + 1, e);
                        break;
                    }
                }
                
                // Limitar salida para consultas grandes
                if count >= 20 {
                    println!("... (mostrando solo primeras 20 filas)");\n                    break;\n                }\n            }\n            \n            if count == 0 {\n                println!(\"ℹ️  La consulta no devolvió resultados\");\n            } else {\n                println!(\"✅ Consulta ejecutada. Filas procesadas: {}\", count);\n            }\n        }\n        Err(e) => {\n            println!(\"❌ Error ejecutando consulta: {:?}\", e);\n        }\n    }\n    \n    Ok(())\n}\n\nfn mostrar_ayuda_troubleshooting() {\n    println!(\"\\n🆘 Ayuda para resolver problemas:\");\n    println!(\"================================\");\n    println!(\"1. ✅ Verifica que Oracle Database esté ejecutándose\");\n    println!(\"2. ✅ Confirma usuario y contraseña\");\n    println!(\"3. ✅ Revisa el string de conexión (host:puerto/servicio)\");\n    println!(\"4. ✅ Asegúrate de tener Oracle Instant Client instalado\");\n    println!(\"5. ✅ Verifica que el cliente esté en el PATH del sistema\");\n    println!(\"\\n💡 Ejemplos de connection string:\");\n    println!(\"   • Local: localhost:1521/XE\");\n    println!(\"   • Remoto: 192.168.1.100:1521/ORCL\");\n    println!(\"   • Con dominio: servidor.empresa.com:1521/PROD\");\n}"