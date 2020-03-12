use rusoto_credential::StaticProvider;

pub struct AWSCredential {
    key: String,
    secret: String,
}

impl AWSCredential {
    pub fn new(key: String, secret: String) -> Self {
        AWSCredential { key, secret }
    }
}

impl Into<StaticProvider> for AWSCredential {
    fn into(self) -> StaticProvider {
        let AWSCredential { key, secret } = self;
        StaticProvider::new_minimal(key, secret)
    }
}
