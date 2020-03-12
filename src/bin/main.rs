use r53ddns::action;
use r53ddns::action::{UpdateInput, WaitInput};
use r53ddns::base::Result;
use r53ddns::domain::{AWSCredential, Route53};

use structopt::StructOpt;

#[derive(StructOpt)]
#[structopt(name = "r53ddns")]
struct Opt {
    #[structopt(short, long, requires = "secret")]
    key: Option<String>,

    #[structopt(short, long, requires = "key")]
    secret: Option<String>,

    #[structopt(short, long, default_value = "300")]
    ttl: u32,

    #[structopt(short, long)]
    zone: String,

    #[structopt(short, long)]
    name: String,

    #[structopt(short, long)]
    ip: Option<String>,
}

async fn run() -> Result<()> {
    let opt = Opt::from_args();

    let credential = if let (Some(key), Some(secret)) = (opt.key, opt.secret) {
        Some(AWSCredential::new(key, secret))
    } else {
        None
    };

    let route53 = Route53::new(credential)?;
    let target_ip = if let Some(ip) = opt.ip {
        ip.parse()?
    } else {
        action::get_ip().await?
    };

    let change_id = action::update(UpdateInput {
        route53: &route53,
        zone_id: opt.zone.into(),
        record_name: opt.name.into(),
        ip: target_ip,
        ttl: opt.ttl.into(),
    })
    .await?;

    action::wait(WaitInput {
        route53: &route53,
        change_id,
        interval: 5.into(),
    })
    .await?;

    Ok(())
}

#[tokio::main]
async fn main() {
    std::process::exit(match run().await {
        Ok(()) => 0,
        Err(e) => {
            eprintln!("r53ddns: {}", e);
            1
        }
    });
}
