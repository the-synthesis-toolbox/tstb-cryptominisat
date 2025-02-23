// Copyright (c) 2016 Robert Grosse

// Permission is hereby granted, free of charge, to any person obtaining a copy of this software and associated documentation files (the "Software"), to deal in the Software without restriction, including without limitation the rights to use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of the Software, and to permit persons to whom the Software is furnished to do so, subject to the following conditions:

// The above copyright notice and this permission notice shall be included in all copies or substantial portions of the Software.

// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.

extern crate cmake;

use cmake::Config;

fn main() {
    let mut cfg = Config::new("cryptominisat");
    cfg.define("CMAKE_BUILD_TYPE", "Release")
        .define("STATICCOMPILE", "ON")
        .define("ENABLE_PYTHON_INTERFACE", "OFF")
        .define("ONLY_SIMPLE", "ON")
        .define("NOZLIB", "ON")
        .define("STATS", "OFF")
        .define("NOVALGRIND", "ON")
        .define("ENABLE_TESTING", "OFF");
    #[cfg(feature = "largemem")]
    {
        cfg.define("LARGEMEM", "ON");
    }
    #[cfg(feature = "m4ri")]
    {
        cfg.define("NOM4RI", "OFF");
    }
    #[cfg(not(feature = "m4ri"))]
    {
        cfg.define("NOM4RI", "ON");
    }
    let dst = cfg.build();
    //println!("cargo:rustc-flags=-L {}/lib", dst.display());
    println!("cargo:rustc-link-search=native={}/lib", dst.display());
    println!("cargo:rustc-link-lib=static=cryptominisat5");

    #[cfg(any(target_os = "macos", target_os = "openbsd"))]
    println!("cargo:rustc-flags=-l dylib=c++");

    #[cfg(target_os = "linux")]
    println!("cargo:rustc-flags=-l dylib=stdc++");
}
