import * as React from 'react';
import {
    AutocompleteInput,
    TextInput,
    ReferenceInput,
    SimpleForm,
    Edit,
    ListButton,
    TopToolbar,
    Toolbar,
    SaveButton,
    FormDataConsumer,
    useGetMany,
    useRecordContext,
    useGetList,
    useGetOne,
    TabbedForm,
    SelectInput,
} from 'react-admin';
import { Grid, Box, Typography, TextField } from '@mui/material';

import { validateForm } from './PointCreate';
import PointNameField from './PointNameField';

import { useWatch } from 'react-hook-form';
import city from './city'
import zipcode from './zipcode';
const PointEditActions = () => (
    <TopToolbar>
        <ListButton />
    </TopToolbar>
);

const FormEditToolBar = () => (
    <Toolbar>
        <SaveButton />
    </Toolbar>
)
var saveSelected1: any;
var saveSelected2: any;
const PointEdit = () => {
    const [saveSelected, setSaveSelected] = React.useState<any>(null);
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
    return (
        <Edit
            title="Sửa thông tin điểm"
            actions={<PointEditActions />}
            mutationMode='pessimistic'
            transform={transform}
            sx={{
                [`& .RaEdit-main`]: {
                    width: '60%',
                },
            }}
        >
            <FormWrapper />

        </Edit>
    );
};

const FormWrapper = () => {
    const record = useRecordContext();
    if (record.p_type === 1) {
        return (
            <>
                <TabbedForm
                    validate={validateForm}
                    toolbar={<FormEditToolBar />}
                >
                    <TabbedForm.Tab
                        label="Chi tiết"
                        sx={{ maxWidth: '40em' }}
                    >
                        <FormDataWithID />
                    </TabbedForm.Tab>
                    <TabbedForm.Tab label="Liên kết" sx={{ maxWidth: '40em' }}>
                        <ExchangingFormData />
                    </TabbedForm.Tab>
                </TabbedForm>

            </>
        );
    }
    else {
        return (
            <>
                <TabbedForm
                    validate={validateForm}
                    toolbar={<FormEditToolBar />}
                >
                    <TabbedForm.Tab
                        label="Chi tiết"
                        sx={{ maxWidth: '40em' }}

                    >
                        <FormDataWithID />
                    </TabbedForm.Tab>
                </TabbedForm>

            </>
        );
    }
}


const ExchangingFormData = () => {

    const [saveSelected, setSaveSelected] = React.useState<any>(null);

    const [filter, setFilter] = React.useState({ reference_q: '', m_type: 0 });
    const { data: choices, isLoading: isLoadingChoices } = useGetList('points', { filter });
    var link_point_id = useWatch({ name: 'link_point_id' })
    if (link_point_id === undefined || link_point_id === null) {
        link_point_id = -1;
    }
    var currentLink;
    var isLoadingCurrentLink;
    try {
        const { data: managerData, isLoading: isManagerLoading } = useGetOne('points', { id: link_point_id });
        currentLink = managerData;
        isLoadingCurrentLink = isManagerLoading;
    } catch (error) {
    }
    var choicesWithCurrentLink = choices
        ? choices.find(choice => choice.id === link_point_id)
            ? choices
            : [...choices, currentLink]
        : [];

    if (link_point_id === undefined || link_point_id === null) {
        choicesWithCurrentLink = [choices];
    }
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

const FormDataWithID = () => {

    const [saveSelected, setSaveSelected] = React.useState<any>(null);

    const [filter, setFilter] = React.useState({ reference_q: '', m_type: 0 });
    const { data: choices, isLoading: isLoadingChoices } = useGetList('managerAccounts', { filter });
    var manager_id = useWatch({ name: 'manager_id' })
    if (manager_id === undefined || manager_id === null) {
        manager_id = -1;
    }
    var currentManager;
    var isLoadingCurrentManager;
    try {
        const { data: managerData, isLoading: isManagerLoading } = useGetOne('managerAccounts', { id: manager_id });
        currentManager = managerData;
        isLoadingCurrentManager = isManagerLoading;
    } catch (error) {
    }
    var choicesWithCurrentManager = choices
        ? choices.find(choice => choice.id === manager_id)
            ? choices
            : [...choices, currentManager]
        : [];

    if (manager_id === undefined || manager_id === null) {
        choicesWithCurrentManager = [choices];
    }

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
                    label="Tên trưởng điểm"
                    suggestionLimit={10}
                    openText='Mở'
                    source='manager_id'
                    onChange={(event, value) => {
                        setSaveSelected(value);
                        saveSelected1 = value;
                    }}
                    noOptionsText='Không tìm thấy trưởng điểm'
                    choices={choicesWithCurrentManager}
                    optionText="name"
                    disabled={isLoadingChoices}
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
    const record = useRecordContext();
    if (info.name !== '' || info.reference !== '') {
        return (
            <>
                <TextField
                    disabled={true}
                    label="Tên trưởng điểm"
                    variant="filled"
                    value={info.name}
                />
                <TextField
                    disabled={true}
                    label="Mã trưởng điểm"
                    variant="filled"
                    value={info.reference}
                />
            </>
        );
    }
    else if (record.manager_id !== undefined) {
        return (<FetchInfo id={record.manager_id} />);
    }
}

const FetchInfo = (id: any) => {
    const { data: info, isLoading, error } = useGetMany<any>(
        'managerAccounts',
        { ids: [id.id] }
    );
    if (isLoading) {
        return (
            <>
                <TextField
                    disabled={true}
                    label="Tên trưởng điểm"
                    variant="filled"
                    value={'Đang tải...'}
                />
                <TextField
                    disabled={true}
                    label="Mã trưởng điểm"
                    variant="filled"
                    value={'Đang tải...'}
                />
            </>
        );
    }
    else {
        return (
            <>
                <TextField
                    disabled={true}
                    label="Tên trưởng điểm"
                    variant="filled"
                    value={info === undefined ? '' : info.map(i => (
                        i.name
                    ))}
                />
                <TextField
                    disabled={true}
                    label="Mã trưởng điểm"
                    variant="filled"
                    value={info === undefined ? '' : info.map(i => (
                        i.reference
                    ))}
                />
            </>
        );
    }
}


const PointInfo = (info: any) => {
    const record = useRecordContext();
    if (info.name !== '' || info.reference !== '') {
        return (
            <>
                <TextField
                    disabled={true}
                    label="Tên điểm tập kết"
                    variant="filled"
                    value={info.name}
                />
                <TextField
                    disabled={true}
                    label="Mã điểm tập kết"
                    variant="filled"
                    value={info.reference}
                />
            </>
        );
    }
    else if (record.link_point_id !== undefined) {
        return (<FetchPointInfo id={record.link_point_id} />);
    }
}

const FetchPointInfo = (id: any) => {
    const { data: info, isLoading, error } = useGetMany<any>(
        'points',
        { ids: [id.id] }
    );
    if (isLoading) {
        return (
            <>
                <TextField
                    disabled={true}
                    label="Tên điểm tập kết"
                    variant="filled"
                    value={'Đang tải...'}
                />
                <TextField
                    disabled={true}
                    label="Mã điểm tập kết"
                    variant="filled"
                    value={'Đang tải...'}
                />
            </>
        );
    }
    else {
        return (
            <>
                <TextField
                    disabled={true}
                    label="Tên điểm tập kết"
                    variant="filled"
                    value={info === undefined ? '' : info.map(i => (
                        i.name
                    ))}
                />
                <TextField
                    disabled={true}
                    label="Mã điểm tập kết"
                    variant="filled"
                    value={info === undefined ? '' : info.map(i => (
                        i.reference
                    ))}
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

const PointTitle = () => <PointNameField sx={{ fontSize: '1.1rem' }} />;

export default PointEdit;
