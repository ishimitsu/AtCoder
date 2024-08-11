use proconio::input;

fn get_places_sums(val: i32) -> i32 {
    let mut sum: i32 = 0;
    let mut next: i32 = val;
    while next > 0 {
        sum += next % 10;
        next = next / 10;
    }

    sum
}

fn some_sums(max: i32, start: i32, end: i32) -> i32 {
    let mut sum: i32 = 0;
    let mut val: i32 = 1;

    while val <= max {
        let places_sums: i32 = get_places_sums(val);
        if places_sums >= start && places_sums <= end {
            sum += val;
            // println!("val {} places {} sum {}", val, places_sums, sum);
        };
        val += 1;
    }

    sum
}

fn main() {
    input! {
        n: i32,
        a: i32,
        b: i32,
    }
    println!("{}", some_sums(n, a, b));
}
