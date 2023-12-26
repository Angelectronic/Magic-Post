use actix_session::Session;
use r2d2_mysql::{
    mysql::prelude::*,
    r2d2, MySqlConnectionManager,
};
use crate::mvc::view::models::AddPoint;

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


pub fn get_all_points(conn: &mut r2d2::PooledConnection<MySqlConnectionManager>) -> Option<Vec<(Option<Vec<u8>>, Option<Vec<u8>>, Option<i8>, Option<Vec<u8>>, Option<Vec<u8>>, Option<Vec<u8>>, Option<Vec<u8>>, Option<Vec<u8>>, Option<Vec<u8>>, Option<Vec<u8>>, Option<Vec<u8>>, Option<Vec<u8>>, Option<Vec<u8>>)>> {
    let first_query = format!("SELECT p1.*,p2.reference as link_point_reference, employees.point_id as manager_id FROM points as p1 LEFT JOIN points as p2 ON p1.link_point_id = p2.id LEFT JOIN employees ON p1.id = employees.point_id WHERE employees.position = 'leader'");

    let first_point = conn.query_map(
        first_query,
        |(id, location, p_type, link_point_id, create_date, reference, name, city, zipcode, phone, link_point_reference, manager_id)| (id, location, p_type, link_point_id, create_date, reference, name, city, zipcode, phone, link_point_reference, manager_id),
    );

    let mut first_point = match first_point {
        Ok(points) => points,
        Err(_) => return None,
    };

    let second_query = format!("SELECT employees.reference as manager_reference FROM points LEFT JOIN employees ON points.id = employees.point_id WHERE employees.position = 'leader'");

    let second_point = conn.query_map(
        second_query,
        |manager_reference| manager_reference,
    );

    let mut second_point = match second_point {
        Ok(points) => points,
        Err(_) => return None,
    };

    let concat_points = first_point.drain(..).zip(second_point.drain(..)).map(|(first, second)| (first.0, first.1, first.2, first.3, first.4, first.5, first.6, first.7, first.8, first.9, first.10, first.11, second)).collect();

    Some(concat_points)
}

pub fn get_transactions_points(conn: &mut r2d2::PooledConnection<MySqlConnectionManager>) -> Option<Vec<(Option<Vec<u8>>, Option<Vec<u8>>, Option<i8>, Option<Vec<u8>>, Option<Vec<u8>>, Option<Vec<u8>>, Option<Vec<u8>>, Option<Vec<u8>>, Option<Vec<u8>>, Option<Vec<u8>>, Option<Vec<u8>>, Option<Vec<u8>>, Option<Vec<u8>>)>> {
    let first_query = format!("SELECT p1.*,p2.reference as link_point_reference, employees.point_id as manager_id FROM points as p1 LEFT JOIN points as p2 ON p1.link_point_id = p2.id LEFT JOIN employees ON p1.id = employees.point_id WHERE employees.position = 'leader' AND p1.type = 0");

    let first_point = conn.query_map(
        first_query,
        |(id, location, p_type, link_point_id, create_date, reference, name, city, zipcode, phone, link_point_reference, manager_id)| (id, location, p_type, link_point_id, create_date, reference, name, city, zipcode, phone, link_point_reference, manager_id),
    );

    let mut first_point = match first_point {
        Ok(points) => points,
        Err(_) => return None,
    };

    let second_query = format!("SELECT employees.reference as manager_reference FROM points LEFT JOIN employees ON points.id = employees.point_id WHERE employees.position = 'leader'");
    let second_point = conn.query_map(
        second_query,
        |manager_reference| manager_reference,
    );

    let mut second_point = match second_point {
        Ok(points) => points,
        Err(_) => return None,
    };

    let concat_points = first_point.drain(..).zip(second_point.drain(..)).map(|(first, second)| (first.0, first.1, first.2, first.3, first.4, first.5, first.6, first.7, first.8, first.9, first.10, first.11, second)).collect();
    Some(concat_points)
}

pub fn get_gathering_points(conn: &mut r2d2::PooledConnection<MySqlConnectionManager>) -> Option<Vec<(Option<Vec<u8>>, Option<Vec<u8>>, Option<i8>, Option<Vec<u8>>, Option<Vec<u8>>, Option<Vec<u8>>, Option<Vec<u8>>, Option<Vec<u8>>, Option<Vec<u8>>, Option<Vec<u8>>, Option<Vec<u8>>, Option<Vec<u8>>, Option<Vec<u8>>)>> {
    let first_query = format!("SELECT p1.*,p2.reference as link_point_reference, employees.point_id as manager_id FROM points as p1 LEFT JOIN points as p2 ON p1.link_point_id = p2.id LEFT JOIN employees ON p1.id = employees.point_id WHERE employees.position = 'leader' AND p1.type = 1");

    let first_point = conn.query_map(
        first_query,
        |(id, location, p_type, link_point_id, create_date, reference, name, city, zipcode, phone, link_point_reference, manager_id)| (id, location, p_type, link_point_id, create_date, reference, name, city, zipcode, phone, link_point_reference, manager_id),
    );

    let mut first_point = match first_point {
        Ok(points) => points,
        Err(_) => return None,
    };

    let second_query = format!("SELECT employees.reference as manager_reference FROM points LEFT JOIN employees ON points.id = employees.point_id WHERE employees.position = 'leader'");

    let second_point = conn.query_map(
        second_query,
        |manager_reference| manager_reference,
    );

    let mut second_point = match second_point {
        Ok(points) => points,
        Err(_) => return None,
    };

    let concat_points = first_point.drain(..).zip(second_point.drain(..)).map(|(first, second)| (first.0, first.1, first.2, first.3, first.4, first.5, first.6, first.7, first.8, first.9, first.10, first.11, second)).collect();
    Some(concat_points)
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

pub fn count_point(conn: &mut r2d2::PooledConnection<MySqlConnectionManager>, p_type: String) -> Option<Vec<Option<u64>>> {
    let p_type = match p_type.as_str() {
        "0" => "1",
        "1" => "0",
        _ => return None,
    };

    let query = format!("SELECT COUNT(*) FROM points WHERE type = {}", p_type);

    conn.query_map(
        query,
        |count| count,
    )
    .ok()
}

pub fn insert_point(conn: &mut r2d2::PooledConnection<MySqlConnectionManager>, point: AddPoint) -> String {
    let count_point = count_point(conn, point.p_type.clone()).unwrap();
    let count_point = count_point[0].clone().unwrap();
    let count_point = count_point + 1;
    let mut reference = String::from("");

    let p_type: String;
    if point.p_type == "1" {
        p_type = String::from("0");
        reference.push_str("GD");  
        if count_point < 10 {
            reference.push_str("000");
            reference.push_str(&count_point.to_string());
        } else if count_point < 100 {
            reference.push_str("00");
            reference.push_str(&count_point.to_string());
        } else if count_point < 1000 {
            reference.push_str("0");
            reference.push_str(&count_point.to_string());
        } else {
            reference.push_str(&count_point.to_string());
        }
    } else {
        p_type = String::from("1");
        reference.push_str("TK");
        if count_point < 10 {
            reference.push_str("000");
            reference.push_str(&count_point.to_string());
        } else if count_point < 100 {
            reference.push_str("00");
            reference.push_str(&count_point.to_string());
        } else if count_point < 1000 {
            reference.push_str("0");
            reference.push_str(&count_point.to_string());
        } else {
            reference.push_str(&count_point.to_string());
        }
    }

    let id_query = "SELECT UUID()";
    let id = conn.query_map(
        id_query,
        |id: String| id,
    ).unwrap();
    
    let id = &id[0];
    let link_point_id = match point.link_point_id {
        Some(link_point_id) => format!("'{}'", link_point_id),
        None => String::from("null"),
    };

    let query = format!("INSERT INTO points (id, location, type, link_point_id, create_date, reference, name, city, zipcode, phone) VALUES ('{}', '{}', {}, {}, CURRENT_DATE(), '{}', '{}', '{}', '{}', '{}')", id, point.location, p_type, link_point_id,reference, point.name, point.city, point.zipcode, point.phone);
    let result = conn.query_drop(query);

    if result.is_ok() {
        let query = format!("UPDATE employees SET point_id = '{}' WHERE id = '{}'", id, point.manager_id);
        let result2 = conn.query_drop(query);
        
        if result2.is_ok() {
            return reference;
        } else {
            return String::from("Error");
        }
    } else {
        return String::from("Error");
    }

}

pub fn delete_point_by_id(conn: &mut r2d2::PooledConnection<MySqlConnectionManager>, id: String) -> bool {
    let query = format!("DELETE FROM points WHERE id = '{}'", id);
    conn.query_drop(query).is_ok()
}

pub fn update_point(conn: &mut r2d2::PooledConnection<MySqlConnectionManager>, point: AddPoint, id: String) -> bool {
    let query = format!("UPDATE points SET location = '{}', name = '{}', city = '{}', zipcode = '{}', phone = '{}', link_point_id = '{}' WHERE id = '{}'", point.location, point.name, point.city, point.zipcode, point.phone, point.link_point_id.unwrap(), id);
    conn.query_drop(query).is_ok()
}

pub fn get_all_packages(conn: &mut r2d2::PooledConnection<MySqlConnectionManager>) -> Option<Vec<(Option<Vec<u8>>, Option<Vec<u8>>, Option<Vec<u8>>, Option<Vec<u8>>, Option<Vec<u8>>, Option<Vec<u8>>, Option<Vec<u8>>, Option<Vec<u8>>, Option<Vec<u8>>, Option<Vec<u8>>, Option<Vec<u8>>, Option<Vec<u8>>, Option<Vec<u8>>, Option<Vec<u8>>, Option<Vec<u8>>)>> {
    let first_query = format!("SELECT id, send_point, receive_point, cur_point, status, send_name, send_date, required_date, shipped_date, send_address FROM package");

    let first_packages: Result<Vec<(Option<Vec<u8>>, Option<Vec<u8>>, Option<Vec<u8>>, Option<Vec<u8>>, Option<Vec<u8>>, Option<Vec<u8>>, Option<Vec<u8>>, Option<Vec<u8>>, Option<Vec<u8>>, Option<Vec<u8>>)>, _> = conn.query_map(
        first_query,
        |(id, send_point, receive_point, cur_point, status, send_name, send_date, required_date, shipped_date, send_address)| (id, send_point, receive_point, cur_point, status, send_name, send_date, required_date, shipped_date, send_address),
    );

    let mut first_packages = match first_packages {
        Ok(packages) => packages,
        Err(_) => return None,
    };

    let second_query = format!("SELECT receive_address, send_phone, receive_phone, receive_name, next_point FROM package");
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


