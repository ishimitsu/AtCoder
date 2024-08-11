use proconio::input;

fn main() {
    input! {
        a: i32,
        b: i32,
    }
    let is_even: bool = (a * b) % 2 == 0;
    if is_even { println!("Even") } else {println!("Odd")};
}
