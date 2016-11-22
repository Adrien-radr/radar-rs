use std::path::Path;
use std::fs::File;
use std::io::prelude::*;
use std::env;

pub fn check_extension(path: &Path, valid_ext: &[&str]) -> bool {
    let ext = &path.extension().expect("The file has no extension").to_str().expect("Extension is not valid utf8");

    for vext in valid_ext.iter() {
        if vext == ext {
            return true;
        }
    }
    false
}

pub fn read_file(path_str: &str) -> String {
    let mut path = env::current_dir().unwrap();
    path.push(Path::new(path_str));
    

    if !path.exists() {
        panic!("Error reading file, path {} doesn't exist.", path.display());
    }

    let mut f = match File::open(&path) {
        Ok(f) => f,
        Err(msg) => panic!("Error reading file {} : {}.", path.display(), msg)
    };

    // read bytes and return as str
    let mut bytes = Vec::new();
    match f.read_to_end(&mut bytes) {
        Ok(_) => {
            match String::from_utf8(bytes) {
                Ok(s) => s,
                Err(msg) => panic!("Found non valid utf8 characters in {} : {}.", path.display(), msg)
            }
        },
        Err(msg) => panic!("Error reading file {} : {}.", path.display(), msg)
    }
}