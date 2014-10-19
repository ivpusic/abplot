use std::io;
use std::io::fs;

/// creating directory for saving temp data
pub fn mk_tmp_dir() {
    let p = Path::new("._tmp");
    match fs::mkdir(&p, io::ALL_PERMISSIONS) {
        Ok(_) => (),
        Err(e) => fail!(e.desc)
    }
}

/// removing directory which was used for saving temp data
pub fn rm_tmp_dir() {
    let p = Path::new("._tmp");
    match fs::rmdir_recursive(&p) {
        Ok(_) => (),
        Err(e) => fail!(e.desc)
    }
}
