use proconio::input;

fn _go_dest(cur_x: i32, cur_y: i32, dest_x: i32, dest_y: i32, time_left: i32) -> bool {
    let distance = (dest_x - cur_x).abs() + (dest_y - cur_y).abs();
    let mut ret: bool = false;

    if distance > time_left { ret = false }
    else if distance == time_left { ret = true }
    else if (distance < time_left) && ((distance - time_left).abs() % 2 == 0) { ret = true };
    ret
}

fn traveling(_n: i32, dests: Vec<Vec<i32>>) -> bool {
    let mut cur_x: i32 = 0;
    let mut cur_y: i32 = 0;
    let mut cur_t: i32 = 0;

    for dest in dests {
        let time_left = dest[0] - cur_t;
        let dest_x = dest[1];
        let dest_y = dest[2];
        if !_go_dest(cur_x, cur_y, dest_x, dest_y, time_left) { return false }

        cur_x = dest_x;
        cur_y = dest_y;
        cur_t = dest[0];
    }

    true
}


fn main() {
    input! {
        n: i32,
        dests: [[i32; 3]; n]
    }
    println!("{}", if traveling(n, dests) {"Yes"} else { "No" });
}

#[cfg(test)]
mod tests {
    use crate::traveling;

    #[test]
    fn test() {
        assert_eq!(true, traveling(2, vec![vec![3, 1, 2], vec![6, 1, 1]]));
        assert_eq!(false, traveling(1, vec![vec![2, 100, 100]]));
        assert_eq!(false, traveling(2, vec![vec![5, 1, 1], vec![100, 1, 1]]));
        assert_eq!(true, traveling(2, vec![vec![6, 1, 1], vec![100, 1, 1]]));
        assert_eq!(true, traveling(3, vec![vec![1, 0, 1], vec![2, 1, 1], vec![3, 0, 1]]));
    }
}