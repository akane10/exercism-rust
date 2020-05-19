pub fn square(s: u32) -> u64 {
    if s < 1 || s > 64 {
        panic!("Square must be between 1 and 64")
    }

    let vec: Vec<u32> = (1..s).collect();
    let num: u64 = vec.iter().fold(1, |acc, _| acc * 2);
    num
}

pub fn total() -> u64 {
    let vec: Vec<u32> = (1..65).collect();

    let all: Vec<u64> = vec.iter().map(|x| square(*x)).collect();
    let sum = all.iter().fold(0, |acc, x| acc + x);
    sum
}
