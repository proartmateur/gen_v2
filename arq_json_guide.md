# 📋 Guía Completa de arq.json

Esta guía explica la estructura y configuración del archivo `arq.json`, que define las arquitecturas y templates disponibles en GenCLI.

## 📚 Índice

1. [Estructura General](#estructura-general)
2. [Campos de Arquitectura](#campos-de-arquitectura)
3. [Configuración de Templates](#configuración-de-templates)
4. [Casos de Uso Reales](#casos-de-uso-reales)
5. [Variables Dinámicas](#variables-dinámicas)
6. [Ejemplos Completos](#ejemplos-completos)
7. [Validación y Errores](#validación-y-errores)

---

## Estructura General

`arq.json` es un **array de objetos**, donde cada objeto define una **capa completa** con sus templates asociados.

```json
[
  {
    "name": "component",
    "short_option": "-c",
    "option": "--component",
    "templates": [ /* ... */ ]
  },
  {
    "name": "atom",
    "short_option": "-a",
    "option": "--atom",
    "templates": [ /* ... */ ]
  }
]
```

### 🔑 Concepto Clave

**Una opción CLI (`-c` o `--component`) → Una capa de arquitectura→ Múltiples templates generados**

```bash
gen -c User "name:string,email:string"
     ↓
Ejecuta arquitectura "component"
     ↓
Genera TODOS los templates de esa capa de arquitectura:
  ✅ UserRepository.ts
  ✅ User.ts (modelo)
  ✅ UserDTO.ts
  ✅ NameVO.ts, EmailVO.ts (per_prop)
```

---

## Campos de Capa de Arquitectura

Cada capa de arquitectura en el array tiene los siguientes campos:

### 📝 Campos Obligatorios

| Campo | Tipo | Descripción | Ejemplo |
|-------|------|-------------|---------|
| `name` | `string` | Identificador interno único | `"component"` |
| `path` | `string` | Ruta base donde se generarán los archivos | `"/src/<ent>"` |
| `short_option` | `string` | Opción corta CLI (una letra con `-`) | `"-c"` |
| `option` | `string` | Opción larga CLI (palabra con `--`) | `"--component"` |
| `templates` | `array` | Lista de templates a generar | `[{...}, {...}]` |

### 📝 Campos Opcionales

| Campo | Tipo | Descripción | Ejemplo |
|-------|------|-------------|---------|
| `description` | `string` | Descripción para el help | `"backend DDD component"` |
| `has_props` | `boolean` | ¿Requiere propiedades? | `true` o `false` |
| `prop_type_place` | `number` | Posición del tipo en `name:string` (1=antes, 2=después) | `2` |
| `prop_prop_place` | `number` | Posición del nombre en `name:string` | `1` |
| `prop_prefix` | `string\|null` | Prefijo antes del tipo | `null` o `"@"` |
| `prop_type_separator` | `string` | Separador entre nombre y tipo | `":"` |

### Ejemplo Completo de Capa de Arquitectura

```json
{
  "name": "component",
  "path": "/src/<ent>",
  "short_option": "-c",
  "option": "--component",
  "description": "backend DDD component",
  "has_props": true,
  "prop_type_place": 2,
  "prop_prop_place": 1,
  "prop_prefix": null,
  "prop_type_separator": ":",
  "templates": [
    /* templates aquí */
  ]
}
```

---

## Configuración de Templates

Cada arquitectura contiene un array `templates` con los templates a generar.

### 📝 Campos de Template

#### Obligatorios

| Campo | Tipo | Descripción | Ejemplo |
|-------|------|-------------|---------|
| `template` | `string` | Ruta relativa al template (desde `.gen_cli/templates/`) | `"/ddd/domain/Model.ts"` |
| `destination` | `string` | Ruta de destino del archivo generado | `"<path>/domain/<ent>.ts"` |

#### Opcionales

| Campo | Tipo | Descripción | Ejemplo |
|-------|------|-------------|---------|
| `per_prop` | `boolean` | ¿Generar un archivo por cada propiedad? | `true` |
| `prop_naming` | `string` | Formato del nombre de propiedad en este contexto | `"<prop>VO"` |
| `per_prop_import` | `string` | Plantilla del import a recolectar | `"import { <prop>VO } from $dq$...$dq$"` |

### Ejemplo de Template Simple

```json
{
  "template": "/ddd/domain/Dto.ts",
  "destination": "<path>/domain/<ent>DTO.ts"
}
```

### Ejemplo de Template con per_prop

```json
{
  "template": "/ddd/domain/ValueObj.ts",
  "destination": "<path>/domain/value_objects/<prop>VO.ts",
  "per_prop": true
}
```

### Ejemplo de Template con per_prop_import

```json
{
  "template": "/ddd/domain/Model.ts",
  "destination": "<path>/domain/<ent>.ts",
  "prop_naming": "<prop>VO",
  "per_prop_import": "import { <prop>VO } from $dq$<path>/domain/value_objects/<prop>VO$dq$"
}
```

---

## Casos de Uso Reales

### 🎯 Caso 1: DDD Component (Arquitectura Completa)

**Objetivo**: Generar una estructura Domain-Driven Design completa con Repository, Model, DTO y Value Objects para Typescript.

```json
{
  "name": "component",
  "path": "/src/<ent>",
  "short_option": "-c",
  "option": "--component",
  "description": "backend DDD component",
  "has_props": true,
  "prop_type_place": 2,
  "prop_prop_place": 1,
  "prop_prefix": null,
  "prop_type_separator": ":",
  "templates": [
    {
      "template": "/ddd/domain/IRepositiry.ts",
      "destination": "<path>/domain/<ent>Repository.ts"
    },
    {
      "template": "/ddd/domain/Model.ts",
      "destination": "<path>/domain/<ent>.ts",
      "prop_naming": "<prop>VO",
      "per_prop_import": "import { <prop>VO } from $dq$<path>/domain/value_objects/<prop>VO$dq$"
    },
    {
      "template": "/ddd/domain/Dto.ts",
      "destination": "<path>/domain/<ent>DTO.ts"
    },
    {
      "template": "/ddd/domain/ValueObj.ts",
      "destination": "<path>/domain/value_objects/<prop>VO.ts",
      "per_prop": true
    }
  ]
}
```

**Uso**:
```bash
gen -c Invoice "client:string,amount:number,date:string"
# o
gen --component Invoice "client:string,amount:number,date:string"
```

**Resultado**:
```
src/Invoice/
├── domain/
│   ├── InvoiceRepository.ts  ← Template 1
│   ├── Invoice.ts            ← Template 2 (con imports de Value Objects)
│   ├── InvoiceDTO.ts         ← Template 3
│   └── value_objects/
│       ├── ClientVO.ts       ← Template 4 (per_prop)
│       ├── AmountVO.ts       ← Template 4 (per_prop)
│       └── DateVO.ts         ← Template 4 (per_prop)
```

**Flujo de Generación**:
1. GenCLI detecta `per_prop: true` en el template 4 (ValueObj.ts)
2. Genera un Value Object por cada propiedad (ClientVO, AmountVO, DateVO)
3. Recolecta los imports definidos en `per_prop_import` del template 2
4. Genera el Model.ts (template 2) inyectando los imports en `$per_prop_imports$`
5. Genera el Repository y DTO (templates 1 y 3)

---

### 🎯 Caso 2: Atom Component (Template Único)

**Objetivo**: Generar un componente React simple sin arquitectura compleja.

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
}
```

**Uso**:
```bash
gen -a Button "label:string,onClick:func,disabled:bool"
# o
gen --atom Button "label:string,onClick:func,disabled:bool"
```

**Resultado**:
```
src/atoms/
└── Button.jsx  ← Un solo archivo
```

**Características**:
- Un solo template en el array
- `has_props: true` → Requiere propiedades
- Path fijo: `/src/atoms` (no depende de la entidad)

---

### 🎯 Caso 3: Raw Component (Sin Propiedades)

**Objetivo**: Generar código sin necesidad de propiedades.

```json
{
  "name": "raw",
  "path": "/src/raw",
  "short_option": "-r",
  "option": "--raw",
  "description": "Code without props",
  "has_props": false,
  "templates": [
    {
      "template": "/raw/component.jsx",
      "destination": "<path>/<ent>.jsx"
    }
  ]
}
```

**Uso**:
```bash
gen -r HomePage
# o
gen --raw HomePage
```

**Nota**: Como `has_props: false`, NO se requiere pasar propiedades. Solo el nombre de la entidad.

**Resultado**:
```
src/raw/
└── HomePage.jsx
```

---

## Variables Dinámicas

Las rutas (`path`, `destination`) y templates soportan variables dinámicas que se reemplazan en tiempo de ejecución.

### Variables de Entidad

| Variable | Descripción | Ejemplo (Input: `UserAccount`) |
|----------|-------------|-------------------------------|
| `<ent>` | Nombre original de la entidad | `UserAccount` |
| `<camel>` | camelCase | `userAccount` |
| `<pascal>` | PascalCase | `UserAccount` |
| `<snake>` | snake_case | `user_account` |
| `<kebab>` | kebab-case | `user-account` |
| `<const>` | CONST_CASE | `USER_ACCOUNT` |

### Variables de Propiedad

| Variable | Descripción | Ejemplo (Prop: `firstName`) |
|----------|-------------|----------------------------|
| `<prop>` | Nombre original de la propiedad | `FirstName` (PascalCase automático) |
| `<camel_prop>` | camelCase | `firstName` |
| `<snake_prop>` | snake_case | `first_name` |
| `<kebab_prop>` | kebab-case | `first-name` |

### Variables Especiales

| Variable | Descripción |
|----------|-------------|
| `<path>` | Path de la arquitectura (ya procesado) |
| `$dq$` | Comillas dobles (`"`) para strings dentro de JSON |

### Ejemplos de Uso

#### En `path` de arquitectura:
```json
"path": "/src/<ent>"
```
→ Con `User` genera: `/src/User`

#### En `destination` de template:
```json
"destination": "<path>/domain/<ent>Repository.ts"
```
→ Con `User` y `path="/src/User"` genera: `/src/User/domain/UserRepository.ts`

#### En `per_prop_import`:
```json
"per_prop_import": "import { <prop>VO } from $dq$<path>/domain/value_objects/<prop>VO$dq$"
```
→ Con `client` y `path="/src/Invoice"` genera:
```typescript
import { ClientVO } from "/src/Invoice/domain/value_objects/ClientVO"
```

---

## Ejemplos Completos

### 🎬 Ejemplo 1: API REST (Controller + Service + Repository)

```json
{
  "name": "api_rest",
  "path": "/src/modules/<kebab>",
  "short_option": "-api",
  "option": "--rest-api",
  "description": "Complete REST API with Controller, Service, and Repository",
  "has_props": true,
  "prop_type_place": 2,
  "prop_prop_place": 1,
  "prop_prefix": null,
  "prop_type_separator": ":",
  "templates": [
    {
      "template": "/api/Controller.ts",
      "destination": "<path>/controllers/<ent>Controller.ts"
    },
    {
      "template": "/api/Service.ts",
      "destination": "<path>/services/<ent>Service.ts"
    },
    {
      "template": "/api/Repository.ts",
      "destination": "<path>/repositories/<ent>Repository.ts"
    },
    {
      "template": "/api/DTO.ts",
      "destination": "<path>/dtos/<ent>DTO.ts"
    },
    {
      "template": "/api/Routes.ts",
      "destination": "<path>/routes/<kebab>.routes.ts"
    }
  ]
}
```

**Uso**:
```bash
gen -api Product "name:string,price:number,stock:number,category:string"
```

**Resultado**:
```
src/modules/product/
├── controllers/
│   └── ProductController.ts
├── services/
│   └── ProductService.ts
├── repositories/
│   └── ProductRepository.ts
├── dtos/
│   └── ProductDTO.ts
└── routes/
    └── product.routes.ts
```

---

### 🎬 Ejemplo 2: GraphQL (Schema + Resolver + Service)

```json
{
  "name": "graphql",
  "path": "/src/graphql/<kebab>",
  "short_option": "-gql",
  "option": "--graphql",
  "description": "GraphQL schema with resolver and service",
  "has_props": true,
  "prop_type_place": 2,
  "prop_prop_place": 1,
  "prop_prefix": null,
  "prop_type_separator": ":",
  "templates": [
    {
      "template": "/graphql/schema.graphql",
      "destination": "<path>/<kebab>.schema.graphql"
    },
    {
      "template": "/graphql/Resolver.ts",
      "destination": "<path>/<ent>Resolver.ts"
    },
    {
      "template": "/graphql/Service.ts",
      "destination": "<path>/<ent>Service.ts"
    },
    {
      "template": "/graphql/Input.ts",
      "destination": "<path>/inputs/<ent>Input.ts"
    }
  ]
}
```

**Uso**:
```bash
gen -gql Order "userId:string,items:array,total:number,status:string"
```

**Resultado**:
```
src/graphql/order/
├── order.schema.graphql
├── OrderResolver.ts
├── OrderService.ts
└── inputs/
    └── OrderInput.ts
```

---

### 🎬 Ejemplo 3: Redux Slice (State Management)

```json
{
  "name": "redux",
  "path": "/src/store/slices",
  "short_option": "-redux",
  "option": "--slice",
  "description": "Redux Toolkit slice with actions and selectors",
  "has_props": true,
  "prop_type_place": 2,
  "prop_prop_place": 1,
  "prop_prefix": null,
  "prop_type_separator": ":",
  "templates": [
    {
      "template": "/redux/slice.ts",
      "destination": "<path>/<camel>Slice.ts"
    },
    {
      "template": "/redux/types.ts",
      "destination": "<path>/<camel>Types.ts"
    },
    {
      "template": "/redux/selectors.ts",
      "destination": "<path>/<camel>Selectors.ts"
    }
  ]
}
```

**Uso**:
```bash
gen -redux ShoppingCart "items:array,total:number,isLoading:bool"
```

**Resultado**:
```
src/store/slices/
├── shoppingCartSlice.ts
├── shoppingCartTypes.ts
└── shoppingCartSelectors.ts
```

---

### 🎬 Ejemplo 4: Python FastAPI (Completo)

```json
{
  "name": "fastapi",
  "path": "/app/<snake>",
  "short_option": "-fast",
  "option": "--fastapi",
  "description": "FastAPI module with router, service, and model",
  "has_props": true,
  "prop_type_place": 1,
  "prop_prop_place": 2,
  "prop_prefix": null,
  "prop_type_separator": ":",
  "templates": [
    {
      "template": "/python/router.py",
      "destination": "<path>/router.py"
    },
    {
      "template": "/python/service.py",
      "destination": "<path>/service.py"
    },
    {
      "template": "/python/model.py",
      "destination": "<path>/model.py"
    },
    {
      "template": "/python/schema.py",
      "destination": "<path>/schema.py"
    }
  ]
}
```

**Uso**:
```bash
gen -fast User "str:username,str:email,int:age,bool:is_active"
```

**Nota**: `prop_type_place: 1` significa que el tipo va primero (`str:username`).

**Resultado**:
```
app/user/
├── router.py
├── service.py
├── model.py
└── schema.py
```

---

## Configuración de Propiedades

### `has_props`

Define si la arquitectura requiere propiedades al ejecutarse.

```json
"has_props": true
```
→ **Requiere**: `gen -c User "name:string,email:string"`

```json
"has_props": false
```
→ **Solo entidad**: `gen -r HomePage`

### `prop_type_place` y `prop_prop_place`

Definen el orden de nombre y tipo en la sintaxis de propiedades.

#### TypeScript/JavaScript Style
```json
"prop_type_place": 2,    // Tipo después
"prop_prop_place": 1,    // Nombre primero
"prop_type_separator": ":"
```
→ Sintaxis: `name:string,age:number`

#### Python Style
```json
"prop_type_place": 1,    // Tipo primero
"prop_prop_place": 2,    // Nombre después
"prop_type_separator": ":"
```
→ Sintaxis: `str:username,int:age`

#### Go Style
```json
"prop_type_place": 2,
"prop_prop_place": 1,
"prop_type_separator": " "
```
→ Sintaxis: `name string,age int`

### `prop_prefix`

Prefijo antes del tipo (usado en algunos lenguajes).

```json
"prop_prefix": "@"
```
→ Sintaxis: `name@string`

```json
"prop_prefix": null
```
→ Sin prefijo: `name:string`

---

## per_prop y per_prop_import

### `per_prop: true`

Genera **un archivo por cada propiedad** en lugar de un solo archivo.

```json
{
  "template": "/ddd/domain/ValueObj.ts",
  "destination": "<path>/domain/value_objects/<prop>VO.ts",
  "per_prop": true
}
```

**Con propiedades** `"client:string,amount:number"`:
- ✅ Genera `ClientVO.ts`
- ✅ Genera `AmountVO.ts`

**Sin `per_prop`**:
- ✅ Genera un solo archivo con todas las propiedades

---

### `per_prop_import`

Cuando un template tiene `per_prop: true`, otros templates pueden recolectar los imports automáticamente.

#### Configuración

```json
{
  "template": "/ddd/domain/Model.ts",
  "destination": "<path>/domain/<ent>.ts",
  "prop_naming": "<prop>VO",
  "per_prop_import": "import { <prop>VO } from $dq$<path>/domain/value_objects/<prop>VO$dq$"
}
```

#### En el Template Model.ts

```typescript
$FILE_HEADER$
$per_prop_imports$

export class $entity$ {
(   private _$camel_prop$: $ent_prop$VO;
)
    // ...
}
```

#### Resultado Generado

```typescript
/**
 * @author myself <myself@example.com>
 * @date 2026-02-15
 */
import { ClientVO } from "/src/Invoice/domain/value_objects/ClientVO"
import { AmountVO } from "/src/Invoice/domain/value_objects/AmountVO"
import { DateVO } from "/src/Invoice/domain/value_objects/DateVO"

export class Invoice {
   private _client: ClientVO;
   private _amount: AmountVO;
   private _date: DateVO;
    // ...
}
```

#### Flujo

1. Template con `per_prop: true` genera Value Objects
2. Template con `per_prop_import` definido recolecta los imports
3. Variable `$per_prop_imports$` en el template se reemplaza con los imports recolectados

---

## `prop_naming`

Define cómo se nombran las propiedades en un contexto específico.

```json
"prop_naming": "<prop>VO"
```

Con propiedad `client:string`:
- `<prop>` se convierte en `ClientVO`
- Útil para tipos personalizados (Value Objects, DTOs, etc.)

**En el template**:
```typescript
private _$camel_prop$: $ent_prop$VO;
```

**Con `prop_naming: "<prop>VO"`**:
```typescript
private _client: ClientVO;
```

---

## Validación y Errores

### ✅ Validaciones Automáticas

GenCLI valida automáticamente:

1. **Campos obligatorios**: `name`, `path`, `short_option`, `option`, `templates`
2. **Array de templates**: Debe tener al menos un template
3. **Rutas válidas**: Templates deben existir en `.gen_cli/templates/`
4. **Opciones únicas**: No puede haber dos arquitecturas con la misma `short_option` o `option`

### ❌ Errores Comunes

#### 1. Template no encontrado
```json
{
  "template": "/ddd/domain/Modell.ts",  // Typo: Modell en lugar de Model
  "destination": "<path>/domain/<ent>.ts"
}
```
**Error**: `Template file not found: .gen_cli/templates/ddd/domain/Modell.ts`

#### 2. Opción duplicada
```json
[
  {
    "name": "component",
    "short_option": "-c",
    "option": "--component"
  },
  {
    "name": "controller",
    "short_option": "-c",  // ❌ Duplicado
    "option": "--controller"
  }
]
```
**Error**: `Duplicate option: -c`

#### 3. Variable no resuelta
```json
"destination": "<path>/domain/<entity>.ts"  // ❌ Debe ser <ent>, no <entity>
```
**Resultado**: Archivo con nombre literal `<entity>.ts`

#### 4. `has_props: false` con propiedades
```bash
gen -r HomePage "title:string"  # ❌ raw no acepta propiedades
```
**Error**: `Architecture 'raw' does not accept properties`

---

## 💡 Mejores Prácticas

### ✅ DO

1. **Usa opciones mnemotécnicas**: `-api` para API, `-gql` para GraphQL, `-rc` para React Component
2. **Agrupa templates relacionados**: Repository, Model, DTO, ValueObjects en la misma arquitectura
3. **Usa variables dinámicas**: `<ent>`, `<path>`, `<prop>` en lugar de valores fijos
4. **Define `description`**: Ayuda a los usuarios a entender qué hace cada opción
5. **Ordena templates por dependencia**: Per_prop primero, luego los que usan imports

### ❌ DON'T

1. **No dupliques opciones**: Cada `-x` y `--xxx` debe ser único
2. **No hardcodees rutas absolutas**: Usa `<path>` en destinations
3. **No mezcles estilos de props**: Si usas `:`, no cambies a ` ` en la misma arquitectura
4. **No olvides `$dq$` en imports**: Las comillas dobles dentro de JSON deben ser `$dq$`

---

## 📚 Recursos

- **Template guide**: Ver `templates.md` para sintaxis de templates
- **Ejemplos reales**: Revisar `.gen_cli/templates/` del proyecto
- **Tests**: `tests/` contiene ejemplos de uso

---

## 🎓 Ejercicio: Crea tu Primera Arquitectura

### Objetivo
Crear una arquitectura para un microservicio Express.js completo.

### Paso 1: Define la arquitectura

```json
{
  "name": "express_microservice",
  "path": "/src/services/<kebab>",
  "short_option": "-svc",
  "option": "--microservice",
  "description": "Express microservice with routes, controller, service, and model",
  "has_props": true,
  "prop_type_place": 2,
  "prop_prop_place": 1,
  "prop_prefix": null,
  "prop_type_separator": ":",
  "templates": [
    {
      "template": "/express/routes.ts",
      "destination": "<path>/routes.ts"
    },
    {
      "template": "/express/controller.ts",
      "destination": "<path>/controller.ts"
    },
    {
      "template": "/express/service.ts",
      "destination": "<path>/service.ts"
    },
    {
      "template": "/express/model.ts",
      "destination": "<path>/model.ts"
    }
  ]
}
```

### Paso 2: Agrega al arq.json

Abre `arq.json` y agrega el objeto al array principal.

### Paso 3: Crea los templates

```bash
mkdir -p .gen_cli/templates/express
touch .gen_cli/templates/express/{routes,controller,service,model}.ts
```

### Paso 4: Usa la arquitectura

```bash
gen -svc Payment "amount:number,method:string,status:string"
# o
gen --microservice Payment "amount:number,method:string,status:string"
```

---

**¡Felicidades!** 🎉 Ahora dominas completamente la configuración de `arq.json` para crear arquitecturas personalizadas en GenCLI.
