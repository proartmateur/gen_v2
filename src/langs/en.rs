use std::collections::HashMap;
use std::sync::LazyLock;

pub static EN_STRINGS: LazyLock<HashMap<&'static str, &'static str>> = LazyLock::new(|| {
    HashMap::from([
        ("app_info.description", r#"Make your project faster with your own architecture!"#),
        ("app_info.credits", "Developed by author with ❤️ from México. year"),
        ("init.already_exists", "already exists!"),
        ("init.mkdir_error", "Error to make dir"),
        ("init.write_file_error", "could not create file"),
        ("init.ready_for_init", "Ready for arq"),
        ("arq.is_ready", "is ready!"),
        ("arq.writing", "Writing:"),
        ("arq.error_make_atom_dir", "Error: Could not create atom directory."),
        ("arq.panic_gen_dir", "Error: Not found: .gen_cli/templates.\n Try to run: gen init"),
        ("template.reading", "📖 Reading template:"),
        ("template.processing", "⚙️  Processing:"),
        ("template.creating_dir", "📁 Creating directory:"),
        ("template.writing_file", "✍️  Writing file:"),
        ("template.file_exists", "⚠️  File already exists:"),
        ("template.overwrite_prompt", "Do you want to overwrite? [y=yes, n=no, a=abort]:"),
        ("template.skipping", "⏭️  Skipping:"),
        ("template.aborting", "❌ Operation aborted by user."),
        ("template.success", "✅ Successfully generated:"),
        ("template.error_read", "❌ Error reading template:"),
        ("template.error_write", "❌ Error writing file:"),
        ("template.error_mkdir", "❌ Error creating directory:"),
        ("template.per_prop_gen", "📦 Generating files per property:"),
        ("template.imports_added", "📥 Imports added:"),
        ("template.completed", "🎉 Generation completed!"),
    ])
});