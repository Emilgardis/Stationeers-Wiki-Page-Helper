use std::collections::BTreeMap;

use serde_derive::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, PartialOrd, Eq, Ord, Hash, Serialize, Deserialize)]
#[serde(rename = "Enums")]
pub struct Enums {
    #[serde(rename = "Enums")]
    pub enums: BTreeMap<String, i64>,
    #[serde(rename = "LogicBatchMethod")]
    pub logic_batch_method: BTreeMap<String, i64>,
    #[serde(rename = "LogicReagentMode")]
    pub logic_reagent_mode: BTreeMap<String, i64>,
    #[serde(rename = "LogicSlotType")]
    pub logic_slot_type: BTreeMap<String, i64>,
    #[serde(rename = "LogicType")]
    pub logic_type: BTreeMap<String, i64>,
}
