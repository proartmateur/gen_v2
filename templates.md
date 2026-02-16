# 📝 Guía Completa de Templates en GenCLI

Esta guía te enseñará a crear templates para GenCLI, desde los más simples hasta arquitecturas complejas con Value Objects.

## 📚 Índice

1. [Conceptos Básicos](#conceptos-básicos)
2. [Variables Disponibles](#variables-disponibles)
3. [Sintaxis de Templates](#sintaxis-de-templates)
4. [Template Sencillo: Componente React](#template-sencillo-componente-react)
5. [Template Intermedio: DTO](#template-intermedio-dto)
6. [Template Avanzado: Repository](#template-avanzado-repository)
7. [Template Complejo: Domain Model con Value Objects](#template-complejo-domain-model-con-value-objects)
8. [Configuración en arq.json](#configuración-en-arqjson)
9. [Ejemplos Completos](#ejemplos-completos)

---

## Conceptos Básicos

### ¿Qué es un Template?

Un template es un archivo de texto que contiene **variables especiales** que GenCLI reemplaza con valores reales al generar código.

### Flujo de Generación

```
Usuario ejecuta:
  gen -c User "name:string,email:string"
       ↓
GenCLI lee template:
  Model.ts con variables $entity$, $camel_prop$, etc.
       ↓
GenCLI reemplaza variables:
  $entity$ → User
  $camel_prop$ → name, email
       ↓
Genera archivo:
  User.ts con código completo
```

---

## Variables Disponibles

### 🔹 Variables de Entidad

Estas variables provienen del nombre de la entidad que pasas al comando.

| Variable | Ejemplo (Input: `UserAccount`) | Descripción |
|----------|-------------------------------|-------------|
| `$entity$` o `<entity>` | `UserAccount` | Nombre original |
| `$camel$` o `<camel>` | `userAccount` | camelCase |
| `$pascal$` o `<pascal>` | `UserAccount` | PascalCase |
| `$snake$` o `<snake>` | `user_account` | snake_case |
| `$kebab$` o `<kebab>` | `user-account` | kebab-case |
| `$const$` o `<const>` | `USER_ACCOUNT` | CONST_CASE |
| `$path$` o `<path>` | `/src/UserAccount` | Ruta configurada |

### 🔹 Variables de Propiedades

Estas variables provienen de las propiedades que pasas al comando (`"name:string,age:number"`).

| Variable | Ejemplo (Prop: `firstName:string`) | Descripción |
|----------|-----------------------------------|-------------|
| `$prop$` | `firstName` | Nombre original |
| `$camel_prop$` | `firstName` | camelCase |
| `$snake_prop$` | `first_name` | snake_case |
| `$kebab_prop$` | `first-name` | kebab-case |
| `$const_prop$` | `FIRST_NAME` | CONST_CASE |
| `$ent_prop$` | `FirstName` | PascalCase (para nombres de tipos) |
| `$prop_type$` | `string` | Tipo de la propiedad |
| `$type_separator$` | `:` | Separador tipo (`: ` para TS, ` ` para Python) |

### 🔹 Variables Especiales

| Variable | Uso |
|----------|-----|
| `$FILE_HEADER$` | Header con autor y fecha (auto-generado) |
| `$ln$` o `<ln>` | Nueva línea |
| `$per_prop_imports$` | Placeholder para imports de per_prop |
| `$author$` | Autor (de config) |
| `$date$` | Fecha actual |

---

## Sintaxis de Templates

### 1️⃣ Reemplazo Simple

```typescript
// Template
export class $entity$ {
    private name: string;
}

// Comando: gen -c User
// Resultado
export class User {
    private name: string;
}
```

### 2️⃣ Bloques Iterativos con `(...)`

Los bloques entre paréntesis `(...)` se repiten **una vez por cada propiedad**.

#### ✅ Reglas de los Bloques Iterativos

1. **Deben empezar en una nueva línea** (el `(` al inicio de la línea)
2. **Deben terminar en una nueva línea con `)` solo** (el `)` en su propia línea)
3. **Deben contener al menos una variable de propiedad** (`$prop$`, `$camel_prop$`, etc.)
4. **Se expanden una vez por propiedad**, reemplazando todas las variables de propiedad

#### Sintaxis

```typescript
// Una línea por bloque
(    private $camel_prop$: $prop_type$;
)

// Múltiples líneas por bloque
(    get $camel_prop$(): $prop_type$ {
        return this._$camel_prop$;
    }

)
```

**⚠️ Importante:** El paréntesis de cierre `)` DEBE estar en su propia línea.

#### Ejemplo

```typescript
// Template
export class $entity$ {
(    private _$camel_prop$: $prop_type$;
)
}

// Comando: gen -c User "name:string,age:number"
// Resultado
export class User {
    private _name: string;
    private _age: number;
}
```

### 3️⃣ Bloques Multi-línea

Para código más complejo, puedes usar múltiples líneas dentro del bloque:

```typescript
// Template
(    get $camel_prop$(): $prop_type$ {
        return this._$camel_prop$;
    }

)

// Resultado (para name y age)
    get name(): string {
        return this._name;
    }

    get age(): number {
        return this._age;
    }
```

### 4️⃣ Múltiples Variables en una Línea

Puedes combinar diferentes variaciones de la propiedad:

```typescript
// Template
(    private _$camel_prop$: $ent_prop$VO;
)

// Comando: gen -c User "firstName:string"
// Resultado
    private _firstName: FirstNameVO;
```

---

## Template Sencillo: Componente React

### 📄 Caso de Uso
Generar un componente React funcional con props.

### 📝 Template: `.gen_cli/templates/atom/component.jsx`

```jsx
$FILE_HEADER$
import React from 'react';
import PropTypes from 'prop-types';

/**
 * $entity$ Component
 */
const $entity$ = ({
(    $camel_prop$,
)
}) => {
    return (
        <div className="$kebab$">
            <h2>$entity$</h2>
(            <p>{$camel_prop$}</p>
)
        </div>
    );
};

$entity$.propTypes = {
(    $camel_prop$: PropTypes.$prop_type$,
)
};

export default $entity$;
```

### ⚙️ Configuración en `arq.json`

```json
{
    "name": "atom",
    "path": "/src/atoms",
    "short_option": "-a",
    "option": "--atom",
    "description": "frontend atom component",
    "has_props": true,
    "prop_type_place": 2,
    "prop_prop_place": 1,
    "prop_prefix": null,
    "prop_type_separator": ":",
    "templates": [
      {
        "template": "/atom/component.jsx",
        "destination": "<path>/<ent>.jsx"
      }
    ]
  },
```

### 🚀 Uso

Forma corta:
```bash
gen -a Button "label:string,onClick:func,disabled:bool"
```
Forma larga:
```bash
gen --atom Button "label:string,onClick:func,disabled:bool"
```

### ✅ Resultado: `src/components/button/Button.jsx`

```jsx
/**
 * @author myself <myself@example.com>
 * @date 2026-02-15 10:30:00
 */
import React from 'react';
import PropTypes from 'prop-types';

/**
 * Button Component
 */
const Button = ({
    label,
    onClick,
    disabled,
}) => {
    return (
        <div className="button">
            <h2>Button</h2>
            <p>{label}</p>
            <p>{onClick}</p>
            <p>{disabled}</p>
        </div>
    );
};

Button.propTypes = {
    label: PropTypes.string,
    onClick: PropTypes.func,
    disabled: PropTypes.bool,
};

export default Button;
```

---
