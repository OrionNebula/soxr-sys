use bindgen::{Builder, CargoCallbacks, callbacks::IntKind};
use std::env;
use std::path::PathBuf;

#[derive(Debug)]
struct CompatCallbacks;

impl bindgen::callbacks::ParseCallbacks for CompatCallbacks {
    fn int_macro(&self, _name: &str, _value: i64) -> Option<IntKind> {
        if cfg!(feature = "compatibility") {
            None
        } else {
            // All macros are flags, and they're type u64
            Some(IntKind::U64)
        }
    }

    fn item_name(&self, original_item_name: &str) -> Option<String> {
        // Rename soxr_datatype_t so compat mode can re-export it without name collision
        if original_item_name == "soxr_datatype_t" {
            Some("_soxr_datatype_t".to_owned())
        } else {
            None
        }
    }

    fn include_file(&self, filename: &str) {
        bindgen::callbacks::ParseCallbacks::include_file(&CargoCallbacks, filename)
    }
}

fn main() {
    add_link_settings();

    let bindings = Builder::default()
        .header("libsoxr/src/soxr.h")
        .default_enum_style(bindgen::EnumVariation::ModuleConsts)
        .parse_callbacks(Box::new(CompatCallbacks))
        .size_t_is_usize(cfg!(feature = "compatibility")) // Replace size_t with usize in compatibility mode
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings");
}

#[cfg(feature = "static")]
fn add_link_settings() {
    // Default distributions of libsoxr (at least through OSX Homebrew) don't include a static library - we have to make it ourselves

    let mut cmake = cmake::Config::new("libsoxr");
    cmake
        .define("CMAKE_BUILD_TYPE", "Release")
        .define("BUILD_TESTS", "OFF")
        .define("BUILD_EXAMPLES", "OFF")
        .define("WITH_OPENMP", "OFF")
        .define("WITH_LSR_BINDINGS", "OFF")
        .define("BUILD_SHARED_LIBS", "OFF")
        .define("WITH_DEV_TRACE", "OFF");

    println!("cargo:rustc-link-lib=static=soxr");

    let dst = cmake.build();
    println!(
        "cargo:rustc-link-search=native={}",
        dst.join("lib").display()
    );
}

#[cfg(not(feature = "static"))]
fn add_link_settings() {
    pkg_config::Config::new()
        .exactly_version("0.1.3")
        .probe("soxr")
        .unwrap();
}
