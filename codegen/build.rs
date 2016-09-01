#[cfg(not(feature = "serde_macros"))]
mod inner {
    extern crate serde_codegen;
    extern crate stopwatch;

    use std::env;
    use std::path::Path;

    pub fn main() {
        let out_dir = env::var_os("OUT_DIR").unwrap();

        let src = Path::new("src/botocore.in.rs");
        let dst = Path::new(&out_dir).join("botocore.rs");

        serde_codegen::expand(&src, &dst).unwrap();
    }
}
#[cfg(feature = "serde_macros")]
mod inner {
    extern crate stopwatch;
    pub fn main() {}
}

fn main() {
    inner::main();
}
