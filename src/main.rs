mod rsa;
mod utils;

use crate::rsa::{decrypt, encrypt};
use rand;
use rand::seq::IteratorRandom;
use rand::thread_rng;

const PUBLIC_KEY_E: u64 = 65537;
const MESSAGE: &str = "MobilityTechnologies";

fn main() {
    // get secret two prime numbers
    let primes = rsa::get_primes().unwrap();
    println!("Read: {} primes", primes.len());
    let (prime_p, prime_q, lcm_pq) = loop {
        let pq = primes.iter().choose_multiple(&mut thread_rng(), 2);
        let p = *pq[0];
        let q = *pq[1];
        let lcm = utils::lcm(p - 1, q - 1);
        if lcm % PUBLIC_KEY_E != 0 {
            break (p, q, lcm);
        }
    };
    println!("prime: p {} q {}", prime_p, prime_q);

    // generate public key
    let public_key_n = prime_p * prime_q;
    println!("Public key N: {}", public_key_n);
    println!("Public key E: {}", PUBLIC_KEY_E);

    // generate secret key
    let (private_key_d, _v) = utils::ext_gcd(PUBLIC_KEY_E, lcm_pq);
    println!("PRIVATE KEY D: {}", private_key_d);
    // println!(
    //     "{} * {} + {} * {} = {}",
    //     PUBLIC_KEY_E,
    //     private_key_d,
    //     lcm_pq,
    //     _v,
    //     (PUBLIC_KEY_E * private_key_d) as i64 + lcm_pq as i64 * _v
    // );

    let encrypt = encrypt(MESSAGE, PUBLIC_KEY_E, public_key_n);
    println!("Encrypted Message: {:?}", encrypt);

    let decrypted = decrypt(&encrypt, private_key_d, public_key_n);
    println!("Decrypted Message: {}", decrypted);
}
