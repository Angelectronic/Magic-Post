use std::fmt::format;

use actix_session::Session;
use r2d2_mysql::{
    mysql::prelude::*,
    r2d2, MySqlConnectionManager,
};

use crate::mvc::view::models::UpdatePackage;

pub fn check_subordinate(session: &Session) -> bool {
    match session.get::<String>("id") {
        Ok(Some(_)) => {
            match session.get::<String>("position") {
                Ok(Some(position)) => {
                    if position != "subordinate" {
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

fn convert_from_option(field: Option<String>) -> String {
    match field {
        Some(a) => format!("'{}'", a),
        None => String::from("null"),
    }
}

pub fn insert_package(conn: &mut r2d2::PooledConnection<MySqlConnectionManager>, package: UpdatePackage) -> bool {
    let send_point = convert_from_option(package.send_point);
    let receive_point = convert_from_option(package.receive_point);
    let cur_point = convert_from_option(package.cur_point);
    let status = convert_from_option(package.status);
    let send_name = convert_from_option(package.send_name);
    let send_date = convert_from_option(package.send_date);
    let required_date = convert_from_option(package.required_date);
    let shipped_date = convert_from_option(package.shipped_date);
    let send_address = convert_from_option(package.send_address);
    let receive_address = convert_from_option(package.receive_address);
    let send_phone = convert_from_option(package.send_phone);
    let receive_phone = convert_from_option(package.receive_phone);
    let receive_name = convert_from_option(package.receive_name);


    let query = format!("INSERT INTO package (id, send_point, receive_point, cur_point, status, send_name, send_date, required_date, shipped_date, send_address, receive_address, send_phone, receive_phone, receive_name) VALUES (UUID(), {}, {}, {}, {}, {}, {}, {}, {}, {}, {}, {}, {}, {})", send_point, receive_point, cur_point, status, send_name, send_date, required_date, shipped_date, send_address, receive_address, send_phone, receive_phone, receive_name);

    conn.query_drop(query).is_ok()
}


pub fn change_status_packaging(conn: &mut r2d2::PooledConnection<MySqlConnectionManager>, package_id: String) -> bool {
    let query = format!("UPDATE package SET status = 'Packaging' WHERE id = '{}'", package_id);
    conn.query_drop(query).is_ok()
}

pub fn change_status_shipped(conn: &mut r2d2::PooledConnection<MySqlConnectionManager>, package_id: String, cur_point: String) -> bool {
    let query = format!("UPDATE package SET status = 'Shipped', cur_point = '{}' WHERE id = '{}'", cur_point, package_id);
    conn.query_drop(query).is_ok()
}

pub fn update_package(conn: &mut r2d2::PooledConnection<MySqlConnectionManager>, package: UpdatePackage, package_id: String) -> bool {
    let send_point = convert_from_option(package.send_point);
    let receive_point = convert_from_option(package.receive_point);
    let cur_point = convert_from_option(package.cur_point);
    let status = convert_from_option(package.status);
    let send_name = convert_from_option(package.send_name);
    let send_date = convert_from_option(package.send_date);
    let required_date = convert_from_option(package.required_date);
    let shipped_date = convert_from_option(package.shipped_date);
    let send_address = convert_from_option(package.send_address);
    let receive_address = convert_from_option(package.receive_address);
    let send_phone = convert_from_option(package.send_phone);
    let receive_phone = convert_from_option(package.receive_phone);
    let receive_name = convert_from_option(package.receive_name);
    let next_point = convert_from_option(package.next_point);

    let query = format!("UPDATE package SET send_point = {}, receive_point = {}, cur_point = {}, status = {}, send_name = {}, send_date = {}, required_date = {}, shipped_date = {}, send_address = {}, receive_address = {}, send_phone = {}, receive_phone = {}, receive_name = {}, next_point = {} WHERE id = '{}'", send_point, receive_point, cur_point, status, send_name, send_date, required_date, shipped_date, send_address, receive_address, send_phone, receive_phone, receive_name, next_point, package_id);

    conn.query_drop(query).is_ok()
}

pub fn get_point_by_id(conn: &mut r2d2::PooledConnection<MySqlConnectionManager>, point_id: String) -> Option<String> {
    let query = format!("SELECT gathering_point FROM points WHERE id = '{}'", point_id);
    let result: Option<String> = conn.query_first(query).unwrap();
    result
}

pub fn update_send_to_gathering(conn: &mut r2d2::PooledConnection<MySqlConnectionManager>, package_id: String, gathering_point_id: String) -> bool {
    let query = format!("UPDATE package SET next_point = '{}' WHERE id = '{}'",  gathering_point_id, package_id);
    conn.query_drop(query).is_ok()
}

pub fn confirm_delivery(conn: &mut r2d2::PooledConnection<MySqlConnectionManager>, delivery_id: String) -> bool {
    let query = format!("UPDATE delivery SET arrived_date = NOW() WHERE id = '{}'", delivery_id);
    let result = conn.query_drop(query).is_ok();

    if result {
        let second_query = format!("UPDATE package SET package.cur_point = package.next_point, package.next_point = null WHERE package.id IN (SELECT package_delivery.package_id FROM package_delivery WHERE package_delivery.delivery_id='{}')", delivery_id);
        conn.query_drop(second_query).is_ok()
    } else {
        false
    }
}