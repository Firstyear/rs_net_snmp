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


extern crate toml;
extern crate rs_net_snmp;

use rs_net_snmp::SNMPVersion;
use rs_net_snmp::NetSNMP;
use rs_net_snmp::display_snmpresults;

#[derive(Debug)]
enum MonitorError {
    Unknown,
}

fn do_work(community: &str, version: SNMPVersion) -> Result<(), MonitorError> {

    let toml = r#"

[localhost]
".1.3.6.1.4.1.2021.9.1.100.8" = { expect = "0", fail = ".1.3.6.1.4.1.2021.9.1.101.8" }

["alina.ipa.blackhats.net.au"]
".1.3.6.1.4.1.2021.2.1.2.1" = { expect = "audispd", fail = ".1.3.6.1.4.1.2021.2.1.101.1" }
".1.3.6.1.4.1.2021.2.1.2.2" = { expect = "audispd", fail = ".1.3.6.1.4.1.2021.2.1.101.2" }

    "#;

    let value = toml::Parser::new(toml).parse().unwrap();

    for (key, value) in value.iter() {


        let mut rssnmp: NetSNMP = NetSNMP::new();
        // Are these okay to unwrap and panic? Or should we be better?
        rssnmp.set_version(version).unwrap();
        rssnmp.set_community(community).unwrap();

        let agent = "tcp6:".to_string() + key;
        println!("host: {:?}", agent);

        rssnmp.set_transport(&agent).unwrap();
        rssnmp.open_session().unwrap();

        for (oid, check) in value.as_table().unwrap().iter() {
            // println!("    oid: {:?} {:?}", oid, check);

            match rssnmp.get_oid(oid) {
                Ok(r) => {
                    display_snmpresults(oid, r);
                },
                Err(e) => {
                    println!("{:?}", e);
                },
            }
        };
        //    Open the host connection
        //    for each oid, get the value
        //    if it doesn't match, get the fail check oid too.
        //    append to (?) the host, oid, value, and if it exists, the fail value too.

        // This value is the set of oids to check

        rssnmp.destroy();

    };

    Ok(())
}


fn main() {
    do_work("public", SNMPVersion::VERSION_2c).unwrap();
}

