fn is_all_uppercase(msg: &str) -> bool {
    let mut is = false;

    let filtered_alphabetic: String = msg
        .chars()
        .filter(|x| x.is_ascii_alphabetic())
        .collect::<String>();

    let mut chars = filtered_alphabetic.chars();

    for i in 1..filtered_alphabetic.len() {
        let some_char = chars.nth(i);

        match some_char {
            Some(y) => {
                is = y.is_ascii_uppercase();

                if !is {
                    break;
                }
            }
            None => {
                break;
            }
        }
    }

    is
}

fn is_question(msg: &str) -> bool {
    let nth = msg.len();

    if nth < 1 {
        return false;
    }
    let is = msg.chars().nth(nth - 1) == Some('?');
    is
}

pub fn reply(message: &str) -> &str {
    let msg_vec = message.chars();
    let filtered_ascii: String = msg_vec
        .filter(|x| x.is_ascii() && !x.is_ascii_whitespace())
        .collect::<String>();

    match filtered_ascii {
        s if is_all_uppercase(&s) => {
            if is_question(&s) {
                return "Calm down, I know what I'm doing!";
            } else {
                return "Whoa, chill out!";
            }
        }
        s if is_question(&s) => "Sure.",
        s if s.len() == 0 => "Fine. Be that way!",
        _ => "Whatever.",
    }
}
