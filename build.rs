//
// BEGIN COPYRIGHT BLOCK
// Copyright (C) 2016 William Brown
// All rights reserved.
//
// License: GPL (version 3 or any later version).
// See LICENSE for details. 
// END COPYRIGHT BLOCK
//
// Author: William Brown <wibrown@redhat.com>
//

extern crate gcc;

use std::env;

fn main() {
    // gcc::compile_library("libnetsnmpnative.a", &["src/rs_net_snmp/native.c"]);
    gcc::Config::new()
        .link("netsnmp")
        .file("src/rs_net_snmp/native.c")
        .compile("libnetsnmpnative.a");
}
