// Copyright (c) 2016 <daggerbot@gmail.com>
// This software is available under the terms of the zlib license.
// See README.md for more information.

use std::ffi::OsStr;
use std::env;
use std::os::unix::ffi::{OsStrExt, OsStringExt};
use std::path::PathBuf;

/// Get a list of possible module directories.
pub fn module_dirs () -> Vec<PathBuf> {
    let mut dirs = Vec::new();

    if let Some(xdg_data_home) = env::var_os("XDG_DATA_HOME") {
        let mut dir = PathBuf::from(xdg_data_home);
        dir.push(::GAME_ID);
        dir.push("modules");
        dirs.push(dir);
    } else if let Some(home) = env::var_os("HOME") {
        let mut dir = PathBuf::from(home);
        dir.push(".local/share");
        dir.push(::GAME_ID);
        dir.push("modules");
        dirs.push(dir);
    } else {
        panic!("HOME not found");
    }

    if let Some(xdg_data_dirs) = env::var_os("XDG_DATA_DIRS") {
        for dir in xdg_data_dirs.into_vec().split(|c| *c == b':') {
            let mut dir = PathBuf::from(OsStr::from_bytes(dir));
            dir.push(::GAME_ID);
            dir.push("modules");
            dirs.push(dir);
        }
    } else {
        dirs.push(PathBuf::from(format!("/usr/local/share/{}/modules", ::GAME_ID)));
        dirs.push(PathBuf::from(format!("/usr/share/{}/modules", ::GAME_ID)));
    }

    dirs
}

/// Get the save directory path.
pub fn save_dir () -> PathBuf {
    if let Some(xdg_config_home) = env::var_os("XDG_CONFIG_HOME") {
        let mut dir = PathBuf::from(xdg_config_home);
        dir.push(::GAME_ID);
        dir
    } else if let Some(home) = env::var_os("HOME") {
        let mut dir = PathBuf::from(home);
        dir.push(".config");
        dir.push(::GAME_ID);
        dir
    } else {
        panic!("HOME not found");
    }
}
