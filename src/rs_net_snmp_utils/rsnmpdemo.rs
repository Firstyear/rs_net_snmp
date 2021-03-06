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
    let mut rssnmp: NetSNMP = NetSNMP::new();
    rssnmp.set_version(SNMPVersion::VERSION_2c).unwrap();
    rssnmp.set_community("public").unwrap();
    rssnmp.set_transport("tcp6:localhost").unwrap();
    rssnmp.open_session().unwrap();
    println!("SNMPv2-MIB::sysName.0 = {:?}", rssnmp.get_oid("SNMPv2-MIB::sysName.0").unwrap() );
    rssnmp.destroy();
}
