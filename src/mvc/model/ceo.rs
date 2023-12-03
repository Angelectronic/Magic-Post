use actix_session::Session;
use r2d2_mysql::{
    mysql::prelude::*,
    r2d2, MySqlConnectionManager,
};

pub fn check_ceo(session: &Session) -> bool {
    match session.get::<String>("id") {
        Ok(Some(_)) => {
            match session.get::<String>("position") {
                Ok(Some(position)) => {
                    if position != "CEO" {
                        return false;
                    }
                },
                _ => return false,
            }
        
        },
        _ => return false,
    }
    true
}


pub fn get_all_points(conn: &mut r2d2::PooledConnection<MySqlConnectionManager>) -> Option<Vec<(Option<Vec<u8>>, Option<Vec<u8>>, Option<i8>)>> {
    conn.query_map(
        "SELECT * FROM points",
        |(id, location, p_type)| (id, location, p_type),
    )
    .ok()
}

pub fn get_transactions_points(conn: &mut r2d2::PooledConnection<MySqlConnectionManager>) -> Option<Vec<(Option<Vec<u8>>, Option<Vec<u8>>, Option<i8>)>> {
    conn.query_map(
        "SELECT * FROM points WHERE p_type = 0",
        |(id, location, p_type)| (id, location, p_type),
    )
    .ok()
}

pub fn get_gathering_points(conn: &mut r2d2::PooledConnection<MySqlConnectionManager>) -> Option<Vec<(Option<Vec<u8>>, Option<Vec<u8>>, Option<i8>)>> {
    conn.query_map(
        "SELECT * FROM points WHERE p_type = 1",
        |(id, location, p_type)| (id, location, p_type),
    )
    .ok()
}

pub fn get_all_leaders(conn: &mut r2d2::PooledConnection<MySqlConnectionManager>) -> Option<Vec<(Option<Vec<u8>>, Option<Vec<u8>>, Option<Vec<u8>>, Option<Vec<u8>>)>> {
    conn.query_map(
        "SELECT id, name, position, point_id FROM employees WHERE position = 'leader'",
        |(id, name, position, point_id)| (id, name, position, point_id),
    )
    .ok()
}

pub fn get_leader_by_point_id(conn: &mut r2d2::PooledConnection<MySqlConnectionManager>, point_id: String) -> Option<Vec<(Option<Vec<u8>>, Option<Vec<u8>>, Option<Vec<u8>>, Option<Vec<u8>>)>> {
    let query = format!("SELECT id, name, position, point_id FROM employees WHERE position = 'leader' AND point_id = '{}'", point_id);

    conn.query_map(
        query,
        |(id, name, position, point_id)| (id, name, position, point_id),
    )
    .ok()
}


pub fn insert_point(conn: &mut r2d2::PooledConnection<MySqlConnectionManager>, location: String, p_type: i8) -> bool {
    let query = format!("INSERT INTO points (id, location, type) VALUES (UUID(), '{}', {})", location, p_type);
    conn.query_drop(query).is_ok()
}

pub fn delete_point_by_id(conn: &mut r2d2::PooledConnection<MySqlConnectionManager>, id: String) -> bool {
    let query = format!("DELETE FROM points WHERE id = '{}'", id);
    conn.query_drop(query).is_ok()
}

pub fn update_point(conn: &mut r2d2::PooledConnection<MySqlConnectionManager>, id: String, location: String, p_type: i8) -> bool {
    let query = format!("UPDATE points SET location = '{}', type = {} WHERE id = '{}'", location, p_type, id);
    conn.query_drop(query).is_ok()
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

pub fn get_all_packages(conn: &mut r2d2::PooledConnection<MySqlConnectionManager>) -> Option<Vec<(Option<Vec<u8>>, Option<Vec<u8>>, Option<Vec<u8>>, Option<Vec<u8>>, Option<Vec<u8>>, Option<Vec<u8>>, Option<Vec<u8>>, Option<Vec<u8>>, Option<Vec<u8>>, Option<Vec<u8>>, Option<Vec<u8>>, Option<Vec<u8>>, Option<Vec<u8>>, Option<Vec<u8>>)>> {
    let first_query = format!("SELECT id, send_point, receive_point, cur_point, status, send_name, send_date, required_date, shipped_date, send_address FROM package");

    let first_packages: Result<Vec<(Option<Vec<u8>>, Option<Vec<u8>>, Option<Vec<u8>>, Option<Vec<u8>>, Option<Vec<u8>>, Option<Vec<u8>>, Option<Vec<u8>>, Option<Vec<u8>>, Option<Vec<u8>>, Option<Vec<u8>>)>, _> = conn.query_map(
        first_query,
        |(id, send_point, receive_point, cur_point, status, send_name, send_date, required_date, shipped_date, send_address)| (id, send_point, receive_point, cur_point, status, send_name, send_date, required_date, shipped_date, send_address),
    );

    let mut first_packages = match first_packages {
        Ok(packages) => packages,
        Err(_) => return None,
    };

    let second_query = format!("SELECT receive_address, send_phone, receive_phone, receive_name FROM package");
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

pub fn get_packages_by_send_point_id(conn: &mut r2d2::PooledConnection<MySqlConnectionManager>, id: String) -> Option<Vec<(Option<Vec<u8>>, Option<Vec<u8>>, Option<Vec<u8>>, Option<Vec<u8>>, Option<Vec<u8>>, Option<Vec<u8>>, Option<Vec<u8>>, Option<Vec<u8>>, Option<Vec<u8>>, Option<Vec<u8>>, Option<Vec<u8>>, Option<Vec<u8>>, Option<Vec<u8>>, Option<Vec<u8>>)>> {
    let first_query = format!("SELECT id, send_point, receive_point, cur_point, status, send_name, send_date, required_date, shipped_date, send_address FROM package WHERE send_point = '{}'", id);

    let first_packages: Result<Vec<(Option<Vec<u8>>, Option<Vec<u8>>, Option<Vec<u8>>, Option<Vec<u8>>, Option<Vec<u8>>, Option<Vec<u8>>, Option<Vec<u8>>, Option<Vec<u8>>, Option<Vec<u8>>, Option<Vec<u8>>)>, _> = conn.query_map(
        first_query,
        |(id, send_point, receive_point, cur_point, status, send_name, send_date, required_date, shipped_date, send_address)| (id, send_point, receive_point, cur_point, status, send_name, send_date, required_date, shipped_date, send_address),
    );

    let mut first_packages = match first_packages {
        Ok(packages) => packages,
        Err(_) => return None,
    };

    let second_query = format!("SELECT receive_address, send_phone, receive_phone, receive_name FROM package WHERE send_point = '{}'", id);
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

pub fn get_packages_by_receive_point_id(conn: &mut r2d2::PooledConnection<MySqlConnectionManager>, id: String) -> Option<Vec<(Option<Vec<u8>>, Option<Vec<u8>>, Option<Vec<u8>>, Option<Vec<u8>>, Option<Vec<u8>>, Option<Vec<u8>>, Option<Vec<u8>>, Option<Vec<u8>>, Option<Vec<u8>>, Option<Vec<u8>>, Option<Vec<u8>>, Option<Vec<u8>>, Option<Vec<u8>>, Option<Vec<u8>>)>> {
    let first_query = format!("SELECT id, send_point, receive_point, cur_point, status, send_name, send_date, required_date, shipped_date, send_address FROM package WHERE receive_point = '{}'", id);

    let first_packages: Result<Vec<(Option<Vec<u8>>, Option<Vec<u8>>, Option<Vec<u8>>, Option<Vec<u8>>, Option<Vec<u8>>, Option<Vec<u8>>, Option<Vec<u8>>, Option<Vec<u8>>, Option<Vec<u8>>, Option<Vec<u8>>)>, _> = conn.query_map(
        first_query,
        |(id, send_point, receive_point, cur_point, status, send_name, send_date, required_date, shipped_date, send_address)| (id, send_point, receive_point, cur_point, status, send_name, send_date, required_date, shipped_date, send_address),
    );

    let mut first_packages = match first_packages {
        Ok(packages) => packages,
        Err(_) => return None,
    };

    let second_query = format!("SELECT receive_address, send_phone, receive_phone, receive_name FROM package WHERE receive_point = '{}'", id);
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
