use crate::utils::fast_pow_with_mod;
use anyhow::Result;
use std::fs;

pub fn get_primes() -> Result<Vec<u64>> {
    Ok(fs::read_to_string(std::env::var("PRIMES")?)
        .expect("Something went wrong reading the prime numbers")
        .split(",")
        .map(|s| s.parse::<_>().unwrap())
        .collect::<Vec<_>>())
}

pub fn encrypt(s: &str, e: u64, n: u64) -> Vec<u64> {
    s.chars().map(|c| encrypt_char(c, e, n)).collect::<Vec<_>>()
}

fn encrypt_char(c: char, e: u64, n: u64) -> u64 {
    fast_pow_with_mod(c as u64, e, n)
}

pub fn decrypt(encrypted: &[u64], d: u64, n: u64) -> String {
    encrypted
        .iter()
        .map(|&a| decrypt_char(a, d, n))
        .collect::<String>()
}

fn decrypt_char(a: u64, d: u64, n: u64) -> char {
    fast_pow_with_mod(a, d, n) as u8 as char
}
