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

#![warn(missing_docs)]

extern crate libc;

// use libc;

extern {
    fn rs_netsnmp_create_session() -> *const libc::c_void;
}

pub struct NetSNMP {
    // I think this just needs to track the pointer to the internal
    // struct for the native helper.
    netsnmp_session: *const libc::c_void,
}

impl NetSNMP {
    pub fn new() -> NetSNMP {
        unsafe {
            NetSNMP { netsnmp_session: rs_netsnmp_create_session() }
        }
    }
}

