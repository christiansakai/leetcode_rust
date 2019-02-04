pub fn hamming_distance(x: i32, y: i32) -> i32 {
    let mut count = 0;
    let mut x = x;
    let mut y = y;

    while x > 0 || y > 0 {
        if x % 2 != y % 2 {
            count += 1;
        }

        x = x / 2;
        y = y / 2;
    }

    count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(hamming_distance(1, 4), 2);
    }
}
