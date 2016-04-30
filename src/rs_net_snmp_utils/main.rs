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

extern crate rs_net_snmp;

use rs_net_snmp::NetSNMP;

fn main() {
    // Create a new session
    let rsnsmp: NetSNMP = NetSNMP::new();
}
