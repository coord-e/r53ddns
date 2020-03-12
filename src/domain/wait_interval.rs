pub struct WaitInterval(u64);

impl From<u64> for WaitInterval {
    fn from(interval: u64) -> Self {
        WaitInterval(interval)
    }
}

impl Into<u64> for WaitInterval {
    fn into(self) -> u64 {
        let WaitInterval(interval) = self;
        interval
    }
}
