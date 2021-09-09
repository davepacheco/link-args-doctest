fn main() {
    println!("cargo:rerun-if-env-changed=TEST_LIBPQ_DIR");
    if let Ok(libpq_dir) = std::env::var("TEST_LIBPQ_DIR") {
        println!("cargo:warning=using libpq dir from environment: {}", libpq_dir);
        println!("cargo:rustc-link-arg=-Wl,-R{}", libpq_dir);
    } else {
        println!("cargo:warning=not specifying libpq dir");
    }
}
