// Copyright (c) 2016 <daggerbot@gmail.com>
// This software is available under the terms of the zlib license.
// See README.md for more information.

#[cfg(unix)]
pub mod unix;

#[cfg(unix)]
pub use self::unix::is_lib_path;
