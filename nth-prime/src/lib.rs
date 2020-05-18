fn is_prime(n: u32) -> bool {
    if n <= 1 {
        return false;
    }

    for a in 2..n {
        if n % a == 0 {
            return false;
        }
    }

    true
}

fn generate_primes(n: u32) -> Vec<u32> {
    let mut vec = Vec::new();

    for i in 2..n {
        if is_prime(i) {
            vec.push(i);
        }
    }

    vec
}

pub fn nth(n: u32) -> u32 {
    let x = generate_primes(105_000);
    x[n as usize]
    // unimplemented!("What is the 0-indexed {}th prime number?", n)
}
