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

typedef void (*rust_callback)(void*);

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

netsnmp_session *
rs_netsnmp_create_null_session() {
    return NULL;
}

netsnmp_pdu *
rs_netsnmp_create_null_variable() {
    return NULL;
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
    // Do I need to free this?
    session->community = strdup(community);
    session->community_len = strlen(community);
    res = 0;
    return res;
}

void *
rs_netsnmp_get_community(netsnmp_session *session) {
    // Returns the current value of session->peername. This way we know if we need to 
    return session->community;
}

int
rs_netsnmp_set_peername(netsnmp_session *session, char *transport)
{
    // Do I need to strdup this?
    // Do I need to free this?
    session->peername = strdup(transport);
    return 0;
}

void *
rs_netsnmp_get_peername(netsnmp_session *session) {
    // Returns the current value of session->peername. This way we know if we need to 
    return session->peername;
}

// I think this should return the active netsnmp_session ptr.
netsnmp_session *
rs_netsnmp_open_session(netsnmp_session *session) {
    // ss --> This is another netsnmp_session. I think we should return this
    netsnmp_session *active = NULL;
    // SOCK_ is only for win32
    // SOCK_STARTUP;
    // The error is here. Active isn't being set.
    active = snmp_open(session);
    if (active == NULL) {
        snmp_sess_perror("ack", &active);
        // SOCK_ is only for win32
        // SOCK_CLEANUP
    }
    // Rust isn't detecting this NULL properly. How can we fix this?
    return active;
}

char *
_rs_netsnmp_variable_to_str(netsnmp_variable_list *var) {
    char *str = (char *)malloc(1 + var->val_len);
    if (str != NULL) {
        memcpy(str, var->val.string, var->val_len);
        str[var->val_len] = '\0';
    }
    return str;
}

int
_rs_netsnmp_display_variables(netsnmp_pdu *response) {
    netsnmp_variable_list *vars = NULL;
    for (vars = response->variables; vars; vars = vars->next_variable) {
        // print_variable(vars->name, vars->name_length, vars);
        if (vars->type == ASN_OCTET_STR) {
            char *value = _rs_netsnmp_variable_to_str(vars);
            if (value != NULL) {
                printf("%s\n", value);
                free(value);
            }
        } else if (vars->type == ASN_TIMETICKS) {
            printf("%lu\n", vars->val.integer);
        } else if (vars->type == ASN_INTEGER) {
            printf("%lu\n", vars->val.integer);
        // } else if (vars->type == ASN_OBJECT_ID) {
        //     printf("%s\n", vars->val.objid);
        } else {
            printf("NOT IMPLEMENTED YET\n");
        }
    }
}

// I think that perhaps the way to do this is to accept a c_void **
// we return a TYPE value
// Then we populate the c_void with the correct data.
// We will need to provide a Free_result function
// How do we handle multi-values responses?

// Or, we could return the response value, then, based on the type we return
// rust can then direct the response into the correct place?
// Or would I just return the first variables value?
// Would I need to memcpy it?
// How would I clean up?

int
rs_netsnmp_get_oid(netsnmp_session *active, char *request_oid, void* callback_target, rust_callback callback) {
    void *cb_target = callback_target; // This is the object (self)
    rust_callback cb = callback; // This is the actual cb
    netsnmp_pdu *pdu = NULL;
    netsnmp_pdu *response = NULL;
    size_t parsed_oid_len = MAX_OID_LEN;
    oid parsed_oid[MAX_OID_LEN];
    int result;
    int rs_result = -1;

    printf("%s = ", request_oid);

    // Test the cb
    cb(cb_target);

    // Create a PDU for the data to land in
    pdu = snmp_pdu_create(SNMP_MSG_GET);
    // Parse the oid we were given.
    if (!snmp_parse_oid(request_oid, parsed_oid, &parsed_oid_len)) {
        // We don't need to call cleanup, rust will do that for us.
        rs_result = 2;
    } else {
        // read_objid(some mib) will allow parse of a name OR an oid?
        // snmp_add_null_var -- zero it
        snmp_add_null_var(pdu, parsed_oid, parsed_oid_len);
        // status = snmp_synch_response -- get the request
        result = snmp_synch_response(active, pdu, &response);
        if (result == STAT_SUCCESS && response->errstat == SNMP_ERR_NOERROR) {
            // process the content of status as a response
            _rs_netsnmp_display_variables(response);
            rs_result = 0;
        } else {
            if (result == STAT_SUCCESS) {
                printf("Error in packet %s \n", snmp_errstring(response->errstat));
            } else if (result == STAT_TIMEOUT) {
                printf("Timeout from %s\n", active->peername);
            } else {
                snmp_sess_perror("rs_net_snmp", active);
                printf("Unknown %d\n", result);
            }
            rs_result = 1;
        }
    }
    // 
    if (response != NULL) {
        snmp_free_pdu(response);
    }
    return rs_result;
}


int
rs_netsnmp_close_session(netsnmp_session *active) {
    snmp_close(active);
    // SOCK_ is only for win32
    // SOCK_CLEANUP;
    return 0;
}

void
rs_netsnmp_destroy_session(netsnmp_session *session) {
    if (session != NULL) {
        snmp_close(session);
        if (session->community) {
            free(session->community);
        }
        if (session->peername) {
            free(session->peername);
        }
        // This causes a valgrind error?
        free(session);
    }
}
