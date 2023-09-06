pub struct Config<'a> {
    pub src_url: &'a str,
    pub ts_template: &'a str,
    pub js_template: &'a str,
    pub docs_url: &'a str,
}

impl Config<'_> {
    pub fn new() -> Self {
        Self {
            src_url: "https://github.com/coswat/acode-cli",
            ts_template: "https://github.com/bajrangCoder/AcodeTSTemplate.git",
            js_template: "https://github.com/deadlyjack/acode-plugin.git",
            docs_url: "https://acode.foxdebug.com/plugin-docs",
        }
    }
}
