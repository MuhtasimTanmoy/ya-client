/*
 * Yagna Market API
 *
 * The version of the OpenAPI document: 1.6.1
 *
 * Generated by: https://openapi-generator.tech
 */

use serde::{Deserialize, Serialize};

pub type NewOffer = DemandOfferBase;
pub type NewDemand = DemandOfferBase;
pub type NewProposal = DemandOfferBase;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DemandOfferBase {
    /// The object which includes all the Demand/Offer/Proposal properties.
    /// This is a JSON object in \"flat convention\" - where keys are full
    /// property names and their values indicate properties.
    ///
    /// The value's Javascript type shall conform with the type of the
    /// property (as indicated in Golem Standards).
    ///
    /// ### Example property object:
    /// ```json
    /// {
    ///     "golem.com.pricing.model": "linear",
    ///     "golem.com.pricing.model.linear.coeffs": [0.001, 0.002, 0.0],
    ///     "golem.com.scheme": "payu",
    ///     "golem.com.scheme.payu.interval_sec": 6.0,
    ///     "golem.com.usage.vector": ["golem.usage.duration_sec", "golem.usage.cpu_sec"],
    ///     "golem.inf.cpu.architecture": "x86_64",
    ///     "golem.inf.cpu.cores": 4,
    ///     "golem.inf.cpu.threads": 7,
    ///     "golem.inf.mem.gib": 10.612468048930168,
    ///     "golem.inf.storage.gib": 81.7227783203125,
    ///     "golem.node.debug.subnet": "market-devnet",
    ///     "golem.node.id.name": "tworec@mf-market-devnet",
    ///     "golem.runtime.name": "vm",
    ///     "golem.runtime.version@v": "0.1.0"
    /// }
    /// ```
    #[serde(rename = "properties")]
    pub properties: serde_json::Value,
    #[serde(rename = "constraints")]
    pub constraints: String,
}

impl DemandOfferBase {
    pub fn new(properties: serde_json::Value, constraints: String) -> DemandOfferBase {
        DemandOfferBase {
            properties,
            constraints,
        }
    }
}
