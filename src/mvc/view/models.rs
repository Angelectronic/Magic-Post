use serde::{Serialize, Deserialize, Deserializer};

/* models */
#[derive(Serialize, Clone, Debug, Deserialize)]
pub struct CreateEmployeeData {
    pub id: String,
    pub name: String,
    pub position: String,
    pub point_id: String,
}

#[derive(Serialize, Clone, Debug, Deserialize)]
pub struct SignupData {
    pub username: String,
    pub password: String,
    pub name: Option<String>,
    pub position: Option<String>,
    pub point_id: Option<String>,
}

#[derive(Serialize, Clone, Debug, Deserialize)]
pub struct LoginData {
    pub username: String,
    pub password: String,
}

#[derive(Serialize, Clone, Debug, Deserialize)]
pub struct UpdateEmployee {
    pub name: Option<String>,
    pub position: Option<String>,
    pub point_id: Option<String>,
}

#[derive(Serialize, Clone, Debug, Deserialize)]
pub struct PointData {
    pub id: String,
    pub location: String,
    pub p_type: String,
    pub gathering_point: Option<String>,
}

#[derive(Serialize, Clone, Debug, Deserialize)]
pub struct AddPoint {
    pub location: Option<String>,
    pub p_type: Option<i8>,
    pub gathering_point: Option<String>,
}

#[derive(Serialize, Clone, Debug, Deserialize)]
pub struct PackageData {
    pub id: String,
    pub send_point: Option<String>,
    pub receive_point: Option<String>,
    pub cur_point: Option<String>,
    pub status: Option<String>,
    pub send_name: Option<String>,
    pub send_date: Option<String>,
    pub required_date: Option<String>,
    pub shipped_date: Option<String>,
    pub send_address: Option<String>,
    pub receive_address: Option<String>,
    pub send_phone: Option<String>,
    pub receive_phone: Option<String>,
    pub receive_name: Option<String>,
    pub next_point: Option<String>
}

#[derive(Serialize, Clone, Debug, Deserialize)]
pub struct UpdatePackage {
    pub send_point: Option<String>,
    pub receive_point: Option<String>,
    pub cur_point: Option<String>,
    pub status: Option<String>,
    pub send_name: Option<String>,
    pub send_date: Option<String>,
    pub required_date: Option<String>,
    pub shipped_date: Option<String>,
    pub send_address: Option<String>,
    pub receive_address: Option<String>,
    pub send_phone: Option<String>,
    pub receive_phone: Option<String>,
    pub receive_name: Option<String>,
    pub next_point: Option<String>
}

#[derive(Serialize, Clone, Debug, Deserialize)]
pub struct HistoryPackageData {
    #[serde(deserialize_with = "deser_pac")]
    pub package_data: PackageData,
    pub status: Option<String>,
    pub time: Option<String>
}

fn deser_pac<'de, D: Deserializer<'de>>(deserializer: D) -> Result<PackageData, D::Error> {
    let v: PackageData = Deserialize::deserialize(deserializer)?;
    Ok(v)
}