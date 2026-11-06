fn main() {
    // The release workflow's build number (About, update checks): absent in local builds.
    println!("cargo:rerun-if-env-changed=XTERMIUM_BUILD");
    // The commit a nightly build is made from (.github/workflows/nightly.yml).
    println!("cargo:rerun-if-env-changed=XTERMIUM_NIGHTLY");
    tauri_build::build()
}
