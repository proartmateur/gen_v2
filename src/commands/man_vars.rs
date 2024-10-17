use gen::environment::env_vars_doc::env_vars_doc;
pub fn man_vars() {
    print!("{}", env_vars_doc());
}