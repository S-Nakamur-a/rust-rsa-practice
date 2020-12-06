pub fn gcd(a: u64, b: u64) -> u64 {
    assert!(a != 0 && b != 0);
    let (a, b) = if a < b { (b, a) } else { (a, b) };
    if a % b == 0 {
        b
    } else {
        gcd(b, a % b)
    }
}

pub fn lcm(a: u64, b: u64) -> u64 {
    (a / gcd(a, b)) * b
}

pub fn ext_gcd(a: u64, b: u64) -> (u64, i64) {
    // find D, v s.t. a * D + b * v = 1
    // euclid
    let (mut s, mut t) = (a as i64, b as i64);
    let (mut x, mut y, mut u, mut v) = (1i64, 0i64, 0i64, 1i64);
    while t != 0 {
        let k = s / t;
        s -= k * t;
        std::mem::swap(&mut s, &mut t);
        x -= k * u;
        std::mem::swap(&mut u, &mut x);
        y -= k * v;
        std::mem::swap(&mut y, &mut v);
    }
    while x < 0 {
        x += b as i64;
        y -= a as i64;
    }
    (x as u64, y)
}

pub fn fast_pow_with_mod(a: u64, b: u64, n: u64) -> u64 {
    (if b == 0 {
        1
    } else if b == 1 {
        a
    } else if b % 2 == 0 {
        let a = fast_pow_with_mod(a, b / 2, n);
        mul_mod64(a, a, n)
    } else {
        mul_mod64(a, fast_pow_with_mod(a, b - 1, n), n)
    }) % n
}

fn mul_mod64(mut x: u64, mut y: u64, m: u64) -> u64 {
    let msb = 0x8000_0000_0000_0000;
    let mut d = 0;
    let mp2 = m >> 1;
    x %= m;
    y %= m;

    if m & msb == 0 {
        for _ in 0..64 {
            d = if d > mp2 { (d << 1) - m } else { d << 1 };
            if x & msb != 0 {
                d += y;
            }
            if d >= m {
                d -= m;
            }
            x <<= 1;
        }
        d
    } else {
        for _ in 0..64 {
            d = if d > mp2 {
                d.wrapping_shl(1).wrapping_sub(m)
            } else {
                // the case d == m && x == 0 is taken care of
                // after the end of the loop
                d << 1
            };
            if x & msb != 0 {
                let (mut d1, overflow) = d.overflowing_add(y);
                if overflow {
                    d1 = d1.wrapping_sub(m);
                }
                d = if d1 >= m { d1 - m } else { d1 };
            }
            x <<= 1;
        }
        if d >= m {
            d - m
        } else {
            d
        }
    }
}

#[test]
fn test_gcd() {
    assert_eq!(gcd(2, 3), 1);
    assert_eq!(gcd(2, 10), 2);
    assert_eq!(gcd(42, 77), 7);
    assert_eq!(gcd(1000, 320), 40);
}

#[test]
fn test_lcm() {
    assert_eq!(lcm(2, 3), 6);
    assert_eq!(lcm(2, 10), 10);
    assert_eq!(lcm(42, 77), 462);
    assert_eq!(lcm(1000, 320), 8000);
}

#[test]
fn test_fast_pow_with_mod() {
    assert_eq!(fast_pow_with_mod(2, 3, 7), 1);
    assert_eq!(fast_pow_with_mod(10, 100, 2), 0);
}
