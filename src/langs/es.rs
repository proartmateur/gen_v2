use std::collections::HashMap;
use std::sync::LazyLock;

pub static ES_STRINGS: LazyLock<HashMap<&str, &str>> = LazyLock::new(|| {
    HashMap::from([
        ("app_info.description", r#"Crea rápidamente tus proyectos una arquitectura propia!"#),
        ("app_info.credits", "Desarrollado por author con ❤️ desde México. year"),
        ("init.already_exists", "ya existe!"),
        ("init.mkdir_error", "Error al crear el directorio"),
        ("init.write_file_error", "No se pudo crear el archivo"),
        ("init.ready_for_init", "Listo para inicializar: init"),
    ])
});