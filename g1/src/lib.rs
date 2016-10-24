// Copyright (c) 2016 <daggerbot@gmail.com>
// This software is available under the terms of the zlib license.
// See README.md for more information.

#![feature(proc_macro)]

extern crate monster_battle_core as core;
extern crate env_logger;
#[macro_use]
extern crate log;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate toml;

mod query;

use core::{AppContext, ModuleQuery, ModuleQueryFn};

/// Exported query function.
#[no_mangle]
pub unsafe extern "C" fn mb_query_modules (app: &AppContext, query: &mut ModuleQuery) {
    env_logger::init().unwrap();
    query::query_modules(app, query);
}

/// Assert at compile time that `query_modules` has the correct signature.
const _MODULE_QUERY_FN: ModuleQueryFn = mb_query_modules;
