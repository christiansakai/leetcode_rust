use std::collections::HashSet;

pub fn num_unique_emails(emails: Vec<String>) -> i32 {
    let mut unique_emails: HashSet<String> = HashSet::new();

    for email in &emails {
        let local_and_domain: Vec<&str> = email
                .split("@")
                .collect();

        let local = &local_and_domain[0];
        let domain = &local_and_domain[1];

        match local.find("+") {
            Some(i) => {
                let mut local_and_domain = (&local[0..i])
                    .to_owned()
                    .replace(".", "");

                local_and_domain.push_str("@");
                local_and_domain.push_str(domain);

                unique_emails.insert(local_and_domain);
            },
            None => {
                let mut local_and_domain = local.replace(".", "");
                local_and_domain.push_str("@");
                local_and_domain.push_str(domain);

                unique_emails.insert(local_and_domain);
            }
        }

    }

    unique_emails.len() as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let emails = vec![
            String::from("alice.z@leetcode.com"),
            String::from("alicez@leetcode.com"),
            String::from("alice+z@leetcode.com"),
            String::from("m.y+@leetcode.com"),
            String::from("my+@leetcode.com"),
        ];

        assert!(num_unique_emails(emails) == 3);
    }
}

