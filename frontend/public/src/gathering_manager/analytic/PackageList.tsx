import * as React from 'react';
import {
    BooleanField,
    Datagrid,
    DateField,
    DateInput,
    List,
    BulkExportButton,
    NumberField,
    SearchInput,
    TextField,
    useRecordContext,
    CreateButton,
    ExportButton,
    downloadCSV,
} from 'react-admin';

import ColoredDestField from './ColoredDestField'
import ColoredFromField from './ColoredFromField';
import ColoredStateField from './ColoredStateField';
import PackageShow from './PackageShow';
import { usePapaParse } from 'react-papaparse';
const packageFilter = [
    <SearchInput placeholder='Tìm kiếm mã hàng' source="package_id_q" alwaysOn />,
    <SearchInput placeholder='Tìm kiếm tên người gửi' source="send_name_q" alwaysOn />,

    <DateInput label='Ngày bắt đầu gửi' source="send_date_gte" alwaysOn />,
    <DateInput label='Ngày sau khi gửi' source="send_date_lt" alwaysOn />,
];

const BulkPackageActions = () => (
    <>
        <BulkExportButton />
    </>
);



const PackageListActions = () => {
    return (
        <>
            <ExportButton />
        </>
    )
};



const PackageList = () => {
    const { jsonToCSV } = usePapaParse();
    const exporter = (pacs) => {
        var newData = [];
        for (let i = 0; i < pacs.length; ++i) {
            newData.push({
                "Mã hàng": pacs[i].package_id,
                "Ngày gửi": (new Date(pacs[i].send_date)).toLocaleDateString(),
                "Người gửi": pacs[i].send_name,
                "Người nhận": pacs[i].receive_name,
                "Địa chỉ gửi": pacs[i].send_address,
                "Địa chỉ nhận": pacs[i].receive_address,
                "Tổng cước": pacs[i].total_cost.toLocaleString(
                    undefined,
                    {
                        style: 'currency',
                        currency: 'VND',
                    }
                ),
            })
        }
        const result = jsonToCSV(newData);
        downloadCSV(result, "danh_sách_hàng");
    };
    return (
        <List
            exporter={exporter}
            filters={packageFilter}
            sort={{ field: 'send_date', order: 'DESC' }}
            perPage={10}
            actions={<PackageListActions />}
            resource='gatheringPackage'
            title='Thống kê hàng'
        >
            <Datagrid
                optimized
                rowClick="expand"
                expand={<PackageShow />}
                bulkActionButtons={<BulkPackageActions />}
                sx={{
                    '& .RaDatagrid-thead': {
                        borderLeftColor: 'transparent',
                        borderLeftWidth: 5,
                        borderLeftStyle: 'solid',
                    },
                }}
            >
                <TextField source='package_id' label='Mã hàng' />
                <DateField source="send_date" label='Ngày gửi' />
                <TextField source='send_name' label='Người gửi' />
                <TextField source='receive_name' label='Người nhận' />
                <CostField label='Tổng cước' />
                <ColoredFromField source='current_from' label='Chuyển từ' />
                <ColoredDestField source='current_dest' label='Điểm đến' />
                <ColoredStateField source='status' label='Trạng thái' />
            </Datagrid>
        </List>
    );
};

const CostField = () => {
    const record = useRecordContext();
    return (
        <NumberField
            source='total_cost'
            options={{ style: 'currency', currency: 'VND' }}
            sx={{ color: 'red' }} />
    );
}
export default PackageList;
