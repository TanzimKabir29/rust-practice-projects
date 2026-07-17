pub trait StrExt {
    fn capitalize_first(&self) -> String;
}

impl StrExt for str {
    fn capitalize_first(self: &Self) -> String {
        let mut chars = self.chars();
        match chars.next() {
            None => String::from(""),
            Some(first_letter) => first_letter.to_uppercase().collect::<String>() + chars.as_str(),
        }
    }
}
