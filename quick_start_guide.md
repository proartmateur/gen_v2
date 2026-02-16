# 🚀 GenCLI v2.0 - Quick Start Guide

## What is GenCLI?

**GenCLI** is a fast, Rust-based code scaffolding tool that generates boilerplate code from configurable templates. In seconds, you can create complete architectures like Domain-Driven Design components, React components, REST APIs, or any custom code structure.

```bash
# Generate a complete DDD architecture in one command
gen -c User "name:string,email:string,age:number"

# Creates:
# ✅ UserRepository.ts
# ✅ User.ts (domain model)
# ✅ UserDTO.ts
# ✅ NameVO.ts, EmailVO.ts, AgeVO.ts (value objects)
```

---

## Why GenCLI?

### 🎯 The Problem

Modern development requires repetitive boilerplate code:
- DDD: Repositories, Models, DTOs, Value Objects
- APIs: Controllers, Services, Routes, Schemas
- Frontend: Components with props, state, types
- Manual creation = time-consuming + inconsistent

### ✅ The Solution

GenCLI eliminates boilerplate through:
- **⚡ Speed**: Generate complete architectures in milliseconds
- **🎨 Consistency**: Same structure every time, zero human error
- **🔧 Flexibility**: 100% customizable templates for your team's conventions
- **🔒 Reliability**: Works offline, no dependencies, no AI hallucinations
- **💰 Zero Cost**: Free, open-source, unlimited usage

**For a deep dive on advantages vs AI tools, see:** [GenCLI Advantages](gen_cli_advantages.md)

---

## Installation

### Prerequisites

- Rust 1.81.0+ (for building from source)

### Build from Source

```bash
# Clone the repository
git clone <your-repo-url>
cd gen_v2

# Build release binary
cargo build --release

# Binary location
./target/release/gen
```

### Optional: Add to PATH

```bash
# Copy to local bin
sudo cp target/release/gen /usr/local/bin/

# Now use from anywhere
gen --help
```

---

## Quick Start

### 1️⃣ Initialize Configuration

GenCLI uses two configuration files:

**`gen_config.json`** - Your personal settings
```json
{
  "arq_json_path": "/absolute/path/to/your/project/arq.json",
  "author": "Your Name",
  "author_email": "your.email@example.com"
}
```

**`arq.json`** - Architecture definitions (already included)
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

### 2️⃣ Your First Generation

Generate a DDD component:

```bash
gen -c Product "name:string,price:number,stock:number"
```

**Output:**
```
✅ ProductRepository.ts
✅ Product.ts
✅ ProductDTO.ts
✅ NameVO.ts
✅ PriceVO.ts
✅ StockVO.ts
```

### 3️⃣ Generate Frontend Components

```bash
gen -a Button "label:string,onClick:func,disabled:bool"
```

**Output:**
```
✅ Button.jsx (React component with props)
```

---

## Core Concepts

### 🔹 Templates

Templates are text files with special variables that get replaced:

```typescript
// Template
export class $ent$ {
(    private _$camel_prop$: $prop_type$;
)
}
```

**Variables:**
- `$ent$` → Entity name (e.g., `User`)
- `$camel_prop$` → Property in camelCase (e.g., `firstName`)
- `$prop_type$` → Property type (e.g., `string`)
- `(...)` → Iterative blocks (repeat per property)

**Learn more:** [Templates Guide](templates.md)

---

### 🔹 Architecture Config (arq.json)

Defines what gets generated when you use a CLI option:

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

**One command** → **Multiple templates** → **Complete architecture**

**Learn more:** [arq.json Guide](arq_json_guide.md)

---

## Common Commands

### Generate Code

```bash
# DDD Component (short option)
gen -c User "name:string,email:string"

# DDD Component (long option)
gen --component User "name:string,email:string"

# Atom Component
gen -a NavBar "links:array"

# Raw (no props)
gen -r HomePage
```

### Help & Documentation

```bash
# General help
gen --help

# Template variables reference
gen man:vars

# View available architectures
cat arq.json
```

---

## Example: Complete E-commerce Module

```bash
# Product Module
gen -c Product "name:string,price:number,stock:number,category:string"

# Order Module
gen -c Order "userId:string,items:array,total:number,status:string"

# Payment Module
gen -c Payment "orderId:string,amount:number,method:string,timestamp:date"
```

**Result:** 12 files in 3 seconds, all consistent, all following your team's conventions.

---

## Template Variables Cheat Sheet

### Entity Variables
- `$ent$` or `<ent>` → PascalCase (e.g., UserAccount)
- `$camel_name$` → camelCase (e.g., userAccount)
- `$snake_name$` → snake_case (e.g., user_account)
- `$kebab_name$` → kebab-case (e.g., user-account)

### Property Variables (in iterative blocks)
- `$prop$` → Original name
- `$ent_prop$` → PascalCase (e.g., FirstName)
- `$camel_prop$` → camelCase (e.g., firstName)
- `$prop_type$` → Type (e.g., string, number)

### Special Variables
- `$FILE_HEADER$` → Auto-generated header
- `$per_prop_imports$` → Collected imports from Value Objects
- `$path$` → Resolved architecture path
- `$author_name$` → From gen_config.json
- `$now$` → Current timestamp

**Full reference:** Run `gen man:vars`

---

## Project Structure

```
gen_v2/
├── arq.json                    # Architecture definitions
├── gen_config.json             # Your personal config
├── .gen_cli/
│   └── templates/              # Template files
│       ├── ddd/
│       │   └── domain/
│       │       ├── Model.ts
│       │       ├── Dto.ts
│       │       ├── IRepositiry.ts
│       │       └── ValueObj.ts
│       └── atom/
│           └── component.jsx
└── src/                        # Generated code goes here
```

---

## Next Steps

### 📚 Deep Dive Documentation

1. **[Templates Guide](templates.md)**
   - Complete variable reference
   - Iterative blocks `(...)`
   - per_prop and per_prop_import
   - Real-world examples

2. **[arq.json Guide](arq_json_guide.md)**
   - Configuration structure
   - Creating custom architectures
   - Short/long options
   - Multiple template orchestration

3. **[GenCLI Advantages](gen_cli_advantages.md)**
   - Why use GenCLI vs AI?
   - GenCLI + AI synergy
   - Cost/speed comparison
   - Best practices

### 🎨 Customize for Your Team

1. **Create custom templates:**
   ```bash
   mkdir -p .gen_cli/templates/your-framework
   # Add your .ts, .jsx, .py files with variables
   ```

2. **Add architecture to arq.json:**
   ```json
   {
     "name": "your_architecture",
     "short_option": "-ya",
     "templates": [ /* your templates */ ]
   }
   ```

3. **Generate:**
   ```bash
   gen -ya MyEntity "prop1:type1,prop2:type2"
   ```

---

## Common Workflows

### 🔄 Workflow 1: Pure GenCLI
```bash
gen -c User "name:string,email:string"
# Edit business logic manually
# Copy/paste patterns for similar entities
```

### 🤖 Workflow 2: GenCLI + AI (Recommended)
```bash
# 1. Structure with GenCLI (instant)
gen -c Order "items:array,total:number"

# 2. Logic with AI (fast)
# Ask Copilot/ChatGPT to implement:
# - Complex validation in Order.ts
# - Business rules in OrderRepository.ts
# - Edge cases in tests

# Result: Best of both worlds
```

---

## Troubleshooting

### Issue: "Template file not found"
**Solution:** Check `arq.json` paths match your `.gen_cli/templates/` structure

### Issue: "Option not found"
**Solution:** Verify `short_option` and `option` in `arq.json`

### Issue: Variables not replaced
**Solution:** 
- Check syntax: `$var$` or `<var>` (both work)
- For iterative blocks: `(` at line start, `)` on own line

### Issue: Path error
**Solution:** Update `arq_json_path` in `gen_config.json` to absolute path

---

## FAQ

**Q: Can I use GenCLI with any programming language?**  
A: Yes! Templates are language-agnostic. Works with TS, JS, Python, Go, Rust, Java, etc.

**Q: How does GenCLI compare to Yeoman or Plop?**  
A: GenCLI is faster (Rust-based), simpler config (JSON), and supports advanced features like per_prop generation and import collection.

**Q: Can I generate from existing code?**  
A: Not directly. GenCLI generates from templates. Use it for new entities/components.

**Q: Does it support monorepos?**  
A: Yes! Configure different `arq.json` per workspace or use dynamic paths.

**Q: Can I share templates with my team?**  
A: Absolutely! Commit `.gen_cli/templates/` and `arq.json` to Git.

---

## Real-World Example

**Before GenCLI:**
```
Time to create Order module:
- Repository: 15 min
- Model with getters: 20 min  
- DTO: 10 min
- 3 Value Objects: 30 min
Total: 75 minutes + inconsistencies
```

**With GenCLI:**
```bash
gen -c Order "customerId:string,items:array,total:number"
# Total: 0.5 seconds
# All files consistent, ready for business logic
```

**Savings:** 75 minutes → 30 seconds = **150x faster** for boilerplate

---

## Support & Community

- **Issues:** File bugs or feature requests on GitHub
- **Templates:** Share your templates in discussions
- **Examples:** Check `templates.md` for advanced patterns

---

## Quick Reference Card

```bash
# Generate
gen -c Entity "prop:type,..."      # DDD component
gen -a Component "prop:type,..."   # Atom
gen -r Name                        # Raw (no props)

# Help
gen --help                         # Commands
gen man:vars                       # Variables

# Config
gen_config.json                    # Personal settings
arq.json                          # Architectures

# Templates
.gen_cli/templates/               # Template files
$ent$, $prop$, $prop_type$        # Basic vars
(...)                             # Iterative blocks
```

---

**Ready to 10x your boilerplate productivity?** Start with `gen -c User "name:string,email:string"` and explore from there! 🚀

For detailed guides, see: [templates.md](templates.md) | [arq_json_guide.md](arq_json_guide.md) | [gen_cli_advantages.md](gen_cli_advantages.md)
