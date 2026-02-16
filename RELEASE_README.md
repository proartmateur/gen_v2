# 🎉 Todo Listo para tu Primer Release!

He configurado todo lo necesario para crear releases automáticos de GenCLI con binarios para múltiples plataformas.

## 📦 ¿Qué se ha creado?

### 1. **GitHub Actions Workflow** (`.github/workflows/release.yml`)
- Compila automáticamente para 4 plataformas
- Crea el release en GitHub con archivos descargables
- Se activa cuando haces push de un tag `v*.*.*`

### 2. **Guía de Release** (`RELEASE_GUIDE.md`)
- Instrucciones detalladas paso a paso
- Troubleshooting
- Mejores prácticas
- Ejemplos de versionado semántico

### 3. **Changelog** (`CHANGELOG.md`)
- Registro de cambios de la versión 2.0.0
- Formato estándar para futuras versiones

### 4. **Script Helper** (`create_release.sh`)
- Automatiza el proceso completo de release
- Ejecuta tests, actualiza versión, crea tags, push a GitHub

---

## 🚀 Crear tu Primer Release (Método Rápido)

### Opción A: Usar el Script Automático

```bash
# Simplemente ejecuta:
./create_release.sh 2.0.0

# El script hará todo por ti:
# ✓ Ejecutar tests
# ✓ Compilar release
# ✓ Actualizar Cargo.toml
# ✓ Crear commit
# ✓ Crear tag v2.0.0
# ✓ Push a GitHub
```

### Opción B: Manualmente

```bash
# 1. Actualiza la versión en Cargo.toml
vim Cargo.toml  # Cambia version = "2.0.0"

# 2. Commit y push
git add Cargo.toml
git commit -m "chore: bump version to 2.0.0"
git push origin main

# 3. Crea y push el tag
git tag v2.0.0
git push origin v2.0.0

# 4. GitHub Actions hace el resto automáticamente
```

---

## ⏱️ ¿Qué Pasa Después?

1. **GitHub Actions se activa** (~1 minuto después del push del tag)
2. **Compila para 4 plataformas** (~5-10 minutos total):
   - Windows 10+ (x86_64-pc-windows-msvc)
   - Linux (x86_64-unknown-linux-gnu)
   - macOS Intel (x86_64-apple-darwin)
   - macOS Apple Silicon (aarch64-apple-darwin)
3. **Crea el release** con descripción automática y 4 archivos descargables

---

## 📥 Los Usuarios Podrán Descargar

### Windows
```powershell
# Descargar gen-v2.0.0-x86_64-pc-windows-msvc.zip
# Extraer gen.exe
# Listo para usar
```

### Linux
```bash
wget https://github.com/TU_USUARIO/gen_v2/releases/download/v2.0.0/gen-v2.0.0-x86_64-unknown-linux-gnu.tar.gz
tar -xzf gen-v2.0.0-x86_64-unknown-linux-gnu.tar.gz
sudo mv gen /usr/local/bin/
gen --version  # ✓ Funciona en Ubuntu, Arch, Debian, etc.
```

### macOS
```bash
# Intel Macs
curl -L https://github.com/TU_USUARIO/gen_v2/releases/download/v2.0.0/gen-v2.0.0-x86_64-apple-darwin.tar.gz | tar -xz

# Apple Silicon (M1/M2/M3)
curl -L https://github.com/TU_USUARIO/gen_v2/releases/download/v2.0.0/gen-v2.0.0-aarch64-apple-darwin.tar.gz | tar -xz

sudo mv gen /usr/local/bin/
gen --version
```

---

## 🔍 Monitorear el Build

### En GitHub:
1. Ve a: `https://github.com/TU_USUARIO/gen_v2/actions`
2. Verás el workflow "Release Build" ejecutándose
3. Click para ver logs en tiempo real

### En tu terminal:
```bash
# Ver tags
git tag

# Ver último tag
git describe --tags

# Ver releases en GitHub (con gh CLI)
gh release list
```

---

## 📋 Checklist Final Antes del Release

- [ ] **Tests pasando**: `cargo test` → 18/18 ✓
- [ ] **Compila sin errores**: `cargo build --release` ✓
- [ ] **No hay debug output**: Sin `println!` innecesarios ✓
- [ ] **Documentación actualizada**: READMEs, guides ✓
- [ ] **CHANGELOG.md actualizado**: Con cambios de esta versión
- [ ] **Versión en Cargo.toml**: Coincide con el tag
- [ ] **Git clean**: `git status` sin cambios pendientes
- [ ] **GitHub Actions configurado**: `.github/workflows/release.yml` commiteado

---

## 🎯 Próximas Versiones

Para futuras releases:

```bash
# Bugfix (2.0.0 → 2.0.1)
./create_release.sh 2.0.1

# Nueva feature (2.0.0 → 2.1.0)
./create_release.sh 2.1.0

# Breaking change (2.0.0 → 3.0.0)
./create_release.sh 3.0.0
```

Recuerda actualizar `CHANGELOG.md` antes de cada release.

---

## 📚 Documentación Completa

- **Guía detallada**: Lee `RELEASE_GUIDE.md` para más información
- **Workflow de GitHub**: `.github/workflows/release.yml`
- **Changelog**: `CHANGELOG.md`

---

## 🐛 Si Algo Sale Mal

### El workflow falla
1. Ve a Actions en GitHub
2. Click en el workflow fallido
3. Revisa los logs
4. Corrige el error
5. Elimina el tag: `git tag -d v2.0.0 && git push --delete origin v2.0.0`
6. Crea nuevamente: `./create_release.sh 2.0.0`

### El release se crea sin assets
- Verifica que GitHub Actions tiene permisos
- Revisa los logs de cada job de compilación
- Asegúrate de que los paths en el workflow son correctos

### Tag ya existe
```bash
# Eliminar y recrear
git tag -d v2.0.0
git push --delete origin v2.0.0
git tag v2.0.0
git push origin v2.0.0
```

---

## ✨ ¡Estás Listo!

Para crear tu primer release, simplemente ejecuta:

```bash
./create_release.sh 2.0.0
```

O sigue las instrucciones manuales en `RELEASE_GUIDE.md`.

**¡Buena suerte con tu primer release!** 🚀

---

**Nota**: Recuerda reemplazar `TU_USUARIO` en los ejemplos con tu nombre de usuario de GitHub real.
