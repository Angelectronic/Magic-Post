import * as React from 'react';
import {
    DateInput,
    SearchInput,
    SelectInput,
} from 'react-admin';

const exchangingDeliveryFilters = [
    <SearchInput placeholder="Mã đơn hàng" source="delivery_id_q" alwaysOn />,
    <SearchInput placeholder="Mã hàng" source="package_id_q" alwaysOn />,
    <SelectInput
        label="Trạng thái"
        source="status"
        choices={[
            { id: 'resolved', name: 'Đã xử lý' },
            { id: 'not-resolved', name: 'Đang xử lý' },
        ]}
    />,
    <SelectInput
        label="Điểm đến"
        source="current_dest"
        choices={[
            { id: 'exchanging', name: 'Điểm giao dịch' },
            { id: 'gathering', name: 'Điểm tập kết' },
            { id: 'receiver', name: 'Người nhận' },
        ]}
    />,
    <DateInput label="Ngày tạo sau" source="create_date_gte" />,
    <DateInput label="Ngày tạo trước" source="create_date_lte" />,
];

export default exchangingDeliveryFilters;
