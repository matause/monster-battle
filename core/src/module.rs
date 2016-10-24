// Copyright (c) 2016 <daggerbot@gmail.com>
// This software is available under the terms of the zlib license.
// See README.md for more information.

use std::any::Any;

use app::AppContext;

/// Information describing a game module.
pub struct ModuleInfo {
    pub name: String,
    pub data: Option<Box<Any + 'static>>,
}

/// Trait for adding available module info.
pub trait ModuleQuery {
    fn add_module (&mut self, info: ModuleInfo);
}

impl ModuleQuery for Vec<ModuleInfo> {
    fn add_module (&mut self, info: ModuleInfo) {
        self.push(info);
    }
}

/// Function signature for module query function.
pub type ModuleQueryFn = unsafe extern "C" fn (&AppContext, &mut ModuleQuery);
