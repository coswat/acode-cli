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
            ts_template: "https://github.com/Acode-Foundation/AcodeTSTemplate.git",
            js_template: "https://github.com/Acode-Foundation/acode-plugin.git",
            docs_url: "https://docs.acode.app/",
        }
    }
}
