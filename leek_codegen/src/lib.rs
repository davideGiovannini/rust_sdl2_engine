use std::env;
use std::path::PathBuf;

pub fn copy_runtimes_libs() {
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

/// Generate a resource key for every png in `assets/textures` and for every
/// ogg in `assets/sounds`.
/// Remember to add `!assets_gen();` in your main.rs to use them
pub fn generate_assets_keys() -> std::io::Result<()> {
    generate_texture_keys_consts()?;
    generate_audio_buffer_consts()
}

pub fn generate_texture_keys_consts() -> std::io::Result<()> {
    use std::fs::File;
    use std::io::Write;
    use std::path::Path;
    let mut assets = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());

    assets.push("assets");
    assets.push("textures");

    let o_dir = env::var("OUT_DIR").unwrap();
    let dest_path = Path::new(&o_dir).join("texture_file_keys.rs");
    let mut file = File::create(&dest_path)?;

    if !assets.exists() {
        return file.write_all(b"");
    }

    let names: Vec<String> = std::fs::read_dir(assets)
        .expect("Can't read assets/textures dir")
        .map(|x| x.unwrap().file_name().into_string().unwrap())
        .filter(|x| x.ends_with(".png"))
        .collect();

    for f in names {
        file.write_all(
            format!(
                "pub const TEX_{}: PathKey = PathKey(\"assets/textures/{}\");\n",
                f.trim_right_matches(".png").to_uppercase(),
                f,
            ).as_bytes(),
        )?;
    }
    Ok(())
}

pub fn generate_audio_buffer_consts() -> std::io::Result<()> {
    use std::fs::File;
    use std::io::Write;
    use std::path::Path;
    let mut assets = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());

    assets.push("assets");
    assets.push("sounds");

    let o_dir = env::var("OUT_DIR").unwrap();
    let dest_path = Path::new(&o_dir).join("audio_file_keys.rs");
    let mut file = File::create(&dest_path)?;

    if !assets.exists() {
        return file.write_all(b"");
    }

    let names: Vec<String> = std::fs::read_dir(assets)
        .expect("Can't read assets/sounds dir")
        .map(|x| x.unwrap().file_name().into_string().unwrap())
        .filter(|x| x.ends_with(".ogg"))
        .collect();

    for f in names {
        file.write_all(
            format!(
                "pub const AUDIO_{}: PathKey = PathKey(\"assets/sounds/{}\");\n",
                f.trim_right_matches(".ogg").to_uppercase(),
                f,
            ).as_bytes(),
        )?;
    }
    Ok(())
}
