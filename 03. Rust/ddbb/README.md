# 🏛️ Script de Conexión a Oracle Database en Rust

Este proyecto contiene **3 versiones diferentes** de un script en Rust para conectarse a bases de datos Oracle, desde básico hasta interactivo.

## 📋 Requisitos Previos

1. **Oracle Instant Client**: Necesitas tener instalado Oracle Instant Client en tu sistema

   - Descarga desde: <https://www.oracle.com/database/technologies/instant-client.html>
   - Asegúrate de que esté en tu PATH

2. **Rust**: Versión 1.70 o superior

## 🚀 Instalación Rápida

```bash
# Clonar o descargar el proyecto
cd ddbb

# Compilar dependencias
cargo build

# Ejecutar (con la versión por defecto)
cargo run
```

## 📝 Versiones Disponibles

### 1. 🔐 Versión con Variables de Entorno (`main.rs` - por defecto)

Usa variables de entorno para las credenciales:

```bash
# En Windows
set ORACLE_USER=tu_usuario
set ORACLE_PASSWORD=tu_contraseña
set ORACLE_CONNECT_STRING=localhost:1521/XE

# En Linux/Mac
export ORACLE_USER="tu_usuario"
export ORACLE_PASSWORD="tu_contraseña"
export ORACLE_CONNECT_STRING="localhost:1521/XE"

cargo run
```

### 2. ⚙️ Versión Simple (`main_simple.rs`)

Configuración directa en el código. Ideal para pruebas rápidas:

```rust
let username = "tu_usuario";
let password = "tu_contraseña"; 
let connect_string = "localhost:1521/XE";
```

### 3. 🎯 Versión Interactiva (`main_interactivo.rs`)

Menú interactivo que te permite:
- Introducir credenciales al momento de ejecución
- Elegir qué consultas ejecutar
- Realizar consultas personalizadas

## 🔄 Cambiar Entre Versiones

### En Windows:

```cmd
configurar.bat
```

### En Linux/Mac:

```bash
chmod +x configurar.sh
./configurar.sh
```

### Manual:

```bash
# Para versión simple
cp src/main_simple.rs src/main.rs

# Para versión interactiva  
cp src/main_interactivo.rs src/main.rs

# Restaurar versión original
cp src/main_backup.rs src/main.rs  # si existe backup
```

## 🔧 Ejemplos de Connection String

- **Local**: `localhost:1521/XE`
- **Remoto**: `192.168.1.100:1521/ORCL`
- **Con servicio**: `servidor.dominio.com:1521/servicio`
- **Con TNS**: `(DESCRIPTION=(ADDRESS=(PROTOCOL=TCP)(HOST=localhost)(PORT=1521))(CONNECT_DATA=(SERVICE_NAME=XE)))`

## ✨ Funcionalidades Incluidas

- ✅ Conexión básica a Oracle
- ✅ Consulta de fecha y hora del servidor
- ✅ Consulta de versión de Oracle
- ✅ Listado de tablas del usuario
- ✅ Consultas personalizadas con parámetros
- ✅ Manejo robusto de errores
- ✅ Mensajes informativos con emojis
- ✅ Múltiples formas de configuración

## 🔍 Consultas de Ejemplo

```rust
// Fecha actual
"SELECT TO_CHAR(SYSDATE, 'DD/MM/YYYY HH24:MI:SS') FROM DUAL"

// Versión de Oracle
"SELECT banner FROM v$version WHERE rownum = 1"

// Contar registros en tabla
"SELECT COUNT(*) FROM nombre_tabla"

// Listar tablas del usuario
"SELECT table_name FROM user_tables ORDER BY table_name"
```

## 🆘 Troubleshooting

| Problema | Solución |
|----------|----------|
| **Error de conexión** | Verifica que Oracle esté ejecutándose y los parámetros sean correctos |
| **Error de cliente** | Asegúrate de tener Oracle Instant Client instalado y en PATH |
| **Error de permisos** | Verifica que el usuario tenga permisos para conectarse |
| **"TNS: could not resolve"** | Revisa el connection string o archivo tnsnames.ora |
| **"ORA-12541"** | Oracle no está ejecutándose en el host/puerto especificado |

## 🛠️ Comandos Útiles

```bash
# Compilar sin ejecutar
cargo check

# Compilar y ejecutar
cargo run

# Compilar en modo release (optimizado)
cargo build --release

# Ejecutar versión optimizada
cargo run --release

# Ver ayuda de cargo
cargo --help
```

## 📚 Estructura del Proyecto

```
ddbb/
├── src/
│   ├── main.rs              # Versión con variables de entorno
│   ├── main_simple.rs       # Versión con configuración manual
│   └── main_interactivo.rs  # Versión interactiva
├── Cargo.toml               # Configuración y dependencias
├── configurar.bat           # Script de configuración (Windows)
├── configurar.sh            # Script de configuración (Unix)
└── README.md               # Esta documentación
```

## 🤝 Contribuir

¡Las contribuciones son bienvenidas! Puedes:
- Agregar nuevos ejemplos de consultas
- Mejorar el manejo de errores
- Optimizar la conexión
- Agregar nuevas funcionalidades

## 📄 Dependencias

- `oracle = "0.5"` - Cliente Oracle para Rust
- `anyhow = "1.0"` - Manejo mejorado de errores