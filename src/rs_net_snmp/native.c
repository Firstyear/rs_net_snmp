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

#include <net-snmp/net-snmp-config.h>
#include <net-snmp/net-snmp-includes.h>
#include <string.h>

netsnmp_session *
rs_netsnmp_create_session() {
    // Create a net-snmp session pointer.
    netsnmp_session *session = NULL;
    session = malloc(sizeof(netsnmp_session));
    if (session == NULL) {
        printf("Unable to allocate memory, all bets are off now!\n");
        return NULL;
    }

    init_snmp ( "rs_net_snmp" );
    snmp_sess_init( session );
    return session;
}

// Returs 0 on success, 1 on failure.
int
rs_netsnmp_set_version(netsnmp_session *session, int version) {
    // These numbers match to the enum in lib.rs
    int res = 1;
    if (version == 0) {
        session->version = SNMP_VERSION_1;
        res = 0;
    } else if (version == 1) {
        session->version = SNMP_VERSION_2c;
        res = 0;
    } else if (version == 2) {
        session->version = SNMP_VERSION_3;
        res = 0;
    }
    return res;
}

// Returs 0 on success, 1 on failure.
int
rs_netsnmp_set_community(netsnmp_session *session, char *community) {
    int res = 1;
    // Do I need to strdup this?
    session->community = community;
    session->community_len = strlen(community);
    res = 0;
    return res;
}

int
rs_netsnmp_open_session(netsnmp_session *session) {
    //
    return 1;
}

void
rs_netsnmp_destroy_session(netsnmp_session *session) {
    if (session != NULL) {
        free(session);
    }
}
