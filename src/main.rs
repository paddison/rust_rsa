use rsa_arbitray_precision::rsa_module;
use rug::Integer;

fn main() {
    let msg = Integer::from(10850);
    let (d, n, e) = rsa_module::generate_key_pair(4096);
    let c = rsa_module::encrypt_msg(&msg, &e, &n);
    let decyphered = rsa_module::decrypt_cypher(&c, &d, &n);
    println!("  msg: {}\n cyph: {}\ndecyp: {}", msg, c.to_string_radix(16), decyphered);
}
