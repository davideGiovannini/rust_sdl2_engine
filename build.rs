use std::env;
use std::path::PathBuf;

fn main() {
    copy_runtimes_libs();
    generate_atlas_struct();
    generate_alto_buffer_struct()
}

fn copy_runtimes_libs() {
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

fn generate_atlas_struct() {
    use std::fs::File;
    use std::io::Write;
    use std::path::Path;
    let mut assets = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());

    assets.pop();
    assets.push("assets");
    assets.push("tiles");

    let o_dir = env::var("OUT_DIR").unwrap();
    let dest_path = Path::new(&o_dir).join("atlas.rs");
    let mut file = File::create(&dest_path).unwrap();

    if !assets.exists() {
        file.write_all(b"pub struct Atlas{}").unwrap();
        return;
    }

    file.write_all(
        b"use sdl2::render::Texture;
use sdl2::render::TextureCreator;
use sdl2::image::LoadTexture;
use sdl2::video::WindowContext;

pub struct Atlas{
",
    ).unwrap();

    let names: Vec<String> = std::fs::read_dir(assets)
        .expect("Can't read assets/tiles dir")
        .map(|x| x.unwrap().file_name().into_string().unwrap())
        .collect();

    for f in names.iter() {
        file.write_all(
            format!("    pub tex_{}: Texture,\n", f.trim_right_matches(".png")).as_bytes(),
        ).unwrap();
    }

    // Constructor
    file.write_all(
        b"}
impl Atlas{
    pub fn new(texture_creator: &TextureCreator<WindowContext>) -> Atlas{
        Atlas{
",
    ).unwrap();

    for f in names {
        file.write_all(
            format!(
                "    tex_{}: texture_creator.load_texture(\"./assets/tiles/{}\").expect(\"Could not load texture: {} !\"),\n",
                f.trim_right_matches(".png"),
                f,
                f
            ).as_bytes(),
        ).unwrap();
    }

    file.write_all(
        b"}
    }
}
",
    ).unwrap();
}





fn generate_alto_buffer_struct() {
    use std::fs::File;
    use std::io::Write;
    use std::path::Path;
    let mut assets = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());

    assets.pop();
    assets.push("assets");
    assets.push("sounds");

    let o_dir = env::var("OUT_DIR").unwrap();
    let dest_path = Path::new(&o_dir).join("alto.rs");
    let mut file = File::create(&dest_path).unwrap();

    if !assets.exists() {
        file.write_all(b"pub struct AudioResources{}").unwrap();
        return;
    }

    file.write_all(
        b"
use std::sync::Arc;
use alto::{Buffer, Context};
use super::alto_utils::load_buffer_from_ogg_file;

pub struct AudioResources{
",
    ).unwrap();

    let names: Vec<String> = std::fs::read_dir(assets)
        .expect("Can't read assets/sounds dir")
        .map(|x| x.unwrap().file_name().into_string().unwrap())
        .collect();

    for f in names.iter() {
        file.write_all(
            format!("    pub buf_{}: Arc<Buffer>,\n", f.trim_right_matches(".ogg")).as_bytes(),
        ).unwrap();
    }

    // Constructor
    file.write_all(
        b"}
impl AudioResources{
    pub fn new(context: &Context) -> Self{
        AudioResources{
",
    ).unwrap();

    for f in names {
        file.write_all(
            format!(
                "    buf_{}: Arc::new(load_buffer_from_ogg_file(\"./assets/sounds/{}\", context).expect(\"Could not load ogg: {} !\")),\n",
                f.trim_right_matches(".ogg"),
                f,
                f
            ).as_bytes(),
        ).unwrap();
    }

    file.write_all(
        b"}
    }
}
",
    ).unwrap();
}
