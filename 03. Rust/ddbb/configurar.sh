#!/bin/bash

# Script para cambiar entre las diferentes versiones del main

echo "🔧 Configurador de versiones del script Oracle"
echo "=============================================="
echo
echo "Versiones disponibles:"
echo "1. main.rs         - Versión con variables de entorno"
echo "2. main_simple.rs  - Versión con configuración manual"
echo "3. main_interactivo.rs - Versión interactiva con menú"
echo

read -p "¿Qué versión quieres usar? [1-3]: " choice

case $choice in
    1)
        echo "✅ Configurando versión con variables de entorno..."
        cp src/main.rs src/main_backup.rs 2>/dev/null || true
        # Ya está configurada por defecto
        echo "🎉 Listo! Usa: cargo run"
        echo "💡 Configura las variables de entorno:"
        echo "   export ORACLE_USER=\"tu_usuario\""
        echo "   export ORACLE_PASSWORD=\"tu_password\""
        echo "   export ORACLE_CONNECT_STRING=\"localhost:1521/XE\""
        ;;
    2)
        echo "✅ Configurando versión simple..."
        cp src/main.rs src/main_backup.rs 2>/dev/null || true
        cp src/main_simple.rs src/main.rs
        echo "🎉 Listo! Edita src/main.rs y configura las credenciales"
        echo "💡 Luego ejecuta: cargo run"
        ;;
    3)
        echo "✅ Configurando versión interactiva..."
        cp src/main.rs src/main_backup.rs 2>/dev/null || true
        cp src/main_interactivo.rs src/main.rs
        echo "🎉 Listo! Usa: cargo run"
        echo "💡 El script te pedirá las credenciales al ejecutarse"
        ;;
    *)
        echo "❌ Opción inválida"
        exit 1
        ;;
esac

echo
echo "🔄 Para compilar: cargo build"
echo "🚀 Para ejecutar: cargo run"