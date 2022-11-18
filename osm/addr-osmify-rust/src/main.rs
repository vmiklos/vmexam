/*
 * Copyright 2019 Miklos Vajna. All rights reserved.
 * Use of this source code is governed by a BSD-style license that can be
 * found in the LICENSE file.
 */

#![deny(warnings)]
#![warn(clippy::all)]
#![warn(missing_docs)]

//! Commandline interface to addr_osmify.

use std::sync::Arc;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let urllib: Arc<dyn addr_osmify::Network> = Arc::new(addr_osmify::StdNetwork {});
    std::process::exit(addr_osmify::main(args, &mut std::io::stdout(), &urllib))
}
