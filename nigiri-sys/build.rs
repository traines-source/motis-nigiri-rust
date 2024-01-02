use std::{env, path::Path};
use std::fs::read_to_string;
use regex::Regex;

fn main() {
    let build_dir = env::var("CARGO_MANIFEST_DIR").unwrap()+"/nigiri/build/";

    let re = Regex::new(r"^.*lib(.*?)\.a").unwrap();
    let deps_file = format!("{build_dir}nigiri_deps.txt");

    for line in read_to_string(deps_file).unwrap().lines() {
        println!("cargo:rustc-link-search=all={}", Path::new(&build_dir).join(line).parent().expect("no parent").display());
        println!("cargo:rustc-link-lib={}", re.replace(line, "${1}"));
    }
    println!("cargo:rustc-link-lib=stdc++");
}