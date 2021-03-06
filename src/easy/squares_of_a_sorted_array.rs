pub fn sorted_squares(a: Vec<i32>) -> Vec<i32> {
    let mut result: Vec<i32> = a
        .into_iter()
        .map(|x| x * x)
        .collect();

    result.sort();
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(
            sorted_squares(vec![-4, -1, 0, 3, 10]),
            vec![0, 1, 9, 16, 100],
        );

        assert_eq!(
            sorted_squares(vec![-7, -3, 2, 3, 11]),
            vec![4, 9, 9, 49, 121],
        );
    }
}

