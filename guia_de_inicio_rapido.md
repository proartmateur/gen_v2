# 🚀 GenCLI v2.0 - Guía de Inicio Rápido

## ¿Qué es GenCLI?

**GenCLI** es una herramienta rápida de scaffolding de código basada en Rust que genera código boilerplate desde templates configurables. En segundos, puedes crear arquitecturas completas como componentes Domain-Driven Design, componentes React, APIs REST, o cualquier estructura de código personalizada.

```bash
# Genera una arquitectura DDD completa en un comando
gen -c User "name:string,email:string,age:number"

# Crea:
# ✅ UserRepository.ts
# ✅ User.ts (modelo de dominio)
# ✅ UserDTO.ts
# ✅ NameVO.ts, EmailVO.ts, AgeVO.ts (value objects)
```

---

## ¿Por qué GenCLI?

### 🎯 El Problema

El desarrollo moderno requiere código boilerplate repetitivo:
- DDD: Repositories, Models, DTOs, Value Objects
- APIs: Controllers, Services, Routes, Schemas
- Frontend: Componentes con props, estado, tipos
- Creación manual = consume tiempo + inconsistente

### ✅ La Solución

GenCLI elimina el boilerplate mediante:
- **⚡ Velocidad**: Genera arquitecturas completas en milisegundos
- **🎨 Consistencia**: Misma estructura cada vez, cero errores humanos
- **🔧 Flexibilidad**: Templates 100% personalizables para las convenciones de tu equipo
- **🔒 Confiabilidad**: Funciona offline, sin dependencias, sin alucinaciones de IA
- **💰 Costo Cero**: Gratis, código abierto, uso ilimitado

**Para profundizar en ventajas vs herramientas de IA, ve:** [Ventajas de GenCLI](gen_cli_advantages.md)

---

## Instalación

### Prerequisitos

- Rust 1.81.0+ (para compilar desde código fuente)

### Compilar desde Código Fuente

```bash
# Clona el repositorio
git clone <url-de-tu-repo>
cd gen_v2

# Compila el binario release
cargo build --release

# Ubicación del binario
./target/release/gen
```

### Opcional: Agregar al PATH

```bash
# Copia a bin local
sudo cp target/release/gen /usr/local/bin/

# Ahora úsalo desde cualquier lugar
gen --help
```

---

## Inicio Rápido

### 1️⃣ Inicializar Configuración

GenCLI usa dos archivos de configuración:

**`gen_config.json`** - Tu configuración personal
```json
{
  "arq_json_path": "/ruta/absoluta/a/tu/proyecto/arq.json",
  "author": "Tu Nombre",
  "author_email": "tu.email@ejemplo.com"
}
```

**`arq.json`** - Definiciones de arquitecturas (ya incluido)
```json
[
  {
    "name": "component",
    "short_option": "-c",
    "option": "--component",
    "templates": [ /* ... */ ]
  }
]
```

### 2️⃣ Tu Primera Generación

Genera un componente DDD:

```bash
gen -c Product "name:string,price:number,stock:number"
```

**Salida:**
```
✅ ProductRepository.ts
✅ Product.ts
✅ ProductDTO.ts
✅ NameVO.ts
✅ PriceVO.ts
✅ StockVO.ts
```

### 3️⃣ Generar Componentes Frontend

```bash
gen -a Button "label:string,onClick:func,disabled:bool"
```

**Salida:**
```
✅ Button.jsx (componente React con props)
```

---

## Conceptos Clave

### 🔹 Templates

Los templates son archivos de texto con variables especiales que se reemplazan:

```typescript
// Template
export class $ent$ {
(    private _$camel_prop$: $prop_type$;
)
}
```

**Variables:**
- `$ent$` → Nombre de la entidad (ej., `User`)
- `$camel_prop$` → Propiedad en camelCase (ej., `firstName`)
- `$prop_type$` → Tipo de propiedad (ej., `string`)
- `(...)` → Bloques iterativos (repite por propiedad)

**Aprende más:** [Guía de Templates](templates.md)

---

### 🔹 Configuración de Arquitectura (arq.json)

Define qué se genera cuando usas una opción CLI:

```json
{
  "name": "component",
  "short_option": "-c",
  "option": "--component",
  "path": "/src/<ent>",
  "templates": [
    {
      "template": "/ddd/domain/Model.ts",
      "destination": "<path>/domain/<ent>.ts"
    }
  ]
}
```

**Un comando** → **Múltiples templates** → **Arquitectura completa**

**Aprende más:** [Guía de arq.json](arq_json_guide.md)

---

## Comandos Comunes

### Generar Código

```bash
# Componente DDD (opción corta)
gen -c User "name:string,email:string"

# Componente DDD (opción larga)
gen --component User "name:string,email:string"

# Componente Atom
gen -a NavBar "links:array"

# Raw (sin props)
gen -r HomePage
```

### Ayuda y Documentación

```bash
# Ayuda general
gen --help

# Referencia de variables de template
gen man:vars

# Ver arquitecturas disponibles
cat arq.json
```

---

## Ejemplo: Módulo E-commerce Completo

```bash
# Módulo Product
gen -c Product "name:string,price:number,stock:number,category:string"

# Módulo Order
gen -c Order "userId:string,items:array,total:number,status:string"

# Módulo Payment
gen -c Payment "orderId:string,amount:number,method:string,timestamp:date"
```

**Resultado:** 12 archivos en 3 segundos, todos consistentes, todos siguiendo las convenciones de tu equipo.

---

## Hoja de Referencia de Variables de Template

### Variables de Entidad
- `$ent$` o `<ent>` → PascalCase (ej., UserAccount)
- `$camel_name$` → camelCase (ej., userAccount)
- `$snake_name$` → snake_case (ej., user_account)
- `$kebab_name$` → kebab-case (ej., user-account)

### Variables de Propiedad (en bloques iterativos)
- `$prop$` → Nombre original
- `$ent_prop$` → PascalCase (ej., FirstName)
- `$camel_prop$` → camelCase (ej., firstName)
- `$prop_type$` → Tipo (ej., string, number)

### Variables Especiales
- `$FILE_HEADER$` → Header auto-generado
- `$per_prop_imports$` → Imports recolectados de Value Objects
- `$path$` → Ruta de arquitectura resuelta
- `$author_name$` → Desde gen_config.json
- `$now$` → Timestamp actual

**Referencia completa:** Ejecuta `gen man:vars`

---

## Estructura del Proyecto

```
gen_v2/
├── arq.json                    # Definiciones de arquitecturas
├── gen_config.json             # Tu configuración personal
├── .gen_cli/
│   └── templates/              # Archivos de template
│       ├── ddd/
│       │   └── domain/
│       │       ├── Model.ts
│       │       ├── Dto.ts
│       │       ├── IRepositiry.ts
│       │       └── ValueObj.ts
│       └── atom/
│           └── component.jsx
└── src/                        # El código generado va aquí
```

---

## Próximos Pasos

### 📚 Documentación Detallada

1. **[Guía de Templates](templates.md)**
   - Referencia completa de variables
   - Bloques iterativos `(...)`
   - per_prop y per_prop_import
   - Ejemplos del mundo real

2. **[Guía de arq.json](arq_json_guide.md)**
   - Estructura de configuración
   - Crear arquitecturas personalizadas
   - Opciones cortas/largas
   - Orquestación de múltiples templates

3. **[Ventajas de GenCLI](gen_cli_advantages.md)**
   - ¿Por qué usar GenCLI vs IA?
   - Sinergia GenCLI + IA
   - Comparación costo/velocidad
   - Mejores prácticas

### 🎨 Personaliza para tu Equipo

1. **Crea templates personalizados:**
   ```bash
   mkdir -p .gen_cli/templates/tu-framework
   # Agrega tus archivos .ts, .jsx, .py con variables
   ```

2. **Agrega arquitectura a arq.json:**
   ```json
   {
     "name": "tu_arquitectura",
     "short_option": "-ya",
     "templates": [ /* tus templates */ ]
   }
   ```

3. **Genera:**
   ```bash
   gen -ya MiEntidad "prop1:tipo1,prop2:tipo2"
   ```

---

## Flujos de Trabajo Comunes

### 🔄 Flujo 1: GenCLI Puro
```bash
gen -c User "name:string,email:string"
# Edita la lógica de negocio manualmente
# Copia/pega patrones para entidades similares
```

### 🤖 Flujo 2: GenCLI + IA (Recomendado)
```bash
# 1. Estructura con GenCLI (instantáneo)
gen -c Order "items:array,total:number"

# 2. Lógica con IA (rápido)
# Pídele a Copilot/ChatGPT que implemente:
# - Validación compleja en Order.ts
# - Reglas de negocio en OrderRepository.ts
# - Casos edge en tests

# Resultado: Lo mejor de ambos mundos
```

---

## Solución de Problemas

### Problema: "Template file not found"
**Solución:** Verifica que las rutas en `arq.json` coincidan con tu estructura `.gen_cli/templates/`

### Problema: "Option not found"
**Solución:** Verifica `short_option` y `option` en `arq.json`

### Problema: Variables no reemplazadas
**Solución:** 
- Verifica sintaxis: `$var$` o `<var>` (ambas funcionan)
- Para bloques iterativos: `(` al inicio de línea, `)` en su propia línea

### Problema: Error de ruta
**Solución:** Actualiza `arq_json_path` en `gen_config.json` con ruta absoluta

---

## FAQ

**P: ¿Puedo usar GenCLI con cualquier lenguaje de programación?**  
R: ¡Sí! Los templates son agnósticos al lenguaje. Funciona con TS, JS, Python, Go, Rust, Java, etc.

**P: ¿Cómo se compara GenCLI con Yeoman o Plop?**  
R: GenCLI es más rápido (basado en Rust), configuración más simple (JSON), y soporta características avanzadas como generación per_prop y recolección de imports.

**P: ¿Puedo generar desde código existente?**  
R: No directamente. GenCLI genera desde templates. Úsalo para nuevas entidades/componentes.

**P: ¿Soporta monorepos?**  
R: ¡Sí! Configura diferentes `arq.json` por workspace o usa rutas dinámicas.

**P: ¿Puedo compartir templates con mi equipo?**  
R: ¡Absolutamente! Commitea `.gen_cli/templates/` y `arq.json` a Git.

---

## Ejemplo del Mundo Real

**Antes de GenCLI:**
```
Tiempo para crear módulo Order:
- Repository: 15 min
- Model con getters: 20 min  
- DTO: 10 min
- 3 Value Objects: 30 min
Total: 75 minutos + inconsistencias
```

**Con GenCLI:**
```bash
gen -c Order "customerId:string,items:array,total:number"
# Total: 0.5 segundos
# Todos los archivos consistentes, listos para lógica de negocio
```

**Ahorro:** 75 minutos → 30 segundos = **150x más rápido** para boilerplate

---

## Soporte y Comunidad

- **Issues:** Reporta bugs o solicita features en GitHub
- **Templates:** Comparte tus templates en discusiones
- **Ejemplos:** Revisa `templates.md` para patrones avanzados

---

## Tarjeta de Referencia Rápida

```bash
# Generar
gen -c Entidad "prop:tipo,..."     # Componente DDD
gen -a Componente "prop:tipo,..."  # Atom
gen -r Nombre                      # Raw (sin props)

# Ayuda
gen --help                         # Comandos
gen man:vars                       # Variables

# Configuración
gen_config.json                    # Configuración personal
arq.json                          # Arquitecturas

# Templates
.gen_cli/templates/               # Archivos de template
$ent$, $prop$, $prop_type$        # Variables básicas
(...)                             # Bloques iterativos
```

---

**¿Listo para multiplicar x10 tu productividad en boilerplate?** Comienza con `gen -c User "name:string,email:string"` y explora desde ahí! 🚀

Para guías detalladas, ve: [templates.md](templates.md) | [arq_json_guide.md](arq_json_guide.md) | [gen_cli_advantages.md](gen_cli_advantages.md)
