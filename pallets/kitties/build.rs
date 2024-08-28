use std::fs::File;
use std::io::Write;

fn main() {
    // print not shown in the console.
    println!("cargo:rerun-if-changed=src/*");
    // but the temp file will be created, means build.sh is called.
    let mut file = File::create("build_ran.txt").unwrap();
    file.write_all(b"The build script ran!").unwrap();

    println!("cargo:rustc-env=SUBSTRATE_CLI_IMPL_VERSION=3.0.0");
}
