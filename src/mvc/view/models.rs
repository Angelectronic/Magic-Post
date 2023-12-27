use serde::{Serialize, Deserialize, Deserializer};

/* models */
#[derive(Serialize, Clone, Debug, Deserialize)]
pub struct CreateEmployeeData {
    pub id: String,
    pub reference: String,
    pub create_date: String,
    pub last_seen: String,
    pub name: String,
    pub sex: String,
    pub email: String,
    pub birthday: String,
    pub phone: Option<String>,
    pub point_id: String,
    pub username: String,
    pub point_reference: Option<String>,
    pub m_type: String,
    pub position: String,
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
pub struct LoginSendBack {
    pub point_id: String,
    pub point_reference: String,
    pub p_type: String,
    pub link_point_id: String,
    pub link_point_reference: String,
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
    pub link_point_id: Option<String>,
    pub create_date: String,
    pub reference:String,
    pub name: String,
    pub city: String,
    pub zipcode: String,
    pub phone: String,
    pub manager_reference: String,
    pub manager_id: String,
    pub link_point_reference: String,
}

#[derive(Serialize, Clone, Debug, Deserialize)]
pub struct AddPoint {
    pub id: Option<String>,
    pub name: String,
    pub location: String,
    pub city: String,
    pub zipcode: String,
    pub phone: String,
    pub manager_id: String,
    pub p_type: String,
    pub manager_reference: String,
    pub link_point_id: Option<String>,
    pub link_point_reference: Option<String>
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