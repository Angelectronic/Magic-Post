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
} from 'react-admin';

import ColoredDestField from './ColoredDestField'
import ColoredFromField from './ColoredFromField';
import ColoredStateField from './ColoredStateField';
import PackageListAside from './PackageListAside';
import PackageShow from './PackageShow';
import green from '@mui/material/colors/green';
import orange from '@mui/material/colors/orange';
import red from '@mui/material/colors/red';

const packageFilter = [
    <SearchInput placeholder='Tìm kiếm mã hàng' source="package_id_q" alwaysOn />,
    <SearchInput placeholder='Tìm kiếm tên người gửi' source="send_name_q" alwaysOn />,
];

const BulkPackageActions = () => (
    <>
        <BulkExportButton />
    </>
);


const rowSx = (record: any, index: any) => {
    let style = {};
    if (!record) {
        return style;
    }
    if (record.status === 'received')
        return {
            ...style,
            borderLeftColor: green[500],
            borderLeftWidth: 5,
            borderLeftStyle: 'solid',
        };
    if (record.status === 'in-transit')
        return {
            ...style,
            borderLeftColor: orange[500],
            borderLeftWidth: 5,
            borderLeftStyle: 'solid',
        };
    if (record.status === 'not-received')
        return {
            ...style,
            borderLeftColor: red[500],
            borderLeftWidth: 5,
            borderLeftStyle: 'solid',
        };
    else {
        return {
            ...style,
            borderLeftColor: 'transparent',
            borderLeftWidth: 5,
            borderLeftStyle: 'solid',
        };
    }
};

const PackageListActions = () => {
    return (
        <>
            <ExportButton />
        </>
    )
};
const PackageList = () => {
    return (
        <List
            filters={packageFilter}
            sort={{ field: 'send_date', order: 'DESC' }}
            perPage={25}
            title='Quản lý hàng điểm tập kết'
            aside={<PackageListAside />}
            actions={<PackageListActions />}
        >
            <Datagrid
                optimized
                rowClick="expand"
                expand={<PackageShow />}
                rowSx={rowSx}
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
