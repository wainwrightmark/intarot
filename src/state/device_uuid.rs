#[derive(PartialEq, Eq, Clone, serde:: Serialize, serde::Deserialize, Debug)]
#[serde(transparent)]
pub struct DeviceUUID(pub String);
