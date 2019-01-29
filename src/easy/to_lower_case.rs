pub fn to_lower_case(string: String) -> String {
    let mut lowercased = String::new();
    for c in string.chars() {
        lowercased.push(c.to_ascii_lowercase());
    }

    lowercased
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(
            to_lower_case(String::from("Hello")),
            String::from("hello"),
        );
        assert_eq!(
            to_lower_case(String::from("Hello")),
            String::from("hello"),
        );
        assert_eq!(
            to_lower_case(String::from("Hello")),
            String::from("hello"),
        );
    }
}

