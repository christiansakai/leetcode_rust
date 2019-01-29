pub fn sort_array_by_parity(a: Vec<i32>) -> Vec<i32> {
    let mut even = Vec::with_capacity(a.len());
    let mut odd = Vec::with_capacity(a.len());

    for el in a {
        if el % 2 == 0 {
            even.push(el);
        } else {
            odd.push(el);
        }
    }

    even.append(&mut odd);
    even
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

