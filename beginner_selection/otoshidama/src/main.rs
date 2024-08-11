use proconio::input;

fn otoshidama(bill: i32, sum: i32) -> (i32, i32, i32) {
    let mut bill_remain:i32;
    let mut sum_remain:i32;

    let mut ten_thou: i32 = 0;
    while ten_thou <= bill {
        sum_remain = sum - ten_thou * 10000;
        if sum_remain < 0 { break };

        bill_remain = bill - ten_thou;
        let mut five_thou = 0;
        while five_thou <= bill_remain {
            let one_thou = bill_remain - five_thou;
            let cur_bill = ten_thou + five_thou + one_thou;
            let sum_cur_bill = 10000 * ten_thou + 5000 * five_thou + 1000 * one_thou;
            if (cur_bill == bill) && (sum == sum_cur_bill) { return (ten_thou, five_thou, one_thou) };

            five_thou += 1;
        }

        ten_thou += 1;
    }

    return (-1, -1, -1);
}


fn main() {
    input! {
        n: i32,
        y: i32,
    }
    let (ten_thou, five_thou, one_thou) = otoshidama(n, y);
    println!("{} {} {}", ten_thou, five_thou, one_thou);
}

#[cfg(test)]
mod tests {
    use crate::otoshidama;

    #[test]
    fn test() {
        let mut one_thou;
        let mut five_thou;
        let mut ten_thou;

        (ten_thou, five_thou, one_thou) = otoshidama(9, 45000);
        assert_eq!(9, ten_thou + five_thou + one_thou);
        assert_eq!(45000, ten_thou * 10000 + five_thou * 5000 + one_thou * 1000);

        assert_eq!(otoshidama(20, 196000), (-1, -1, -1));

        (ten_thou, five_thou, one_thou) = otoshidama(1000, 1234000);
        assert_eq!(1000, ten_thou + five_thou + one_thou);
        assert_eq!(1234000, ten_thou * 10000 + five_thou * 5000 + one_thou * 1000);

        (ten_thou, five_thou, one_thou) = otoshidama(2000, 20000000);
        assert_eq!(2000, ten_thou + five_thou + one_thou);
        assert_eq!(20000000, ten_thou * 10000 + five_thou * 5000 + one_thou * 1000);
    }
}