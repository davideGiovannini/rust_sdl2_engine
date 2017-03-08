use std::env;
use std::path::PathBuf;

fn main() {
    let target = env::var("TARGET").unwrap();
    let mut out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    let manifest_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
    let mut dll_dir = manifest_dir.clone();

    if target == "x86_64-pc-windows-gnu" {
        dll_dir.push("runtimes");
        dll_dir.push("win64");
    } else if target == "i686-pc-windows-gnu" {
        dll_dir.push("runtimes");
        dll_dir.push("win32");
    }
    if target.contains("pc-windows") {
        out_dir.pop();
        out_dir.pop();
        out_dir.pop();

        for entry in std::fs::read_dir(dll_dir).expect("Can't read DLL dir") {
            let entry_path = entry.expect("Invalid fs entry").path();
            let file_name_result = entry_path.file_name();
            let mut new_file_path = out_dir.clone();
            if let Some(file_name) = file_name_result {
                new_file_path.push(file_name.to_str().unwrap());
                std::fs::copy(&entry_path, new_file_path.as_path())
                    .expect("Can't copy from DLL dir");
            }
        }
    }
}
