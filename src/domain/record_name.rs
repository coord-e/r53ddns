pub struct RecordName(String);

impl From<String> for RecordName {
    fn from(id: String) -> Self {
        RecordName(id)
    }
}

impl ToString for RecordName {
    fn to_string(&self) -> String {
        let RecordName(id) = self;
        id.clone()
    }
}
