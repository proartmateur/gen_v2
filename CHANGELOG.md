# Changelog

All notable changes to GenCLI will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Planned
- Interactive mode for template generation
- Template validation tool
- Template marketplace/sharing

---

## [2.0.0] - 2026-02-15

### Added
- **Template System**
  - Iterative prop blocks with `(...)` syntax
  - Support for multi-line template blocks
  - `$FILE_HEADER$` auto-generation with author and timestamp
  - `$per_prop_imports$` placeholder for collected imports
  - `$type_separator$` and `$prop_type$` variables
  - Six property variable flavors: `prop`, `ent_prop`, `camel_prop`, `snake_prop`, `kebab_prop`, `const_prop`

- **Architecture Configuration**
  - `arq.json` configuration with short and long options
  - `per_prop` support for generating one file per property
  - `per_prop_import` for automatic import collection
  - `prop_naming` for custom property type naming
  - Support for multiple templates per architecture

- **Documentation**
  - `templates.md` - Complete template guide with examples
  - `arq_json_guide.md` - Architecture configuration guide
  - `gen_cli_advantages.md` - GenCLI vs AI comparison and synergy
  - `quick_start_guide.md` - Quick start guide in English
  - `guia_de_inicio_rapido.md` - Quick start guide in Spanish
  - `RELEASE_GUIDE.md` - Release process documentation
  - Improved `env_vars_doc.rs` with comprehensive variable reference

- **Features**
  - Automatic directory creation for nested paths
  - File overwrite prompts with skip/yes/no/all options
  - Internationalization (Spanish and English)
  - Clean, emoji-enhanced console output
  - `gen man:vars` command for variable reference

- **Example Templates**
  - Domain-Driven Design (DDD) templates:
    - `IRepositiry.ts` - Repository interface
    - `Model.ts` - Domain model with Value Objects
    - `Dto.ts` - Data Transfer Object
    - `ValueObj.ts` - Value Object with validation
  - Atomic Design:
    - `component.jsx` - React functional component
  - Raw templates for prop-less generation

- **Development**
  - GitHub Actions workflow for multi-platform releases
  - Automated builds for Windows, Linux, macOS (Intel & Apple Silicon)
  - 18 comprehensive tests covering all features

### Fixed
- **Critical Bug**: `$raw_name$` was incorrectly mapped to `author_email` instead of `raw_name`
- **Path Resolution**: `<ent>` placeholder in paths now correctly resolves to entity name
- **Regex Issues**:
  - Added dotall mode to handle multi-line content in iterative blocks
  - Fixed closing parenthesis detection to require newline before `)`
  - Fixed anchor to prevent matching `constructor(` as iterative block
  - Changed `clean_prop_value` to replace ALL prop variable variants, not just primary
- **Property Extraction**: Now correctly strips only outer parentheses, preserving code parentheses like `getValue()`

### Changed
- **Template Replacer**
  - Pre-resolves `$ln$`/`<ln>` before regex processing
  - Unified 6 separate regex patterns into one combined pattern
  - Improved prop value cleaning to handle complex multi-line blocks
- **Output Formatting**
  - Removed debug `println!` statements for cleaner production output
  - Added structured emoji logging for better UX
  - Improved error messages with context
- **Code Organization**
  - Added `Clone` derive to `EnvVars` and `PropVars` for per_prop processing
  - Refactored `option_process.rs` with two-pass approach for imports
  - Improved `env_mapper.rs` path processing

### Security
- No security issues identified in this release

---

## [1.0.0] - 2025-12-01 (Hypothetical)

### Added
- Initial release of GenCLI
- Basic template variable replacement
- Simple prop syntax support
- Manual file generation
- Basic configuration via `gen_config.json`

---

## Legend

- **Added**: New features
- **Changed**: Changes in existing functionality
- **Deprecated**: Soon-to-be removed features
- **Removed**: Removed features
- **Fixed**: Bug fixes
- **Security**: Security fixes
