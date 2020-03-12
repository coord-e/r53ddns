pub enum RecordType {
    A,
    AAAA,
}

impl ToString for RecordType {
    fn to_string(&self) -> String {
        String::from(match self {
            RecordType::A => "A",
            RecordType::AAAA => "AAAA",
        })
    }
}
