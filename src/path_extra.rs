use std::path::Path;

pub trait PathExtra {
    fn ex_base(&self) -> Option<String>;
    fn ex_parent(&self) -> Option<String>;
    fn ex_string(&self) -> String;
}

impl PathExtra for Path {
    fn ex_base(&self) -> Option<String> {
        self.file_name()
            .and_then(|name| name.to_str())
            .map(|name| name.to_string())
    }

    fn ex_parent(&self) -> Option<String> {
        self.parent()
            .and_then(|p| p.to_str())
            .map(|p| p.to_string())
    }

    fn ex_string(&self) -> String {
        self.to_str().map(|p| p.to_string()).unwrap_or_default()
    }
}
