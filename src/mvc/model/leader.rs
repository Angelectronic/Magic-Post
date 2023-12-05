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

pub fn get_employees_by_point_id(conn: &mut r2d2::PooledConnection<MySqlConnectionManager>, point_id: String) -> Option<Vec<(Option<Vec<u8>>, Option<Vec<u8>>, Option<Vec<u8>>, Option<Vec<u8>>)>> {
    let query = format!("SELECT id, name, position, point_id FROM employees WHERE point_id = '{}'", point_id);
    conn.query_map(
        query,
        |(id, name, position, point_id)| {
            (id, name, position, point_id)
        },
    ).ok()
}