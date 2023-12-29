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
    RefreshButton,
    useRecordContext,
} from 'react-admin';
import jsonExport from 'jsonexport'
import { useMediaQuery, Divider, Tabs, Tab, Theme } from '@mui/material';
import ManagerReferenceField from '../../visitors/ManagerReferenceField';
import AddressField from '../../visitors/AddressField';
import PointReferenceField from './PointReferenceField';
const ListActions = () => (
    <TopToolbar>
        <CreateButton />
        <ExportButton />
    </TopToolbar>
);

const BulkPointActions = () => (
    <>
        <BulkDeleteButton
            mutationMode='pessimistic'
            confirmTitle='Xóa %{smart_count} điểm'
            confirmContent='Bạn có chắc chắc muốn xóa %{smart_count} điểm này?'
        />
        <BulkExportButton />
    </>
);
const PointList = () => (
    <List
        title='Danh sách các điểm'
        filterDefaultValues={{ p_type: 0 }}
        perPage={25}
        filters={pointFilters}
        actions={<ListActions />}
        sort={{ field: 'create_date', order: 'DESC' }}
    >
        <TabbedDatagrid />
    </List>
);

export var globalFilters: any;

const pointFilters = [
    <SearchInput placeholder="Mã điểm" source="reference_q" alwaysOn />
];

const tabs = [
    { id: 'gathering', name: 'Điểm tập kết', val: 0 },
    { id: 'exchanging', name: 'Điểm giao dịch', val: 1 },
];

const TabbedDatagrid = () => {
    const listContext = useListContext();
    const { filterValues, setFilters, displayedFilters } = listContext;
    globalFilters = filterValues;

    const handleChange = useCallback(
        (event: React.ChangeEvent<{}>, value: any) => {

            setFilters &&
                setFilters(
                    { ...filterValues, p_type: value },
                    displayedFilters,
                    false // no debounce, we want the filter to fire immediately
                );

        },
        [displayedFilters, filterValues, setFilters]
    );
    return (
        <Fragment>
            <Tabs
                variant="fullWidth"
                centered
                value={filterValues.p_type}
                indicatorColor="primary"
                onChange={handleChange}
            >
                {tabs.map(choice => (
                    <Tab
                        key={choice.id}
                        label={
                            <span>
                                {choice.name} (
                                <Count
                                    filter={{
                                        ...filterValues,
                                        p_type: choice.val,
                                    }}
                                    sx={{ lineHeight: 'inherit' }}
                                />
                                )
                            </span>
                        }
                        value={choice.val}
                    />
                ))}
            </Tabs>
            <Divider />
            {(
                <>
                    {filterValues.p_type === 0 && (
                        <Datagrid
                            rowClick="edit"
                            bulkActionButtons={<BulkPointActions />}
                            sort={{ field: 'create_date', order: 'DESC' }}
                        >
                            <DateField source='create_date' label="Ngày tạo" />
                            <TextField source="reference" label="Mã bưu chính" />
                            <TextField source='name' label="Tên điểm" />
                            <TypeField label='Loại điểm' />
                            <AddressField label='Địa chỉ' />
                            <TextField source='phone' label='SĐT' />
                            <ManagerReferenceField label='Trưởng điểm' />
                        </Datagrid>
                    )}
                    {filterValues.p_type === 1 && (
                        <Datagrid
                            rowClick="edit"
                            bulkActionButtons={<BulkPointActions />}
                            sort={{ field: 'create_date', order: 'DESC' }}
                        >
                            <DateField source='create_date' label="Ngày tạo" />
                            <TextField source="reference" label="Mã" />
                            <TextField source='name' label="Tên điểm" />
                            <TypeField label='Loại điểm' />
                            <AddressField label='Địa chỉ' />
                            <TextField source='phone' label='SĐT' />
                            <ManagerReferenceField label='Trưởng điểm' />
                            <PointReferenceField label="Liên kết" />
                        </Datagrid>
                    )}
                </>
            )}
        </Fragment>
    );
};

const TypeField = () => {
    const record = useRecordContext();
    if (record.p_type === 0) {
        record.type_name = 'Điểm tập kết';
        return (
            <TextField source='type_name' label='Loại điểm' />
        );
    }
    else if (record.p_type === 1) {
        record.type_name = 'Điểm giao dịch';
        return (
            <TextField source='type_name' label='Loại điểm' />
        );
    }
}
export default PointList;
