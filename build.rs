use std::{env, fs, process};
use std::path::Path;

use dotenv::dotenv;

fn main() {
    dotenv().ok();

    let sdl2_lib_path = match env::var("SDL2_LIB_PATH") {
        Ok(result) => result,
        Err(e) => {
            println!("Failed to parse env variable SDL2_LIB_PATH: {}", e);
            process::exit(-1);
        },
    };

    let sdl2_lib_name = match env::var("SDL2_LIB_NAME") {
        Ok(result) => result,
        Err(e) => {
            println!("Failed to parse env variable SDL2_LIB_NAME: {}", e);
            process::exit(-1);
        },
    };

    let target_dir = match env::var("TARGET_DIR") {
        Ok(result) => result,
        Err(e) => {
            println!("Failed to parse env variable TARGET_DIR: {}", e);
            process::exit(-1);
        },
    };

    let dll_path = Path::new(&sdl2_lib_path).join(&sdl2_lib_name);
    let target_lib_path = Path::new(&target_dir).join(&sdl2_lib_name);

    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed={}", dll_path.display());
    println!("cargo:rustc-link-search=native={}", sdl2_lib_path);

    match fs::copy(&dll_path, &target_lib_path) {
        Ok(_) => {
            println!("Successfully copied the DLL to {}", target_lib_path.display());
        },
        Err(e) => {
            println!("Failed to copy the DLL: {}", e);
            process::exit(-1);
        },
    }
}