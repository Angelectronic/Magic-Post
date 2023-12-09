use super::models::{CreateEmployeeData, PointData, PackageData, HistoryPackageData};

/* view function */
pub fn view_employees(employees: Vec<(Option<Vec<u8>>, Option<Vec<u8>>, Option<Vec<u8>>, Option<Vec<u8>>)>) -> Vec<CreateEmployeeData> {
    employees.into_iter().map(|(id, name, position, point_id)| {
        let convert_utf8 = |data: Option<Vec<u8>>| -> String {
            data.map(|v| String::from_utf8(v).unwrap_or_default()).unwrap_or_default()
        };
        CreateEmployeeData {
            id: convert_utf8(id),
            name: convert_utf8(name),
            position: convert_utf8(position),
            point_id: convert_utf8(point_id),
        }
    }).collect()
}

pub fn view_points(points: Vec<(Option<Vec<u8>>, Option<Vec<u8>>, Option<i8>, Option<Vec<u8>>)>) -> Vec<PointData> {
    points.into_iter().map(|(id, location, p_type, gathering_point)| {
        let convert_utf8 = |data: Option<Vec<u8>>| -> String {
            data.map(|v| String::from_utf8(v).unwrap_or_default()).unwrap_or_default()
        };
        
        let p_type = match p_type {
            Some(0) => String::from("Điểm giao dịch"),
            Some(1) => String::from("Điểm tập kết"),
            _ => String::from(""),
        };
        
        PointData {
            id: convert_utf8(id),
            location: convert_utf8(location),
            p_type,
            gathering_point: Some(convert_utf8(gathering_point)),
        
        }    
    }).collect()
}

pub fn view_packages(packages: Vec<(Option<Vec<u8>>, Option<Vec<u8>>, Option<Vec<u8>>, Option<Vec<u8>>, Option<Vec<u8>>, Option<Vec<u8>>, Option<Vec<u8>>, Option<Vec<u8>>, Option<Vec<u8>>, Option<Vec<u8>>, Option<Vec<u8>>, Option<Vec<u8>>, Option<Vec<u8>>, Option<Vec<u8>>, Option<Vec<u8>>)>) -> Vec<PackageData> {
    packages.into_iter().map(|(id, send_point, receive_point, cur_point, status, send_name, send_date, required_date, shipped_date, send_address, receive_address, send_phone, receive_phone, receive_name, next_point)| {
        let convert_utf8 = |data: Option<Vec<u8>>| -> String {
            data.map(|v| String::from_utf8(v).unwrap_or_default()).unwrap_or_default()
        };

        PackageData {
            id: convert_utf8(id),
            send_point: Some(convert_utf8(send_point)),
            receive_point: Some(convert_utf8(receive_point)),
            cur_point: Some(convert_utf8(cur_point)),
            status: Some(convert_utf8(status)),
            send_name: Some(convert_utf8(send_name)),
            send_date: Some(convert_utf8(send_date)),
            required_date: Some(convert_utf8(required_date)),
            shipped_date: Some(convert_utf8(shipped_date)),
            send_address: Some(convert_utf8(send_address)),
            receive_address: Some(convert_utf8(receive_address)),
            send_phone: Some(convert_utf8(send_phone)),
            receive_phone: Some(convert_utf8(receive_phone)),
            receive_name: Some(convert_utf8(receive_name)),
            next_point: Some(convert_utf8(next_point))
        }

    }).collect()
}

pub fn view_package_cur_point(package_cur_history: Vec<(Option<Vec<u8>>, Option<Vec<u8>>, Option<Vec<u8>>, Option<Vec<u8>>, Option<Vec<u8>>, Option<Vec<u8>>, Option<Vec<u8>>, Option<Vec<u8>>, Option<Vec<u8>>, Option<Vec<u8>>, Option<Vec<u8>>, Option<Vec<u8>>, Option<Vec<u8>>, Option<Vec<u8>>, Option<Vec<u8>>, Option<Vec<u8>>, Option<Vec<u8>>)>) -> Vec<HistoryPackageData> {
    package_cur_history.into_iter().map(|(id, send_point, receive_point, cur_point, package_status, send_name, send_date, required_date, shipped_date, send_address, status, time, receive_address, send_phone, receive_phone, receive_name, next_point)| {
        let convert_utf8 = |data: Option<Vec<u8>>| -> String {
            data.map(|v| String::from_utf8(v).unwrap_or_default()).unwrap_or_default()
        };

        HistoryPackageData {
            package_data: PackageData {
                id: convert_utf8(id),
                send_point: Some(convert_utf8(send_point)),
                receive_point: Some(convert_utf8(receive_point)),
                cur_point: Some(convert_utf8(cur_point)),
                status: Some(convert_utf8(package_status)),
                send_name: Some(convert_utf8(send_name)),
                send_date: Some(convert_utf8(send_date)),
                required_date: Some(convert_utf8(required_date)),
                shipped_date: Some(convert_utf8(shipped_date)),
                send_address: Some(convert_utf8(send_address)),
                receive_address: Some(convert_utf8(receive_address)),
                send_phone: Some(convert_utf8(send_phone)),
                receive_phone: Some(convert_utf8(receive_phone)),
                receive_name: Some(convert_utf8(receive_name)),
                next_point: Some(convert_utf8(next_point))
            },
            status: Some(convert_utf8(status)),
            time: Some(convert_utf8(time)),
        }

    }).collect()
}