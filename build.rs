fn main() -> miette::Result<()> {
    let path = std::path::PathBuf::from("src"); // include path
    let path2 = std::path::PathBuf::from("./decklink/Mac/include/");
    let mut b = autocxx_build::Builder::new("src/main.rs", &[&path, &path2])
    .extra_clang_args(&["-isysroot/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX14.2.sdk"])
    .build()?;

    b.flag_if_supported("-std=c++14")
        .file("./decklink/Mac/include/DeckLinkAPIDispatch.cpp")
        .compile("autocxx-demo"); // arbitrary library name, pick anything

    println!("cargo:rustc-link-lib=framework=CoreFoundation");
    println!("cargo:rerun-if-changed=src/main.rs");
    // Add instructions to link to any C++ libraries you need.
    Ok(())
}
