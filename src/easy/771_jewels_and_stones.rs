use std::collections::HashSet;

pub fn num_jewels_in_stones(j: String, s: String) -> i32 {
    let mut are_jewels = HashSet::new();
    for jewel in j.chars() {
        are_jewels.insert(jewel);
    }

    let mut jewel_count = 0;
    for stone in s.chars() {
        if are_jewels.contains(&stone) {
            jewel_count += 1;
        }
    }

    jewel_count
}

#[cfg(test)]
mod tests {
    #[test]
    fn test() {
        let j = String::from("aA");
        let s = String::from("aaAbbbbb");

        assert!(num_jewels_in_stones(j, s) == 3);
    }
}

