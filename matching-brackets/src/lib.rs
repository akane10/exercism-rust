fn is_match(pair: (char, char)) -> bool {
    match pair {
        ('(', ')') => true,
        ('{', '}') => true,
        ('[', ']') => true,
        _ => false,
    }
}

fn is_open(c: char) -> bool {
    match c {
        '(' => true,
        '{' => true,
        '[' => true,
        _ => false,
    }
}

pub fn brackets_are_balanced(string: &str) -> bool {
    let str_vec: Vec<char> = string
        .chars()
        .filter(|x| match x {
            '(' => true,
            ')' => true,
            '{' => true,
            '}' => true,
            '[' => true,
            ']' => true,
            _ => false,
        })
        .collect();

    match (str_vec.len(), str_vec) {
        (0, _) => true,
        (l, _) if l % 2 != 0 => false,
        (l, val) if l == 2 => is_match((val[0], val[1])),
        (_, val) => {
            let mut stack = Vec::new();
            let mut result = false;

            for i in 0..val.len() {
                let c = val[i];

                match is_open(c) {
                    true => stack.push(c),
                    false => {
                        let x = stack[stack.len() - 1];
                        let matched = is_match((x, c));

                        if matched {
                            result = true;
                            stack.pop();
                            continue;
                        } else {
                            result = false;
                            break;
                        }
                    }
                }
            }

            result

            /*
            USING FOLD

            type Result = bool;
            type Until = bool;
            let result: (Vec<char>, Result, Until) =
                val.iter()
                    .fold(([].to_vec(), false, false), |acc, v| match acc {
                        (vec, r, true) => (vec, r, true),
                        (mut vec, _, false) => match is_open(*v) {
                            true => {
                                vec.push(*v);
                                (vec, true, false)
                            }
                            false => {
                                let x = vec[vec.len() - 1];

                                if is_match((x, *v)) {
                                    vec.pop();
                                    (vec, true, false)
                                } else {
                                    (vec, false, true)
                                }
                            }
                        },
                    });

            result.1

            */
        }
    }
}
