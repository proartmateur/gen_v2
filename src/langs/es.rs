use std::collections::HashMap;
use std::sync::LazyLock;

pub static ES_STRINGS: LazyLock<HashMap<&str, &str>> = LazyLock::new(|| {
    HashMap::from([
        ("app_info.description", r#"Crea rápidamente tus proyectos una arquitectura propia!"#),
        ("app_info.credits", "Desarrollado por author con ❤️ desde México. year"),
        ("init.already_exists", "ya existe!"),
        ("init.mkdir_error", "Error al crear el directorio"),
        ("init.write_file_error", "No se pudo crear el archivo"),
        ("init.ready_for_init", "Listo para inicializar: arq"),
        ("arq.is_ready", "está listo!"),
        ("arq.writing", "Escribiendo:"),
        ("arq.error_make_atom_dir", "Error: No se pudo crear el directorio atom."),
        ("arq.panic_gen_dir", "Error: No se encontró: .gen_cli/templates.\n Intenta ejecutar: gen init"),
        ("template.reading", "📖 Leyendo template:"),
        ("template.processing", "⚙️  Procesando:"),
        ("template.creating_dir", "📁 Creando directorio:"),
        ("template.writing_file", "✍️  Escribiendo archivo:"),
        ("template.file_exists", "⚠️  El archivo ya existe:"),
        ("template.overwrite_prompt", "¿Deseas sobrescribir? [s=sí, n=no, a=abortar]:"),
        ("template.skipping", "⏭️  Omitiendo:"),
        ("template.aborting", "❌ Operación abortada por el usuario."),
        ("template.success", "✅ Generado exitosamente:"),
        ("template.error_read", "❌ Error al leer template:"),
        ("template.error_write", "❌ Error al escribir archivo:"),
        ("template.error_mkdir", "❌ Error al crear directorio:"),
        ("template.per_prop_gen", "📦 Generando archivos por propiedad:"),
        ("template.imports_added", "📥 Imports agregados:"),
        ("template.completed", "🎉 Generación completada!"),
    ])
});