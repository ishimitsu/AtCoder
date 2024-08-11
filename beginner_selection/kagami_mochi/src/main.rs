use proconio::input;

const MAX_SIZE:i32 = 100;

fn count_step(d: Vec<i32>) -> i32 {
    let mut steps = 0;
    let mut prev_size = MAX_SIZE + 1;
    for &x in &d {
        if x < prev_size {
            steps += 1;
            prev_size = x;
        }
    }

    steps
}

fn kagami_mochi(d: Vec<i32>, _n: i32) -> i32 {
    let mut sorted_d = d;
    sorted_d.sort();
    sorted_d.reverse();
    count_step(sorted_d)
}


fn main() {
    input! {
        n: i32,
        d: [i32; n],
    }
    println!("{}", kagami_mochi(d, n));
}

#[cfg(test)]
mod tests {
    use crate::kagami_mochi;

    #[test]
    fn test1() {
        assert_eq!(kagami_mochi(vec![10, 8, 8, 6], 4), 3);
        assert_eq!(kagami_mochi(vec![15, 15, 15], 3), 1);
        assert_eq!(kagami_mochi(vec![50, 30, 50, 100, 50, 80, 30], 7), 4);
    }
}