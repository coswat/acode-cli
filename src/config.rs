pub struct Config<'a> {
    pub ts_template: &'a str,
    pub js_template: &'a str,
}

impl Config<'_> {
    pub fn new() -> Self {
        Self {
            ts_template: "https://github.com/Acode-Foundation/AcodeTSTemplate.git",
            js_template: "https://github.com/Acode-Foundation/acode-plugin.git",
        }
    }
}
