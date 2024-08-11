use proconio::input;

fn is_even (x: i32) -> bool {
    if x % 2 == 0 { true } else { false }
}

fn get_card (a: Vec<i32>) -> (Vec<i32>, Vec<i32>) {
    let mut alice:Vec<i32> = Vec::new();
    let mut bob:Vec<i32> = Vec::new();
    let mut n: i32 = 0;

    for &x in &a {
        if is_even(n) { alice.push(x) } else { bob.push(x) };
        n += 1;
    }
    (alice, bob)
}

fn sort_a (a: Vec<i32>) -> Vec<i32> {
    let mut sorted_a = a;
    sorted_a.sort();
    sorted_a.reverse();
    sorted_a
}


fn main() {
    input! {
        n: i32,
        a: [i32; n],
    }
    let sorted_a: Vec<i32> = sort_a(a);
    let (alice_card, bob_card) = get_card(sorted_a);
    let sum_diff: i32 = alice_card.iter().sum::<i32>() - bob_card.iter().sum::<i32>();

    println!("{}", sum_diff);
}
