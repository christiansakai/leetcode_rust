pub fn sort_array_by_parity(a: Vec<i32>) -> Vec<i32> {
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let sorted = sort_array_by_parity(vec![3, 1, 2, 4]);

        assert!(
            sorted == vec![2, 4, 3, 1]
            || sorted == vec![2, 4, 1, 3]
        );
    }
}

