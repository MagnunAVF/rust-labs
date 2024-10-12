// URL: https://www.hackerrank.com/challenges/a-very-big-sum/problem?isFullScreen=true

fn a_very_big_sum(ar: &[i64]) -> i64 {
    let mut sum = 0;
    for value in ar {
        sum += value;
    }

    sum
}

pub fn main() {
    println!("\n04 - A Very Big Sum");

    let a = [1000000001, 1000000002, 1000000003, 1000000004, 1000000005];
    println!(" big sum => {:?} = {}", a, a_very_big_sum(&a));
}
