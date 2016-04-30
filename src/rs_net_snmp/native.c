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

void
rs_netsnmp_destroy_session(netsnmp_session *session) {
    if (session != NULL) {
        free(session);
    }
}
