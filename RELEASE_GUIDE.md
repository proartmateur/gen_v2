# 📦 Guía de Release - GenCLI

Esta guía explica cómo crear releases de GenCLI con binarios compilados para múltiples plataformas.

## 🎯 Plataformas Soportadas

El workflow de GitHub Actions compila automáticamente para:

- ✅ **Windows 10+** (x86_64-pc-windows-msvc)
- ✅ **Linux** (x86_64-unknown-linux-gnu) - Ubuntu, Arch/Carchy, Debian, etc.
- ✅ **macOS Intel** (x86_64-apple-darwin)
- ✅ **macOS Apple Silicon** (aarch64-apple-darwin) - M1/M2/M3

## 🚀 Cómo Crear un Release

### Paso 1: Asegúrate de que todo está listo

```bash
# Verifica que el código compila
cargo build --release

# Ejecuta los tests
cargo test

# Verifica que no hay errores
cargo clippy
```

### Paso 2: Actualiza la versión en Cargo.toml

Edita `Cargo.toml` y actualiza el campo `version`:

```toml
[package]
name = "gen"
version = "2.0.0"  # ← Actualiza aquí
```

### Paso 3: Commit y push de cambios

```bash
git add Cargo.toml
git commit -m "chore: bump version to 2.0.0"
git push origin main
```

### Paso 4: Crea y push el tag de versión

```bash
# Crea el tag (debe seguir formato v*.*.*)
git tag v2.0.0

# Push del tag (esto activa el workflow)
git push origin v2.0.0
```

### Paso 5: Espera a que GitHub Actions compile

1. Ve a tu repositorio en GitHub
2. Click en la pestaña **Actions**
3. Verás el workflow "Release Build" ejecutándose
4. Espera ~5-10 minutos a que compile para todas las plataformas

### Paso 6: Verifica el Release

1. Ve a la pestaña **Releases** en GitHub
2. Verás el nuevo release con 4 archivos descargables:
   - `gen-v2.0.0-x86_64-pc-windows-msvc.zip` (Windows)
   - `gen-v2.0.0-x86_64-unknown-linux-gnu.tar.gz` (Linux)
   - `gen-v2.0.0-x86_64-apple-darwin.tar.gz` (macOS Intel)
   - `gen-v2.0.0-aarch64-apple-darwin.tar.gz` (macOS Apple Silicon)

## 📋 Checklist Pre-Release

Antes de crear un release, asegúrate de:

- [ ] Código compilando sin errores
- [ ] Tests pasando (18/18)
- [ ] Documentación actualizada (README, templates.md, etc.)
- [ ] Versión actualizada en `Cargo.toml`
- [ ] CHANGELOG.md actualizado (opcional pero recomendado)
- [ ] No hay debug `println!` en código de producción
- [ ] Configuración de ejemplo (`gen_config.json`, `arq.json`) funcionando

## 🔄 Flujo de Versionado

Usamos [Semantic Versioning](https://semver.org/):

- **MAJOR** (v3.0.0): Cambios que rompen compatibilidad
- **MINOR** (v2.1.0): Nuevas características compatibles
- **PATCH** (v2.0.1): Bugfixes

Ejemplos:
```bash
# Primera versión pública
git tag v1.0.0

# Agregaste nuevas arquitecturas/templates
git tag v1.1.0

# Arreglaste un bug
git tag v1.0.1

# Cambiaste la estructura de arq.json (breaking change)
git tag v2.0.0
```

## 🛠️ Compilación Local (Opcional)

Si quieres compilar manualmente para otras plataformas:

### Instalar targets

```bash
# Windows desde Linux/Mac
rustup target add x86_64-pc-windows-gnu

# Linux desde Mac/Windows
rustup target add x86_64-unknown-linux-gnu

# macOS desde Linux/Windows
rustup target add x86_64-apple-darwin
rustup target add aarch64-apple-darwin
```

### Compilar

```bash
# Para Windows
cargo build --release --target x86_64-pc-windows-gnu

# Para Linux
cargo build --release --target x86_64-unknown-linux-gnu

# Para macOS Intel
cargo build --release --target x86_64-apple-darwin

# Para macOS Apple Silicon
cargo build --release --target aarch64-apple-darwin
```

**Nota:** Compilar para macOS desde Linux/Windows requiere herramientas adicionales como `osxcross`.

### Empaquetar

```bash
# Linux/macOS
cd target/x86_64-unknown-linux-gnu/release
tar -czf gen-v2.0.0-x86_64-unknown-linux-gnu.tar.gz gen

# Windows
cd target/x86_64-pc-windows-gnu/release
zip gen-v2.0.0-x86_64-pc-windows-gnu.zip gen.exe
```

## 🔧 Personalizar el Release

### Editar la descripción del release

Edita `.github/workflows/release.yml` en la sección `body:`:

```yaml
body: |
  ## GenCLI ${{ github.ref_name }}
  
  Tu descripción personalizada aquí.
  
  ### Novedades
  - Feature 1
  - Feature 2
  
  ### Bugfixes
  - Fix 1
```

### Agregar más plataformas

Agrega nuevas entradas en la matriz `matrix.include`:

```yaml
# Linux ARM64 (Raspberry Pi, etc.)
- target: aarch64-unknown-linux-gnu
  os: ubuntu-latest
  archive: tar.gz

# Windows ARM
- target: aarch64-pc-windows-msvc
  os: windows-latest
  archive: zip
```

## 📝 Crear CHANGELOG.md (Recomendado)

Crea un archivo `CHANGELOG.md` para documentar cambios:

```markdown
# Changelog

## [2.0.0] - 2026-02-15

### Added
- Template system with iterative blocks
- per_prop and per_prop_import support
- Multi-platform releases
- Comprehensive documentation

### Fixed
- Bug in $raw_name$ variable mapping
- Path resolution for <ent> placeholders

### Changed
- Improved error messages with i18n
- Cleaner output without debug info

## [1.0.0] - 2026-01-01

### Added
- Initial release
- Basic template support
```

## 🚨 Troubleshooting

### Error: "tag already exists"

Si el tag ya existe:

```bash
# Elimina el tag local
git tag -d v2.0.0

# Elimina el tag remoto
git push --delete origin v2.0.0

# Crea nuevamente
git tag v2.0.0
git push origin v2.0.0
```

### Error: "workflow not found"

Asegúrate de que `.github/workflows/release.yml` está en la rama `main` antes de crear el tag.

### Error: "cargo build failed"

Verifica que el código compila localmente primero:

```bash
cargo clean
cargo build --release
```

### Release sin assets

Si el release se crea pero sin archivos:
1. Revisa los logs en GitHub Actions
2. Verifica que el workflow tiene permisos de escritura
3. Asegúrate de que `GITHUB_TOKEN` tiene acceso

## 🎉 Después del Release

1. **Anuncia el release**
   - Actualiza README.md con link al release
   - Comparte en redes sociales/comunidad
   - Actualiza documentación de instalación

2. **Monitorea issues**
   - Revisa si hay problemas específicos de plataforma
   - Responde preguntas de instalación

3. **Planea el siguiente release**
   - Recolecta feedback
   - Crea issues para features solicitados
   - Documenta bugs reportados

## 📦 Ejemplo de README.md para usuarios

Actualiza tu README.md con instrucciones de instalación:

```markdown
## Instalación

### Binarios Pre-compilados (Recomendado)

Descarga el binario para tu plataforma desde [Releases](https://github.com/tu-usuario/gen_v2/releases/latest):

**Windows 10+**
```bash
# Descarga gen-v2.0.0-x86_64-pc-windows-msvc.zip
# Extrae gen.exe
# Mueve a C:\Windows\System32 o agrega a PATH
```

**Linux (Ubuntu, Arch, etc.)**
```bash
# Descarga y extrae
wget https://github.com/tu-usuario/gen_v2/releases/download/v2.0.0/gen-v2.0.0-x86_64-unknown-linux-gnu.tar.gz
tar -xzf gen-v2.0.0-x86_64-unknown-linux-gnu.tar.gz

# Mueve a PATH
sudo mv gen /usr/local/bin/

# Verifica
gen --version
```

**macOS**
```bash
# Descarga y extrae (Intel o Apple Silicon según tu Mac)
curl -L https://github.com/tu-usuario/gen_v2/releases/download/v2.0.0/gen-v2.0.0-x86_64-apple-darwin.tar.gz | tar -xz

# Mueve a PATH
sudo mv gen /usr/local/bin/

# Verifica
gen --version
```

### Compilar desde Código Fuente

```bash
git clone https://github.com/tu-usuario/gen_v2
cd gen_v2
cargo build --release
./target/release/gen --version
```
```

---

**¡Listo!** Con esto tienes un sistema completo de releases automatizados. 🚀

Para tu primer release, sigue la sección "Cómo Crear un Release" paso a paso.
