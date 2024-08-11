use proconio::input;

const T: &'static [&'static str] = &["dreamer", "dream", "eraser", "erase"];

fn daydream(s: &str) -> bool {
    for t in T {
        let slice_end = t.len();
        if slice_end > s.len() {
            continue;
        }

        let slice_s = s.get(0..t.len()).unwrap();
        if &slice_s == t {
            if t.len() == s.len() {
                return true
            } else {
                let next_s = s.get(t.len()..s.len()).unwrap();
                if daydream(next_s) { return true }
            }
        }
    }

    false
}

fn main() {
    input! {
        s: String,
    };

    println!("{}", if daydream(s.as_str()) { "YES" } else { "NO" });
}

#[cfg(test)]
mod tests {
    use crate::daydream;

    #[test]
    fn test() {
        assert_eq!(true, daydream("erasedream"));
        assert_eq!(true, daydream("dreameraser"));
        assert_eq!(false, daydream("dreamerer"));
    }
}