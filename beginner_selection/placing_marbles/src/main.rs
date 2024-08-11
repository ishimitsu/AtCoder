use proconio::input;

fn main() {
    input! {
        s: i32,
    }
    let s1: i32 = s / 100;
    let s2: i32 = (s / 10) % 10;
    let s3: i32 = (s % 100) % 10;

    println!("{}", s1 + s2 + s3)
}
