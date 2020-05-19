pub fn is_armstrong_number(num: u32) -> bool {
    let num_vec: Vec<u32> = num
        .to_string()
        .chars()
        .map(|x| x.to_digit(10).unwrap())
        .collect();

    let sum = num_vec.iter().fold(0, |acc, x| {
        let b: u32 = num_vec.len() as u32;
        let res = x.pow(b) + acc;
        res
    });

    sum == num
}
