use std::{env, path::Path};

fn main() {
    let dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    // TODO find programmatically
    let dirs = ["./", "deps/abseil-cpp/","deps/boost/","deps/Catch2/","deps/cista/","deps/CMakeFiles/","deps/date/","deps/doctest/","deps/fmt/","deps/geo/","deps/googletest/","deps/miniz/","deps/protobuf/","deps/res/","deps/unordered_dense/","deps/utl/","deps/wyhash/","deps/zlib/"];
    for d in &dirs {
        println!("cargo:rustc-link-search=all={}", Path::new(&dir).join("nigiri/build/").join(d).display());
    }
    println!("cargo:rustc-link-lib=nigiri");
    println!("cargo:rustc-link-lib=stdc++");
    println!("cargo:rustc-link-lib=geo");
    println!("cargo:rustc-link-lib=utl");
    println!("cargo:rustc-link-lib=fmt");
    println!("cargo:rustc-link-lib=miniz");
    println!("cargo:rustc-link-lib=date-tz");
    println!("cargo:rustc-link-lib=ianatzdb");
}
