// Copyright (c) 2016 <daggerbot@gmail.com>
// This software is available under the terms of the zlib license.
// See README.md for more information.

use std::io;
use std::mem;
use std::path::Path;

use core::{AppContext, ModuleInfo, ModuleQueryFn};
use dylib::DynamicLibrary;

use os;

pub fn test (app: AppContext) {
    let mut modules = Vec::new();

    for dir in app.module_dirs() {
        match dir.read_dir() {
            Ok(read_dir) => {
                for ent in read_dir.map(|r| r.unwrap()) {
                    let path = ent.path();
                    let meta = ent.metadata().unwrap();

                    if meta.is_file() && os::is_lib_path(&path) {
                        query_lib(&app, &mut modules, path);
                    }
                }
            },

            Err(err) => {
                if err.kind() != io::ErrorKind::NotFound {
                    error!("can't read dir '{}': {}", dir.display(), err);
                }
            },
        }
    }
}

fn query_lib<P: AsRef<Path>> (app: &AppContext, modules: &mut Vec<ModuleInfo>, path: P) {
    let path = path.as_ref();

    let lib = match DynamicLibrary::open(Some(path)) {
        Ok(lib) => lib,
        Err(err) => {
            error!("can't load library '{}': {}", path.display(), err);
            return;
        },
    };

    let query_fn: ModuleQueryFn;

    unsafe {
        let ptr: *mut () = match lib.symbol("mb_query_modules") {
            Ok(ptr) => ptr,
            Err(err) => {
                error!("can't query modules for '{}': {}", path.display(), err);
                return;
            },
        };

        query_fn = mem::transmute(ptr);
        query_fn(app, modules);
    }
}
