pub fn private_key(p: u64) -> u64 {
    let primes: Vec<u64> = vec![
        2, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 773, 967, 3461, 6131,
    ];

    let x: Vec<u64> = primes.into_iter().filter(|x| x < &&p).collect();

    if x.len() < 1 {
        1
    } else {
        x[x.len() - 1]
    }
}

// g**a mod p
pub fn public_key(p: u64, g: u64, a: u64) -> u64 {
    let mut exponent = a;
    let modulus = p;
    let mut base = g;

    let mut result = 1;

    match modulus {
        1 => 0,
        _ => {
            loop {
                if exponent > 0 {
                    if exponent % 2 == 1 {
                        result = result * base % modulus;
                    }
                    exponent = exponent >> 1;
                    base = base * base % modulus;
                } else {
                    break;
                }
            }
            result
        }
    }
}

// B**a mod p
pub fn secret(p: u64, b_pub: u64, a: u64) -> u64 {
    let res = public_key(p, b_pub, a);
    res
}
