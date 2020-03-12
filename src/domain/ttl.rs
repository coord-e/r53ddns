pub struct TTL(u32);

impl From<u32> for TTL {
    fn from(ttl: u32) -> Self {
        TTL(ttl)
    }
}

impl Into<i64> for TTL {
    fn into(self) -> i64 {
        let TTL(ttl) = self;
        ttl as i64
    }
}
