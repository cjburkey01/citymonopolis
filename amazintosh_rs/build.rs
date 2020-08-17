use gl_generator::{Api, Fallbacks, Profile, Registry, StructGenerator};
use std::env;
use std::fs::File;
use std::path::Path;

fn main() {
    let mut file = File::create(
        &Path::new(&env::var("OUT_DIR").expect("failed to get OUT_DIR environment variable"))
            .join("bindings.rs"),
    )
    .expect("Failed to get output bindings file");

    Registry::new(Api::Gl, (3, 3), Profile::Core, Fallbacks::All, [])
        .write_bindings(StructGenerator, &mut file)
        .expect("failed to write Rust OpenGL bindings");
}
