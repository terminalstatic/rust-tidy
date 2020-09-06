extern crate bindgen;
extern crate regex;

use regex::Regex;
use std::error::Error;
use std::fs::OpenOptions;
use std::io::prelude::*;

/*bindgen wrapper.h -o src/tidy.rs --rustified-enum '^Tidy.*' --whitelist-function '^tidy.*' --whitelist-var '^tidy.*'

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _TidyOption {
    pub TidyOptionId: ::std::os::raw::c_int,
    pub TidyConfigCategory: TidyConfigCategory,
    pub name: ctmbstr,
}

valgrind --leak-check=full --show-leak-kinds=all --track-origins=yes --verbose --log-file=valgrind-out.txt --sim-hints=no-nptl-pthread-stackcache target/debug/rust-tidy
*/
extern crate pkg_config;

fn main() -> Result<(), Box<dyn Error>> {
    pkg_config::Config::new()
        .atleast_version("5.2.0")
        .probe("tidy")
        .unwrap();

    let bindings = bindgen::Builder::default()
        .header("wrapper.h")
        // Tell cargo to invalidate the built crate whenever any of the
        // included header files changed.
        .rustified_enum("^Tidy.*")
        .whitelist_function("^tidy.*")
        .whitelist_var("^tidy.*")
        .layout_tests(false)
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        // Finish the builder and generate the bindings.
        .generate()
        // Unwrap the Result and panic on failure.
        .expect("Unable to generate bindings");

    bindings
        .write_to_file("src/tidy.rs")
        .expect("Couldn't write bindings!");

    let re = Regex::new(r"(?s)pub struct _TidyOption \{.+?\}").unwrap();
    let mut file_r = OpenOptions::new().read(true).open("src/tidy.rs")?;

    let mut contents = String::new();
    file_r.read_to_string(&mut contents)?;
    //println!("{}", contents);
    drop(file_r);
    assert!(re.is_match(&contents));

    let new_val = " pub struct _TidyOption {
        pub TidyOptionId: ::std::os::raw::c_int,
        pub TidyConfigCategory: TidyConfigCategory,
        pub name: ctmbstr,
    }";
    let replaced = re.replace(&contents, new_val);
    let mut file_w = OpenOptions::new()
        .write(true)
        .truncate(true)
        .open("src/tidy.rs")?;
    file_w.write(replaced.as_bytes())?;
    drop(file_w);

    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=path/to/Cargo.lock");
    println!("cargo:rerun-if-changed=wrapper.h");
    println!("cargo:rustc-link-lib=tidy");
    Ok(())
}
