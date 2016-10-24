// Copyright (c) 2016 <daggerbot@gmail.com>
// This software is available under the terms of the zlib license.
// See README.md for more information.

use std::fs::File;
use std::io::{self, Read};
use std::str::FromStr;

use core::{AppContext, ModuleQuery};
use serde::Deserialize;
use toml;

/// Info used when detecting ROMs.
#[derive(Deserialize)]
struct RomQueryInfo {
    len: u64,
    name_offset: usize,
    sha256: String,
}

/// Deserialized from 'roms.toml'.
#[derive(Deserialize)]
struct RomQueryFile {
    roms: Vec<RomQueryInfo>,
}

impl RomQueryFile {
    fn load (app: &AppContext) -> Option<RomQueryFile> {
        for path in app.module_dirs().map(|dir| dir.join("g1/roms.toml")) {
            let mut file = match File::open(&path) {
                Ok(file) => file,
                Err(err) => {
                    if err.kind() != io::ErrorKind::NotFound {
                        error!("can't open '{}': {}", path.display(), err);
                    }
                    continue;
                },
            };

            let mut src = String::new();

            if let Err(err) = file.read_to_string(&mut src) {
                error!("can't read '{}': {}", path.display(), err);
                continue;
            }

            let value = match toml::Value::from_str(src.as_str()) {
                Ok(value) => value,
                Err(err) => {
                    error!("can't parse '{}': {}", path.display(), err.first().unwrap());
                    continue;
                },
            };

            let mut decoder = toml::Decoder::new(value);
            let query_file = match RomQueryFile::deserialize(&mut decoder) {
                Ok(f) => f,
                Err(err) => {
                    error!("can't deserialize '{}': {}", path.display(), err);
                    continue;
                },
            };

            return Some(query_file);
        }

        None
    }
}

/// Detect supported game ROMs.
pub fn query_modules (app: &AppContext, _: &mut ModuleQuery) {
    let _query_file = RomQueryFile::load(app);
}
