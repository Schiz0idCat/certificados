pub trait StringExtras: ToString {
    fn to_title_case(&self) -> String {
        self.to_string()
            .to_lowercase()
            .split_whitespace()
            .map(|word| {
                let mut chars = word.chars();

                match chars.next() {
                    None => String::new(),
                    Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(),
                }
            })
            .collect::<Vec<String>>()
            .join(" ")
    }
}

impl<T: ToString> StringExtras for T {}
