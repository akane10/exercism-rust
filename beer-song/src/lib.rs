fn take_it_or_go_to_store(n: u32) -> String {
    match n {
        0 => String::from("Go to the store and buy some more"),
        1 => String::from("Take it down and pass it around"),
        _ => String::from("Take one down and pass it around"),
    }
}

fn get_total(n: u32, is_up: bool) -> String {
    match n {
        0 => {
            if is_up {
                String::from("No more bottles")
            } else {
                String::from("no more bottles")
            }
        }
        1 => String::from("1 bottle"),
        _ => format!("{} bottles", n),
    }
}

pub fn verse(n: u32) -> String {
    // unimplemented!("emit verse {}", n)
    let remain = match n {
        0 => 99,
        _ => n - 1,
    };
    let of_beer = String::from("of beer");
    let on_the_wall = String::from("on the wall");
    let first = format!(
        "{} {} {}, {} {}",
        get_total(n, true),
        of_beer,
        on_the_wall,
        get_total(n, false),
        of_beer
    );
    let second = format!(
        "{}, {} {} {}",
        take_it_or_go_to_store(n),
        get_total(remain, false),
        of_beer,
        on_the_wall
    );

    let all = format!("{}.\n{}.\n", first, second);
    all
}

pub fn sing(start: u32, end: u32) -> String {
    let mut s = String::new();

    for n in end..(start + 1) {
        s.insert_str(0, &verse(n));

        if n != start {
            s.insert_str(0, "\n");
        }
    }

    s
}
