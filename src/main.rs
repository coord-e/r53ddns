use std::time::Duration;
use tokio::time::delay_for;
use rusoto_core::{Region, HttpClient};
use rusoto_credential::StaticProvider;
use rusoto_route53::{Route53, Route53Client, ChangeResourceRecordSetsRequest, ChangeResourceRecordSetsResponse, ChangeBatch, Change, ResourceRecordSet, ResourceRecord, GetChangeRequest, GetChangeResponse};

async fn get_ip() -> Result<String, reqwest::Error> {
    let mut content = reqwest::get("http://checkip.amazonaws.com").await?.text().await?;
    if content.ends_with('\n') {
        content.pop();
    }
    Ok(content)
}

async fn check_change_status(client: &Route53Client, id: &str) -> Result<String, Box<dyn std::error::Error>> {
    let request = GetChangeRequest { id: id.to_string() };
    let GetChangeResponse { change_info } = client.get_change(request).await?;
    Ok(change_info.status)
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let access_key_id = "<key id>".to_string();
    let secret_access_key = "<secret key>".to_string();
    let hosted_zone_id = "<id>".to_string();
    let value = get_ip().await?;
    let name = "<name>".to_string();
    let ttl = 300;

    eprintln!("IP: {}", value);

    let dispatcher = HttpClient::new()?;
    let provider = StaticProvider::new_minimal(access_key_id, secret_access_key);

    let resource_record = ResourceRecord { value };
    let resource_records = Some(vec![resource_record]);
    let resource_record_set = ResourceRecordSet { type_: String::from("A"), name, resource_records, ttl: Some(ttl), .. ResourceRecordSet::default() };
    let change = Change { action: String::from("UPSERT"), resource_record_set };
    let change_batch = ChangeBatch { changes: vec![change], comment: None };
    let request = ChangeResourceRecordSetsRequest { change_batch, hosted_zone_id };

    let client = Route53Client::new_with(dispatcher, provider, Region::UsEast1);
    match client.change_resource_record_sets(request).await {
        Ok(ChangeResourceRecordSetsResponse { change_info }) => {
            let id = change_info.id.trim_start_matches("/change/").to_string();
            println!("{}", id);
            while check_change_status(&client, &id).await? == "PENDING" {
                eprintln!("Waiting...");
                delay_for(Duration::from_secs(5)).await;
            }
        },
        Err(err) => {
            eprintln!("Error: {:?}", err);
        }
    };
    Ok(())
}
