extern crate gcc;

fn main() {
    gcc::compile_library("libnetsnmpnative.a", &["src/rs_net_snmp/native.c"]);
}
