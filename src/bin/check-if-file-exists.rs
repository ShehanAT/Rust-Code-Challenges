use std::path;
use std::fs;

trait FileMetadata {
    fn exists(&self) -> bool;

    fn is_writeable(&self) -> bool;

    fn is_readable(&self) -> bool;
}

impl FileMetadata for path::Path {
    fn is_readable(&self) -> bool {
        fs::File::open(&self).is_ok()
    }

    fn is_writeable(&self) -> bool {
        !fs::metadata(&self).unwrap().permissions().readonly()
    }

    fn exists(&self) -> bool {
        path::Path::new(&self).exists()
    }
}

fn main() {

}

#[test]
fn writeable() { 
    use tempfile;

    let f = tempfile::NamedTempFile::new().unwrap();
    assert!(f.path().is_writeable());

    fs::remove_file(f.path()).unwrap();
}

#[test]
fn read_only() {
    use tempfile;

    let f = tempfile::NamedTempFile::new().unwrap();
    let mut perms = fs::metadata(f.path()).unwrap().permissions();
    perms.set_readonly(true);
    fs::set_permissions(f.path(), perms).unwrap();
    assert_eq!(f.path().is_writeable(), false);

    
    println!("{:}", fs::remove_file(f.path()).unwrap_err());
}