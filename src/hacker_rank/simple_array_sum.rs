// URL: https://www.hackerrank.com/challenges/simple-array-sum/problem?isFullScreen=true

fn simple_array_sum(ar: &[i32]) -> i32 {
    let mut sum = 0;
    for numb in ar {
        sum += numb;
    }

    sum
}

pub fn main() {
    println!("\n02 - Simple Array Sum");

    let ar = [1, 2, 3, 4, 10, 11];
    println!(" array sum => {:?} = {}", ar, simple_array_sum(&ar));
}
