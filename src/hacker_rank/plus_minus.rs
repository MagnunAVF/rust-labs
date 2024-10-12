// URL: https://www.hackerrank.com/challenges/plus-minus/problem?isFullScreen=true

fn plus_minus(arr: &[i32]) {
    let n = arr.len();
    let mut positives = 0;
    let mut negatives = 0;
    let mut zeros = 0;

    for &value in arr {
        if value > 0 {
            positives += 1;
        } else if value < 0 {
            negatives += 1;
        } else {
            zeros += 1;
        }
    }

    let positives_ratio = (positives as f32) / (n as f32);
    let negatives_ratio = (negatives as f32) / (n as f32);
    let zeros_ratio = (zeros as f32) / (n as f32);

    println!("{:.6}", positives_ratio);
    println!("{:.6}", negatives_ratio);
    println!("{:.6}", zeros_ratio);
}

pub fn main() {
    println!("\n06 - Plus Minus");

    let a = [-4, 3, -9, 0, 4, 1];
    println!(" plus minus => {:?}", a);
    plus_minus(&a);
}
