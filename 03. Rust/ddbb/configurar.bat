@echo off
REM Script para cambiar entre las diferentes versiones del main

echo 🔧 Configurador de versiones del script Oracle
echo ==============================================
echo.
echo Versiones disponibles:
echo 1. main.rs         - Version con variables de entorno
echo 2. main_simple.rs  - Version con configuracion manual
echo 3. main_interactivo.rs - Version interactiva con menu
echo.

set /p choice="¿Que version quieres usar? [1-3]: "

if "%choice%"=="1" (
    echo ✅ Configurando version con variables de entorno...
    if exist src\main_backup.rs del src\main_backup.rs >nul 2>&1
    copy src\main.rs src\main_backup.rs >nul 2>&1
    echo 🎉 Listo! Usa: cargo run
    echo 💡 Configura las variables de entorno:
    echo    set ORACLE_USER=tu_usuario
    echo    set ORACLE_PASSWORD=tu_password
    echo    set ORACLE_CONNECT_STRING=localhost:1521/XE
) else if "%choice%"=="2" (
    echo ✅ Configurando version simple...
    if exist src\main_backup.rs del src\main_backup.rs >nul 2>&1
    copy src\main.rs src\main_backup.rs >nul 2>&1
    copy src\main_simple.rs src\main.rs >nul 2>&1
    echo 🎉 Listo! Edita src\main.rs y configura las credenciales
    echo 💡 Luego ejecuta: cargo run
) else if "%choice%"=="3" (
    echo ✅ Configurando version interactiva...
    if exist src\main_backup.rs del src\main_backup.rs >nul 2>&1
    copy src\main.rs src\main_backup.rs >nul 2>&1
    copy src\main_interactivo.rs src\main.rs >nul 2>&1
    echo 🎉 Listo! Usa: cargo run
    echo 💡 El script te pedirá las credenciales al ejecutarse
) else (
    echo ❌ Opcion invalida
    exit /b 1
)

echo.
echo 🔄 Para compilar: cargo build
echo 🚀 Para ejecutar: cargo run
pause