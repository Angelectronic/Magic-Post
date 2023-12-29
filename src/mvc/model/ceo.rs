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

pub fn get_all_leaders(conn: &mut r2d2::PooledConnection<MySqlConnectionManager>) -> Option<Vec<(Option<Vec<u8>>, Option<Vec<u8>>, Option<Vec<u8>>, Option<Vec<u8>>, Option<Vec<u8>>, Option<Vec<u8>>, Option<Vec<u8>>, Option<Vec<u8>>, Option<Vec<u8>>, Option<Vec<u8>>, Option<Vec<u8>>, Option<Vec<u8>>, Option<Vec<u8>>, Option<Vec<u8>>)>> {
    let first_query = format!("SELECT employees.id, employees.reference, employees.create_date, employees.last_seen, employees.name, employees.sex, employees.email, employees.birthday, employees.phone, employees.point_id, employees.username FROM employees LEFT JOIN points ON employees.point_id = points.id WHERE employees.position = 'leader'");
    let first_leader = conn.query_map(
        first_query,
        |(id, reference, create_date, last_seen, name, sex, email, birthday, phone, point_id, username)| (id, reference, create_date, last_seen, name, sex, email, birthday, phone, point_id, username),
    );

    let mut first_leader = match first_leader {
        Ok(leaders) => leaders,
        Err(_) => return None,
    };

    let second_query = format!("SELECT points.reference, points.type, employees.position FROM employees LEFT JOIN points ON employees.point_id = points.id WHERE employees.position = 'leader'");
    let second_leader = conn.query_map(
        second_query,
        |(reference, p_type, position)| (reference, p_type, position),
    );

    let mut second_leader = match second_leader {
        Ok(leaders) => leaders,
        Err(_) => return None,
    };

    let concat_leaders = first_leader.drain(..).zip(second_leader.drain(..)).map(|(first, second)| (first.0, first.1, first.2, first.3, first.4, first.5, first.6, first.7, first.8, first.9, first.10, second.0, second.1, second.2)).collect();
    Some(concat_leaders)
}

pub fn get_leader_by_point_id(conn: &mut r2d2::PooledConnection<MySqlConnectionManager>, point_id: String) -> Option<Vec<(Option<Vec<u8>>, Option<Vec<u8>>, Option<Vec<u8>>, Option<Vec<u8>>, Option<Vec<u8>>, Option<Vec<u8>>, Option<Vec<u8>>, Option<Vec<u8>>, Option<Vec<u8>>, Option<Vec<u8>>, Option<Vec<u8>>, Option<Vec<u8>>, Option<Vec<u8>>, Option<Vec<u8>>)>> {
    let first_query = format!("SELECT employees.id, employees.reference, employees.create_date, employees.last_seen, employees.name, employees.sex, employees.email, employees.birthday, employees.phone, employees.point_id, employees.username FROM employees LEFT JOIN points ON employees.point_id = points.id WHERE employees.position = 'leader' AND employees.point_id = '{}'", point_id);

    let first_leader = conn.query_map(
        first_query,
        |(id, reference, create_date, last_seen, name, sex, email, birthday, phone, point_id, username)| (id, reference, create_date, last_seen, name, sex, email, birthday, phone, point_id, username),
    );

    let mut first_leader = match first_leader {
        Ok(leaders) => leaders,
        Err(_) => return None,
    };

    let second_query = format!("SELECT points.reference, points.type, employees.position FROM employees LEFT JOIN points ON employees.point_id = points.id WHERE employees.position = 'leader' AND employees.point_id = '{}'", point_id);

    let second_leader = conn.query_map(
        second_query,
        |(reference, p_type, position)| (reference, p_type, position),
    );

    let mut second_leader = match second_leader {
        Ok(leaders) => leaders,
        Err(_) => return None,
    };

    let concat_leaders = first_leader.drain(..).zip(second_leader.drain(..)).map(|(first, second)| (first.0, first.1, first.2, first.3, first.4, first.5, first.6, first.7, first.8, first.9, first.10, second.0, second.1, second.2)).collect();

    Some(concat_leaders)
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
        Some(link_point_id) => match link_point_id.as_str() {
            "" => String::from("null"),
            _ => link_point_id,
        },
        None => String::from("null"),
    };

    let query = format!("INSERT INTO points (id, location, type, link_point_id, create_date, reference, name, city, zipcode, phone) VALUES ('{}', '{}', {}, {}, CURRENT_DATE(), '{}', '{}', '{}', '{}', '{}')", id, point.location, p_type, link_point_id,reference, point.name, point.city, point.zipcode, point.phone);

    let result = conn.query_drop(query);

    if result.is_ok() {
        // Check if manager exists
        let verify_query = format!("SELECT id FROM employees WHERE id = '{}'", point.manager_id);
        let verify = conn.query_map(
            verify_query,
            |id: String| id,
        ).unwrap();
        if verify.len() == 0 {
            return String::from("Error updating manager but added point");
        }

        let query = format!("UPDATE employees SET point_id = '{}' WHERE id = '{}'", id, point.manager_id);
        let result2 = conn.query_drop(query);
        
        if result2.is_ok() {
            return reference;
        } else {
            return String::from("Error updating manager but added point");
        }
    } else {
        return String::from("Error adding point");
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

pub fn get_item_by_package_id(conn: &mut r2d2::PooledConnection<MySqlConnectionManager>, package_id: String) -> Option<Vec<(Option<Vec<u8>>, Option<i32>, Option<i32>)>> {
    let query = format!("SELECT item_name, quantity, value FROM items WHERE package_id = '{}'", package_id);
    conn.query_map(
        query,
        |(item_name, item_quantity, item_value)| (item_name, item_quantity, item_value),
    ).ok()
}

pub fn get_all_packages(conn: &mut r2d2::PooledConnection<MySqlConnectionManager>) -> Option<Vec<(Option<Vec<u8>>, Option<Vec<u8>>, Option<Vec<u8>>, Option<Vec<u8>>, Option<Vec<u8>>, Option<Vec<u8>>, Option<Vec<u8>>, Option<Vec<u8>>, Option<Vec<u8>>, Option<Vec<u8>>, Option<Vec<u8>>, Option<Vec<u8>>, Option<Vec<u8>>, Option<Vec<u8>>, Option<Vec<u8>>, Option<Vec<u8>>, Option<i32>, Option<i32>, Option<i32>, Option<i32>, Option<i32>, Option<i32>, Option<i8>, Option<i8>, Option<f32>, Option<Vec<u8>>, Option<Vec<u8>>, Option<i32>, Option<i32>)>> {
    let first_query = format!("SELECT package.id,package.package_id,package.send_name,package.send_date,package.send_phone,package.send_address,package.send_point,package.receive_name,package.receive_phone,package.receive_address,package.receive_point,IF(p2.type=0,'exchanging','gathering') as current_from FROM package LEFT JOIN points as p1 ON package.send_point = p1.id LEFT JOIN points as p4 ON package.receive_point = p4.id LEFT JOIN points as p2 ON package.cur_point = p2.id LEFT JOIN points as p3 ON package.next_point = p3.id");

    let first_packages = conn.query_map(
        first_query,
        |(id, package_id, send_name, send_date, send_phone, send_address, send_point, receive_name, receive_phone, receive_address, receive_point, current_from)| (id, package_id, send_name, send_date, send_phone, send_address, send_point, receive_name, receive_phone, receive_address, receive_point, current_from),
    );

    let mut first_packages = match first_packages {
        Ok(packages) => packages,
        Err(_) => return None,
    };

    let second_query = format!("SELECT p2.reference as from_point_id,IF(p3.type=0,'exchanging','gathering') as current_dest,p3.reference as dest_point_id,package.status,package.main_cost,package.other_cost,package.gtgt_cost,package.other_service_cost,package.total_cost,package.vat,package.package_type, package.instruction_type FROM package LEFT JOIN points as p1 ON package.send_point = p1.id LEFT JOIN points as p4 ON package.receive_point = p4.id LEFT JOIN points as p2 ON package.cur_point = p2.id LEFT JOIN points as p3 ON package.next_point = p3.id");


    let second_packages = conn.query_map(
        second_query,
        |(from_point_id, current_dest, dest_point_id, status, main_cost, other_cost, gtgt_cost, other_service_cost, total_cost, vat, package_type, instruction_type)| (from_point_id, current_dest, dest_point_id, status, main_cost, other_cost, gtgt_cost, other_service_cost, total_cost, vat, package_type, instruction_type),
    );

    let mut second_packages = match second_packages {
        Ok(packages) => packages,
        Err(_) => return None,
    };

    let third_query = format!("SELECT package.weight,package.special_service,package.note,package.cod,package.receive_other_cost FROM package LEFT JOIN points as p1 ON package.send_point = p1.id LEFT JOIN points as p4 ON package.receive_point = p4.id LEFT JOIN points as p2 ON package.cur_point = p2.id LEFT JOIN points as p3 ON package.next_point = p3.id");

    let third_packages = conn.query_map(
        third_query,
        |(weight, special_service, note, cod, receive_other_cost)| (weight, special_service, note, cod, receive_other_cost),
    );

    let mut third_packages = match third_packages {
        Ok(packages) => packages,
        Err(_) => return None,
    };

    let concat_packages = first_packages.drain(..).zip(second_packages.drain(..)).zip(third_packages.drain(..)).map(|((first, second), third)| (first.0, first.1, first.2, first.3, first.4, first.5, first.6, first.7, first.8, first.9, first.10, first.11, second.0, second.1, second.2, second.3, second.4, second.5, second.6, second.7, second.8, second.9, second.10, second.11, third.0, third.1, third.2, third.3, third.4)).collect();

    Some(concat_packages)
}

pub fn get_packages_at_point_id(conn: &mut r2d2::PooledConnection<MySqlConnectionManager>, point_id: String) -> Option<Vec<(Option<Vec<u8>>, Option<Vec<u8>>, Option<Vec<u8>>, Option<Vec<u8>>, Option<Vec<u8>>, Option<Vec<u8>>, Option<Vec<u8>>, Option<Vec<u8>>, Option<Vec<u8>>, Option<Vec<u8>>, Option<Vec<u8>>, Option<Vec<u8>>, Option<Vec<u8>>, Option<Vec<u8>>, Option<Vec<u8>>, Option<Vec<u8>>, Option<i32>, Option<i32>, Option<i32>, Option<i32>, Option<i32>, Option<i32>, Option<i8>, Option<i8>, Option<f32>, Option<Vec<u8>>, Option<Vec<u8>>, Option<i32>, Option<i32>, Option<Vec<u8>>)>> {
    let first_query = format!("SELECT package.id,package.package_id,package.send_name,package.send_date,package.send_phone,package.send_address,package.send_point,package.receive_name,package.receive_phone,package.receive_address,package.receive_point,IF(p2.type=0,'exchanging','gathering') as current_from FROM package LEFT JOIN points as p1 ON package.send_point = p1.id LEFT JOIN points as p4 ON package.receive_point = p4.id LEFT JOIN points as p2 ON package.cur_point = p2.id LEFT JOIN points as p3 ON package.next_point = p3.id INNER JOIN cur_point_history ON package.id = cur_point_history.package_id WHERE package.cur_point = '{}' AND cur_point_history.status = 'send'", point_id);

    let first_packages = conn.query_map(
        first_query,
        |(id, package_id, send_name, send_date, send_phone, send_address, send_point, receive_name, receive_phone, receive_address, receive_point, current_from)| (id, package_id, send_name, send_date, send_phone, send_address, send_point, receive_name, receive_phone, receive_address, receive_point, current_from),
    );

    let mut first_packages = match first_packages {
        Ok(packages) => packages,
        Err(_) => return None,
    };

    let second_query = format!("SELECT p2.reference as from_point_id,IF(p3.type=0,'exchanging','gathering') as current_dest,p3.reference as dest_point_id,package.status,package.main_cost,package.other_cost,package.gtgt_cost,package.other_service_cost,package.total_cost,package.vat,package.package_type, package.instruction_type FROM package LEFT JOIN points as p1 ON package.send_point = p1.id LEFT JOIN points as p4 ON package.receive_point = p4.id LEFT JOIN points as p2 ON package.cur_point = p2.id LEFT JOIN points as p3 ON package.next_point = p3.id INNER JOIN cur_point_history ON package.id = cur_point_history.package_id WHERE package.cur_point = '{}' AND cur_point_history.status = 'send'", point_id);
    let second_packages = conn.query_map(
        second_query,
        |(from_point_id, current_dest, dest_point_id, package_status, main_cost, other_cost, gtgt_cost, other_service_cost, total_cost, vat, package_type, instruction_type)| (from_point_id, current_dest, dest_point_id, package_status, main_cost, other_cost, gtgt_cost, other_service_cost, total_cost, vat, package_type, instruction_type),
    );

    let mut second_packages = match second_packages {
        Ok(packages) => packages,
        Err(_) => return None,
    };

    let third_query = format!("SELECT package.weight,package.special_service,package.note,package.cod,package.receive_other_cost, cur_point_history.time FROM package LEFT JOIN points as p1 ON package.send_point = p1.id LEFT JOIN points as p4 ON package.receive_point = p4.id LEFT JOIN points as p2 ON package.cur_point = p2.id LEFT JOIN points as p3 ON package.next_point = p3.id INNER JOIN cur_point_history ON package.id = cur_point_history.package_id WHERE package.cur_point = '{}' AND cur_point_history.status = 'send'", point_id);
    let third_packages = conn.query_map(
        third_query,
        |(weight, special_service, note, cod, receive_other_cost, time)| (weight, special_service, note, cod, receive_other_cost, time),
    );

    let mut third_packages = match third_packages {
        Ok(packages) => packages,
        Err(_) => return None,
    };

    let concat_packages = first_packages.drain(..).zip(second_packages.drain(..)).zip(third_packages.drain(..)).map(|((first, second), third)| (first.0, first.1, first.2, first.3, first.4, first.5, first.6, first.7, first.8, first.9, first.10, first.11, second.0, second.1, second.2, second.3, second.4, second.5, second.6, second.7, second.8, second.9, second.10, second.11, third.0, third.1, third.2, third.3, third.4, third.5)).collect();
    Some(concat_packages)
}

pub fn get_packages_next_point_id(conn: &mut r2d2::PooledConnection<MySqlConnectionManager>, point_id: String) -> Option<Vec<(Option<Vec<u8>>, Option<Vec<u8>>, Option<Vec<u8>>, Option<Vec<u8>>, Option<Vec<u8>>, Option<Vec<u8>>, Option<Vec<u8>>, Option<Vec<u8>>, Option<Vec<u8>>, Option<Vec<u8>>, Option<Vec<u8>>, Option<Vec<u8>>, Option<Vec<u8>>, Option<Vec<u8>>, Option<Vec<u8>>, Option<Vec<u8>>, Option<i32>, Option<i32>, Option<i32>, Option<i32>, Option<i32>, Option<i32>, Option<i8>, Option<i8>, Option<f32>, Option<Vec<u8>>, Option<Vec<u8>>, Option<i32>, Option<i32>)>> {
    let first_query = format!("SELECT package.id,package.package_id,package.send_name,package.send_date,package.send_phone,package.send_address,package.send_point,package.receive_name,package.receive_phone,package.receive_address,package.receive_point,IF(p2.type=0,'exchanging','gathering') as current_from FROM package LEFT JOIN points as p1 ON package.send_point = p1.id LEFT JOIN points as p4 ON package.receive_point = p4.id LEFT JOIN points as p2 ON package.cur_point = p2.id LEFT JOIN points as p3 ON package.next_point = p3.id WHERE package.next_point = '{}'", point_id);

    let first_packages = conn.query_map(
        first_query,
        |(id, package_id, send_name, send_date, send_phone, send_address, send_point, receive_name, receive_phone, receive_address, receive_point, current_from)| (id, package_id, send_name, send_date, send_phone, send_address, send_point, receive_name, receive_phone, receive_address, receive_point, current_from),
    );

    let mut first_packages = match first_packages {
        Ok(packages) => packages,
        Err(_) => return None,
    };

    let second_query = format!("SELECT p2.reference as from_point_id,IF(p3.type=0,'exchanging','gathering') as current_dest,p3.reference as dest_point_id,package.status,package.main_cost,package.other_cost,package.gtgt_cost,package.other_service_cost,package.total_cost,package.vat,package.package_type, package.instruction_type FROM package LEFT JOIN points as p1 ON package.send_point = p1.id LEFT JOIN points as p4 ON package.receive_point = p4.id LEFT JOIN points as p2 ON package.cur_point = p2.id LEFT JOIN points as p3 ON package.next_point = p3.id WHERE package.next_point = '{}'", point_id);


    let second_packages = conn.query_map(
        second_query,
        |(from_point_id, current_dest, dest_point_id, status, main_cost, other_cost, gtgt_cost, other_service_cost, total_cost, vat, package_type, instruction_type)| (from_point_id, current_dest, dest_point_id, status, main_cost, other_cost, gtgt_cost, other_service_cost, total_cost, vat, package_type, instruction_type),
    );

    let mut second_packages = match second_packages {
        Ok(packages) => packages,
        Err(_) => return None,
    };

    let third_query = format!("SELECT package.weight,package.special_service,package.note,package.cod,package.receive_other_cost FROM package LEFT JOIN points as p1 ON package.send_point = p1.id LEFT JOIN points as p4 ON package.receive_point = p4.id LEFT JOIN points as p2 ON package.cur_point = p2.id LEFT JOIN points as p3 ON package.next_point = p3.id WHERE package.next_point = '{}'", point_id);

    let third_packages = conn.query_map(
        third_query,
        |(weight, special_service, note, cod, receive_other_cost)| (weight, special_service, note, cod, receive_other_cost),
    );

    let mut third_packages = match third_packages {
        Ok(packages) => packages,
        Err(_) => return None,
    };

    let concat_packages = first_packages.drain(..).zip(second_packages.drain(..)).zip(third_packages.drain(..)).map(|((first, second), third)| (first.0, first.1, first.2, first.3, first.4, first.5, first.6, first.7, first.8, first.9, first.10, first.11, second.0, second.1, second.2, second.3, second.4, second.5, second.6, second.7, second.8, second.9, second.10, second.11, third.0, third.1, third.2, third.3, third.4)).collect::<Vec<_>>();

    Some(concat_packages)
}