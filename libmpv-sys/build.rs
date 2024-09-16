use std::env;
use std::path::PathBuf;

#[cfg(not(feature = "use-bindgen"))]
fn main() {
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    let crate_path = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
    std::fs::copy(
        crate_path.join("pregenerated_bindings.rs"),
        out_path.join("bindings.rs"),
    )
    .expect("Couldn't find pregenerated bindings!");

    let mpv_flags = pkg_config::Config::new().probe("mpv").unwrap();
    for flag in mpv_flags.libs {
        println!("cargo:rustc-link-lib={}", flag);
    }
    for path in mpv_flags.link_paths {
        println!("cargo:rustc-link-search={}", path.display());
    }
    #[cfg(target_os = "macos")]
    {
        let sdl2_flags = pkg_config::Config::new().probe("SDL2").unwrap();
        for flag in sdl2_flags.libs {
            println!("cargo:rustc-link-lib={}", flag);
        }
        for path in sdl2_flags.link_paths {
            println!("cargo:rustc-link-search={}", path.display());
        }
    }
}

#[cfg(feature = "use-bindgen")]
fn main() {
    let bindings = bindgen::Builder::default()
        .formatter(bindgen::Formatter::Prettyplease)
        .header("include/client.h")
        .header("include/render.h")
        .header("include/render_gl.h")
        .header("include/stream_cb.h")
        .impl_debug(true)
        .opaque_type("mpv_handle")
        .opaque_type("mpv_render_context")
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");

    let mpv_flags = pkg_config::Config::new().probe("mpv").unwrap();
    for flag in mpv_flags.libs {
        println!("cargo:rustc-link-lib={}", flag);
    }
    for path in mpv_flags.link_paths {
        println!("cargo:rustc-link-search={}", path.display());
    }

    #[cfg(target_os = "macos")]
    {
        let sdl2_flags = pkg_config::Config::new().probe("SDL2").unwrap();
        for flag in sdl2_flags.libs {
            println!("cargo:rustc-link-lib={}", flag);
        }
        for path in sdl2_flags.link_paths {
            println!("cargo:rustc-link-search={}", path.display());
        }
    }
}
