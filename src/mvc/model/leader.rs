use actix_session::Session;
use r2d2_mysql::{
    mysql::prelude::*,
    r2d2, MySqlConnectionManager,
};

pub fn check_leader(session: &Session) -> bool {
    match session.get::<String>("id") {
        Ok(Some(_)) => {
            match session.get::<String>("position") {
                Ok(Some(position)) => {
                    if position != "leader" {
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

pub fn get_employees_by_point_id(conn: &mut r2d2::PooledConnection<MySqlConnectionManager>, point_id: String) -> Option<Vec<(Option<Vec<u8>>, Option<Vec<u8>>, Option<Vec<u8>>, Option<Vec<u8>>, Option<Vec<u8>>, Option<Vec<u8>>, Option<Vec<u8>>, Option<Vec<u8>>, Option<Vec<u8>>, Option<Vec<u8>>, Option<Vec<u8>>, Option<Vec<u8>>, Option<Vec<u8>>, Option<Vec<u8>>)>> {
    let first_query = format!("SELECT employees.id, employees.reference, employees.create_date, employees.last_seen, employees.name, employees.sex, employees.email, employees.birthday, employees.phone, employees.point_id, employees.username FROM employees LEFT JOIN points ON employees.point_id = points.id WHERE employees.point_id = '{}'", point_id);

    let first_employees = conn.query_map(
        first_query,
        |(id, reference, create_date, last_seen, name, sex, email, birthday, phone, point_id, username)| (id, reference, create_date, last_seen, name, sex, email, birthday, phone, point_id, username),
    );

    let mut first_employees = match first_employees {
        Ok(employees) => employees,
        Err(_) => return None,
    };

    let second_query = format!("SELECT points.reference, points.type, employees.position FROM employees LEFT JOIN points ON employees.point_id = points.id WHERE employees.point_id = '{}'", point_id);

    let second_employees = conn.query_map(
        second_query,
        |(reference, m_type, position)| (reference, m_type, position),
    );

    let mut second_employees = match second_employees {
        Ok(employees) => employees,
        Err(_) => return None,
    };

    let concat_employees = first_employees.drain(..).zip(second_employees.drain(..)).map(|(first, second)| (first.0, first.1, first.2, first.3, first.4, first.5, first.6, first.7, first.8, first.9, first.10, second.0, second.1, second.2)).collect();
    Some(concat_employees)
}

pub fn get_cur_point_history_by_pointid(conn: &mut r2d2::PooledConnection<MySqlConnectionManager>, id: String) -> Option<Vec<(Option<Vec<u8>>, Option<Vec<u8>>, Option<Vec<u8>>, Option<Vec<u8>>, Option<Vec<u8>>, Option<Vec<u8>>, Option<Vec<u8>>, Option<Vec<u8>>, Option<Vec<u8>>, Option<Vec<u8>>, Option<Vec<u8>>, Option<Vec<u8>>, Option<Vec<u8>>, Option<Vec<u8>>, Option<Vec<u8>>, Option<Vec<u8>>, Option<Vec<u8>>)>> {
    let first_query = format!("SELECT id, send_point, receive_point, cur_point, package.status, send_name, send_date, required_date, shipped_date, send_address, cur_point_history.status, cur_point_history.time FROM package INNER JOIN cur_point_history ON package.cur_point = cur_point_history.point_id WHERE cur_point_history.point_id = '{}'", id);

    let first_packages: Result<Vec<(Option<Vec<u8>>, Option<Vec<u8>>, Option<Vec<u8>>, Option<Vec<u8>>, Option<Vec<u8>>, Option<Vec<u8>>, Option<Vec<u8>>, Option<Vec<u8>>, Option<Vec<u8>>, Option<Vec<u8>>, Option<Vec<u8>>, Option<Vec<u8>>)>, _> = conn.query_map(
        first_query,
        |(id, send_point, receive_point, cur_point, package_status, send_name, send_date, required_date, shipped_date, send_address, status, time)| (id, send_point, receive_point, cur_point, package_status, send_name, send_date, required_date, shipped_date, send_address, status, time),
    );

    let mut first_packages = match first_packages {
        Ok(packages) => packages,
        Err(_) => return None,
    };

    let second_query = format!("SELECT receive_address, send_phone, receive_phone, receive_name, next_point FROM package INNER JOIN cur_point_history ON package.cur_point = cur_point_history.point_id WHERE cur_point_history.point_id = '{}'", id);
    let second_packages: Result<Vec<(Option<Vec<u8>>, Option<Vec<u8>>, Option<Vec<u8>>, Option<Vec<u8>>, Option<Vec<u8>>)>, _> = conn.query_map(
        second_query,
        |(receive_address, send_phone, receive_phone, receive_name, next_point)| (receive_address, send_phone, receive_phone, receive_name, next_point),
    );

    let mut second_packages = match second_packages {
        Ok(packages) => packages,
        Err(_) => return None,
    };

    let concat_packages: Vec<(Option<Vec<u8>>, Option<Vec<u8>>, Option<Vec<u8>>, Option<Vec<u8>>, Option<Vec<u8>>, Option<Vec<u8>>, Option<Vec<u8>>, Option<Vec<u8>>, Option<Vec<u8>>, Option<Vec<u8>>, Option<Vec<u8>>, Option<Vec<u8>>, Option<Vec<u8>>, Option<Vec<u8>>, Option<Vec<u8>>, Option<Vec<u8>>, Option<Vec<u8>>)> = first_packages.drain(..).zip(second_packages.drain(..)).map(|(first, second)| (first.0, first.1, first.2, first.3, first.4, first.5, first.6, first.7, first.8, first.9, first.10, first.11, second.0, second.1, second.2, second.3, second.4)).collect();
    Some(concat_packages)

}