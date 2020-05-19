pub fn build_proverb(list: &[&str]) -> String {
    let end = list.len();
    let mut str = String::new();

    for i in 0..end {
        let a = list[i];
        let s: String = if i == end - 1 {
            let a = list[0];
            let s = format!("And all for the want of a {}.", a);
            s
        } else {
            let b = list[i + 1];
            let s = format!("For want of a {} the {} was lost.\n", a, b);
            s
        };
        str.push_str(&s);
    }

    str
}
