import * as React from 'react';
import { Fragment, useCallback } from 'react';
import {
    AutocompleteInput,
    BooleanField,
    Count,
    Datagrid,
    DatagridConfigurable,
    DateField,
    DateInput,
    ExportButton,
    FilterButton,
    List,
    NullableBooleanInput,
    NumberField,
    ReferenceField,
    ReferenceInput,
    SearchInput,
    SelectColumnsButton,
    TextField,
    TextInput,
    TopToolbar,
    useListContext,
    CreateButton,
    BulkDeleteButton,
    BulkExportButton,
} from 'react-admin';
import { useMediaQuery, Divider, Tabs, Tab, Theme } from '@mui/material';
import ImportButton from './ImportButton';
const ListActions = (props) => {
    return (
        < TopToolbar >
            <CreateButton />
            <ExportButton />
            <ImportButton />
        </TopToolbar >
    );
}

const BulkAccountActions = () => (
    <>
        <BulkDeleteButton
            mutationMode='pessimistic'
            confirmTitle='Xóa %{smart_count} tài khoản'
            confirmContent='Bạn có chắc chắc muốn xóa %{smart_count} tài khoản này?'
        />
        <BulkExportButton />
    </>
);
const EmployeeAccountList = () => (
    <List
        title='Danh sách tài khoản nhân viên tập kết'
        perPage={25}
        filters={pointFilters}
        actions={<ListActions />}
    >
        <TabbedDatagrid />
    </List>
);


const pointFilters = [
    <SearchInput placeholder="Mã nhân viên" source="reference_q" alwaysOn />,
    <SearchInput placeholder="Tên nhân viên" source="name_q" alwaysOn />
];


const TabbedDatagrid = (hasChange: any) => {
    const listContext = useListContext();
    return (
        <Fragment>
            {(
                <>
                    <Datagrid
                        rowClick="edit"
                        bulkActionButtons={<BulkAccountActions />}
                        sort={{ field: 'create_date', order: 'DESC' }}
                    >
                        <DateField source='create_date' label="Ngày tạo" />
                        <DateField source='last_seen' label="Ngày đăng nhập cuối" />
                        <TextField source="reference" label="Mã nhân viên" />
                        <TextField source='name' label="Tên nhân viên" />
                        <TextField source='email' label="Email" />
                        <TextField source='sex' label="Giới tính" />
                        <TextField source='point_id' label='Điểm phụ trách' />
                    </Datagrid>

                </>
            )}
        </Fragment>
    );
};

export default EmployeeAccountList;
