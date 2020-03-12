use std::time::Duration;

use crate::base::Result;
use crate::domain::{ChangeID, ChangeStatus, Route53, WaitInterval};

use log::info;

pub struct WaitInput<'route53> {
    pub route53: &'route53 Route53,
    pub change_id: ChangeID,
    pub interval: WaitInterval,
}

pub async fn wait(input: WaitInput<'_>) -> Result<()> {
    use tokio::time::delay_for;

    let interval = input.interval.into();
    while input.route53.check_change_status(&input.change_id).await? == ChangeStatus::Pending {
        info!("Waiting the change to be in sync...");
        delay_for(Duration::from_secs(interval)).await;
    }
    info!("Done.");

    Ok(())
}
