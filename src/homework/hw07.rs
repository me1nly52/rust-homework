pub fn invert_the_case(s: String) -> String {
    s.chars()
     .map(|c| {
         if c.is_lowercase() {
             c.to_uppercase().next().unwrap()
         } else if c.is_uppercase() {
             c.to_lowercase().next().unwrap()
         } else {
             c
         }
     })
     .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_invert_the_case() {
        let data = [
            ("Hello", "hELLO"),
            ("Привет", "пРИВЕТ"),
        ];

        for (a, b) in data {
            assert_eq!(invert_the_case(a.to_string()), b.to_string());
            assert_eq!(invert_the_case(b.to_string()), a.to_string());
        }
    }
}
