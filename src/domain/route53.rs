use crate::base::Result;
use crate::domain::{AWSCredential, ChangeID, ChangeStatus, RecordName, RecordType, ZoneID, TTL};

use rusoto_credential::StaticProvider;
use rusoto_route53 as rusoto;

pub struct Route53(rusoto::Route53Client);

impl Route53 {
    pub fn new(credential: Option<AWSCredential>) -> Result<Self> {
        use rusoto_core::{HttpClient, Region};

        let client = if let Some(cred) = credential {
            let dispatcher = HttpClient::new()?;
            rusoto::Route53Client::new_with(
                dispatcher,
                Into::<StaticProvider>::into(cred),
                Region::UsEast1,
            )
        } else {
            rusoto::Route53Client::new(Region::UsEast1)
        };

        Ok(Route53(client))
    }

    fn client(&self) -> &rusoto::Route53Client {
        let Route53(client) = self;
        client
    }

    pub async fn upsert_record_simple(
        &self,
        zone_id: ZoneID,
        record_name: RecordName,
        record_type: RecordType,
        value: String,
        ttl: Option<TTL>,
    ) -> Result<ChangeID> {
        use rusoto_route53::{
            Change, ChangeBatch, ChangeResourceRecordSetsRequest, ChangeResourceRecordSetsResponse,
            ResourceRecord, ResourceRecordSet, Route53,
        };

        let resource_record = ResourceRecord { value };
        let resource_records = Some(vec![resource_record]);
        let resource_record_set = ResourceRecordSet {
            type_: record_type.to_string(),
            name: record_name.to_string(),
            resource_records,
            ttl: ttl.map(Into::into),
            ..ResourceRecordSet::default()
        };
        let change = Change {
            action: String::from("UPSERT"),
            resource_record_set,
        };
        let change_batch = ChangeBatch {
            changes: vec![change],
            comment: None,
        };
        let request = ChangeResourceRecordSetsRequest {
            change_batch,
            hosted_zone_id: zone_id.to_string(),
        };

        let ChangeResourceRecordSetsResponse { change_info } =
            self.client().change_resource_record_sets(request).await?;
        Ok(change_info.id.into())
    }

    pub async fn check_change_status(&self, change_id: &ChangeID) -> Result<ChangeStatus> {
        use rusoto_route53::{GetChangeRequest, GetChangeResponse, Route53};

        let request = GetChangeRequest {
            id: change_id.clone().to_string(),
        };
        let GetChangeResponse { change_info } = self.client().get_change(request).await?;
        change_info.status.parse()
    }
}
