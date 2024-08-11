use proconio::input;

fn pay50coins(have50: i32, price: i32) -> i32 {
    let mut c50: i32 = 0;
    while c50 <= have50 {
        let remain_paid50: i32 = price - c50 * 50;
        if remain_paid50 == 0 {
            return 1
        };
        c50 += 1;
    }
    0
}

fn pay_100_50_coins(have100: i32, have50: i32, price: i32) -> i32 {
    let mut cnt: i32 = 0;
    let mut c100: i32 = 0;
    while c100 <= have100 {
        let remain: i32 = price - c100 * 100;
        cnt += pay50coins(have50, remain);
        c100 += 1;
    }
    cnt
}

fn pay_500_100_50_coins(have500: i32, have100: i32, have50: i32, price: i32) -> i32 {
    let mut cnt: i32 = 0;
    let mut c500: i32 = 0;
    while c500 <= have500 {
        let remain: i32 = price - c500 * 500;
        cnt += pay_100_50_coins(have100, have50, remain);
        c500 += 1;
    }
    cnt
}


fn get_total_payment_selection(have500: i32, have100: i32, have50:i32, price: i32) -> i32 {
    if have500 > 0 { pay_500_100_50_coins(have500, have100, have50, price) }
    else if have100 > 0 { pay_100_50_coins(have100, have50, price) }
    else { pay50coins(have50, price) }
}

fn main() {
    input! {
        a: i32,
        b: i32,
        c: i32,
        x: i32,
    }
    println!("{}", get_total_payment_selection(a, b, c, x));
}
