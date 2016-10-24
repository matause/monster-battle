// Copyright (c) 2016 <daggerbot@gmail.com>
// This software is available under the terms of the zlib license.
// See README.md for more information.

use std::os::unix::ffi::OsStrExt;
use std::path::Path;

/// Determines whether a path appears to specify a library.
pub fn is_lib_path<P: AsRef<Path>> (path: P) -> bool {
    match path.as_ref().file_name() {
        None => false,
        Some(name) => {
            let name = name.as_bytes();
            name.starts_with(b"lib") && name.ends_with(b".so")
        },
    }
}
