pub fn factors(n: u64) -> Vec<u64> {
    let mut vec = Vec::new();
    let mut fac = 2;
    let mut temp = n;

    loop {
        if temp % fac == 0 {
            temp = temp / fac;
            vec.push(fac);
        } else if temp > 1 {
            fac += 1;
        } else {
            break;
        }
    }

    vec
}
