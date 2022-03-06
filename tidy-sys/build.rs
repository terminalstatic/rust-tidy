extern crate bindgen;
extern crate regex;

use glob::glob;
use glob::Paths;
use regex::Regex;
use std::error::Error;
use std::fs::OpenOptions;
use std::io::prelude::*;
use std::iter::Iterator;
use std::path;

extern crate pkg_config;

fn strip_to_include(mut paths: Paths, prefix: &str) -> Option<String> {
    let next = paths.next();
    match next {
        Some(v) => {
            let p = v.unwrap().into_os_string().into_string().unwrap();
            let mut r = p.trim_start_matches(prefix);
            if path::is_separator(r.chars().next().unwrap()) {
                r = &r[1..]
            }
            println!("Entry: {} {}", p, r);
            return Some(r.to_string());
        }
        _ => None,
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let out_fn = "src/bindings.rs";

    let lib = pkg_config::Config::new()
        .atleast_version("5.2.0")
        .probe("tidy")
        .unwrap();

    let out_dir = std::env::var("OUT_DIR").unwrap();
    dbg!(&lib.include_paths);
    if lib.include_paths.len() == 0 {
        panic!("No include dir found, can't find tidy.h or tidybuffio.h")
    }

    let h_files: [&str; 2] = ["tidy.h", "tidybuffio.h"];
    let mut includes: [Option<String>; 2] = Default::default();

    for (i, find) in h_files.iter().enumerate() {
        for dir in &lib.include_paths {
            let fileglob = dir.join("**").join(find);
            let mut i1 = strip_to_include(
                glob(fileglob.to_str().unwrap()).unwrap(),
                dir.clone().into_os_string().to_str().unwrap(),
            );
            if i1.is_some() {
                includes[i] = i1.take();
                break;
            }
        }
    }

    if !(includes[0].is_some() && includes[1].is_some()) {
        panic!("Required include files tidy.h or tidybuffio.h not found")
    }

    let wrapper_path = path::Path::new(&out_dir).join("wrapper.h");
    let mut file_w = OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .open(&wrapper_path)?;

    let h_text: String = format!(
        "#include <{}>\n#include <{}>\n",
        includes[0].as_ref().unwrap(),
        includes[1].as_ref().unwrap()
    );

    file_w.write(h_text.as_bytes())?;
    drop(file_w);

    let bindings = bindgen::Builder::default()
        .header(wrapper_path.to_path_buf().to_str().unwrap())
        .clang_arg(format!("{}{}", "-I", lib.include_paths[0]
        .clone()
        .into_os_string()
        .to_str()
        .unwrap()))
        .rustified_enum("^Tidy.*")
        .whitelist_function("^tidy.*")
        .whitelist_var("^tidy.*")
        .layout_tests(false)
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .generate()
        .expect("Unable to generate bindings");

    bindings
        .write_to_file(out_fn)
        .expect("Couldn't write bindings!");

    let re = Regex::new(r"(?s)pub struct _TidyOption \{.+?\}").unwrap();
    let mut file_r = OpenOptions::new().read(true).open(out_fn)?;

    let mut contents = String::new();
    file_r.read_to_string(&mut contents)?;

    drop(file_r);
    assert!(re.is_match(&contents));

    let new_val = " pub struct _TidyOption {
        pub TidyOptionId: ::std::os::raw::c_int,
        pub TidyConfigCategory: TidyConfigCategory,
        pub name: ctmbstr,
    }";
    let replaced = re.replace(&contents, new_val);
    let mut file_w = OpenOptions::new().write(true).truncate(true).open(out_fn)?;
    file_w.write(replaced.as_bytes())?;
    drop(file_w);

    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=path/to/Cargo.lock");
    println!("cargo:rerun-if-changed=wrapper.h");
    println!("cargo:rustc-link-lib=tidy");
    Ok(())
}
