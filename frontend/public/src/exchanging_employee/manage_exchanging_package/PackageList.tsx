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
    useGetMany,
    useListContext,
    useRedirect,
    useNotify,
} from 'react-admin';

import ColoredDestField from './ColoredDestField'
import ColoredFromField from './ColoredFromField';
import ColoredStateField from './ColoredStateField';
import PackageListAside from './PackageListAside';
import PackageShow from './PackageShow';
import green from '@mui/material/colors/green';
import orange from '@mui/material/colors/orange';
import red from '@mui/material/colors/red';
import { Button } from 'react-admin';
import { BulkActionProps } from 'react-admin';
import { LegendToggleRounded } from '@mui/icons-material';
const packageFilter = [
    <SearchInput placeholder='Tìm kiếm mã hàng' source="package_id_q" alwaysOn />,
    <SearchInput placeholder='Tìm kiếm tên người gửi' source="send_name_q" alwaysOn />,
];

const BulkPackageActions = () => (
    <>
        <BulkExportButton />
        <BulkActionButton />
    </>
);

export let selIds = [];
const BulkActionButton = () => {
    const { selectedIds: selectedIds } = useListContext();
    const { data: data, isLoading: isLoading } = useGetMany("exchangingPackage", { ids: selectedIds })
    const redirect = useRedirect();
    const notify = useNotify();
    const handleBulkClick = () => {
        selIds = selectedIds;
        var fail = 0;
        for (let i = 0; i < data?.length; ++i) {
            if (data[i].status !== 'received') {
                notify("Không thể tạo đơn với những hàng này", { type: "error" });
                fail = 1;
                break;
            }
        }
        if (!fail) {
            redirect("/exchangingDelivery/create");
        }
    }
    return (
        <Button
            color="secondary"
            label={"Tạo đơn hàng"}
            onClick={handleBulkClick}
            disabled={isLoading}
        />
    );
};

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
            <CreateButton label='Ghi nhận hàng' />
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
            title='Quản lý hàng điểm giao dịch'
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
