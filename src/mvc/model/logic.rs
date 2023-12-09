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

pub fn get_employee_by_id(conn: &mut r2d2::PooledConnection<MySqlConnectionManager>, id: String) -> Vec<(Option<Vec<u8>>, Option<Vec<u8>>, Option<Vec<u8>>, Option<Vec<u8>>)> {
    let query = format!("SELECT id, name, position, point_id FROM employees WHERE id = '{}'", id);

    conn.query_map(
        query,
        |(id, name, position, point_id)| (id, name, position, point_id),
    )
    .unwrap()
}

pub fn delete_employee_by_id(conn: &mut r2d2::PooledConnection<MySqlConnectionManager>, id: String) -> bool {
    let query = format!("DELETE FROM employees WHERE id = '{}'", id);
    conn.query_drop(query).is_ok()
}

pub fn update_employee_by_id(conn: &mut r2d2::PooledConnection<MySqlConnectionManager>, id: String, name: Option<String>, position: Option<String>, point_id: Option<String>) -> bool {
    let name = match name {
        Some(name) => format!("'{}'", name),
        None => String::from("null"),
    };    
    let position = match position {
        Some(position) => format!("'{}'", position),
        None => String::from("null"),
    };
    let point_id = match point_id {
        Some(point_id) => format!("'{}'", point_id),
        None => String::from("null"),
    };

    let query = format!("UPDATE employees SET name = {}, position = {}, point_id = {} WHERE id = '{}'", name, position, point_id, id);
    conn.query_drop(query).is_ok()
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

pub fn get_packages_by_send_point_id(conn: &mut r2d2::PooledConnection<MySqlConnectionManager>, id: String) -> Option<Vec<(Option<Vec<u8>>, Option<Vec<u8>>, Option<Vec<u8>>, Option<Vec<u8>>, Option<Vec<u8>>, Option<Vec<u8>>, Option<Vec<u8>>, Option<Vec<u8>>, Option<Vec<u8>>, Option<Vec<u8>>, Option<Vec<u8>>, Option<Vec<u8>>, Option<Vec<u8>>, Option<Vec<u8>>, Option<Vec<u8>>)>> {
    let first_query = format!("SELECT id, send_point, receive_point, cur_point, status, send_name, send_date, required_date, shipped_date, send_address FROM package WHERE send_point = '{}'", id);

    let first_packages: Result<Vec<(Option<Vec<u8>>, Option<Vec<u8>>, Option<Vec<u8>>, Option<Vec<u8>>, Option<Vec<u8>>, Option<Vec<u8>>, Option<Vec<u8>>, Option<Vec<u8>>, Option<Vec<u8>>, Option<Vec<u8>>)>, _> = conn.query_map(
        first_query,
        |(id, send_point, receive_point, cur_point, status, send_name, send_date, required_date, shipped_date, send_address)| (id, send_point, receive_point, cur_point, status, send_name, send_date, required_date, shipped_date, send_address),
    );

    let mut first_packages = match first_packages {
        Ok(packages) => packages,
        Err(_) => return None,
    };

    let second_query = format!("SELECT receive_address, send_phone, receive_phone, receive_name, next_point FROM package WHERE send_point = '{}'", id);
    let second_packages: Result<Vec<(Option<Vec<u8>>, Option<Vec<u8>>, Option<Vec<u8>>, Option<Vec<u8>>, Option<Vec<u8>>)>, _> = conn.query_map(
        second_query,
        |(receive_address, send_phone, receive_phone, receive_name, next_point)| (receive_address, send_phone, receive_phone, receive_name, next_point),
    );

    let mut second_packages = match second_packages {
        Ok(packages) => packages,
        Err(_) => return None,
    };

    let concat_packages: Vec<(Option<Vec<u8>>, Option<Vec<u8>>, Option<Vec<u8>>, Option<Vec<u8>>, Option<Vec<u8>>, Option<Vec<u8>>, Option<Vec<u8>>, Option<Vec<u8>>, Option<Vec<u8>>, Option<Vec<u8>>, Option<Vec<u8>>, Option<Vec<u8>>, Option<Vec<u8>>, Option<Vec<u8>>, Option<Vec<u8>>)> = first_packages.drain(..).zip(second_packages.drain(..)).map(|(first, second)| (first.0, first.1, first.2, first.3, first.4, first.5, first.6, first.7, first.8, first.9, second.0, second.1, second.2, second.3, second.4)).collect();
    Some(concat_packages)

}

pub fn get_packages_by_receive_point_id(conn: &mut r2d2::PooledConnection<MySqlConnectionManager>, id: String) -> Option<Vec<(Option<Vec<u8>>, Option<Vec<u8>>, Option<Vec<u8>>, Option<Vec<u8>>, Option<Vec<u8>>, Option<Vec<u8>>, Option<Vec<u8>>, Option<Vec<u8>>, Option<Vec<u8>>, Option<Vec<u8>>, Option<Vec<u8>>, Option<Vec<u8>>, Option<Vec<u8>>, Option<Vec<u8>>, Option<Vec<u8>>)>> {
    let first_query = format!("SELECT id, send_point, receive_point, cur_point, status, send_name, send_date, required_date, shipped_date, send_address FROM package WHERE receive_point = '{}'", id);

    let first_packages: Result<Vec<(Option<Vec<u8>>, Option<Vec<u8>>, Option<Vec<u8>>, Option<Vec<u8>>, Option<Vec<u8>>, Option<Vec<u8>>, Option<Vec<u8>>, Option<Vec<u8>>, Option<Vec<u8>>, Option<Vec<u8>>)>, _> = conn.query_map(
        first_query,
        |(id, send_point, receive_point, cur_point, status, send_name, send_date, required_date, shipped_date, send_address)| (id, send_point, receive_point, cur_point, status, send_name, send_date, required_date, shipped_date, send_address),
    );

    let mut first_packages = match first_packages {
        Ok(packages) => packages,
        Err(_) => return None,
    };

    let second_query = format!("SELECT receive_address, send_phone, receive_phone, receive_name, next_point FROM package WHERE receive_point = '{}'", id);
    let second_packages: Result<Vec<(Option<Vec<u8>>, Option<Vec<u8>>, Option<Vec<u8>>, Option<Vec<u8>>, Option<Vec<u8>>)>, _> = conn.query_map(
        second_query,
        |(receive_address, send_phone, receive_phone, receive_name, next_point)| (receive_address, send_phone, receive_phone, receive_name, next_point),
    );

    let mut second_packages = match second_packages {
        Ok(packages) => packages,
        Err(_) => return None,
    };

    let concat_packages: Vec<(Option<Vec<u8>>, Option<Vec<u8>>, Option<Vec<u8>>, Option<Vec<u8>>, Option<Vec<u8>>, Option<Vec<u8>>, Option<Vec<u8>>, Option<Vec<u8>>, Option<Vec<u8>>, Option<Vec<u8>>, Option<Vec<u8>>, Option<Vec<u8>>, Option<Vec<u8>>, Option<Vec<u8>>, Option<Vec<u8>>)> = first_packages.drain(..).zip(second_packages.drain(..)).map(|(first, second)| (first.0, first.1, first.2, first.3, first.4, first.5, first.6, first.7, first.8, first.9, second.0, second.1, second.2, second.3, second.4)).collect();
    Some(concat_packages)

}

pub fn get_packages_by_id(conn: &mut r2d2::PooledConnection<MySqlConnectionManager>, id: String) -> Option<Vec<(Option<Vec<u8>>, Option<Vec<u8>>, Option<Vec<u8>>, Option<Vec<u8>>, Option<Vec<u8>>, Option<Vec<u8>>, Option<Vec<u8>>, Option<Vec<u8>>, Option<Vec<u8>>, Option<Vec<u8>>, Option<Vec<u8>>, Option<Vec<u8>>, Option<Vec<u8>>, Option<Vec<u8>>)>> {
    let first_query = format!("SELECT id, send_point, receive_point, cur_point, status, send_name, send_date, required_date, shipped_date, send_address FROM package WHERE id = '{}'", id);

    let first_packages: Result<Vec<(Option<Vec<u8>>, Option<Vec<u8>>, Option<Vec<u8>>, Option<Vec<u8>>, Option<Vec<u8>>, Option<Vec<u8>>, Option<Vec<u8>>, Option<Vec<u8>>, Option<Vec<u8>>, Option<Vec<u8>>)>, _> = conn.query_map(
        first_query,
        |(id, send_point, receive_point, cur_point, status, send_name, send_date, required_date, shipped_date, send_address)| (id, send_point, receive_point, cur_point, status, send_name, send_date, required_date, shipped_date, send_address),
    );

    let mut first_packages = match first_packages {
        Ok(packages) => packages,
        Err(_) => return None,
    };

    let second_query = format!("SELECT receive_address, send_phone, receive_phone, receive_name FROM package WHERE id = '{}'", id);
    let second_packages: Result<Vec<(Option<Vec<u8>>, Option<Vec<u8>>, Option<Vec<u8>>, Option<Vec<u8>>)>, _> = conn.query_map(
        second_query,
        |(receive_address, send_phone, receive_phone, receive_name)| (receive_address, send_phone, receive_phone, receive_name),
    );

    let mut second_packages = match second_packages {
        Ok(packages) => packages,
        Err(_) => return None,
    };

    let concat_packages: Vec<(Option<Vec<u8>>, Option<Vec<u8>>, Option<Vec<u8>>, Option<Vec<u8>>, Option<Vec<u8>>, Option<Vec<u8>>, Option<Vec<u8>>, Option<Vec<u8>>, Option<Vec<u8>>, Option<Vec<u8>>, Option<Vec<u8>>, Option<Vec<u8>>, Option<Vec<u8>>, Option<Vec<u8>>)> = first_packages.drain(..).zip(second_packages.drain(..)).map(|(first, second)| (first.0, first.1, first.2, first.3, first.4, first.5, first.6, first.7, first.8, first.9, second.0, second.1, second.2, second.3)).collect();
    Some(concat_packages)

}