// The Config
#[derive(Debug)]
pub struct Config {
    pub author: Option<String>,
    pub author_email: Option<String>,
    pub templates_root: String,
    pub current_dir: String,
    pub arq_file: String
}