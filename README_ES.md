# Gen CLI Version 2.0 </br>
[![GitHub License][github-license-badge]][github-license-badge-link]


Herramienta para escribir código de manera rápida y adaptada a tu propia arquitectura.

## ¿Cómo lo uso?
0. Instalar en la raíz de tu proyecto
1. Crear tu arquitectura en formato JSON en la raíz de tu proyecto,
2. Crear tus templates.
3. Comenzar a usar.

# Instalación
## Guía de inicio rápido
1. Descarga una versión compilada para tu sistema operativo y pega en la raíz de tu proyecto.
2. Inicializa el CLI
```bash
./gen init
```
Este comando creará un archivo de configuración *"gen_config.json"* y el directorio *".gen_cli/templates"* en la raíz de tu proyecto.

3. Crea tu archivo de arquitectura.
```bash
./gen arq
```
Este comando creará un archivo arq.json con un ejemplo.

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
