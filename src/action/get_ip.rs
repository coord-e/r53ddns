use crate::base::{Error, Result};
use crate::domain::IPAddress;

pub async fn get_ip() -> Result<IPAddress> {
    let mut content = reqwest::get("http://checkip.amazonaws.com")
        .await
        .map_err(Error::GetIP)?
        .text()
        .await
        .map_err(Error::GetIP)?;

    if content.ends_with('\n') {
        content.pop();
    }
    content.parse()
}
