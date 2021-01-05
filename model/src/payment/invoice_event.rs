use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use strum_macros::{EnumString, ToString};

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct InvoiceEvent {
    pub invoice_id: String,
    pub event_date: DateTime<Utc>,
    pub event_type: InvoiceEventType,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, EnumString, ToString)]
pub enum InvoiceEventType {
    #[strum(to_string = "RECEIVED")]
    InvoiceReceivedEvent,
    #[strum(to_string = "ACCEPTED")]
    InvoiceAcceptedEvent,
    #[strum(to_string = "REJECTED")]
    InvoiceRejectedEvent {
        rejection: crate::payment::Rejection,
    },
    #[strum(to_string = "CANCELLED")]
    InvoiceCancelledEvent,
    #[strum(to_string = "SETTLED")]
    InvoiceSettledEvent,
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::payment::{Rejection, RejectionReason};
    use chrono::TimeZone;

    #[test]
    fn test_serialize() {
        let ie = InvoiceEvent {
            invoice_id: "ajdik".to_string(),
            event_date: Utc
                .datetime_from_str("2020-12-21T15:51:21.126645Z", "%+")
                .unwrap(),
            event_type: InvoiceEventType::InvoiceSettledEvent,
        };

        assert_eq!(
            "{\"invoiceId\":\"ajdik\",\
              \"eventDate\":\"2020-12-21T15:51:21.126645Z\",\
              \"eventType\":\"InvoiceSettledEvent\"\
             }",
            serde_json::to_string(&ie).unwrap()
        );
    }

    #[test]
    fn test_deserialize() {
        let ie: InvoiceEvent = serde_json::from_str(
            "{\"invoiceId\":\"ajdik\",\
                \"eventDate\":\"2020-12-21T15:51:21.126645Z\",\
                \"eventType\":\"InvoiceAcceptedEvent\"\
            }",
        )
        .unwrap();

        assert_eq!(
            InvoiceEvent {
                invoice_id: "ajdik".to_string(),
                event_date: Utc
                    .datetime_from_str("2020-12-21T15:51:21.126645Z", "%+")
                    .unwrap(),
                event_type: InvoiceEventType::InvoiceAcceptedEvent,
            },
            ie
        );
    }

    #[test]
    fn test_serialize_event() {
        let iet = InvoiceEventType::InvoiceRejectedEvent {
            rejection: Rejection {
                rejection_reason: RejectionReason::UnsolicitedService,
                total_amount_accepted: Default::default(),
                message: None,
            },
        };
        assert_eq!(
            "{\"InvoiceRejectedEvent\":\
                {\"rejection\":\
                    {\"rejectionReason\":\"UNSOLICITED_SERVICE\",\
                        \"totalAmountAccepted\":\"0\"\
                    }\
                }\
            }",
            serde_json::to_string(&iet).unwrap()
        );
    }

    #[test]
    fn test_deserialize_event() {
        let iet: InvoiceEventType = serde_json::from_str("\"InvoiceReceivedEvent\"").unwrap();
        assert_eq!(InvoiceEventType::InvoiceReceivedEvent, iet);
    }

    #[test]
    fn test_deserialize_from_str() {
        let iet: InvoiceEventType = "RECEIVED".parse().unwrap();
        assert_eq!(InvoiceEventType::InvoiceReceivedEvent, iet);
    }

    #[test]
    fn test_deserialize_to_string() {
        assert_eq!(InvoiceEventType::InvoiceSettledEvent.to_string(), "SETTLED");
    }
}
