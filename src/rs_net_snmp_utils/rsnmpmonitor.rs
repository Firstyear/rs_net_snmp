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


//// TODO WILLIAM:
// Make command line switch for filepath
// optional: Make it show single value outputs too? So we can link expected proc names, hostname, etc.
// That will be a change to check_host, where check is None


extern crate toml;
extern crate rs_net_snmp;

use rs_net_snmp::SNMPVersion;
use rs_net_snmp::NetSNMP;
use rs_net_snmp::SNMPResult;

use std::fs::File;
// use std::io;
use std::io::prelude::*;

use std::process::exit;


// This condenses all the possible options to a success or not
// Is there a better way to write this string?
fn get_oid_or_alt(rssnmp: &mut NetSNMP, oid: &str, expect: &toml::Value, altoid: &str) -> (bool, String) {
    let mut status = format!("{} -> ", oid);
    let success = match rssnmp.get_oid(oid) {
        Ok(r) => {
            // Now we need to process the possible options.
            if r.is_empty() {
                // There is no oid, so it must be okay.
                status.push_str("No Values");
                true
            } else {
                let result = r.first().unwrap();
                // How can we check T against this type?

                // I'm not sure I like this inner assignment, but it makes formatting and flow nicer ....

                let inner_success = match result {
                    &SNMPResult::AsnOctetStr { s: ref sv} => {
                        status.push_str(&format!("{}", *sv));
                        *sv == expect.as_str().unwrap()
                    },
                    &SNMPResult::AsnInteger { i: ref iv} => {
                        status.push_str(&format!("{}", *iv));
                        *iv == expect.as_integer().unwrap() as isize
                    },
                    &SNMPResult::AsnTimeticks { i: ref iv} => {
                        status.push_str(&format!("{}", *iv));
                        *iv == expect.as_integer().unwrap() as isize
                    },
                };

                status.push_str(&format!(" status {}", inner_success));

                // This finishes the println! above for the status
                inner_success
            }
        },
        Err(e) => {
            status.push_str(&format!("Error: {:?}", e));
            false
        },
    };

    if !success {
        status.push_str(&format!("\n    fail {} -> ", altoid));
        match rssnmp.get_oid(altoid) {
            Ok(r) => {
                // Now we need to process the possible options.
                if r.is_empty() {
                    // There is no oid, so it must be okay.
                    status.push_str("NO DATA")
                } else {
                    let result = r.first().unwrap();
                    // How can we check T against this type?

                    match result {
                        &SNMPResult::AsnOctetStr { s: ref sv} => {
                            status.push_str(&format!("{}", sv))
                        },
                        &SNMPResult::AsnInteger { i: ref iv} => {
                            status.push_str(&format!("{}", iv))
                        },
                        &SNMPResult::AsnTimeticks { i: ref iv} => {
                            status.push_str(&format!("{}", iv))
                        },
                    }
                }
            },
            Err(e) => {
                status.push_str(&format!("Error: {:?}", e));
            }
        }
    }

    (success, status)
}

fn get_toml_data() -> String {
    let mut input = String::new();
    // io::stdin().read_to_string(&mut input).unwrap();
    File::open("monitor.toml").and_then(|mut f| {
        f.read_to_string(&mut input)
    }).unwrap();
    input
}

fn check_host(hostname: &str, value: &toml::Value, community: &str, version: SNMPVersion) -> bool {

    let mut success = true;

    let mut rssnmp: NetSNMP = NetSNMP::new();
    // Are these okay to unwrap and panic? Or should we be better?
    rssnmp.set_version(version).unwrap();
    rssnmp.set_community(community).unwrap();

    let agent = "tcp6:".to_string() + hostname;

    rssnmp.set_transport(&agent).unwrap();

    match rssnmp.open_session() {
        Ok(()) => {
            for (oid, check) in value.as_table().unwrap().iter() {
                let t = check.as_table().unwrap();
                let expect = t.get("expect").unwrap();
                let altoid = t.get("fail").unwrap().as_str().unwrap();

                // println!("    oid: {:?} {:?}", oid, check);
                let (res, status) = get_oid_or_alt(&mut rssnmp, oid, expect, altoid);

                if res {
                    println!("{} : {}", agent, status);
                } else {
                    success = false;
                    writeln!(&mut std::io::stderr(), "{} : {}", agent, status).unwrap();
                }

            };
        },
        Err(e) => {
            success = false;
            println!("Error: {:?}", e);
        }
    }

    //    append to (?) the host, oid, value, and if it exists, the fail value too.


    rssnmp.destroy();

    success
}

fn do_work(community: &str, version: SNMPVersion) -> bool {
    let toml = get_toml_data();
    let mut success = true;

    match toml::Parser::new(&toml).parse() {
        Some(value) => {
            for (hostname, value) in value.iter() {
                if !check_host(hostname, value, community, version) {
                    success = false;
                }
            }
        }
        None => {
            println!("No values in toml. Please check the file!");
        }
    }
    success
}


fn main() {
    if !do_work("public", SNMPVersion::VERSION_2c) {
        exit(1);
    }
}

