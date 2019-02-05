pub fn peak_index_in_a_mountain_array(a: Vec<i32>) -> i32 {
    let mut index = 1;
    loop {
        if a[index - 1] < a[index] && a[index] > a[index + 1] {
            return index as i32;
        }

        index += 1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert!(
            peak_index_in_a_mountain_array(
                vec![0, 1, 0],
            ) == 1
        );

        assert!(
            peak_index_in_a_mountain_array(
                vec![0, 2, 1, 0],
            ) == 1
        );
    }
}
