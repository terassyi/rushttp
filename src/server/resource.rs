use std::path::{Path, PathBuf};
use std::fs::File;
use crate::server::error::Error;
use std::io::Read;

pub fn get_path(root: &str, path: &str) -> PathBuf {
    let root = Path::new(root);
    root.join(Path::new(path))
}

pub fn validate(root: &str, path: &str) -> bool {
    let root = Path::new(root); // root must be absolute path
    if !root.is_absolute() { return false; }
    let path = match Path::new(path).strip_prefix("../") {
        Ok(p) => p,
        Err(_) => Path::new(path),
    };
    let resource_path = root.join(path);
    resource_path.exists()
}

pub fn read(path: &str) -> Result<String, Error> {
    let mut file = File::open(path)?;
    let mut buf = [0u8; 256];
    file.read(&mut buf)?;
    Ok(String::from_utf8(buf.to_vec()).unwrap())
}

#[cfg(test)]
mod tests {
    use std::env;
    #[test]
    fn test_validate() {
        let cwd = env::current_dir().unwrap();
        let root = cwd.join(super::Path::new("src/static/assets/html"));
        assert_eq!(super::validate(root.to_str().unwrap(), "index.html"), true);
    }
    #[test]
    fn test_validate_invalid() {
        let cwd = env::current_dir().unwrap();
        let root = cwd.join(super::Path::new("src/static/assets/html"));
        assert_eq!(super::validate(root.to_str().unwrap(), "../../../server/resource.rs"), false);
    }
}