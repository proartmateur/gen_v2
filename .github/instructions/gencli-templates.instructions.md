---
applyTo: "**/*.json,**/*.md,**/*.ts,**/*.tsx,**/*.py,**/*.jsx"
---

# GenCLI Template & arq.json Authoring Skill

You are helping a developer create **templates** and **arq.json entries** for **GenCLI**, a Rust-based CLI code scaffolding tool. This skill gives you complete context to generate correct templates and configurations.

---

## How GenCLI Works

```
gen -c User "name:string,email:string"
      │          │              │
   arq option  entity name    props (name:type,...)
      │
      ▼
arq.json → finds matching option → reads all templates → replaces variables → writes files
```

GenCLI has two config files:
- **`gen_config.json`** — personal settings (author, email, path to `arq.json`, templates root)
- **`arq.json`** — architecture definitions (what templates to generate and how)

Templates live inside `.gen_cli/templates/` (configurable via `gen_config.json`).

---

## arq.json Structure

`arq.json` is an **array** of architecture entries. Each entry defines one CLI option that generates one or more files.

### Full Entry Schema

```json
{
  "name": "mvc",                        // internal identifier
  "path": "/src/modules/$snake_name$",  // base path (supports entity variables)
  "short_option": "-m",                 // CLI short flag
  "option": "--mvc",                    // CLI long flag
  "description": "Description here",   // shown in --help

  // --- PROP PARSING (required if using typed props like "name:str") ---
  "has_props": true,
  "prop_type_separator": ":",           // char between name and type  e.g. ":" → "name:str"
  "prop_prop_place": 1,                 // 1 = name comes FIRST
  "prop_type_place": 2,                 // 2 = type comes SECOND
  "prop_prefix": null,                  // usually null

  "templates": [ /* ... */ ]
}
```

### Template Entry Schema

```json
{
  "template": "/mvc/router.py",                         // path relative to templates root
  "destination": "<path>/infrastructure/http/routers.py", // output path (supports all variables)

  // --- PER-PROP OPTIONS (optional) ---
  "per_prop": true,                                     // generate ONE FILE per property
  "per_prop_import": "from <path>.value_objects import <prop>VO" // import line collected into $per_prop_imports$
}
```

---

## Variables Reference

### Entity Variables (in templates, `path`, and `destination`)

All forms below are equivalent — use `$var$` or `<var>` interchangeably.

| Variable | Example input: `user_account` | Notes |
|---|---|---|
| `$ent$` / `<ent>` | `UserAccount` | PascalCase — main entity name |
| `$camel_name$` / `<camel_name>` | `userAccount` | camelCase |
| `$snake_name$` / `<snake_name>` | `user_account` | snake_case |
| `$kebab_name$` / `<kebab_name>` | `user-account` | kebab-case |
| `$const_name$` / `<const_name>` | `USER_ACCOUNT` | SCREAMING_SNAKE |
| `$raw_name$` / `<raw_name>` | `user_account` | exactly as typed in CLI |
| `$path$` / `<path>` | `/src/modules/user_account` | resolved `path` from arq.json entry |
| `$dq$` / `<dq>` | `"` | double-quote (used inside JSON `per_prop_import`) |
| `$ln$` / `<ln>` | newline `\n` | explicit line break |
| `$FILE_HEADER$` | `/** @author ... */` | auto-generated header |
| `$author_name$` / `$author_email$` / `$now$` | — | from gen_config.json |

### Prop Variables (inside iterative blocks in templates)

Used **only inside `(...)` blocks** in template files.

| Variable | Example prop: `first_name:string` | Notes |
|---|---|---|
| `$prop$` / `<prop>` | `first_name` | original name as typed |
| `$camel_prop$` / `<camel_prop>` | `firstName` | camelCase |
| `$snake_prop$` / `<snake_prop>` | `first_name` | snake_case |
| `$kebab_prop$` / `<kebab_prop>` | `first-name` | kebab-case |
| `$const_prop$` / `<const_prop>` | `FIRST_NAME` | SCREAMING_SNAKE |
| `$ent_prop$` / `<ent_prop>` | `FirstName` | PascalCase (good for class names) |
| `$prop_type$` / `<prop_type>` | `string` | the type part after the separator |
| `$type_separator$` / `<type_separator>` | `:` | the separator char configured in arq.json |

### Prop Variables in `destination` and `per_prop_import` (outside templates)

| Variable | Result |
|---|---|
| `<prop>` | PascalCase name (e.g. `FirstName`) |
| `<camel_prop>` | camelCase name |
| `<snake_prop>` | snake_case name |
| `<path>` | resolved entity path |
| `$dq$` | double-quote character |

---

## Iterative Blocks `(...)`

Code between `(` and `)` is repeated **once per property**. This is the core feature for generating fields, getters, etc.

### ✅ Rules (CRITICAL — violations cause silent failures)

1. **`(` MUST be the FIRST character on its line** — no leading spaces/tabs
2. **`)` MUST be on its own line** (nothing else on that line)
3. The block **must contain at least one prop variable** (`$prop$`, `$camel_prop$`, etc.)
4. Content indentation goes **after** the `(`, not before it

### Correct Syntax

```
(    private $camel_prop$: $prop_type$;
)
```

```
(    $snake_prop$: $prop_type$ = Field(...)
)
```

### ❌ Wrong — `(` has leading spaces (block won't be recognized)

```
    ($camel_prop$: $prop_type$
    )
```

### Multi-line block example

```
(    get $camel_prop$(): $prop_type$ {
        return this._$camel_prop$;
    }

)
```

---

## Common Patterns by Language

### TypeScript / JavaScript

```typescript
// Template: Model.ts
export class $ent$ {
(    private _$camel_prop$: $prop_type$;
)

    constructor(
(        $camel_prop$: $prop_type$,
)
    ) {
(        this._$camel_prop$ = $camel_prop$;
)
    }
}
```

### Python (Pydantic / FastAPI)

```python
# Template: schema.py
from pydantic import BaseModel

class $ent$Schema(BaseModel):
(    $snake_prop$: $prop_type$
)

    class Config:
        orm_mode = True
```

**Important for Python**: indentation inside the block uses spaces **after** the `(`:

```
(    $snake_prop$: $prop_type$
)
```
→ generates:
```python
    name: str
    email: str
```

### Go

```go
// Template: model.go
type $ent$ struct {
(	$ent_prop$ $prop_type$
)
}
```

---

## `per_prop` Templates

Use `per_prop: true` when you need **one output file per property** (e.g., Value Objects).

### arq.json

```json
{
  "template": "/ddd/ValueObj.ts",
  "destination": "<path>/domain/value_objects/<prop>VO.ts",
  "per_prop": true
}
```

### Template file (ValueObj.ts)

```typescript
export class $ent_prop$VO {
    constructor(private readonly value: $prop_type$) {}
    getValue(): $prop_type$ { return this.value; }
}
```

→ Running `gen -c User "name:string,email:string"` generates:
- `NameVO.ts`
- `EmailVO.ts`

---

## `per_prop_import` — Collecting Imports

When per_prop generates files, another template may need to import them all. Use `per_prop_import` on the template that collects the import lines, and place `$per_prop_imports$` in the template that uses them.

### arq.json

```json
{
  "template": "/ddd/Model.ts",
  "destination": "<path>/domain/$ent$.ts",
  "per_prop_import": "import { <prop>VO } from $dq$<path>/domain/value_objects/<prop>VO$dq$"
},
{
  "template": "/ddd/ValueObj.ts",
  "destination": "<path>/domain/value_objects/<prop>VO.ts",
  "per_prop": true
}
```

### Template: Model.ts

```typescript
$per_prop_imports$

export class $ent$ {
(    private _$camel_prop$: $ent_prop$VO;
)
}
```

→ Result:

```typescript
import { NameVO } from "/src/User/domain/value_objects/NameVO"
import { EmailVO } from "/src/User/domain/value_objects/EmailVO"

export class User {
    private _name: NameVO;
    private _email: EmailVO;
}
```

---

## Entity Variables in `path`

The `path` field in an arq.json entry supports all entity variables:

```json
"path": "/src/<ent>"           // → /src/User
"path": "/src/$snake_name$s"   // → /src/users
"path": "/src/$kebab_name$"    // → /src/user-account
```

Use `$snake_name$` for Python module conventions, `<ent>` for TypeScript/Java conventions.

---

## Complete Example: Python MVC Module

### arq.json entry

```json
{
  "name": "mvc",
  "path": "/src/modules/$snake_name$",
  "short_option": "-m",
  "option": "--mvc",
  "description": "FastAPI MVC module with router, schemas, and service",
  "has_props": true,
  "prop_type_separator": ":",
  "prop_prop_place": 1,
  "prop_type_place": 2,
  "prop_prefix": null,
  "templates": [
    { "template": "/mvc/router.py",    "destination": "<path>/infrastructure/http/routers.py" },
    { "template": "/mvc/schemas.py",   "destination": "<path>/infrastructure/http/schemas.py" },
    { "template": "/mvc/__init__.py",  "destination": "<path>/__init__.py" },
    { "template": "/mvc/__init__.py",  "destination": "<path>/infrastructure/__init__.py" },
    { "template": "/mvc/__init__.py",  "destination": "<path>/infrastructure/http/__init__.py" }
  ]
}
```

### Template: schemas.py

```python
from pydantic import BaseModel
from uuid import UUID


class $ent$CreateRequest(BaseModel):
($snake_prop$: $prop_type$
)


class $ent$Response(BaseModel):
    id: UUID
($snake_prop$: $prop_type$
)
```

### Command

```bash
gen -m Petra "name:str,email:str,created_at:datetime"
```

### Result: `/src/modules/petra/infrastructure/http/schemas.py`

```python
from pydantic import BaseModel
from uuid import UUID


class PetraCreateRequest(BaseModel):
    name: str
    email: str
    created_at: datetime


class PetraResponse(BaseModel):
    id: UUID
    name: str
    email: str
    created_at: datetime
```

---

## Checklist When Creating Templates

- [ ] `(` is at column 0 (start of line), no leading whitespace
- [ ] `)` is alone on its own line
- [ ] Prop variables (`$camel_prop$`, etc.) only appear inside `(...)` blocks or the `per_prop_import` string
- [ ] Entity variables (`$ent$`, `$snake_name$`, etc.) appear outside blocks freely
- [ ] `prop_type_separator`, `prop_prop_place`, `prop_type_place` are set in arq.json if using typed props like `name:str`
- [ ] `per_prop: true` templates use `<prop>` in `destination` for unique filenames
- [ ] `$per_prop_imports$` placeholder exists in the template that collects imports
- [ ] Template file path in arq.json is relative to the templates root (starts with `/`)
- [ ] `destination` starts with `<path>/` to use the resolved base path
- [ ] For Python conventions: use `$snake_name$` in `path`; for TypeScript: use `<ent>`
