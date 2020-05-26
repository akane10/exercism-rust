pub fn collatz(n: u64) -> Option<u64> {
    let mut steps = 0;
    let mut res = n;

    if n == 0 {
        return None;
    }

    loop {
        match res {
            1 => break,
            x if x % 2 == 0 => {
                res = res / 2;
                steps += 1
            }
            _ => {
                res = res * 3 + 1;
                steps += 1
            }
        }
    }

    Some(steps)
}
