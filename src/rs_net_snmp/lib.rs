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

//! This crate provides bindings for Rust to the C netsnmp library.

extern crate libc;

use std::os::raw::c_char;
use std::ffi::CString;
use std::ffi::CStr;


// use libc;

#[repr(C)]
#[derive(Debug)]
/// The possible set of SNMP versions avaliable
pub enum SNMPVersion {
    /// Version 1
    VERSION_1,
    /// Version 2c
    VERSION_2c,
    /// Version 3
    VERSION_3,
}

/// From net-snmp headers, to match an int type in the result.
pub const ASN_INTEGER: isize = 2; // I think ....
/// From net-snmp headers, to match a *char type in the result.
pub const ASN_OCTET_STR: isize = 4;
/// From net-snmp headers, to match an oid type in the result.
pub const ASN_OID: isize = 6;
/// From net-snmp headers, to match an int type in the result.
pub const ASN_TIMETICKS: isize = 67;

#[derive(Debug)]
/// Errors that can occur during SNMP processing
pub enum SNMPError {
    /// Out of memory
    NoMemory,
    /// An error establishing the session has occured
    Session,
    /// A function was called In the incorrect state.
    InvalidState,
    /// An unknown error has occured.
    Unknown,
}

#[derive(Debug)]
#[repr(C)]
/// Wraps the types of responses that the library can possibly return from a
/// query to an SNMP server.
pub enum SNMPResult {
    /// An octet string was returned
    AsnOctetStr { /// The octet string
                    s: String },
    /// A integer was returned
    AsnInteger { /// The integer
                    i: isize },
    /// An integer representing time in seconds since some point.
    AsnTimeticks { /// The time as integer seconds
                    i: isize },
}

#[derive(Debug)]
#[repr(C)]
/// The current state of the struct. This protects certain values.
enum SNMPState {
    /// The NetSnmp structure is just made. Only set_* can be called.
    New,
    /// We have moved from "New" to disconnected. Only mib get or disconnect
    /// can be called.
    Connected,
    /// We have encountered an error. We can now only disconnect and destroy
    /// the session.
    Error,
    /// The object has been disconnected. It is ready for disposal
    Disconnected,
    /// No more actions can be taken on this instance.
    Destroyed,
}

extern {
    fn rs_netsnmp_create_session() -> *const libc::c_void;
    // There must be a better way than this.
    fn rs_netsnmp_create_null_session() -> *const libc::c_void;
    fn rs_netsnmp_set_version(session: *const libc::c_void, version: isize) -> isize;
    fn rs_netsnmp_set_community(session: *const libc::c_void, community: *const c_char) -> isize;
    fn rs_netsnmp_set_peername(session: *const libc::c_void, hostname: *const c_char) -> isize;
    fn rs_netsnmp_open_session(session: *const libc::c_void) -> *const libc::c_void;
    fn rs_netsnmp_get_oid(session: *const libc::c_void, oid: *const c_char, target: *mut NetSNMP, cb: extern fn(*mut NetSNMP, isize, *const libc::c_void) -> isize) -> isize;
    fn rs_netsnmp_destroy_session(session: *const libc::c_void);
}

/// NetSNMP tracks an active connection session to an snmpd daemon.
#[derive(Debug)]
#[repr(C)]
pub struct NetSNMP {
    netsnmp_session: *const libc::c_void,
    active_session: *const libc::c_void,
    active_variables: Vec<SNMPResult>,
    state: SNMPState,
}

extern "C" fn _set_result_cb(target: *mut NetSNMP, asntype: isize, data: *const libc::c_void) -> isize {
    match asntype {
        ASN_OCTET_STR => {
            let ptr: *const c_char = data as *const c_char;
            unsafe {
                let value = CStr::from_ptr(ptr).to_string_lossy().into_owned();
                // println!("{:?}", value);
                // println!("{:?}", (*target));
                (*target).active_variables.push(SNMPResult::AsnOctetStr {s: value} );
            }
            return 0;
        }
        ASN_TIMETICKS => {
            let ival: isize = data as isize;
            unsafe {
                (*target).active_variables.push(SNMPResult::AsnTimeticks {i : ival} );
            }
            return 0;
        }
        ASN_INTEGER => {
            let ival: isize = data as isize;
            unsafe {
                (*target).active_variables.push(SNMPResult::AsnInteger {i : ival} );
            }
            return 0;
        }
        _ => {
            println!("Not implemented");
            return 1;
        }
    }
}


impl NetSNMP {
    /// Creates a new NetSNMP struct with a blank session.
    pub fn new() -> NetSNMP {
        unsafe {
            NetSNMP {
                netsnmp_session: rs_netsnmp_create_session(),
                active_session: rs_netsnmp_create_null_session(),
                // active_variable: rs_netsnmp_create_null_variable(),
                active_variables: Vec::new(),
                state: SNMPState::New,
            }
        }
    }

    /// Set the version of this session. Please see SNMPVersion.
    pub fn set_version(&self, version: SNMPVersion) -> Result<(), SNMPError> {
        match self.state {
            SNMPState::New => {},
            _ => return Err(SNMPError::InvalidState)
        }
        unsafe {
            match rs_netsnmp_set_version(self.netsnmp_session, version as isize) {
                0 => Ok(()),
                _ => Err(SNMPError::Unknown),
            }
        }
    }

    /// For snmp version 1 and 2c, define the community we will connect to.
    pub fn set_community(&self, community: &str) -> Result<(), SNMPError> {
        match self.state {
            SNMPState::New => {},
            _ => return Err(SNMPError::InvalidState)
        }
        let c_community = CString::new(community).unwrap();
        unsafe {
            match rs_netsnmp_set_community(self.netsnmp_session, c_community.as_ptr()) {
                0 => Ok(()),
                _ => Err(SNMPError::Unknown),
            }
        }
    }

    /// Set the transport specifier. An example is tcp6:localhost:161.
    /// This should match man 1 snmpcmd AGENT SPECIFICATION
    pub fn set_transport(&self, transport: &str) -> Result<(), SNMPError> {
        match self.state {
            SNMPState::New => {},
            _ => return Err(SNMPError::InvalidState)
        }
        let c_transport = CString::new(transport).unwrap();
        unsafe {
            match rs_netsnmp_set_peername(self.netsnmp_session, c_transport.as_ptr()) {
                0 => Ok(()),
                _ => Err(SNMPError::Unknown),
            }
        }
    }

    /// Create the socket, and open the session to the remote Snmp daemon. This
    /// moves the connection state to "connected".
    pub fn open_session(&mut self) -> Result<(), SNMPError> {
        match self.state {
            SNMPState::New => {},
            _ => return Err(SNMPError::InvalidState)
        }
        self.state = SNMPState::Connected;
        // This should check the VERSION of snmp, and then that the right
        //  parts have been filled in. 
        unsafe {
            self.active_session = rs_netsnmp_open_session(self.netsnmp_session);
        }
        if self.active_session.is_null() {
            self.state = SNMPState::Error;
            Err(SNMPError::Session)
        } else {
            Ok(())
        }
    }


    /// Given an OID string, return the value, or unit () if no value exists.
    pub fn get_oid(&mut self, oid: &str) -> Result<Option<&Vec<SNMPResult>>, SNMPError> {
        // TODO: To handle if the oid doesn't exist, this makes an empty vec.
        // It should return a better error.
        match self.state {
            SNMPState::Connected => {},
            _ => return Err(SNMPError::InvalidState)
        }
        // Empty the vector
        self.active_variables.clear();
        let c_oid = CString::new(oid).unwrap();
        unsafe {
            match rs_netsnmp_get_oid(self.active_session, c_oid.as_ptr(), self, _set_result_cb) {
                0 => {
                    // println!("get_oid() {:?}", self.active_variables);
                    Ok(Some(&self.active_variables))
                },
                3 => {
                    Ok(None)
                },
                _ => Err(SNMPError::Unknown),
            }
        }

    }

    /// Close and disconnect the active session to an snmp daemon
    pub fn close_session(&mut self) -> Result<(), SNMPError> {
        match self.state {
            SNMPState::Connected => {},
            _ => return Err(SNMPError::InvalidState)
        }
        // Is it even possible for this to fail?
        self.state = SNMPState::Disconnected;
        unsafe {
            rs_netsnmp_destroy_session(self.netsnmp_session);
        }
        Ok(())
    }

    /// When you are finished with NetSNMP you *must* call this function.
    /// This triggers the various C components to be freed and disconnected.
    /// This implies disconnection of the session!
    pub fn destroy(&mut self) {
        match self.state {
            SNMPState::Destroyed => return (),
            _ => {},
        }
        self.state = SNMPState::Destroyed;
        unsafe {
            // It looks like net-snmp leaks memory :(
            rs_netsnmp_destroy_session(self.netsnmp_session);
        }
    }
}

/// Given a &vec<SNMPResults>, display these in a reasonable
/// format to the formatter.
pub fn display_snmpresults(oid: &str, results: Option<&Vec<SNMPResult>>) {
    match results {
        Some(r) => {
            for v in r {
                print!("{} = ", oid);
                match v {
                    &SNMPResult::AsnOctetStr { s: ref sv} => print!("{} ", sv),
                    &SNMPResult::AsnInteger { i: ref iv} => print!("{} ", iv),
                    &SNMPResult::AsnTimeticks { i: ref iv} => print!("{} ", iv),
                    // _ => { println!("Unable to format this value!") }
                }
                println!("");
            }
        }
        None => {
            println!("{} = No values", oid);
        }
    }
}


