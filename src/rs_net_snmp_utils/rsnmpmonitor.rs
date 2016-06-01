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
use rs_net_snmp::SNMPResult;
use rs_net_snmp::display_snmpresults;

use std::fs::File;
// use std::io;
use std::io::prelude::*;

#[derive(Debug)]
enum MonitorError {
    Unknown,
}

// Would this be an option? Or a result? 
fn get_oid(rssnmp: &mut NetSNMP, oid: &str) {
    match rssnmp.get_oid(oid) {
        Ok(r) => {
            display_snmpresults(oid, r);
            //    if it doesn't match, get the fail check oid too.
        },
        Err(e) => {
            println!("{:?}", e);
        },
    }
}

// This condenses all the possible options to a success or not
fn get_oid_or_alt<T>(rssnmp: &mut NetSNMP, oid: &str, expect: T, altoid: &str) {
    println!("get_oid_or_alt {} else {}", oid, altoid);
    // Rewrite this to "and then"
    match rssnmp.get_oid(oid) {
        Ok(r) => {
            // Unwrap the option
            match r {
                Some(v) => {
                    println!("{:?}", v);
                },
                None => {
                    println!("No value");
                }
            }
        },
        Err(e) => {
            println!("{:?}", e);
        },
    }
}

fn get_toml_data() -> String {
    let mut input = String::new();
    // io::stdin().read_to_string(&mut input).unwrap();
    File::open("monitor.toml").and_then(|mut f| {
        f.read_to_string(&mut input)
    }).unwrap();
    input
}

fn check_host(hostname: &str, value: &toml::Value, community: &str, version: SNMPVersion) -> Result<(), MonitorError> {

    let mut success = true;

    let mut rssnmp: NetSNMP = NetSNMP::new();
    // Are these okay to unwrap and panic? Or should we be better?
    rssnmp.set_version(version).unwrap();
    rssnmp.set_community(community).unwrap();

    let agent = "tcp6:".to_string() + hostname;
    println!("host: {:?}", agent);

    rssnmp.set_transport(&agent).unwrap();

    match rssnmp.open_session() {
        Ok(()) => {
            for (oid, check) in value.as_table().unwrap().iter() {
                let t = check.as_table().unwrap();
                let expect = t.get("expect").unwrap();
                let altoid = t.get("fail").unwrap().as_str().unwrap();

                println!("    oid: {:?} {:?}", oid, check);
                get_oid_or_alt(&mut rssnmp, oid, expect, altoid);

            };
        },
        Err(e) => {
            success = false;
            println!("{:?}", e);
        }
    }

    //    append to (?) the host, oid, value, and if it exists, the fail value too.


    rssnmp.destroy();

    println!("{}", success);

    Ok(())
}

fn do_work(community: &str, version: SNMPVersion) {
    let toml = get_toml_data();

    let value = toml::Parser::new(&toml).parse().unwrap();

    for (hostname, value) in value.iter() {
        check_host(hostname, value, community, version);
    }
}


fn main() {
    do_work("public", SNMPVersion::VERSION_2c);
}

