use std::collections::HashMap;

pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut result = Vec::new();
    let mut map = HashMap::new();

    for (index, num) in nums.iter().enumerate() {
        let complement = target - num;

        if let Some(&complement_index) = map.get(&complement) {
            if index != complement_index {
                result.push(complement_index as i32);
                result.push(index as i32);
                break;
            }
        }

        map.insert(num, index);
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(
            two_sum(vec![2, 7, 11, 15], 9),
            vec![0, 1],
        );
    }
}
