use r2d2_mysql::{
    mysql::prelude::*,
    r2d2, MySqlConnectionManager,
};

pub fn get_all_employees(conn: &mut r2d2::PooledConnection<MySqlConnectionManager>) -> Vec<(Option<Vec<u8>>, Option<Vec<u8>>, Option<Vec<u8>>, Option<Vec<u8>>, Option<Vec<u8>>, Option<Vec<u8>>)> {
    conn.query_map(
        "SELECT * FROM employees",
        |(id, name, position, point_id, username, password)| (id, name, position, point_id, username, password),
    )
    .unwrap()
}