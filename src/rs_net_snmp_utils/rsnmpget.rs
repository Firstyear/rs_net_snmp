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
extern crate getopts;

use getopts::Options;
use std::env;

use rs_net_snmp::SNMPVersion;
use rs_net_snmp::display_snmpresults;
use rs_net_snmp::NetSNMP;


fn print_usage(program: &str, opts: Options) {
    let brief = format!("Usage: {} [options] AGENT OID", program);
    print!("{}", opts.usage(&brief));
}


/// This is a helper for getting the value.
/// by oid / agent / community / version for v1 and v2c
fn get_oid_1_2c(oid: &str, community: &str, agent: &str, version: SNMPVersion) {
    let mut rssnmp: NetSNMP = NetSNMP::new();
    // Are these okay to unwrap and panic? Or should we be better?
    rssnmp.set_version(version).unwrap();
    rssnmp.set_community(community).unwrap();
    rssnmp.set_transport(agent).unwrap();
    rssnmp.open_session().unwrap();

    match rssnmp.get_oid(oid) {
        Ok(r) => {
            display_snmpresults(oid, r);
        },
        Err(e) => {
            println!("{:?}", e);
        },
    }

    rssnmp.destroy();
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let program = args[0].clone();

    let mut opts = Options::new();
    opts.optopt("c", "community", "Set community name", "COMMUNITY");
    opts.optopt("v", "version", "Set version of snmp to use. valid 1, 2c, 3", "VERSION");
    opts.optflag("h", "help", "Print this help menu");

    let matches = match opts.parse(&args[1..]) {
        Ok(m) => { m }
        Err(f) => { panic!(f.to_string()) }
    };

    if matches.opt_present("h") {
        print_usage(&program, opts);
        return;
    }

    let version: SNMPVersion = match matches.opt_str("v") {
        Some(c) => {
            // Now convert the str to the enum.
            match c.as_ref() {
                "1" => SNMPVersion::VERSION_1,
                "2c" => SNMPVersion::VERSION_2c,
                "3" => SNMPVersion::VERSION_3,
                _ => {
                    // Print a better error here ....
                    print_usage(&program, opts);
                    return;
                }
            }
        }
        None => {
            print_usage(&program, opts);
            return;
        }
    };

    let agent = match matches.free.get(0) {
        Some(a) => { a.clone() }
        None => {
            print_usage(&program, opts);
            return;
        }
    };

    let oid = match matches.free.get(1) {
        Some(o) => { o.clone() }
        None => {
            print_usage(&program, opts);
            return;
        }
    };

    // We have the version, which dictates what we need next ....
    // Should this actually be an if?
    match version {
        SNMPVersion::VERSION_3 => {
            // Print a better error here ....
            print_usage(&program, opts);
            return;
        }
        _ => {
            // 1 and 2 basically take the same options.
            // We can just match on the needed options, return if they are none
            // Then we can trust the unwrap later ....
            // Though I consider there could be a better way.
            let community = match matches.opt_str("c") {
                Some(c) => { c }
                None => {
                    print_usage(&program, opts);
                    return;
                }
            };
            get_oid_1_2c(&oid, &community, &agent, version);

        }
    }

}
