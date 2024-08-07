use indexmap::IndexMap;
use serde_derive::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Stationpedia")]
pub struct Stationpedia {
    pub pages: Vec<Page>,
    pub reagents: IndexMap<String, Reagent>,
    #[serde(rename = "scriptCommands")]
    pub script_commands: std::collections::BTreeMap<String, Command>,
    pub core_prefabs: Vec<CorePrefab>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]

pub struct CorePrefabSlot {
    #[serde(rename = "SlotClass")]
    pub slot_class: String,
    #[serde(rename = "SlotName")]
    pub slot_name: String,
    #[serde(rename = "StringHash")]
    pub string_hash: i64,
    #[serde(rename = "StringKey")]
    pub string_key: String,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]

pub struct HumanPrefab {
    #[serde(rename = "BaseNutritionStorage")]
    pub base_nutrition_storage: f64,
    #[serde(rename = "CriticalHealth")]
    pub critical_health: f64,
    #[serde(rename = "CriticalHydration")]
    pub critical_hydration: f64,
    #[serde(rename = "CriticalHygiene")]
    pub critical_hygiene: f64,
    #[serde(rename = "CriticalMood")]
    pub critical_mood: f64,
    #[serde(rename = "CriticalNutrition")]
    pub critical_nutrition: f64,
    #[serde(rename = "CriticalOxygen")]
    pub critical_oxygen: f64,
    #[serde(rename = "DehydrationDamageRateAwake")]
    pub dehydration_damage_rate_awake: f64,
    #[serde(rename = "DehydrationDamageRateSleeping")]
    pub dehydration_damage_rate_sleeping: f64,
    #[serde(rename = "FoodQuality")]
    pub food_quality: f64,
    #[serde(rename = "FullNutrition")]
    pub full_nutrition: f64,
    #[serde(rename = "Hydration")]
    pub hydration: f64,
    #[serde(rename = "Hygiene")]
    pub hygiene: f64,
    #[serde(rename = "MaxFoodQuality")]
    pub max_food_quality: f64,
    #[serde(rename = "MaxHydration")]
    pub max_hydration: f64,
    #[serde(rename = "MaxHygiene")]
    pub max_hygiene: f64,
    #[serde(rename = "MaxMood")]
    pub max_mood: f64,
    #[serde(rename = "MaxOxygenStorage")]
    pub max_oxygen_storage: f64,
    #[serde(rename = "Mood")]
    pub mood: f64,
    #[serde(rename = "Nutrition")]
    pub nutrition: f64,
    #[serde(rename = "NutritionDamageRateAwake")]
    pub nutrition_damage_rate_awake: f64,
    #[serde(rename = "NutritionDamageRateSleeping")]
    pub nutrition_damage_rate_sleeping: f64,
    #[serde(rename = "Oxygenation")]
    pub oxygenation: f64,
    #[serde(rename = "Slots")]
    pub slots: Vec<CorePrefabSlot>,
    #[serde(rename = "ToxicPartialPressureDamage")]
    pub toxic_partial_pressure_damage: f64,
    #[serde(rename = "ToxicPartialPressureWarning")]
    pub toxic_partial_pressure_warning: f64,
    #[serde(rename = "WarningHealth")]
    pub warning_health: f64,
    #[serde(rename = "WarningHydration")]
    pub warning_hydration: f64,
    #[serde(rename = "WarningHygiene")]
    pub warning_hygiene: f64,
    #[serde(rename = "WarningMood")]
    pub warning_mood: f64,
    #[serde(rename = "WarningNutrition")]
    pub warning_nutrition: f64,
    #[serde(rename = "WarningOxygen")]
    pub warning_oxygen: f64,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]

pub struct CorePrefabItem {
    #[serde(rename = "SlotClass")]
    pub slot_class: String,
    #[serde(rename = "SortingClass")]
    pub sorting_class: String,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]

pub struct CorePrefabThermal {
    #[serde(rename = "Convection")]
    pub convection: f64,
    #[serde(rename = "Radiation")]
    pub radiation: f64,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]

pub struct CorePrefab {
    #[serde(rename = "Human")]
    pub human: Option<HumanPrefab>,
    #[serde(rename = "Item")]
    pub item: CorePrefabItem,
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "Thermal")]
    pub thermal: Option<CorePrefabThermal>,
    #[serde(rename = "Organ")]
    pub organ: Option<serde_json::value::Value>,
    #[serde(rename = "Lungs")]
    pub lungs: Option<Lungs>,
    #[serde(rename = "Brain")]
    pub brain: Option<serde_json::value::Value>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Lungs {
    #[serde(rename = "BreathableType")]
    pub breathable_type: String,
    #[serde(rename = "TemperatureMax")]
    pub temperature_max: f64,
    #[serde(rename = "TemperatureMin")]
    pub temperature_min: f64,
    #[serde(rename = "ToxicTypes")]
    pub toxic_types: Vec<String>,
    #[serde(rename = "Volume")]
    pub volume: f64,
}

impl Stationpedia {
    pub fn lookup_prefab_name(&self, prefab_name: &'_ str) -> Option<&Page> {
        self.pages.iter().find(|p| p.prefab_name == prefab_name)
    }

    pub fn lookup_key(&self, key: &str) -> Option<&Page> {
        self.pages.iter().find(|p| p.key == key)
    }

    pub fn lookup_hash(&self, hash: i64) -> Option<&Page> {
        self.pages.iter().find(|p| p.prefab_hash == hash)
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Reagent {
    #[serde(rename = "Hash")]
    pub hash: i64,
    #[serde(rename = "Unit")]
    pub unit: String,
    #[serde(rename = "Sources")]
    pub sources: Option<IndexMap<String, f64>>,
}

#[derive(Clone, Debug, PartialEq, PartialOrd, Eq, Ord, Hash, Serialize, Deserialize)]
pub struct Command {
    pub desc: String,
    pub example: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Page {
    #[serde(rename = "ConnectionInsert")]
    pub connection_insert: Vec<ConnectionInsert>,
    #[serde(rename = "ConstructedByKits")]
    pub constructs: Vec<Constructs>,
    #[serde(rename = "Description")]
    pub description: String,
    #[serde(rename = "Device")]
    pub device: Option<Device>,
    /// the item , if none then deprecated
    #[serde(rename = "Item")]
    pub item: Option<Item>,
    #[serde(rename = "Structure")]
    pub structure: Option<Structure>,
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
    #[serde(rename = "BasePowerDraw")]
    pub base_power_draw: Option<String>,
    #[serde(rename = "MaxPressure")]
    pub max_pressure: Option<String>,
    #[serde(rename = "GrowthTime")]
    pub growth_time: Option<String>,
    #[serde(rename = "CircuitHolder", default)]
    pub circuit_holder: bool,
    #[serde(rename = "Thermal")]
    pub thermal: Option<Thermal>,
    #[serde(rename = "ResourceConsumer")]
    pub resource_consumer: Option<ResourceConsumer>,
    #[serde(rename = "Chargeable")]
    pub chargeable: Option<Chargeable>,
    #[serde(rename = "InternalAtmosphere")]
    pub internal_atmosphere: Option<InternalAtmosphere>,
    #[serde(rename = "SourceCode", default)]
    pub source_code: bool,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct InternalAtmosphere {
    #[serde(rename = "Volume")]
    pub volume: f64,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Chargeable {
    #[serde(rename = "PowerMaximum")]
    pub power_maximum: f64,
}

#[derive(Clone, Debug, PartialEq, PartialOrd, Ord, Eq, Serialize, Deserialize)]
pub struct ResourceConsumer {
    #[serde(rename = "ConsumedResources")]
    pub consumed_resources: Vec<String>,
    #[serde(rename = "ProcessedReagents")]
    pub processed_reagents: Vec<i64>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Thermal {
    #[serde(rename = "Radiation")]
    pub radiation: f64,
    #[serde(rename = "Convection")]
    pub convection: f64,
}

#[derive(Clone, Debug, PartialEq, PartialOrd, Eq, Ord, Hash, Serialize, Deserialize)]
pub struct Constructs {
    #[serde(rename = "NameOfThing")]
    pub name_of_thing: String,
    #[serde(rename = "PageLink")]
    pub page_link: String,
    #[serde(rename = "PrefabHash")]
    pub prefab_hash: i64,
}

#[derive(Clone, Debug, PartialEq, PartialOrd, Eq, Ord, Hash, Serialize, Deserialize)]
pub struct Structure {
    #[serde(rename = "SmallGrid")]
    pub small_grid: bool,
    #[serde(rename = "BuildStates")]
    pub build_states: BuildStates,
}

#[derive(Clone, Debug, PartialEq, PartialOrd, Eq, Ord, Hash, Serialize, Deserialize)]
pub struct BuildStates(pub Vec<BuildState>);

#[derive(Clone, Debug, PartialEq, PartialOrd, Eq, Ord, Hash, Serialize, Deserialize)]
pub struct BuildState {
    #[serde(rename = "Tool")]
    pub tool: Option<Vec<Tool>>,
    #[serde(rename = "ToolExit")]
    pub tool_exit: Option<Vec<Tool>>,
    #[serde(rename = "CanManufacture", default)]
    pub can_manufacture: bool,
    #[serde(rename = "MachineTier")]
    pub machine_tier: Option<MachineTier>,
}

#[derive(Clone, Debug, PartialEq, PartialOrd, Eq, Ord, Hash, Serialize, Deserialize)]
pub enum MachineTier {
    Undefined,
    TierOne,
    TierTwo,
    TierThree,
    Max,
}

#[derive(Clone, Debug, PartialEq, PartialOrd, Eq, Ord, Hash, Serialize, Deserialize)]
pub struct Tool {
    #[serde(rename = "IsTool", default)]
    pub is_tool: bool,
    #[serde(rename = "PrefabName")]
    pub prefab_name: String,
    #[serde(rename = "Quantity")]
    pub quantity: Option<i64>,
}

#[derive(Clone, Debug, PartialEq, PartialOrd, Eq, Ord, Hash, Serialize, Deserialize)]
pub struct SlotInsert {
    #[serde(rename = "SlotIndex")]
    pub slot_index: String,
    #[serde(rename = "SlotName")]
    pub slot_name: String,
    #[serde(rename = "SlotType")]
    pub slot_type: String,
}

#[derive(Clone, Debug, PartialEq, PartialOrd, Eq, Ord, Hash, Serialize, Deserialize)]
pub struct LogicInsert {
    #[serde(rename = "LogicAccessTypes")]
    pub logic_access_types: String,
    #[serde(rename = "LogicName")]
    pub logic_name: String,
}

#[derive(Clone, Debug, PartialEq, PartialOrd, Eq, Ord, Hash, Serialize, Deserialize)]
pub struct LogicSlotInsert {
    #[serde(rename = "LogicAccessTypes")]
    pub logic_access_types: String,
    #[serde(rename = "LogicName")]
    pub logic_name: String,
}

#[derive(Clone, Debug, PartialEq, PartialOrd, Eq, Ord, Hash, Serialize, Deserialize)]
pub struct ModeInsert {
    #[serde(rename = "LogicAccessTypes")]
    pub logic_access_types: String,
    #[serde(rename = "LogicName")]
    pub logic_name: String,
}

#[derive(Clone, Debug, PartialEq, PartialOrd, Eq, Ord, Hash, Serialize, Deserialize)]
pub struct ConnectionInsert {
    #[serde(rename = "LogicAccessTypes")]
    pub logic_access_types: String,
    #[serde(rename = "LogicName")]
    pub logic_name: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct LogicInfo {
    #[serde(rename = "LogicSlotTypes")]
    pub logic_slot_types: IndexMap<String, LogicSlot>,
    #[serde(rename = "LogicTypes")]
    pub logic_types: LogicTypes,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct LogicSlot {
    #[serde(flatten)]
    pub slot_types: IndexMap<String, String>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct LogicTypes {
    #[serde(flatten)]
    pub types: IndexMap<String, String>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Memory {
    #[serde(rename = "Instructions")]
    pub instructions: Option<IndexMap<String, Instruction>>,
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

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
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
    pub reagents: Option<IndexMap<String, f64>>,
    #[serde(rename = "SlotClass")]
    pub slot_class: String,
    #[serde(rename = "SortingClass")]
    pub sorting_class: String,
    #[serde(rename = "Recipes", default)]
    pub recipes: Vec<Recipe>,
    #[serde(rename = "Food")]
    pub food: Option<Food>,
    #[serde(rename = "Wearable", default)]
    pub wearable: bool,
    #[serde(rename = "Suit")]
    pub suit: Option<Suit>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]

pub struct Suit {
    #[serde(rename = "HygineReductionMultiplier")]
    pub hygine_reduction_multiplier: f64,
    #[serde(rename = "WasteMaxPressure")]
    pub pressure: f64,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Food {
    #[serde(rename = "NutritionQuality")]
    pub nutrition_quality: i64,
    #[serde(rename = "NutritionValue")]
    pub nutrition_value: Option<f64>,
    #[serde(rename = "NutritionQualityReadable")]
    pub nutrition_quality_readable: String,
    #[serde(rename = "MoodBonus")]
    pub mood_bonus: Option<f64>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Recipe {
    #[serde(rename = "CreatorPrefabName")]
    pub creator_prefab_name: String,
    #[serde(rename = "TierName")]
    pub tier_name: String,
    #[serde(rename = "Time", default)]
    pub time: f64,
    #[serde(rename = "Energy", default)]
    pub energy: f64,
    #[serde(rename = "Temperature")]
    pub temperature: RecipeTemperature,
    #[serde(rename = "Pressure")]
    pub pressure: RecipePressure,
    #[serde(rename = "RequiredMix")]
    pub required_mix: RecipeGasMix,
    #[serde(rename = "CountTypes", default)]
    pub count_types: i64,
    #[serde(flatten)]
    pub reagents: IndexMap<String, f64>,
}

#[derive(Clone, Debug, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct RecipeTemperature {
    #[serde(rename = "Start")]
    pub start: f64,
    #[serde(rename = "Stop")]
    pub stop: f64,
    #[serde(rename = "IsValid")]
    pub is_valid: bool,
}

#[derive(Clone, Debug, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct RecipePressure {
    #[serde(rename = "Start")]
    pub start: f64,
    #[serde(rename = "Stop")]
    pub stop: f64,
    #[serde(rename = "IsValid")]
    pub is_valid: bool,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RecipeGasMix {
    #[serde(rename = "Rule")]
    pub rule: i64,
    #[serde(rename = "IsAny")]
    pub is_any: bool,
    #[serde(rename = "IsAnyToRemove")]
    pub is_any_to_remove: bool,
    #[serde(flatten)]
    pub reagents: IndexMap<String, f64>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
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
    #[serde(rename = "Fabricator")]
    pub fabricator: Option<Fabricator>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Fabricator {
    #[serde(rename = "Tier")]
    pub tier: i64,
    #[serde(rename = "TierName")]
    pub tier_name: MachineTier,
    #[serde(rename = "Recipes")]
    pub recipes: IndexMap<String, Recipe>,
}
