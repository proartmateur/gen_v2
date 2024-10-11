# Gen CLI Version 2.0 </br>
[![GitHub License][github-license-badge]][github-license-badge-link]


Herramienta para escribir código de manera rápida y adaptada a tu propia arquitectura.

## ¿Cómo lo uso?
0. Instalar en la raíz de tu proyecto
1. Crear tu arquitectura en formato JSON en la raíz de tu proyecto,
2. Crear tus templates.
3. Comenzar a usar.


# Guía de inicio rápido
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
Este comando creará un archivo *"arq.json"* con un ejemplo y su respectivo template en *"./.gen_cli/templates/component.tsx"*

Elmotivo de este ejemplo es explicar cómo estructurar una arquitectura y sus templates.

4. Comenzar a utilizar

Si todo salió bien, podemos crear un nuevo átomo de la siguiente forma:

```bash
./gen -a my_atom
```

o también de la siguiente manera:

```bash
./gen --atom my_atom
```

Esto va a crear una carpeta atom dentro de *"./src"* en caso de no existir. Dentro de ella va a crear un archivo: *"MyAtom.tsx"*

# Definir tu arquitectura
## ¿Qué es arq.json?

Es el archivo que te permitirá modelar tu  **arq**uitectura. Su nombre proviene del español: **arq**uitectura.

## Estructura de arq.json

El archivo contiene un arreglo de ArqItem.

**Definición de ArqItem:**
```typescript
interface ArqItem {
    name: string
    path: string
    short_option: string
    option: string
    description: string
    templates: Array<ArqTemplate>
    has_props?: string
}

interface ArqTemplate {
    template: string
    destination: string
    per_prop?: boolean
    prop_naming?: string
    per_prop_import?: string
}
```

### Explicación de cada atributo

**name: string**

Representa el nombre del  item

**path: string**

Es la ruta donde se van a crear los archivos


**short_option: string**

Define una opción corta para la línea de comandos.

*Ejemplo: -a*
```bash
gen -a my_atom
```
**Tip:** Escribe - al inicio de tu opción corta para diferenciarlo de los comandos nativos del CLI.

**option: string**

Define una opción para la línea de comandos.

*Ejemplo: --atom*
```bash
gen --atom my_atom
```

**Tip:** Escribe -- al inicio de tu opción corta para diferenciarlo de los comandos nativos del CLI y de la opción corta.

**description: string**

Es una breve explicación de lo que hace tu opción, este texto se mostrará al solicitar la ayuda del CLI.

**templates: Array<ArqTemplate>**

Aquí definimos dónde se encuentra el template y cómo se formará el archivo resultante.

Podemos especificar uno o varios archivos. Esto es útil si deseas crear un archivo con su test.

**has_props?: string**

Algunas ocasiones vamos a necesitar crear clases o interfaces. Para definir sus atributos, podemos habilitar esta opción con valor *true*

*Ejemplo:*
```bash
gen --atom my_atom name:string,min:number
```

**Tip 1:** Es importante separar cada prop con una coma "," y no dejar espacios, como se muestra en el ejemplo.

**Tip 2:** El tipo de dato depende totalmente del lenguaje que uses en tu archivo resultante, en este ejemplo se usa typescript. En caso de que tu lenguaje no soporte tipado o anotaciones puedes omitir el tipo de dato:

*Ejemplo:*
```bash
gen --atom my_atom name,min
```

# Instalación
## Windows
## Mac OS
## Linux

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
