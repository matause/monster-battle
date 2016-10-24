// Copyright (c) 2016 <daggerbot@gmail.com>
// This software is available under the terms of the zlib license.
// See README.md for more information.

mod app;
mod module;
mod os;

pub use app::AppContext;
pub use module::{ModuleInfo, ModuleQuery, ModuleQueryFn};

pub const GAME_ID: &'static str = "monster-battle";
pub const GAME_TITLE: &'static str = "Monster Battle";
