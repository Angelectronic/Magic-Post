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
        Some(a) => match a.as_str() {
            "" => String::from("null"),
            _ => format!("'{}'", a),
        },
        None => String::from("null"),
    }
}
pub fn insert_package(conn: &mut r2d2::PooledConnection<MySqlConnectionManager>, package: UpdatePackage) -> bool {
    let send_name = convert_from_option(package.send_name);
    let send_phone = convert_from_option(package.send_phone);
    let send_address = convert_from_option(package.send_address);
    let send_point = convert_from_option(package.send_point.clone());
    let receive_name = convert_from_option(package.receive_name);
    let receive_phone = convert_from_option(package.receive_phone);
    let receive_address = convert_from_option(package.receive_address);
    let receive_point = convert_from_option(package.receive_point);
    let from_point_id = convert_from_option(package.send_point);
    let dest_point_id = String::from("null");
    let status = convert_from_option(package.status);
    let main_cost = package.main_cost;
    let other_cost = package.other_cost;
    let gtgt_cost = package.gtgt_cost;
    let other_service_cost = package.other_service_cost;
    let total_cost = package.total_cost;
    let vat = package.vat;
    let package_type = package.package_type;
    let instruction_type = package.instruction_type;
    let weight = package.weight;
    let special_service = convert_from_option(Some(package.special_service));
    let note = convert_from_option(Some(package.note));
    let cod = package.cod;
    let receive_other_cost = package.receive_other_cost;

    let mut package_id = String::from("VN");
    for _ in 0..4 {
        let random_number = rand::random::<u8>();
        package_id.push_str(&random_number.to_string());
    }
    package_id.push_str("VN");

    let id: Option<String> = conn.query_first("SELECT UUID()").unwrap();
    let id = id.unwrap();

    let query = format!("INSERT INTO package VALUES ('{}', '{}', {}, CURRENT_DATE(), {}, {}, {}, {}, null, {}, {}, {}, {}, {}, {}, {}, {}, {}, {}, {}, {}, {}, {}, {}, {}, {}, {}, {})", id, package_id, send_name,  send_phone, send_address, send_point, receive_name, receive_phone, receive_address, receive_point, from_point_id, dest_point_id, status, main_cost, other_cost, gtgt_cost, other_service_cost, total_cost, vat, package_type, instruction_type, weight, special_service, note, cod, receive_other_cost);
    let result = conn.query_drop(query).is_ok();
    
    if result { 
        for item in package.items {
            let item_name = convert_from_option(Some(item.item_name));
            let item_quantity = item.item_quantity;
            let item_value = item.item_value;
            let item_query = format!("INSERT INTO items VALUES ({}, {}, {}, '{}')", item_name, item_quantity, item_value, id);
            let res = conn.query_drop(item_query).is_ok();
            if !res {
                return false;
            }
        }
        true
    } else {
        false
    }

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
    let send_name = convert_from_option(package.send_name);
    let send_phone = convert_from_option(package.send_phone);
    let send_address = convert_from_option(package.send_address);
    let send_point = convert_from_option(package.send_point);
    let receive_name = convert_from_option(package.receive_name);
    let receive_phone = convert_from_option(package.receive_phone);
    let receive_address = convert_from_option(package.receive_address);
    let receive_point = convert_from_option(package.receive_point);
    let from_point_id = convert_from_option(package.from_point_id);
    let dest_point_id = convert_from_option(package.dest_point_id);
    let status = convert_from_option(package.status);
    let main_cost = package.main_cost;
    let other_cost = package.other_cost;
    let gtgt_cost = package.gtgt_cost;
    let other_service_cost = package.other_service_cost;
    let total_cost = package.total_cost;
    let vat = package.vat;
    let package_type = package.package_type;
    let instruction_type = package.instruction_type;
    let weight = package.weight;
    let special_service = package.special_service;
    let note = package.note;
    let cod = package.cod;
    let receive_other_cost = package.receive_other_cost;

    let query = format!("UPDATE package SET send_name = {}, send_phone = {}, send_address = {}, send_point = {}, receive_name = {}, receive_phone = {}, receive_address = {}, receive_point = {}, from_point_id = {}, dest_point_id = {}, status = {}, main_cost = {}, other_cost = {}, gtgt_cost = {}, other_service_cost = {}, total_cost = {}, vat = {}, package_type = {}, instruction_type = {}, weight = {}, special_service = {}, note = {}, cod = {}, receive_other_cost = {} WHERE id = '{}'", send_name, send_phone, send_address, send_point, receive_name, receive_phone, receive_address, receive_point, from_point_id, dest_point_id, status, main_cost, other_cost, gtgt_cost, other_service_cost, total_cost, vat, package_type, instruction_type, weight, special_service, note, cod, receive_other_cost, package_id);

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