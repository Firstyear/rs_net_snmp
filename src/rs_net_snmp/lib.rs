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
    // fn rs_netsnmp_get_community(session: *const libc::c_void) -> *mut c_char;
    fn rs_netsnmp_set_peername(session: *const libc::c_void, hostname: *const c_char) -> isize;
    // fn rs_netsnmp_get_peername(session: *const libc::c_void) -> *mut c_char;
    fn rs_netsnmp_open_session(session: *const libc::c_void) -> *const libc::c_void;
    fn rs_netsnmp_get_oid(session: *const libc::c_void, oid: *const c_char) -> isize;
    fn rs_netsnmp_destroy_session(session: *const libc::c_void);
}

/// NetSNMP tracks an active connection session to an snmpd daemon.
pub struct NetSNMP {
    // I think this just needs to track the pointer to the internal
    // struct for the native helper.
    netsnmp_session: *const libc::c_void,
    active_session: *const libc::c_void,
    state: SNMPState,
}

impl NetSNMP {
    /// Creates a new NetSNMP struct with a blank session.
    pub fn new() -> NetSNMP {
        unsafe {
            NetSNMP {
                netsnmp_session: rs_netsnmp_create_session(),
                active_session: rs_netsnmp_create_null_session(),
                state: SNMPState::New,
            }
        }
    }

    /// Set the version of this session. Please see SNMPVersion.
    pub fn set_version(&self, version: SNMPVersion) -> Result<(), SNMPError> {
        // Need to make the enum able to call the right version
        // set thing.
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
        // Is there a better way than taking self as mut?
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
    pub fn get_oid(&self, oid: &str) -> Result<(), SNMPError> {
        match self.state {
            SNMPState::Connected => {},
            _ => return Err(SNMPError::InvalidState)
        }
        // Perhaps this should be Result<Option<>, SNMPError>> ??
        let c_oid = CString::new(oid).unwrap();
        unsafe {
            match rs_netsnmp_get_oid(self.active_session, c_oid.as_ptr()) {
                0 => Ok(()),
                _ => Err(SNMPError::Unknown),
            }
        }
    }

    // We should have a private fn that after the OID is grabbed,
    // Set the state to resultset or something?
    // we parse the result. This means we make a vec!()
    // Then we do while true, vec.push next value.
    // We need a wrapper for the values though  
    // The wrapper will need an enum of them types, and a way to store the
    // possible values?
    // as they go.
    // Once we have the values out, we free the results, and put the state back
    // to connected for the next get.

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
            SNMPState::Connected => {},
            SNMPState::Disconnected => {},
            SNMPState::Error => {},
            // _ => return Err(SNMPError::InvalidState)
            _ => return (),
        }
        self.state = SNMPState::Destroyed;
        unsafe {
            // It looks like net-snmp leaks memory :(
            rs_netsnmp_destroy_session(self.netsnmp_session);
        }
    }
}

