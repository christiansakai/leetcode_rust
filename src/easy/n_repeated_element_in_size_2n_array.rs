use std::collections::HashMap;

pub fn repeated_n_times(a: Vec<i32>) -> i32 {
    let mut map = HashMap::new();
    for el in &a {
        match map.get(el) {
            None => {
                map.insert(*el, 1);
            },
            Some(n) => {
                map.insert(*el, n + 1);
            }
        }
    }

    let n: i32 = a.len() as i32 / 2;

    for el in &a {
        let count = map.get(&el).unwrap();
        if *count == n {
            return *el;
        }
    }

    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(repeated_n_times(vec![1, 2, 3, 3]), 3);
        assert_eq!(repeated_n_times(vec![2, 1, 2, 5, 3, 2]), 2);
        assert_eq!(repeated_n_times(vec![5, 1, 5, 2, 5, 3, 5, 4]), 5);
    }
}

