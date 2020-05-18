pub fn square_of_sum(n: u32) -> u32 {
    let mut vec = Vec::new();

    if n == 1 {
        return n;
    }

    for i in 1..n + 1 {
        vec.push(i);
    }
    let sum = vec.iter().fold(0, |acc, x| acc + x);
    sum * sum
}

pub fn sum_of_squares(n: u32) -> u32 {
    let mut vec = Vec::new();

    if n == 1 {
        return n;
    }

    for i in 1..n + 1 {
        vec.push(i);
    }
    let seq: Vec<u32> = vec.iter().map(|x| x * x).collect();
    let sum = seq.iter().fold(0, |acc, x| acc + x);
    sum
}

pub fn difference(n: u32) -> u32 {
    let dif = square_of_sum(n) - sum_of_squares(n);
    dif
}
