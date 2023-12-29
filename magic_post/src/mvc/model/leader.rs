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

pub fn get_cur_point_history_by_pointid(conn: &mut r2d2::PooledConnection<MySqlConnectionManager>, id: String) -> Option<Vec<(Option<Vec<u8>>, Option<Vec<u8>>, Option<Vec<u8>>, Option<Vec<u8>>, Option<Vec<u8>>, Option<Vec<u8>>, Option<Vec<u8>>, Option<Vec<u8>>, Option<Vec<u8>>, Option<Vec<u8>>, Option<Vec<u8>>, Option<Vec<u8>>, Option<Vec<u8>>, Option<Vec<u8>>, Option<Vec<u8>>, Option<Vec<u8>>, Option<i32>, Option<i32>, Option<i32>, Option<i32>, Option<i32>, Option<i32>, Option<i8>, Option<i8>, Option<f32>, Option<Vec<u8>>, Option<Vec<u8>>, Option<i32>, Option<i32>, Option<Vec<u8>>, Option<Vec<u8>>)>> {
    let first_query = format!("SELECT package.id,package.package_id,package.send_name,package.send_date,package.send_phone,package.send_address,package.send_point,package.receive_name,package.receive_phone,package.receive_address,package.receive_point,IF(p2.type=0,'exchanging','gathering') as current_from FROM package LEFT JOIN points as p1 ON package.send_point = p1.id LEFT JOIN points as p4 ON package.receive_point = p4.id LEFT JOIN points as p2 ON package.cur_point = p2.id LEFT JOIN points as p3 ON package.next_point = p3.id INNER JOIN cur_point_history ON package.id = cur_point_history.package_id WHERE cur_point_history.point_id = '{}'", id);

    let first_packages = conn.query_map(
        first_query,
        |(id, package_id, send_name, send_date, send_phone, send_address, send_point, receive_name, receive_phone, receive_address, receive_point, current_from)| (id, package_id, send_name, send_date, send_phone, send_address, send_point, receive_name, receive_phone, receive_address, receive_point, current_from),
    );

    let mut first_packages = match first_packages {
        Ok(packages) => packages,
        Err(_) => return None,
    };

    let second_query = format!("SELECT p2.reference as from_point_id,IF(p3.type=0,'exchanging','gathering') as current_dest,p3.reference as dest_point_id,package.status,package.main_cost,package.other_cost,package.gtgt_cost,package.other_service_cost,package.total_cost,package.vat,package.package_type, package.instruction_type FROM package LEFT JOIN points as p1 ON package.send_point = p1.id LEFT JOIN points as p4 ON package.receive_point = p4.id LEFT JOIN points as p2 ON package.cur_point = p2.id LEFT JOIN points as p3 ON package.next_point = p3.id INNER JOIN cur_point_history ON package.id = cur_point_history.package_id WHERE cur_point_history.point_id = '{}'", id);
    let second_packages = conn.query_map(
        second_query,
        |(from_point_id, current_dest, dest_point_id, package_status, main_cost, other_cost, gtgt_cost, other_service_cost, total_cost, vat, package_type, instruction_type)| (from_point_id, current_dest, dest_point_id, package_status, main_cost, other_cost, gtgt_cost, other_service_cost, total_cost, vat, package_type, instruction_type),
    );

    let mut second_packages = match second_packages {
        Ok(packages) => packages,
        Err(_) => return None,
    };

    let third_query = format!("SELECT package.weight,package.special_service,package.note,package.cod,package.receive_other_cost, cur_point_history.status, cur_point_history.time FROM package LEFT JOIN points as p1 ON package.send_point = p1.id LEFT JOIN points as p4 ON package.receive_point = p4.id LEFT JOIN points as p2 ON package.cur_point = p2.id LEFT JOIN points as p3 ON package.next_point = p3.id INNER JOIN cur_point_history ON package.id = cur_point_history.package_id WHERE cur_point_history.point_id = '{}'", id);
    let third_packages = conn.query_map(
        third_query,
        |(weight, special_service, note, cod, receive_other_cost, status, time)| (weight, special_service, note, cod, receive_other_cost, status, time),
    );

    let mut third_packages = match third_packages {
        Ok(packages) => packages,
        Err(_) => return None,
    };

    let concat_packages = first_packages.drain(..).zip(second_packages.drain(..)).zip(third_packages.drain(..)).map(|((first, second), third)| (first.0, first.1, first.2, first.3, first.4, first.5, first.6, first.7, first.8, first.9, first.10, first.11, second.0, second.1, second.2, second.3, second.4, second.5, second.6, second.7, second.8, second.9, second.10, second.11, third.0, third.1, third.2, third.3, third.4, third.5, third.6)).collect();
    Some(concat_packages)

}