use super::models::{CreateEmployeeData, PointData, PackageData, HistoryPackageData, PackageItem, PackageDataTime, GetDelivery, PackageHistory};

pub fn view_employees(employees: Vec<(Option<Vec<u8>>, Option<Vec<u8>>, Option<Vec<u8>>, Option<Vec<u8>>, Option<Vec<u8>>, Option<Vec<u8>>, Option<Vec<u8>>, Option<Vec<u8>>, Option<Vec<u8>>, Option<Vec<u8>>, Option<Vec<u8>>, Option<Vec<u8>>, Option<Vec<u8>>, Option<Vec<u8>>)>) -> Vec<CreateEmployeeData> {
    employees.into_iter().map(|(id, reference, create_date, last_seen, name, sex, email, birthday, phone, point_id, username, point_reference, p_type, position)| {
        let convert_utf8 = |data: Option<Vec<u8>>| -> String {
            data.map(|v| String::from_utf8(v).unwrap_or_default()).unwrap_or_default()
        };

        let m_type = convert_utf8(p_type);
        let m_type = match m_type.as_str() {
            "0" => String::from("1"),
            "1" => String::from("0"),
            _ => String::from(""),
        };

        CreateEmployeeData {
            id: convert_utf8(id),
            reference: convert_utf8(reference),
            create_date: convert_utf8(create_date),
            last_seen: convert_utf8(last_seen),
            name: convert_utf8(name),
            sex: convert_utf8(sex),
            email: convert_utf8(email),
            birthday: convert_utf8(birthday),
            phone: Some(convert_utf8(phone)),
            point_id: convert_utf8(point_id),
            username: convert_utf8(username),
            point_reference: Some(convert_utf8(point_reference)),
            m_type: m_type,
            position: convert_utf8(position),
        }
    }).collect()
}

pub fn view_points(points: Vec<(Option<Vec<u8>>, Option<Vec<u8>>, Option<i8>, Option<Vec<u8>>, Option<Vec<u8>>, Option<Vec<u8>>, Option<Vec<u8>>, Option<Vec<u8>>, Option<Vec<u8>>, Option<Vec<u8>>, Option<Vec<u8>>, Option<Vec<u8>>, Option<Vec<u8>>)>) -> Vec<PointData> {
    points.into_iter().map(|(id, location, p_type, link_point_id, create_date, reference, name, city, zipcode, phone, link_point_reference, manager_id, manager_reference)| {
        let convert_utf8 = |data: Option<Vec<u8>>| -> String {
            data.map(|v| String::from_utf8(v).unwrap_or_default()).unwrap_or_default()
        };
        
        let p_type = match p_type {
            Some(0) => String::from("1"),
            Some(1) => String::from("0"),
            _ => String::from(""),
        };
        
        PointData {
            id: convert_utf8(id),
            location: convert_utf8(location),
            p_type,
            link_point_id: Some(convert_utf8(link_point_id)),
            create_date: convert_utf8(create_date),
            reference: convert_utf8(reference),
            name: convert_utf8(name),
            city: convert_utf8(city),
            zipcode: convert_utf8(zipcode),
            phone: convert_utf8(phone),
            manager_reference: convert_utf8(manager_reference),
            manager_id: convert_utf8(manager_id),
            link_point_reference: convert_utf8(link_point_reference),
        }    
    }).collect()
}

pub fn view_packages(packages: Vec<(Option<Vec<u8>>, Option<Vec<u8>>, Option<Vec<u8>>, Option<Vec<u8>>, Option<Vec<u8>>, Option<Vec<u8>>, Option<Vec<u8>>, Option<Vec<u8>>, Option<Vec<u8>>, Option<Vec<u8>>, Option<Vec<u8>>, Option<Vec<u8>>, Option<Vec<u8>>, Option<Vec<u8>>, Option<Vec<u8>>, Option<Vec<u8>>, Option<i32>, Option<i32>, Option<i32>, Option<i32>, Option<i32>, Option<i32>, Option<i8>, Option<i8>, Option<f32>, Option<Vec<u8>>, Option<Vec<u8>>, Option<i32>, Option<i32>, Vec<(Option<Vec<u8>>, Option<i32>, Option<i32>)>)>) -> Vec<PackageData> {
    packages.into_iter().map(|(id, package_id, send_name, send_date, send_phone, send_address, send_point, receive_name, receive_phone, receive_address, receive_point, current_from, from_point_id, current_dest, dest_point_id, status, main_cost, other_cost, gtgt_cost, other_service_cost, total_cost, vat, package_type, instruction_type, weight, special_service, note, cod, receive_other_cost, items)| {
        let convert_utf8 = |data: Option<Vec<u8>>| -> String {
            data.map(|v| String::from_utf8(v).unwrap_or_default()).unwrap_or_default()
        };

        PackageData {
            id: convert_utf8(id),
            package_id: convert_utf8(package_id),
            send_name: Some(convert_utf8(send_name)),
            send_date: Some(convert_utf8(send_date)),
            send_phone: Some(convert_utf8(send_phone)),
            send_address: Some(convert_utf8(send_address)),
            send_point: Some(convert_utf8(send_point)),
            receive_name: Some(convert_utf8(receive_name)),
            receive_phone: Some(convert_utf8(receive_phone)),
            receive_address: Some(convert_utf8(receive_address)),
            receive_point: Some(convert_utf8(receive_point)),
            current_from: Some(convert_utf8(current_from)),
            from_point_id: Some(convert_utf8(from_point_id)),
            current_dest: Some(convert_utf8(current_dest)),
            dest_point_id: Some(convert_utf8(dest_point_id)),
            status: Some(convert_utf8(status)),
            main_cost: main_cost.unwrap_or_default(),
            other_cost: other_cost.unwrap_or_default(),
            gtgt_cost: gtgt_cost.unwrap_or_default(),
            other_service_cost: other_service_cost.unwrap_or_default(),
            total_cost: total_cost.unwrap_or_default(),
            vat: vat.unwrap_or_default(),
            package_type: package_type.unwrap_or_default(),
            instruction_type: instruction_type.unwrap_or_default(),
            weight: weight.unwrap_or_default(),
            special_service: convert_utf8(special_service),
            note: convert_utf8(note),
            cod: cod.unwrap_or_default(),
            receive_other_cost: receive_other_cost.unwrap_or_default(),
            items: items.into_iter().map(|(item_name, item_quantity, item_value)| {
                let convert_utf8 = |data: Option<Vec<u8>>| -> String {
                    data.map(|v| String::from_utf8(v).unwrap_or_default()).unwrap_or_default()
                };

                PackageItem {
                    item_name: convert_utf8(item_name),
                    item_quantity: item_quantity.unwrap_or_default(),
                    item_value: item_value.unwrap_or_default(),
                }
            }).collect()
        }
    }).collect()
}

pub fn view_package_cur_point(package_cur_history: Vec<(Option<Vec<u8>>, Option<Vec<u8>>, Option<Vec<u8>>, Option<Vec<u8>>, Option<Vec<u8>>, Option<Vec<u8>>, Option<Vec<u8>>, Option<Vec<u8>>, Option<Vec<u8>>, Option<Vec<u8>>, Option<Vec<u8>>, Option<Vec<u8>>, Option<Vec<u8>>, Option<Vec<u8>>, Option<Vec<u8>>, Option<Vec<u8>>, Option<i32>, Option<i32>, Option<i32>, Option<i32>, Option<i32>, Option<i32>, Option<i8>, Option<i8>, Option<f32>, Option<Vec<u8>>, Option<Vec<u8>>, Option<i32>, Option<i32>, Vec<(Option<Vec<u8>>, Option<i32>, Option<i32>)>, Option<Vec<u8>>, Option<Vec<u8>>)>) -> Vec<HistoryPackageData> {
    package_cur_history.into_iter().map(|(id, package_id, send_name, send_date, send_phone, send_address, send_point, receive_name, receive_phone, receive_address, receive_point, current_from, from_point_id, current_dest, dest_point_id, package_status, main_cost, other_cost, gtgt_cost, other_service_cost, total_cost, vat, package_type, instruction_type, weight, special_service, note, cod, receive_other_cost, item, status, time)| {
        let convert_utf8 = |data: Option<Vec<u8>>| -> String {
            data.map(|v| String::from_utf8(v).unwrap_or_default()).unwrap_or_default()
        };

        HistoryPackageData {
            package_data: PackageData {
                id: convert_utf8(id),
                package_id: convert_utf8(package_id),
                send_name: Some(convert_utf8(send_name)),
                send_date: Some(convert_utf8(send_date)),
                send_phone: Some(convert_utf8(send_phone)),
                send_address: Some(convert_utf8(send_address)),
                send_point: Some(convert_utf8(send_point)),
                receive_name: Some(convert_utf8(receive_name)),
                receive_phone: Some(convert_utf8(receive_phone)),
                receive_address: Some(convert_utf8(receive_address)),
                receive_point: Some(convert_utf8(receive_point)),
                current_from: Some(convert_utf8(current_from)),
                from_point_id: Some(convert_utf8(from_point_id)),
                current_dest: Some(convert_utf8(current_dest)),
                dest_point_id: Some(convert_utf8(dest_point_id)),
                status: Some(convert_utf8(package_status)),
                main_cost: main_cost.unwrap_or_default(),
                other_cost: other_cost.unwrap_or_default(),
                gtgt_cost: gtgt_cost.unwrap_or_default(),
                other_service_cost: other_service_cost.unwrap_or_default(),
                total_cost: total_cost.unwrap_or_default(),
                vat: vat.unwrap_or_default(),
                package_type: package_type.unwrap_or_default(),
                instruction_type: instruction_type.unwrap_or_default(),
                weight: weight.unwrap_or_default(),
                special_service: convert_utf8(special_service),
                note: convert_utf8(note),
                cod: cod.unwrap_or_default(),
                receive_other_cost: receive_other_cost.unwrap_or_default(),
                items: item.into_iter().map(|(item_name, item_quantity, item_value)| {
                    let convert_utf8 = |data: Option<Vec<u8>>| -> String {
                        data.map(|v| String::from_utf8(v).unwrap_or_default()).unwrap_or_default()
                    };

                    PackageItem {
                        item_name: convert_utf8(item_name),
                        item_quantity: item_quantity.unwrap_or_default(),
                        item_value: item_value.unwrap_or_default(),
                    }
                }).collect()
            },
            status: Some(convert_utf8(status)),
            time: Some(convert_utf8(time)),
        }

    }).collect()
}

pub fn view_packages_arrive_time(packages: Vec<(Option<Vec<u8>>, Option<Vec<u8>>, Option<Vec<u8>>, Option<Vec<u8>>, Option<Vec<u8>>, Option<Vec<u8>>, Option<Vec<u8>>, Option<Vec<u8>>, Option<Vec<u8>>, Option<Vec<u8>>, Option<Vec<u8>>, Option<Vec<u8>>, Option<Vec<u8>>, Option<Vec<u8>>, Option<Vec<u8>>, Option<Vec<u8>>, Option<i32>, Option<i32>, Option<i32>, Option<i32>, Option<i32>, Option<i32>, Option<i8>, Option<i8>, Option<f32>, Option<Vec<u8>>, Option<Vec<u8>>, Option<i32>, Option<i32>, Option<Vec<u8>>, Vec<(Option<Vec<u8>>, Option<i32>, Option<i32>)>)>) -> Vec<PackageDataTime> {
    packages.into_iter().map(|(id, package_id, send_name, send_date, send_phone, send_address, send_point, receive_name, receive_phone, receive_address, receive_point, current_from, from_point_id, current_dest, dest_point_id, status, main_cost, other_cost, gtgt_cost, other_service_cost, total_cost, vat, package_type, instruction_type, weight, special_service, note, cod, receive_other_cost, arrive_at_dest, items)| {
        let convert_utf8 = |data: Option<Vec<u8>>| -> String {
            data.map(|v| String::from_utf8(v).unwrap_or_default()).unwrap_or_default()
        };

        PackageDataTime {
            id: convert_utf8(id),
            package_id: convert_utf8(package_id),
            send_name: Some(convert_utf8(send_name)),
            send_date: Some(convert_utf8(send_date)),
            send_phone: Some(convert_utf8(send_phone)),
            send_address: Some(convert_utf8(send_address)),
            send_point: Some(convert_utf8(send_point)),
            receive_name: Some(convert_utf8(receive_name)),
            receive_phone: Some(convert_utf8(receive_phone)),
            receive_address: Some(convert_utf8(receive_address)),
            receive_point: Some(convert_utf8(receive_point)),
            current_from: Some(convert_utf8(current_from)),
            from_point_id: Some(convert_utf8(from_point_id)),
            current_dest: Some(convert_utf8(current_dest)),
            dest_point_id: Some(convert_utf8(dest_point_id)),
            status: Some(convert_utf8(status)),
            main_cost: main_cost.unwrap_or_default(),
            other_cost: other_cost.unwrap_or_default(),
            gtgt_cost: gtgt_cost.unwrap_or_default(),
            other_service_cost: other_service_cost.unwrap_or_default(),
            total_cost: total_cost.unwrap_or_default(),
            vat: vat.unwrap_or_default(),
            package_type: package_type.unwrap_or_default(),
            instruction_type: instruction_type.unwrap_or_default(),
            weight: weight.unwrap_or_default(),
            special_service: convert_utf8(special_service),
            note: convert_utf8(note),
            cod: cod.unwrap_or_default(),
            receive_other_cost: receive_other_cost.unwrap_or_default(),
            arrive_at_dest: Some(convert_utf8(arrive_at_dest)),
            items: items.into_iter().map(|(item_name, item_quantity, item_value)| {
                let convert_utf8 = |data: Option<Vec<u8>>| -> String {
                    data.map(|v| String::from_utf8(v).unwrap_or_default()).unwrap_or_default()
                };

                PackageItem {
                    item_name: convert_utf8(item_name),
                    item_quantity: item_quantity.unwrap_or_default(),
                    item_value: item_value.unwrap_or_default(),
                }
            }).collect()
        }
    }).collect()
}

pub fn view_delivery(deliveries: Vec<(Option<Vec<u8>>, Option<Vec<u8>>, Option<Vec<u8>>, Option<Vec<u8>>, Option<Vec<u8>>, Option<Vec<u8>>, Option<Vec<u8>>, Option<Vec<u8>>, Option<Vec<u8>>, Option<Vec<u8>>, Vec<PackageData>)>) -> Vec<GetDelivery> {
    deliveries.into_iter().map(|(id, delivery_id, begin_date, expected_date, arrived_date, current_from, from_point_id, current_dest, dest_point_id, final_state, packages)| {
        let convert_utf8 = |data: Option<Vec<u8>>| -> String {
            data.map(|v| String::from_utf8(v).unwrap_or_default()).unwrap_or_default()
        };
        
        GetDelivery {
            id: convert_utf8(id),
            delivery_id: convert_utf8(delivery_id),
            begin_date: convert_utf8(begin_date),
            expected_date: Some(convert_utf8(expected_date)),
            arrived_date: Some(convert_utf8(arrived_date)),
            current_from: convert_utf8(current_from),
            from_point_id: convert_utf8(from_point_id),
            current_dest: convert_utf8(current_dest),
            dest_point_id: convert_utf8(dest_point_id),
            final_state: Some(convert_utf8(final_state)),
            packages: packages,
        }
    }).collect()
}

pub fn view_package_history(package_history: Vec<(Option<Vec<u8>>, Option<Vec<u8>>, Option<Vec<u8>>, Option<Vec<u8>>)>) -> Vec<PackageHistory> {
    package_history.into_iter().map(|(begin_date, arrived_date, from_point_id, dest_point_id)| {
        let convert_utf8 = |data: Option<Vec<u8>>| -> String {
            data.map(|v| String::from_utf8(v).unwrap_or_default()).unwrap_or_default()
        };

        PackageHistory {
            begin_date: convert_utf8(begin_date),
            arrived_date: Some(convert_utf8(arrived_date)),
            from_point_id: convert_utf8(from_point_id),
            dest_point_id: convert_utf8(dest_point_id)
        }
    }).collect()
}

