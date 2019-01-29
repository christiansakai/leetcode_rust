use std::collections::HashMap;

pub fn anagram_mappings(a: Vec<i32>, b: Vec<i32>) -> Vec<i32> {
    let mut b_map: HashMap<i32, Vec<i32>> = HashMap::new(); 
    for (index, el) in b.into_iter().enumerate() {
        match b_map.get_mut(&el) {
            None => {
                b_map.insert(el, vec![index as i32]);
            },
            Some(indexes_vec) => {
                indexes_vec.push(index as i32);
            },
        }
    }

    let mut result = Vec::new();
    for el in a {
        let b_indexes_vec = b_map.get_mut(&el).unwrap();
        let index = b_indexes_vec.pop().unwrap();
        result.push(index);
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let a = vec![12, 28, 46, 32, 50];
        let b = vec![50, 12, 32, 46, 28];

        assert!(anagram_mappings(a, b) == vec![1, 4, 3, 2, 0]);
    }
}

