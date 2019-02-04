pub fn min_deletion_size(_a: Vec<String>) -> i32 {
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(
            min_deletion_size(vec![
                String::from("cba"),
                String::from("daf"),
                String::from("ghi"),
            ]),
            1
        );

        assert_eq!(
            min_deletion_size(vec![
                String::from("a"),
                String::from("b"),
            ]),
            0
        );
    }
}
