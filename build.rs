// build.rs

extern crate fs_extra;
extern crate foreman;
extern crate bindgen;

use std::process::Command;
use fs_extra::dir::{copy, CopyOptions};
use foreman::{LibKind, SearchKind};

fn main() {
    let out_dir = foreman::out_dir().unwrap();
    let vex_dir = out_dir.join("vex");

    {
        // Copy and build vex in OUT_DIR
        let options = CopyOptions { overwrite: false, skip_exist: true, buffer_size: 1<<16 };
        copy("vex", &out_dir, &options).unwrap();

        // i assume this is the right thing to do for windows?
        let makefile = if std::env::consts::OS == "windows" {
            "Makefile-msvc"
        } else {
            "Makefile-gcc"
        };

        Command::new("make").args(&["-f", makefile, "-j", &format!("{}", foreman::num_jobs().unwrap())])
            .current_dir(&vex_dir)
            .status().unwrap();

        // Tell rustc to link to libvex
        foreman::link_search(SearchKind::Native, &vex_dir);
        foreman::link_lib(LibKind::Static, "vex");
    }

    {
        // Generate bindings
        let bindings = bindgen::Builder::default()
            .header("wrapper.h")
            .blacklist_type("_IRStmt__bindgen_ty_1__bindgen_ty_1")
            .clang_arg(&format!("-I{}", &vex_dir.to_str().unwrap()))
            .generate()
            .expect("Unable to generate bindings");
        bindings.write_to_file(out_dir.join("bindings.rs"))
            .expect("Couldn't write bindings!");
    }
}
