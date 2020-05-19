pub fn sum_of_multiples(limit: u32, factors: &[u32]) -> u32 {
    let mut temp = Vec::new();

    for i in 0..factors.len() {
        let cur_val = factors[i];
        let mut count = 1;

        loop {
            let x = cur_val * count;

            if x < limit && cur_val > 0 {
                let some_val = temp.iter().find(|&&xx| xx == x);
                match some_val {
                    Some(_) => {
                        count += 1;
                        continue;
                    }
                    None => {
                        temp.push(x);
                        count += 1;
                        continue;
                    }
                };
            } else {
                break;
            }
        }
    }

    let sum = temp.iter().fold(0, |acc, x| acc + x);
    sum
}
