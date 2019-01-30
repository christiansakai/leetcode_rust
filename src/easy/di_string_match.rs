use std::collections::VecDeque;

pub fn di_string_match(s: String) -> Vec<i32> {
    let mut values = VecDeque::new();
    for i in 0..s.len() + 1 {
        values.push_back(i as i32);
    }

    let mut result = Vec::new();
    if s.starts_with("I") {
        result.push(values.pop_front().unwrap());
    } else {
        result.push(values.pop_back().unwrap());
    }

    for c in s.chars() {
        if c == 'I' {
            result.push(values.pop_back().unwrap());
        } else {
            result.push(values.pop_front().unwrap());
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(
            di_string_match(String::from("IDID")),
            vec![0, 4, 1, 3, 2],
        );

        assert_eq!(
            di_string_match(String::from("III")),
            vec![0, 1, 2, 3],
        );

        assert_eq!(
            di_string_match(String::from("DDI")),
            vec![3, 2, 0, 1],
        );
    }
}
