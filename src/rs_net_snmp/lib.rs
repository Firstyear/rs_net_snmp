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

use std::os::raw::c_char;
use std::ffi::CString;


// use libc;

#[repr(C)]
#[derive(Debug)]
pub enum SNMPVersion {
    VERSION_1,
    VERSION_2c,
    VERSION_3,
}

#[derive(Debug)]
pub enum SNMPError {
    NoMemory,
    Unknown,
}

extern {
    fn rs_netsnmp_create_session() -> *const libc::c_void;
    fn rs_netsnmp_set_version(session: *const libc::c_void, version: isize) -> isize;
    fn rs_netsnmp_set_community(session: *const libc::c_void, community: *const c_char) -> isize;
    fn rs_netsnmp_set_peername(session: *const libc::c_void, hostname: *const c_char) -> isize;
    fn rs_netsnmp_get_peername(session: *const libc::c_void) -> *mut c_char;
    fn rs_netsnmp_destroy_session(session: *const libc::c_void);
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

    pub fn set_version(&self, version: SNMPVersion) -> Result<(), SNMPError> {
        // Need to make the enum able to call the right version
        // set thing.
        unsafe {
            match rs_netsnmp_set_version(self.netsnmp_session, version as isize) {
                0 => Ok(()),
                _ => Err(SNMPError::Unknown),
            }
        }
    }

    pub fn set_community(&self, community: &str) -> Result<(), SNMPError> {
        let c_community = CString::new(community).unwrap();
        unsafe {
            match rs_netsnmp_set_community(self.netsnmp_session, c_community.as_ptr()) {
                0 => Ok(()),
                _ => Err(SNMPError::Unknown),
            }
        }
    }

    pub fn set_transport(&self, transport: &str) -> Result<(), SNMPError> {
        // This should match man 1 snmpcmd AGENT SPECIFICATION
        // This api later could be broken up, it's very 1:1 atm.
        let c_transport = CString::new(transport).unwrap();
        unsafe {
            match rs_netsnmp_set_peername(self.netsnmp_session, c_transport.into_raw()) {
                0 => Ok(()),
                _ => Err(SNMPError::Unknown),
            }
        }
    }

    pub fn open_session(&self) -> Result<(), SNMPError> {
        // This should check the VERSION of snmp, and then that the right
        //  parts have been filled in. 

        Ok(())
    }

    // WARNING: This would be much better in a drop ....
    pub fn destroy(&self) {
        // We need to check *all* the places where we might have called cstring into_raw, and see if we need to dealloc
        unsafe {
            let session_peername = rs_netsnmp_get_peername(self.netsnmp_session);
            if (!session_peername.is_null()) {
                let _peername = CString::from_raw(session_peername);
                //Now it will go out of scope and freed correctly
            }
            rs_netsnmp_destroy_session(self.netsnmp_session);
        }
    }
}

