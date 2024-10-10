# Gen CLI Version 2.0 </br>
[![GitHub License][github-license-badge]][github-license-badge-link]


Herramienta para escribir código de manera rápida y adaptada a tu propia arquitectura.

## ¿Cómo lo uso?
0. Instalar en la raíz de tu proyecto
1. Crear tu arquitectura en formato JSON en la raíz de tu proyecto,
2. Crear tus templates.
3. Comenzar a usar.

# Instalación
## Instalación rápida
1. Descarga una versión compilada para tu sistema operativo y pega en la raíz de tu proyecto .
2. Crea un archivo "gen_config.json" en la raíz de tu proyecto.
3. Copia y pega dentro de "gen_config.json"

```json
{
    "headers_doc": false,
    "templates_root": "",
    "arq_file":"arq.json"
}
```
4. Especifica la ruta absoluta donde deseas guardar tus templates dentro de la clave "templates_root".

Ejemplo:
```json
{
    "headers_doc": false,
    "templates_root": "/Users/MYSELF/Devs/MY_PROJECT/.gen_cli/templates",
    "arq_file":"arq.json"
}
```
Recomendación: En la raíz de tu proyecto crea una carpeta .gen_cli y adentro crea una carpeta templates.


## Para quien desee contribuir

### Tech Stack

Para modificar el CLI necesitas instalar:

- Rust v1.81.0+

**Run**

```bash
cargo run
```

**Build Debug**

```bash
cargo build
```

**Build Release**

```bash
cargo build --release
```

[github-license-badge]: https://img.shields.io/badge/licence-apache_2.0-0582A3?style=for-the-badge

[github-license-badge-link]: https://github.com/proartmateur/gen_v2/blob/main/LICENCE
