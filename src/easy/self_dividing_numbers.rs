pub fn self_dividing_numbers(left: i32, right: i32) -> Vec<i32> {
    let mut result = Vec::new();

    for n in left..right + 1 {
        if is_self_dividing_number(n) {
            result.push(n);
        }
    }

    result
}

fn is_self_dividing_number(n: i32) -> bool {
    let mut m = n;
    while m > 0 {
        let digit = m % 10;

        if digit == 0 || n % digit != 0 {
            return false;
        }

        m = m / 10;
    }

    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(
            self_dividing_numbers(1, 22),
            vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 11, 12, 15, 22],
        );
    }
}
