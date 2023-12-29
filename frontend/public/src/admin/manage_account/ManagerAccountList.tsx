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
    useRecordContext,
} from 'react-admin';
import { useMediaQuery, Divider, Tabs, Tab, Theme } from '@mui/material';
import PointReferenceField from './PointReferenceField';

const ListActions = () => (
    <TopToolbar>
        <CreateButton />
        <ExportButton />
    </TopToolbar>
);

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
const ManagerAccountsList = () => (
    <List
        title='Danh sách tài khoản trưởng điểm'
        perPage={25}
        filters={pointFilters}
        actions={<ListActions />}
        sort={{ field: 'name', order: 'DESC' }}
    >
        <TabbedDatagrid />
    </List>
);


const pointFilters = [
    <SearchInput placeholder="Mã trưởng điểm" source="reference_q" alwaysOn />,
    <SearchInput placeholder="Tên trưởng điểm" source="name_q" alwaysOn />,
];


const tabs = [
    { id: 'all', name: 'Tất cả', val: 0 },
    { id: 'gathering_manager', name: 'Trưởng điểm tập kết', val: 1 },
    { id: 'exchanging_manager', name: 'Trưởng điểm giao dịch', val: 2 },
];

const TabbedDatagrid = (hasChange: any) => {
    const listContext = useListContext();
    const { filterValues, setFilters, displayedFilters } = listContext;
    const isXSmall = useMediaQuery<Theme>(theme =>
        theme.breakpoints.down('sm')
    );
    const [currentTab, setCurrentTab] = React.useState(filterValues.m_type !== undefined ? filterValues.m_type : 0);
    const handleChange = useCallback(
        (event: React.ChangeEvent<{}>, value: any) => {
            setCurrentTab(value);
            if (value === 0) {
                setFilters &&
                    setFilters(
                        {},
                        [],
                        false // no debounce, we want the filter to fire immediately
                    );

            }
            else {
                setFilters &&
                    setFilters(
                        { ...filterValues, m_type: value },
                        displayedFilters,
                        false // no debounce, we want the filter to fire immediately
                    );
            }
        },
        [displayedFilters, filterValues, setFilters]
    );

    return (
        <Fragment>
            <Tabs
                variant="fullWidth"
                centered
                value={currentTab}
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
                                    filter={
                                        choice.val === 0 ? {} : { ...filterValues, m_type: choice.val }
                                    }
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
                    <Datagrid
                        rowClick="edit"
                        bulkActionButtons={<BulkAccountActions />}
                        sort={{ field: 'name', order: 'DESC' }}
                    >
                        <DateField source='create_date' label="Ngày tạo" />
                        <TextField source="reference" label="Mã trưởng điểm" />
                        <TextField source='name' label="Tên" />
                        <SexField label="Giới tính"/>
                        <TextField source='email' label="Email" />
                        <PointReferenceField label='Điểm phụ trách' />
                    </Datagrid>

                </>
            )}
        </Fragment>
    );
};

const SexField = () => {
    const record = useRecordContext();
    if (record.sex === 'male') {
        record.sex1 = 'Nam';
    }
    else if (record.sex === 'female') {
        record.sex1 = 'Nữ';
    }
    return (
        <TextField source='sex1' label="Giới tính" />
    );
}
export default ManagerAccountsList;
