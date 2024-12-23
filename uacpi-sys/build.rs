use std::{
    env,
    error::Error,
    path::{Path, PathBuf},
};
use std::process::Command;

const SOURCES: &[&str] = &[
    "source/default_handlers.c",
    "source/event.c",
    "source/interpreter.c",
    "source/io.c",
    "source/mutex.c",
    "source/namespace.c",
    "source/notify.c",
    "source/opcodes.c",
    "source/opregion.c",
    "source/osi.c",
    "source/registers.c",
    "source/resources.c",
    "source/shareable.c",
    "source/sleep.c",
    "source/stdlib.c",
    "source/tables.c",
    "source/types.c",
    "source/uacpi.c",
    "source/utilities.c",
];

fn init_submodule(uacpi_path: &Path) {
    if !uacpi_path.join("README.md").exists() {
        Command::new("git")
            .args(["submodule", "update", "--init"])
            .current_dir(uacpi_path)
            .status()
            .expect("failed to retrieve uACPI sources with git");
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let project_dir = env::var_os("CARGO_MANIFEST_DIR").unwrap();
    let uacpi_path = Path::new(&project_dir).join("uacpi-src");

    init_submodule(&uacpi_path);

    let uacpi_path_str = uacpi_path.to_str().unwrap();

    let sources = SOURCES
        .iter()
        .map(|file| format!("{uacpi_path_str}/{file}"));

    let mut cc = cc::Build::new();

    cc.files(sources)
        .include(format!("{uacpi_path_str}/include"))
        .define("UACPI_SIZED_FREES", "1")
        .flag("-fno-stack-protector")
        .flag("-mgeneral-regs-only")
        .flag("-nostdlib")
        .flag("-ffreestanding");

    if cfg!(target_arch = "x86_64") || cfg!(target_arch = "x86") {
        cc.flag("-mno-red-zone");
    }

    if cfg!(feature = "reduced-hardware") {
        cc.define("UACPI_REDUCED_HARDWARE", "1");
    }

    cc.compile("uacpi");

    let bindings = bindgen::Builder::default()
        .header("wrapper.h")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .clang_args(&[
            "-Iuacpi-src/include",
            "-DUACPI_SIZED_FREES=1",
            #[cfg(feature = "reduced-hardware")]
            "-DUACPI_REDUCED_HARDWARE=1",
            "-ffreestanding",
        ])
        .prepend_enum_name(false)
        .use_core()
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");

    Ok(())
}
