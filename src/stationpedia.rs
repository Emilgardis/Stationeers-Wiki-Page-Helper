use serde_derive::{Deserialize, Serialize};
use std::collections::BTreeMap;

#[derive(Clone, Debug, PartialEq, PartialOrd, Serialize, Deserialize)]
#[serde(rename = "Stationpedia", deny_unknown_fields)]
pub struct Stationpedia {
    pub pages: Vec<Page>,
    pub reagents: BTreeMap<String, Reagent>,
    #[serde(rename = "scriptCommands")]
    pub script_commands: BTreeMap<String, Command>,
}

#[derive(Clone, Debug, PartialEq, PartialOrd, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Reagent {
    #[serde(rename = "Hash")]
    pub hash: i64,
    #[serde(rename = "Unit")]
    pub unit: String,
    #[serde(rename = "Sources")]
    pub sources: Option<BTreeMap<String, f64>>,
}

#[derive(Clone, Debug, PartialEq, PartialOrd, Eq, Ord, Hash, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Command {
    pub desc: String,
    pub example: String,
}

#[derive(Clone, Debug, PartialEq, PartialOrd, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Page {
    #[serde(rename = "ConnectionInsert")]
    pub connection_insert: Vec<ConnectionInsert>,
    #[serde(rename = "Description")]
    pub description: String,
    #[serde(rename = "Device")]
    pub device: Option<Device>,
    /// the item name, if none then deprecated
    #[serde(rename = "Item")]
    pub item: Option<Item>,
    #[serde(rename = "Key")]
    pub key: String,
    #[serde(rename = "LogicInfo")]
    pub logic_info: Option<LogicInfo>,
    #[serde(rename = "LogicInsert")]
    pub logic_insert: Vec<LogicInsert>,
    #[serde(rename = "LogicSlotInsert")]
    pub logic_slot_insert: Vec<LogicSlotInsert>,
    #[serde(rename = "Memory")]
    pub memory: Option<Memory>,
    #[serde(rename = "ModeInsert")]
    pub mode_insert: Vec<ModeInsert>,
    #[serde(rename = "PrefabHash")]
    pub prefab_hash: i64,
    #[serde(rename = "PrefabName")]
    pub prefab_name: String,
    #[serde(rename = "SlotInserts")]
    pub slot_inserts: Vec<SlotInsert>,
    #[serde(rename = "Title")]
    pub title: String,
    #[serde(rename = "TransmissionReceiver")]
    pub transmission_receiver: Option<bool>,
    #[serde(rename = "WirelessLogic")]
    pub wireless_logic: Option<bool>,
}

#[derive(Clone, Debug, PartialEq, PartialOrd, Eq, Ord, Hash, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SlotInsert {
    #[serde(rename = "SlotIndex")]
    pub slot_index: String,
    #[serde(rename = "SlotName")]
    pub slot_name: String,
    #[serde(rename = "SlotType")]
    pub slot_type: String,
}

#[derive(Clone, Debug, PartialEq, PartialOrd, Eq, Ord, Hash, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct LogicInsert {
    #[serde(rename = "LogicAccessTypes")]
    pub logic_access_types: String,
    #[serde(rename = "LogicName")]
    pub logic_name: String,
}

#[derive(Clone, Debug, PartialEq, PartialOrd, Eq, Ord, Hash, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct LogicSlotInsert {
    #[serde(rename = "LogicAccessTypes")]
    pub logic_access_types: String,
    #[serde(rename = "LogicName")]
    pub logic_name: String,
}

#[derive(Clone, Debug, PartialEq, PartialOrd, Eq, Ord, Hash, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ModeInsert {
    #[serde(rename = "LogicAccessTypes")]
    pub logic_access_types: String,
    #[serde(rename = "LogicName")]
    pub logic_name: String,
}

#[derive(Clone, Debug, PartialEq, PartialOrd, Eq, Ord, Hash, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ConnectionInsert {
    #[serde(rename = "LogicAccessTypes")]
    pub logic_access_types: String,
    #[serde(rename = "LogicName")]
    pub logic_name: String,
}

#[derive(Clone, Debug, PartialEq, PartialOrd, Eq, Ord, Hash, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct LogicInfo {
    #[serde(rename = "LogicSlotTypes")]
    pub logic_slot_types: BTreeMap<String, LogicSlot>,
    #[serde(rename = "LogicTypes")]
    pub logic_types: LogicTypes,
}

#[derive(Clone, Debug, PartialEq, PartialOrd, Eq, Ord, Hash, Serialize, Deserialize)]
pub struct LogicSlot {
    #[serde(flatten)]
    pub slot_types: BTreeMap<String, String>,
}

#[derive(Clone, Debug, PartialEq, PartialOrd, Eq, Ord, Hash, Serialize, Deserialize)]
pub struct LogicTypes {
    #[serde(flatten)]
    pub types: BTreeMap<String, String>,
}

#[derive(Clone, Debug, PartialEq, PartialOrd, Eq, Ord, Hash, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Memory {
    #[serde(rename = "Instructions")]
    pub instructions: Option<BTreeMap<String, Instruction>>,
    #[serde(rename = "MemoryAccess")]
    pub memory_access: String,
    #[serde(rename = "MemorySize")]
    pub memory_size: i64,
    #[serde(rename = "MemorySizeReadable")]
    pub memory_size_readable: String,
}

#[derive(Clone, Debug, PartialEq, PartialOrd, Eq, Ord, Hash, Serialize, Deserialize)]
pub struct Instruction {
    #[serde(rename = "Description")]
    pub description: String,
    #[serde(rename = "Type")]
    pub type_: String,
    #[serde(rename = "Value")]
    pub value: i64,
}

#[derive(Clone, Debug, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct Item {
    #[serde(rename = "Consumable")]
    pub consumable: Option<bool>,
    #[serde(rename = "FilterType")]
    pub filter_type: Option<String>,
    #[serde(rename = "Ingredient")]
    pub ingredient: Option<bool>,
    #[serde(rename = "MaxQuantity")]
    pub max_quantity: Option<f64>,
    #[serde(rename = "Reagents")]
    pub reagents: Option<BTreeMap<String, f64>>,
    #[serde(rename = "SlotClass")]
    pub slot_class: String,
    #[serde(rename = "SortingClass")]
    pub sorting_class: String,
}

#[derive(Clone, Debug, PartialEq, PartialOrd, Eq, Ord, Hash, Serialize, Deserialize)]
pub struct Device {
    #[serde(rename = "ConnectionList")]
    pub connection_list: Vec<Vec<String>>,
    #[serde(rename = "DevicesLength")]
    pub devices_length: Option<i64>,
    #[serde(rename = "HasActivateState")]
    pub has_activate_state: bool,
    #[serde(rename = "HasAtmosphere")]
    pub has_atmosphere: bool,
    #[serde(rename = "HasColorState")]
    pub has_color_state: bool,
    #[serde(rename = "HasLockState")]
    pub has_lock_state: bool,
    #[serde(rename = "HasModeState")]
    pub has_mode_state: bool,
    #[serde(rename = "HasOnOffState")]
    pub has_on_off_state: bool,
    #[serde(rename = "HasOpenState")]
    pub has_open_state: bool,
    #[serde(rename = "HasReagents")]
    pub has_reagents: bool,
}
