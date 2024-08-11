use proconio::input;

fn is_even (x: i32) -> i32 {
    if x % 2 == 0 { x / 2 } else { 0 }
}

fn is_vec_even_loop (a: Vec<i32>, even_cnt: i32) -> i32 {
    let mut new_a= Vec::new();

    for &x in &a {
        let new_x: i32 = is_even(x);
        if new_x > 0 {
            new_a.push(new_x);
        } else {
            return even_cnt;
        }
    }

    is_vec_even_loop(new_a, even_cnt + 1)
}


fn main() {
    input! {
        n: i32,
        a: [i32; n],
    }
    println!("{}", is_vec_even_loop(a, 0));
}
