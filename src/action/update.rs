use crate::base::Result;
use crate::domain::{
    ChangeID, IPAddress, IPAddressKind::*, RecordName, RecordType, Route53, ZoneID, TTL,
};

pub struct UpdateInput<'route53> {
    pub route53: &'route53 Route53,
    pub zone_id: ZoneID,
    pub record_name: RecordName,
    pub ip: IPAddress,
    pub ttl: TTL,
}

pub async fn update<'route53>(input: UpdateInput<'route53>) -> Result<ChangeID> {
    let record_type = match input.ip.kind() {
        IPV4 => RecordType::A,
        IPV6 => RecordType::AAAA,
    };
    input
        .route53
        .upsert_record_simple(
            input.zone_id,
            input.record_name,
            record_type,
            input.ip.to_string(),
            Some(input.ttl),
        )
        .await
}
