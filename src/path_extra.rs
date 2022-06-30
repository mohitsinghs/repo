use std::path::{Component, Path, PathBuf};

// Some helper methods to convert Path and PathBuf to String or str
pub trait PathExtra {
    fn x_base(&self) -> Option<String>;
    fn x_parent(&self) -> Option<String>;
    fn x_string(&self) -> String;
    fn x_str(&self) -> &str;
}

impl PathExtra for Path {
    fn x_base(&self) -> Option<String> {
        self.file_name()
            .and_then(|name| name.to_str())
            .map(|name| name.to_string())
    }

    fn x_parent(&self) -> Option<String> {
        self.parent()
            .and_then(|p| p.to_str())
            .map(|p| p.to_string())
    }

    fn x_str(&self) -> &str {
        self.to_str().unwrap_or_default()
    }

    fn x_string(&self) -> String {
        self.to_str().map(|p| p.to_string()).unwrap_or_default()
    }
}

impl PathExtra for PathBuf {
    fn x_base(&self) -> Option<String> {
        self.file_name()
            .and_then(|name| name.to_str())
            .map(|name| name.to_string())
    }

    fn x_parent(&self) -> Option<String> {
        self.parent()
            .and_then(|p| p.to_str())
            .map(|p| p.to_string())
    }

    fn x_str(&self) -> &str {
        self.to_str().unwrap_or_default()
    }

    fn x_string(&self) -> String {
        self.to_str().map(|p| p.to_string()).unwrap_or_default()
    }
}

// Some helper methods to convert Path and PathBuf
pub trait CompExtra {
    fn x_str(&self) -> &str;
}

impl CompExtra for Component<'_> {
    fn x_str(&self) -> &str {
        self.as_os_str().to_str().unwrap_or_default()
    }
}
