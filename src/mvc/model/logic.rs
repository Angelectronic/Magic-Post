use r2d2_mysql::{
    mysql::prelude::*,
    r2d2, MySqlConnectionManager,
};

use crate::mvc::view::models::{SignupData, UpdateEmployee};

use super::ceo::get_item_by_package_id;

pub fn get_all_employees(conn: &mut r2d2::PooledConnection<MySqlConnectionManager>) -> Option<Vec<(Option<Vec<u8>>, Option<Vec<u8>>, Option<Vec<u8>>, Option<Vec<u8>>, Option<Vec<u8>>, Option<Vec<u8>>, Option<Vec<u8>>, Option<Vec<u8>>, Option<Vec<u8>>, Option<Vec<u8>>, Option<Vec<u8>>, Option<Vec<u8>>, Option<Vec<u8>>, Option<Vec<u8>>)>> {
    let first_query = format!("SELECT employees.id, employees.reference, employees.create_date, employees.last_seen, employees.name, employees.sex, employees.email, employees.birthday, employees.phone, employees.point_id, employees.username FROM employees LEFT JOIN points ON employees.point_id = points.id");

    let first_employees = conn.query_map(
        first_query,
        |(id, reference, create_date, last_seen, name, sex, email, birthday, phone, point_id, username)| (id, reference, create_date, last_seen, name, sex, email, birthday, phone, point_id, username),
    );

    let mut first_employees = match first_employees {
        Ok(employees) => employees,
        Err(_) => return None,
    };

    let second_query = format!("SELECT points.reference, points.type, employees.position FROM employees LEFT JOIN points ON employees.point_id = points.id");

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

pub fn get_employee_by_id(conn: &mut r2d2::PooledConnection<MySqlConnectionManager>, id: String) -> Option<Vec<(Option<Vec<u8>>, Option<Vec<u8>>, Option<Vec<u8>>, Option<Vec<u8>>, Option<Vec<u8>>, Option<Vec<u8>>, Option<Vec<u8>>, Option<Vec<u8>>, Option<Vec<u8>>, Option<Vec<u8>>, Option<Vec<u8>>, Option<Vec<u8>>, Option<Vec<u8>>, Option<Vec<u8>>)>> {
    let first_query = format!("SELECT employees.id, employees.reference, employees.create_date, employees.last_seen, employees.name, employees.sex, employees.email, employees.birthday, employees.phone, employees.point_id, employees.username FROM employees LEFT JOIN points ON employees.point_id = points.id WHERE employees.id = '{}'", id);

    let first_employees = conn.query_map(
        first_query,
        |(id, reference, create_date, last_seen, name, sex, email, birthday, phone, point_id, username)| (id, reference, create_date, last_seen, name, sex, email, birthday, phone, point_id, username),
    );

    let mut first_employees = match first_employees {
        Ok(employees) => employees,
        Err(_) => return None,
    };

    let second_query = format!("SELECT points.reference, points.type, employees.position FROM employees LEFT JOIN points ON employees.point_id = points.id WHERE employees.id = '{}'", id);

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

pub fn delete_employee_by_id(conn: &mut r2d2::PooledConnection<MySqlConnectionManager>, id: String) -> bool {
    let query = format!("DELETE FROM employees WHERE id = '{}'", id);
    conn.query_drop(query).is_ok()
}

pub fn update_employee_by_id(conn: &mut r2d2::PooledConnection<MySqlConnectionManager>, employee: UpdateEmployee, id: String) -> bool {
    let convert = |data: Option<String>| match data {
        Some(data) => format!("'{}'", data),
        None => String::from("null"),
    };

    let name = convert(employee.name);
    let sex = convert(employee.sex);
    let email = convert(employee.email);
    let birthday = convert(employee.birthday);
    let phone = convert(employee.phone);
    let point_id = convert(employee.point_id);
    let username = convert(employee.username);
    let position = convert(employee.position);

    let query = format!("UPDATE employees SET name={}, sex={},email={},birthday={},phone={},point_id={},username={},position={} WHERE id='{}'", name, sex, email, birthday, phone, point_id, username, position, id);

    conn.query_drop(query).is_ok()
}

pub fn update_employee_password_by_id(conn: &mut r2d2::PooledConnection<MySqlConnectionManager>, password: String, id: String) -> bool {
    let password = bcrypt::hash(password, bcrypt::DEFAULT_COST).unwrap();
    let query = format!("UPDATE employees SET password='{}' WHERE id='{}'", password, id);

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

pub fn format_nested_package(conn: &mut r2d2::PooledConnection<MySqlConnectionManager>, package: Vec<(Option<Vec<u8>>, Option<Vec<u8>>, Option<Vec<u8>>, Option<Vec<u8>>, Option<Vec<u8>>, Option<Vec<u8>>, Option<Vec<u8>>, Option<Vec<u8>>, Option<Vec<u8>>, Option<Vec<u8>>, Option<Vec<u8>>, Option<Vec<u8>>, Option<Vec<u8>>, Option<Vec<u8>>, Option<Vec<u8>>, Option<Vec<u8>>, Option<i32>, Option<i32>, Option<i32>, Option<i32>, Option<i32>, Option<i32>, Option<i8>, Option<i8>, Option<f32>, Option<Vec<u8>>, Option<Vec<u8>>, Option<i32>, Option<i32>)>) -> Vec<(Option<Vec<u8>>, Option<Vec<u8>>, Option<Vec<u8>>, Option<Vec<u8>>, Option<Vec<u8>>, Option<Vec<u8>>, Option<Vec<u8>>, Option<Vec<u8>>, Option<Vec<u8>>, Option<Vec<u8>>, Option<Vec<u8>>, Option<Vec<u8>>, Option<Vec<u8>>, Option<Vec<u8>>, Option<Vec<u8>>, Option<Vec<u8>>, Option<i32>, Option<i32>, Option<i32>, Option<i32>, Option<i32>, Option<i32>, Option<i8>, Option<i8>, Option<f32>, Option<Vec<u8>>, Option<Vec<u8>>, Option<i32>, Option<i32>, Vec<(Option<Vec<u8>>, Option<i32>, Option<i32>)>)> {
    package.into_iter().map(|package| {
        let convert_utf8 = |data: Option<Vec<u8>>| -> String {
            data.map(|v| String::from_utf8(v).unwrap_or_default()).unwrap_or_default()
        };

        let id = convert_utf8(package.0.clone());
        let package_item = get_item_by_package_id(conn, id.clone());
        let package_item = package_item.unwrap();

        let concat = (package.0, package.1, package.2, package.3, package.4, package.5, package.6, package.7, package.8, package.9, package.10, package.11, package.12, package.13, package.14, package.15, package.16, package.17, package.18, package.19, package.20, package.21, package.22, package.23, package.24, package.25, package.26, package.27, package.28, package_item);
        concat
    }).collect::<Vec<_>>()
}

pub fn get_packages_by_send_point_id(conn: &mut r2d2::PooledConnection<MySqlConnectionManager>, id: String) -> Option<Vec<(Option<Vec<u8>>, Option<Vec<u8>>, Option<Vec<u8>>, Option<Vec<u8>>, Option<Vec<u8>>, Option<Vec<u8>>, Option<Vec<u8>>, Option<Vec<u8>>, Option<Vec<u8>>, Option<Vec<u8>>, Option<Vec<u8>>, Option<Vec<u8>>, Option<Vec<u8>>, Option<Vec<u8>>, Option<Vec<u8>>, Option<Vec<u8>>, Option<i32>, Option<i32>, Option<i32>, Option<i32>, Option<i32>, Option<i32>, Option<i8>, Option<i8>, Option<f32>, Option<Vec<u8>>, Option<Vec<u8>>, Option<i32>, Option<i32>)>> {
    let first_query = format!("SELECT package.id,package.package_id,package.send_name,package.send_date,package.send_phone,package.send_address,package.send_point,package.receive_name,package.receive_phone,package.receive_address,package.receive_point,IF(p2.type=0,'exchanging','gathering') as current_from FROM package LEFT JOIN points as p1 ON package.send_point = p1.id LEFT JOIN points as p4 ON package.receive_point = p4.id LEFT JOIN points as p2 ON package.cur_point = p2.id LEFT JOIN points as p3 ON package.next_point = p3.id WHERE package.send_point = '{}'", id);

    let first_packages = conn.query_map(
        first_query,
        |(id, package_id, send_name, send_date, send_phone, send_address, send_point, receive_name, receive_phone, receive_address, receive_point, current_from)| (id, package_id, send_name, send_date, send_phone, send_address, send_point, receive_name, receive_phone, receive_address, receive_point, current_from),
    );

    let mut first_packages = match first_packages {
        Ok(packages) => packages,
        Err(_) => return None,
    };

    let second_query = format!("SELECT p2.reference as from_point_id,IF(p3.type=0,'exchanging','gathering') as current_dest,p3.reference as dest_point_id,package.status,package.main_cost,package.other_cost,package.gtgt_cost,package.other_service_cost,package.total_cost,package.vat,package.package_type, package.instruction_type FROM package LEFT JOIN points as p1 ON package.send_point = p1.id LEFT JOIN points as p4 ON package.receive_point = p4.id LEFT JOIN points as p2 ON package.cur_point = p2.id LEFT JOIN points as p3 ON package.next_point = p3.id WHERE package.send_point = '{}'", id);


    let second_packages = conn.query_map(
        second_query,
        |(from_point_id, current_dest, dest_point_id, status, main_cost, other_cost, gtgt_cost, other_service_cost, total_cost, vat, package_type, instruction_type)| (from_point_id, current_dest, dest_point_id, status, main_cost, other_cost, gtgt_cost, other_service_cost, total_cost, vat, package_type, instruction_type),
    );

    let mut second_packages = match second_packages {
        Ok(packages) => packages,
        Err(_) => return None,
    };

    let third_query = format!("SELECT package.weight,package.special_service,package.note,package.cod,package.receive_other_cost FROM package LEFT JOIN points as p1 ON package.send_point = p1.id LEFT JOIN points as p4 ON package.receive_point = p4.id LEFT JOIN points as p2 ON package.cur_point = p2.id LEFT JOIN points as p3 ON package.next_point = p3.id WHERE package.send_point = '{}'", id);

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

pub fn get_packages_by_receive_point_id(conn: &mut r2d2::PooledConnection<MySqlConnectionManager>, id: String) -> Option<Vec<(Option<Vec<u8>>, Option<Vec<u8>>, Option<Vec<u8>>, Option<Vec<u8>>, Option<Vec<u8>>, Option<Vec<u8>>, Option<Vec<u8>>, Option<Vec<u8>>, Option<Vec<u8>>, Option<Vec<u8>>, Option<Vec<u8>>, Option<Vec<u8>>, Option<Vec<u8>>, Option<Vec<u8>>, Option<Vec<u8>>, Option<Vec<u8>>, Option<i32>, Option<i32>, Option<i32>, Option<i32>, Option<i32>, Option<i32>, Option<i8>, Option<i8>, Option<f32>, Option<Vec<u8>>, Option<Vec<u8>>, Option<i32>, Option<i32>)>> {
    let first_query = format!("SELECT package.id,package.package_id,package.send_name,package.send_date,package.send_phone,package.send_address,package.send_point,package.receive_name,package.receive_phone,package.receive_address,package.receive_point,IF(p2.type=0,'exchanging','gathering') as current_from FROM package LEFT JOIN points as p1 ON package.send_point = p1.id LEFT JOIN points as p4 ON package.receive_point = p4.id LEFT JOIN points as p2 ON package.cur_point = p2.id LEFT JOIN points as p3 ON package.next_point = p3.id WHERE package.receive_point = '{}'", id);

    let first_packages = conn.query_map(
        first_query,
        |(id, package_id, send_name, send_date, send_phone, send_address, send_point, receive_name, receive_phone, receive_address, receive_point, current_from)| (id, package_id, send_name, send_date, send_phone, send_address, send_point, receive_name, receive_phone, receive_address, receive_point, current_from),
    );

    let mut first_packages = match first_packages {
        Ok(packages) => packages,
        Err(_) => return None,
    };

    let second_query = format!("SELECT p2.reference as from_point_id,IF(p3.type=0,'exchanging','gathering') as current_dest,p3.reference as dest_point_id,package.status,package.main_cost,package.other_cost,package.gtgt_cost,package.other_service_cost,package.total_cost,package.vat,package.package_type, package.instruction_type FROM package LEFT JOIN points as p1 ON package.send_point = p1.id LEFT JOIN points as p4 ON package.receive_point = p4.id LEFT JOIN points as p2 ON package.cur_point = p2.id LEFT JOIN points as p3 ON package.next_point = p3.id WHERE package.receive_point = '{}'", id);


    let second_packages = conn.query_map(
        second_query,
        |(from_point_id, current_dest, dest_point_id, status, main_cost, other_cost, gtgt_cost, other_service_cost, total_cost, vat, package_type, instruction_type)| (from_point_id, current_dest, dest_point_id, status, main_cost, other_cost, gtgt_cost, other_service_cost, total_cost, vat, package_type, instruction_type),
    );

    let mut second_packages = match second_packages {
        Ok(packages) => packages,
        Err(_) => return None,
    };

    let third_query = format!("SELECT package.weight,package.special_service,package.note,package.cod,package.receive_other_cost FROM package LEFT JOIN points as p1 ON package.send_point = p1.id LEFT JOIN points as p4 ON package.receive_point = p4.id LEFT JOIN points as p2 ON package.cur_point = p2.id LEFT JOIN points as p3 ON package.next_point = p3.id WHERE package.receive_point = '{}'", id);

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

pub fn get_packages_by_id(conn: &mut r2d2::PooledConnection<MySqlConnectionManager>, id: String) -> Option<Vec<(Option<Vec<u8>>, Option<Vec<u8>>, Option<Vec<u8>>, Option<Vec<u8>>, Option<Vec<u8>>, Option<Vec<u8>>, Option<Vec<u8>>, Option<Vec<u8>>, Option<Vec<u8>>, Option<Vec<u8>>, Option<Vec<u8>>, Option<Vec<u8>>, Option<Vec<u8>>, Option<Vec<u8>>, Option<Vec<u8>>, Option<Vec<u8>>, Option<i32>, Option<i32>, Option<i32>, Option<i32>, Option<i32>, Option<i32>, Option<i8>, Option<i8>, Option<f32>, Option<Vec<u8>>, Option<Vec<u8>>, Option<i32>, Option<i32>)>> {
    let first_query = format!("SELECT package.id,package.package_id,package.send_name,package.send_date,package.send_phone,package.send_address,package.send_point,package.receive_name,package.receive_phone,package.receive_address,package.receive_point,IF(p2.type=0,'exchanging','gathering') as current_from FROM package LEFT JOIN points as p1 ON package.send_point = p1.id LEFT JOIN points as p4 ON package.receive_point = p4.id LEFT JOIN points as p2 ON package.cur_point = p2.id LEFT JOIN points as p3 ON package.next_point = p3.id WHERE package.id = '{}'", id);

    let first_packages = conn.query_map(
        first_query,
        |(id, package_id, send_name, send_date, send_phone, send_address, send_point, receive_name, receive_phone, receive_address, receive_point, current_from)| (id, package_id, send_name, send_date, send_phone, send_address, send_point, receive_name, receive_phone, receive_address, receive_point, current_from),
    );

    let mut first_packages = match first_packages {
        Ok(packages) => packages,
        Err(_) => return None,
    };

    let second_query = format!("SELECT p2.reference as from_point_id,IF(p3.type=0,'exchanging','gathering') as current_dest,p3.reference as dest_point_id,package.status,package.main_cost,package.other_cost,package.gtgt_cost,package.other_service_cost,package.total_cost,package.vat,package.package_type, package.instruction_type FROM package LEFT JOIN points as p1 ON package.send_point = p1.id LEFT JOIN points as p4 ON package.receive_point = p4.id LEFT JOIN points as p2 ON package.cur_point = p2.id LEFT JOIN points as p3 ON package.next_point = p3.id WHERE package.id = '{}'", id);


    let second_packages = conn.query_map(
        second_query,
        |(from_point_id, current_dest, dest_point_id, status, main_cost, other_cost, gtgt_cost, other_service_cost, total_cost, vat, package_type, instruction_type)| (from_point_id, current_dest, dest_point_id, status, main_cost, other_cost, gtgt_cost, other_service_cost, total_cost, vat, package_type, instruction_type),
    );

    let mut second_packages = match second_packages {
        Ok(packages) => packages,
        Err(_) => return None,
    };

    let third_query = format!("SELECT package.weight,package.special_service,package.note,package.cod,package.receive_other_cost FROM package LEFT JOIN points as p1 ON package.send_point = p1.id LEFT JOIN points as p4 ON package.receive_point = p4.id LEFT JOIN points as p2 ON package.cur_point = p2.id LEFT JOIN points as p3 ON package.next_point = p3.id WHERE package.id = '{}'", id);

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