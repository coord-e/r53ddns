pub struct ChangeID(String);

impl From<&str> for ChangeID {
    fn from(id: &str) -> Self {
        let trimmed = id.trim().trim_start_matches("/change/");
        ChangeID(trimmed.to_string())
    }
}

impl From<String> for ChangeID {
    fn from(id: String) -> Self {
        ChangeID::from(id.as_str())
    }
}

impl ToString for ChangeID {
    fn to_string(&self) -> String {
        let ChangeID(id) = self;
        id.clone()
    }
}
