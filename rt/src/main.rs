// Copyright (c) 2016 <daggerbot@gmail.com>
// This software is available under the terms of the zlib license.
// See README.md for more information.

extern crate dylib;
extern crate env_logger;
extern crate monster_battle_core as core;
#[macro_use]
extern crate log;

mod os;
mod test;

use std::env;

use core::AppContext;

fn main () {
    env_logger::init().unwrap();

    let exe_path = env::current_exe().expect("can't get current executable path");
    let app = if cfg!(feature = "portable") {
        AppContext::new_portable(exe_path)
    } else {
        AppContext::new(exe_path)
    };

    test::test(app);
}
