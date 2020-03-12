use std::time::Duration;
use tokio::time::delay_for;
use rusoto_core::{Region, HttpClient};
use rusoto_credential::StaticProvider;
use rusoto_route53::{Route53, Route53Client, ChangeResourceRecordSetsRequest, ChangeResourceRecordSetsResponse, ChangeBatch, Change, ResourceRecordSet, ResourceRecord, GetChangeRequest, GetChangeResponse};
use structopt::StructOpt;

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

#[derive(StructOpt)]
#[structopt(name = "r53update")]
struct Opt {
    #[structopt(short, long, requires = "secret")]
    key: Option<String>,

    #[structopt(short, long, requires = "key")]
    secret: Option<String>,

    #[structopt(short, long, default_value = "300")]
    ttl: i64,

    #[structopt(short, long)]
    zone: String,

    #[structopt(short, long)]
    name: String,

    #[structopt(short, long)]
    ip: Option<String>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let opt = Opt::from_args();

    let hosted_zone_id = opt.zone;
    let value = if let Some(ip) = opt.ip { ip } else {
        get_ip().await?
    };
    let name = opt.name;
    let ttl = opt.ttl;

    let client =
        if let (Some(key), Some(secret)) = (opt.key, opt.secret) {
            let provider = StaticProvider::new_minimal(key, secret);
            let dispatcher = HttpClient::new()?;
            Route53Client::new_with(dispatcher, provider, Region::UsEast1)
        } else {
            Route53Client::new(Region::UsEast1)
        };

    let resource_record = ResourceRecord { value };
    let resource_records = Some(vec![resource_record]);
    let resource_record_set = ResourceRecordSet { type_: String::from("A"), name, resource_records, ttl: Some(ttl), .. ResourceRecordSet::default() };
    let change = Change { action: String::from("UPSERT"), resource_record_set };
    let change_batch = ChangeBatch { changes: vec![change], comment: None };
    let request = ChangeResourceRecordSetsRequest { change_batch, hosted_zone_id };

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
