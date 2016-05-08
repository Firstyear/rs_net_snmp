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
    let mut rssnmp: NetSNMP = NetSNMP::new();
    // Are these okay to unwrap and panic? Or should we be better?
    rssnmp.set_version(SNMPVersion::VERSION_2c).unwrap();
    rssnmp.set_community("public").unwrap();
    rssnmp.set_transport("tcp6:localhost").unwrap();
    rssnmp.open_session().unwrap();
    rssnmp.get_oid(".1.3.6.1.2.1.1.5.0").unwrap();
    rssnmp.get_oid(".1.3.6.1.2.1.1.9.1.4.1").unwrap();
    rssnmp.get_oid(".1.3.6.1.2.1.1.9.1.2.7").unwrap();
    // rssnmp.get_oid(".1.3.6.1.2.1.1.5.0").unwrap();
    rssnmp.destroy();
}
