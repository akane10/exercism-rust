pub fn raindrops(n: u32) -> String {
    let mut str = String::new();

    let a = n % 3 == 0;
    let b = n % 5 == 0;
    let c = n % 7 == 0;

    if a {
        str.push_str("Pling");
    }

    if b {
        str.push_str("Plang");
    }

    if c {
        str.push_str("Plong");
    }

    if !a && !b && !c {
        str.push_str(&n.to_string());
    }

    str
}
