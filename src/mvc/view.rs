pub fn view_all_employees(employees: Vec<(Option<Vec<u8>>, Option<Vec<u8>>, Option<Vec<u8>>, Option<Vec<u8>>, Option<Vec<u8>>, Option<Vec<u8>>)>) -> Vec<(String, String, String, String, String, String)> {
    employees.into_iter().map(|(id, name, position, point_id, username, password)| {
        let id = match id {
            Some(id) => String::from_utf8(id).unwrap(),
            None => String::from(""),
        };
        let name = match name {
            Some(name) => String::from_utf8(name).unwrap(),
            None => String::from(""),
        };
        let position = match position {
            Some(position) => String::from_utf8(position).unwrap(),
            None => String::from(""),
        };
        let point_id = match point_id {
            Some(point_id) => String::from_utf8(point_id).unwrap(),
            None => String::from(""),
        };
        let username = match username {
            Some(username) => String::from_utf8(username).unwrap(),
            None => String::from(""),
        };
        let password = match password {
            Some(password) => String::from_utf8(password).unwrap(),
            None => String::from(""),
        };
        (id, name, position, point_id, username, password)
    }).collect()
}