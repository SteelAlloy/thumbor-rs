fn main() {
    // Copy the images to the output when generating documentation
    println!("cargo:rerun-if-changed=assets/doc");
    std::fs::create_dir_all("target/doc")
        .expect("Failed to create target/doc directory when building documentation.");
    std::fs::copy("assets/doc/logo.svg", "target/doc/logo.svg")
        .expect("Failed to copy crate logo when building documentation.");
}
