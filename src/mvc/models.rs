use r2d2_mysql::{
    mysql::prelude::*,
    r2d2, MySqlConnectionManager,
};

pub fn get_all_employees(conn: &mut r2d2::PooledConnection<MySqlConnectionManager>) -> Vec<(Option<Vec<u8>>, Option<Vec<u8>>, Option<Vec<u8>>, Option<Vec<u8>>)> {
    conn.query_map(
        "SELECT id, name, position, point_id FROM employees",
        |(id, name, position, point_id)| (id, name, position, point_id),
    )
    .unwrap()
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