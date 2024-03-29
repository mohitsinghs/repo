use std::path::{Component, Path, PathBuf};

// Some helper methods to convert Path and PathBuf to String or str
pub trait PathExtra {
    fn base(&self) -> Option<String>;
    fn to_string(&self) -> String;
    fn base_lowercase(&self) -> String;
}

impl PathExtra for Path {
    fn base(&self) -> Option<String> {
        self.file_name()
            .and_then(|name| name.to_str())
            .map(|name| name.to_string())
    }

    fn to_string(&self) -> String {
        self.to_str().map(|p| p.to_string()).unwrap_or_default()
    }

    fn base_lowercase(&self) -> String {
        self.base().map(|p| p.to_lowercase()).unwrap_or_default()
    }
}

impl PathExtra for PathBuf {
    fn base(&self) -> Option<String> {
        self.file_name()
            .and_then(|name| name.to_str())
            .map(|name| name.to_string())
    }

    fn to_string(&self) -> String {
        self.to_str().map(|p| p.to_string()).unwrap_or_default()
    }

    fn base_lowercase(&self) -> String {
        self.base().map(|p| p.to_lowercase()).unwrap_or_default()
    }
}

// Some helper methods to convert Path and PathBuf
pub trait CompExtra {
    fn to_str(&self) -> &str;
}

impl CompExtra for Component<'_> {
    fn to_str(&self) -> &str {
        self.as_os_str().to_str().unwrap_or_default()
    }
}
