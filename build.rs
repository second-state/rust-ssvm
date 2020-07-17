// Copyright (C) 2020 Second State.
// This file is part of Rust-SSVM.

// Rust-SSVM is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as
// published by the Free Software Foundation, either version 3 of the
// License, or (at your option) any later version.

// Rust-SSVM is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU Affero General Public License for more details.

// You should have received a copy of the GNU Affero General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.

use std::path::Path;
extern crate cmake;
use cmake::Config;

fn build_link_ssvm_dylib() {
    let dst = Config::new("ssvm-evmc").no_build_target(true).build();
    let evmcssvm_path = Path::new(&dst).join("build/tools/ssvm-evmc");
    println!("cargo:rustc-link-search=native={}", evmcssvm_path.display());
    println!("cargo:rustc-link-lib=dylib=ssvm-evmc");
}

fn main() {
    build_link_ssvm_dylib();
}
