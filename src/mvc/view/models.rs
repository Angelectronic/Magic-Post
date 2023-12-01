use serde::{Serialize, Deserialize};

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
pub struct PointData {
    pub id: String,
    pub location: String,
    pub p_type: String,
}
