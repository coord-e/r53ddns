pub struct ZoneID(String);

impl From<String> for ZoneID {
    fn from(id: String) -> Self {
        ZoneID(id)
    }
}

impl ToString for ZoneID {
    fn to_string(&self) -> String {
        let ZoneID(id) = self;
        id.clone()
    }
}
