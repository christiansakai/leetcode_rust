pub fn flip_and_invert_image(a: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let mut result = Vec::with_capacity(a.len());

    for row in a {
        let mut new_row = Vec::new();
        for el in row.iter().rev() {
            if *el == 1 {
                new_row.push(0);
            } else {
                new_row.push(1);
            }
        }

        result.push(new_row);
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(
            flip_and_invert_image(
                vec![
                    vec![1, 1, 0],
                    vec![1, 0, 1],
                    vec![0, 0, 0],
                ]
            ),
            vec![
                vec![1, 0, 0],
                vec![0, 1, 0],
                vec![1, 1, 1],
            ],
        );

        assert_eq!(
            flip_and_invert_image(
                vec![
                    vec![1, 1, 0, 0],
                    vec![1, 0, 0, 1],
                    vec![0, 1, 1, 1],
                    vec![1, 0, 1, 0],
                ]
            ),
            vec![
                vec![1, 1, 0, 0],
                vec![0, 1, 1, 0],
                vec![0, 0, 0, 1],
                vec![1, 0, 1, 0],
            ],
        );
    }
}
