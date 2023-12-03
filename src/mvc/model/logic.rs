use actix_session::Session;
use r2d2_mysql::{
    mysql::prelude::*,
    r2d2, MySqlConnectionManager,
};

use crate::mvc::view::models::SignupData;

pub fn get_all_employees(conn: &mut r2d2::PooledConnection<MySqlConnectionManager>) -> Vec<(Option<Vec<u8>>, Option<Vec<u8>>, Option<Vec<u8>>, Option<Vec<u8>>)> {
    conn.query_map(
        "SELECT id, name, position, point_id FROM employees",
        |(id, name, position, point_id)| (id, name, position, point_id),
    )
    .unwrap()
}

pub fn insert_employee(conn: &mut r2d2::PooledConnection<MySqlConnectionManager>, data: SignupData) -> bool {
    let password = bcrypt::hash(data.password, bcrypt::DEFAULT_COST).unwrap();

    let name = match data.name {
        Some(name) => format!("'{}'", name),
        None => String::from("null"),
    };    
    let position = match data.position {
        Some(position) => format!("'{}'", position),
        None => String::from("null"),
    };
    let point_id = match data.point_id {
        Some(point_id) => format!("'{}'", point_id),
        None => String::from("null"),
    };

    let query = format!("INSERT INTO employees (id, username, password, name, position, point_id) VALUES (UUID(), '{}', '{}', {}, {}, {})", data.username, password, name, position, point_id);
 
    conn.query_drop(query).is_ok()
}

pub fn check_employee_by_username(conn: &mut r2d2::PooledConnection<MySqlConnectionManager>, username: String) -> Vec<(Option<Vec<u8>>, Option<Vec<u8>>, Option<Vec<u8>>, Option<Vec<u8>>)> {
    let query = format!("SELECT id, name, position, point_id FROM employees WHERE username = '{}'", username);

    conn.query_map(
        query,
        |(id, name, position, point_id)| (id, name, position, point_id),
    )
    .unwrap()
}

pub fn verify_employee_by_username_password(conn: &mut r2d2::PooledConnection<MySqlConnectionManager>, username: String, password: String) -> Vec<(Option<Vec<u8>>, Option<Vec<u8>>, Option<Vec<u8>>, Option<Vec<u8>>)> {
    let query = format!("SELECT id, name, position, point_id, password FROM employees WHERE username = '{}'", username);

    let mut employees: Vec<(Vec<u8>, Option<Vec<u8>>, Option<Vec<u8>>, Option<Vec<u8>>, Vec<u8>)> = conn.query_map(
        query,
        |(id, name, position, point_id, password_real)| (id, name, position, point_id, password_real),
    )
    .unwrap();

    if employees.len() > 0 {
        let employee = employees.remove(0);
        let password_real = String::from_utf8(employee.4).unwrap_or_default();  

        let check_password = bcrypt::verify(password, password_real.as_str()).unwrap_or_default();
        if check_password {
            vec![(Some(employee.0), employee.1, employee.2, employee.3)]
        } else {
            vec![]
        }
    } else {
        vec![]
    }
}
