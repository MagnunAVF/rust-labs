// URL: https://www.hackerrank.com/challenges/diagonal-difference/problem?isFullScreen=true

fn diagonal_difference(arr: &[Vec<i32>]) -> i32 {
    let n = arr.len();
    let mut left_diag_sum: i32 = 0;
    let mut right_diag_sum: i32 = 0;

    // square matrix:
    // 1 2 3
    // 4 5 6
    // 9 8 9
    // left diag  => 00, 11, 22 => i, i
    // right diag => 20, 11, 02 => n-1-i, i
    for i in 0..n {
        left_diag_sum += arr[i][i];
        right_diag_sum += arr[n - 1 - i][i];
    }

    let diff = (left_diag_sum - right_diag_sum).abs();

    diff
}

pub fn main() {
    println!("\n05 - Diagonal Difference");

    let a = [vec![11, 2, 4], vec![4, 5, 6], vec![10, 8, -12]]; // 15
    println!(" diagonal diff => {:?} = {}", a, diagonal_difference(&a));
}
