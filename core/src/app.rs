// Copyright (c) 2016 <daggerbot@gmail.com>
// This software is available under the terms of the zlib license.
// See README.md for more information.

use std::path::{Path, PathBuf};
use std::vec;

use os;

/// Application shared information which gets passed around during game initialization.
pub struct AppContext {
    exe_path: PathBuf,
    portable: bool,
}

impl AppContext {
    pub fn exe_dir (&self) -> &Path {
        self.exe_path.parent().expect("can't get directory of exe path")
    }

    pub fn exe_path (&self) -> &Path { self.exe_path.as_ref() }
    pub fn is_portable (&self) -> bool { self.portable }

    pub fn module_dirs (&self) -> vec::IntoIter<PathBuf> {
        if self.portable {
            vec!(self.exe_dir().join("modules")).into_iter()
        } else {
            self.os_module_dirs().into_iter()
        }
    }

    pub fn new<P: AsRef<Path>> (exe_path: P) -> AppContext {
        AppContext {
            exe_path: PathBuf::from(exe_path.as_ref()),
            portable: false,
        }
    }

    pub fn new_portable<P: AsRef<Path>> (exe_path: P) -> AppContext {
        AppContext {
            exe_path: PathBuf::from(exe_path.as_ref()),
            portable: true,
        }
    }

    pub fn save_dir (&self) -> PathBuf {
        if self.portable {
            self.exe_dir().join("save")
        } else {
            self.os_save_dir()
        }
    }
}

#[cfg(all(unix, not(target_os = "macos")))]
impl AppContext {
    fn os_module_dirs (&self) -> Vec<PathBuf> {
        os::unix::module_dirs()
    }

    fn os_save_dir (&self) -> PathBuf {
        os::unix::save_dir()
    }
}
