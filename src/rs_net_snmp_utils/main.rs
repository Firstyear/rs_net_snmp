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
use rs_net_snmp::SNMPVersion;

fn main() {
    // Create a new session
    let rssnmp: NetSNMP = NetSNMP::new();
    // Are these okay to unwrap and panic? Or should we be better?
    rssnmp.set_community("public").unwrap();
    rssnmp.set_version(SNMPVersion::VERSION_2c).unwrap();
    rssnmp.destroy();
}
