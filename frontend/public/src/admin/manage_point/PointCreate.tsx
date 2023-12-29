import * as React from 'react';
import {
    Create,
    SimpleForm,
    TextInput,
    ReferenceInput,
    AutocompleteInput,
    useRecordContext,
    useGetMany,
    useGetList,
    useGetOne,
    TabbedForm,
    SelectInput,
} from 'react-admin';
import { Box, Typography, TextField } from '@mui/material';
import { globalFilters } from './PointList';
import { AnyRecord } from 'dns';
import zipcode from './zipcode';
import city from './city';

import { useWatch } from 'react-hook-form';

export const validateForm = (
    values: Record<string, any>
): Record<string, any> => {
    const errors = {} as any;
    if (!values.name) {
        errors.name = 'ra.validation.required';
    }

    if (!values.location) {
        errors.location = 'ra.validation.required';
    }
    if (!values.phone) {
        errors.phone = 'ra.validation.required';
    }
    if (!values.city) {
        errors.city = 'ra.validation.required';
    }

    if (!values.manager_id) {
        errors.manager_id = 'ra.validation.required';
    }
    if (globalFilters.p_type === 1) {
        if (!values.link_point_id) {
            errors.link_point_id = 'ra.validation.required';
        }
    }
    return errors;
};

var saveSelected1: any;
var saveSelected2: any;
const PointCreate = () => {
    const transform = (data: any) => {
        if (saveSelected1 === undefined || saveSelected1 === null || saveSelected1.reference === null || saveSelected1.reference === undefined) {
            data.manager_id = undefined;
            data.manager_reference = '';
        }
        else {
            data.manager_reference = saveSelected1.reference;
        }

        if (saveSelected2 === undefined || saveSelected2 === null || saveSelected2.reference === null || saveSelected2.reference === undefined) {
            data.link_point_id = undefined;
            data.link_point_reference = '';
        }
        else {
            data.link_point_reference = saveSelected2.reference;
        }

        if (data.city !== null && data.city !== undefined) {
            for (let i = 0; i < 63; ++i) {
                if (data.city === city[i]) {
                    data.zipcode = zipcode[i];
                    break;
                }
            }
        }

        if (data.link_point_id === '') {
            data.link_point_id = null;
        }
        return data;
    }
    if (globalFilters.p_type === undefined) {
        globalFilters.p_type = 0;
    }
    return (
        <Create
            transform={transform}
            sx={{
                [`& .RaCreate-main`]: {
                    width: '60%',
                },
            }}
            title={globalFilters.p_type === 0 ? 'Tạo điểm tập kết mới' : 'Tạo điểm giao dịch mới'}
            redirect="../"
        >

            <TabbedForm
                validate={validateForm}
                defaultValues={{
                    name: '',
                    location: '',
                    manager_reference: '',
                    city: '',
                    zipcode: '',
                    p_type: globalFilters.p_type,
                    phone: '',
                }}>
                <TabbedForm.Tab
                    label="Chi tiết"
                    sx={{ maxWidth: '50em' }}
                >
                    <FormData />
                </TabbedForm.Tab>
                {globalFilters.p_type === 1 && (
                    <TabbedForm.Tab label="Liên kết" sx={{ maxWidth: '40em' }}>
                        <ExchangingFormData />
                    </TabbedForm.Tab>
                )}
            </TabbedForm>

        </Create>
    );
}

const ExchangingFormData = () => {
    const [saveSelected, setSaveSelected] = React.useState<any>(null);

    const [filter, setFilter] = React.useState({ reference_q: '', p_type: 0 });
    const { data: choices, isLoading: isLoadingChoices } = useGetList('points', { filter });

    return (
        <>
            <SectionTitle label="Điểm tập kết" />
            <Box display={{ xs: 'block', sm: 'flex' }}>
                <AutocompleteInput
                    autoFocus
                    label="Tên điểm tập kết"
                    suggestionLimit={10}
                    openText='Mở'
                    source='link_point_id'
                    onChange={(event, value) => {
                        setSaveSelected(value);
                        saveSelected2 = value;
                    }}
                    noOptionsText='Không tìm thấy điểm tập kết'
                    choices={choices}
                    optionText="name"
                    disabled={isLoadingChoices}
                />
            </Box>
            <PointInfo
                name={saveSelected !== null && saveSelected.name !== undefined ? saveSelected.name : ''}
                reference={saveSelected !== null && saveSelected.reference !== undefined ? saveSelected.reference : ''}
            />
        </>
    );
}
const FormData = () => {

    const [saveSelected, setSaveSelected] = React.useState<any>(null);

    const [filter, setFilter] = React.useState({ reference_q: '', m_type: 0 });
    const { data: choices, isLoading: isLoadingChoices } = useGetList('managerAccounts', { filter });

    const cityChoice = city.map((value) => ({
        id: value,
        name: value
    }));
    return (
        <>
            <SectionTitle label="Thông tin chung" />
            <Box display={{ xs: 'block', sm: 'flex', width: '100%' }}>
                <Box flex={1} mr={{ xs: 0, sm: '1em' }}>
                    <TextInput label="Tên điểm" source="name" isRequired fullWidth />
                </Box>
            </Box>
            <SectionTitle label="Địa chỉ" />
            <TextInput
                source="location"
                multiline
                fullWidth
                helperText={false}
                label='Địa chỉ'
                isRequired
            />
            <Box display={{ xs: 'block', sm: 'flex' }}>
                <Box flex={2} mr={{ xs: 0, sm: '2em' }}>
                    <SelectInput source="city" label='Thành phố' isRequired choices={cityChoice} />
                </Box>
                <Box flex={2} mr={{ xs: 0, sm: '2em' }}>
                    <ZipcodeField />
                </Box>
                <Box flex={2} mr={{ xs: 0, sm: '2em' }}>
                    <TextInput isRequired label='SĐT' source="phone" fullWidth helperText={false} />
                </Box>
            </Box>
            <Separator />
            <SectionTitle label="Trưởng điểm" />
            <Box display={{ xs: 'block', sm: 'flex' }}>
                <AutocompleteInput
                    autoFocus
                    label="Tên trưởng điểm"
                    suggestionLimit={10}
                    openText='Mở'
                    source='manager_id'
                    onChange={(event, value) => {
                        setSaveSelected(value);
                        saveSelected1 = value;
                    }}
                    noOptionsText='Không tìm thấy trưởng điểm'
                    choices={choices}
                    optionText="name"
                    disabled={isLoadingChoices}
                    isRequired
                />
            </Box>
            <ManagerInfo
                name={saveSelected !== null && saveSelected.name !== undefined ? saveSelected.name : ''}
                reference={saveSelected !== null && saveSelected.reference !== undefined ? saveSelected.reference : ''}
            />

        </>
    );
}

const ZipcodeField = () => {
    var cur_city = useWatch({ name: 'city' })
    var cur_zipcode = '';
    if (cur_city !== '') {
        for (let i = 0; i < 63; ++i) {
            if (cur_city === city[i]) {
                cur_zipcode = zipcode[i];
                break;
            }
        }
        return (
            <TextField
                disabled={true}
                label="Mã zip"
                variant="filled"
                value={cur_zipcode}
            />
        )
    }

    else {
        return (
            <TextField
                disabled={true}
                label="Mã zip"
                variant="filled"
                value={""}
            />
        );
    }
}
const ManagerInfo = (info: any) => {
    if (info.name !== '' || info.reference !== '') {
        return (
            <>
                <TextField
                    disabled={true}
                    label="Tên trưởng điểm"
                    variant="outlined"
                    value={info.name}
                />
                <TextField
                    disabled={true}
                    label="Mã trưởng điểm"
                    variant="outlined"
                    value={info.reference}
                />
            </>
        );
    }
}

const PointInfo = (info: any) => {
    if (info.name !== '' || info.reference !== '') {
        return (
            <>
                <TextField
                    disabled={true}
                    label="Tên điểm tập kết"
                    variant="outlined"
                    value={info.name}
                />
                <TextField
                    disabled={true}
                    label="Mã điểm tập kết"
                    variant="outlined"
                    value={info.reference}
                />
            </>
        );
    }
}

const SectionTitle = ({ label }: { label: string }) => {

    return (
        <Typography variant="h6" gutterBottom>
            {label}
        </Typography>
    );
};

const Separator = () => <Box pt="1em" />;

export default PointCreate;
