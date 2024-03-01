#![allow(dead_code)]
fn collatz_length(mut n: i32) -> u32 {
    let mut length = 1;
    while n != 1 {
        if n % 2 == 0 {
            n /= 2;
        } else {
            n = 3 * n + 1;
        }
        length += 1;
    }

    length
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_collatz_length() {
        assert_eq!(collatz_length(1), 1);
        assert_eq!(collatz_length(3), 8);
    }
}
