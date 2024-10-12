// URL: https://www.hackerrank.com/challenges/compare-the-triplets/problem?isFullScreen=true

fn compare_triplets(a: &[i32], b: &[i32]) -> Vec<i32> {
    let mut scores: Vec<i32> = vec![0, 0];

    for (i, &value) in a.iter().enumerate() {
        if value > b[i] {
            scores[0] += 1;
        } else if value < b[i] {
            scores[1] += 1;
        }
    }

    scores
}

pub fn main() {
    println!("\n03 - Compare The Triplets");

    let a = [5, 6, 7];
    let b = [3, 6, 10]; // 1 1
    println!(
        " triplets compare => {:?} and {:?} = {:?}",
        a,
        b,
        compare_triplets(&a, &b)
    );
}
