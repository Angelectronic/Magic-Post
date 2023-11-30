use serde::{Serialize, Deserialize};

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

pub fn view_employees(employees: Vec<(Option<Vec<u8>>, Option<Vec<u8>>, Option<Vec<u8>>, Option<Vec<u8>>)>) -> Vec<CreateEmployeeData> {
    employees.into_iter().map(|(id, name, position, point_id)| {
        let convert_utf8 = |data: Option<Vec<u8>>| -> String {
            data.map(|v| String::from_utf8(v).unwrap_or_default()).unwrap_or_default()
        };
            
        CreateEmployeeData {
            id: convert_utf8(id),
            name: convert_utf8(name),
            position: convert_utf8(position),
            point_id: convert_utf8(point_id),
        }
    }).collect()
}

pub fn view_points(points: Vec<(Option<Vec<u8>>, Option<Vec<u8>>, Option<i8>)>) -> Vec<PointData> {
    points.into_iter().map(|(id, location, p_type)| {
        let convert_utf8 = |data: Option<Vec<u8>>| -> String {
            data.map(|v| String::from_utf8(v).unwrap_or_default()).unwrap_or_default()
        };
        
        let p_type = match p_type {
            Some(0) => String::from("Điểm giao dịch"),
            Some(1) => String::from("Điểm tập kết"),
            _ => String::from(""),
        };

        PointData {
            id: convert_utf8(id),
            location: convert_utf8(location),
            p_type,
        }    
    }).collect()
}