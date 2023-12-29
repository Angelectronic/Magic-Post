import * as React from 'react';
import {
    Create,
    SimpleForm,
    TextInput,
    DateInput,
    SelectInput,
    useRedirect,
    useNotify,
    required,
    useGetList,
    AutocompleteInput,
} from 'react-admin';
import { Box, Typography, TextField } from '@mui/material';
import { useWatch } from 'react-hook-form';
import { SpeedSharp } from '@mui/icons-material';

export const validateForm = (
    values: Record<string, any>
): Record<string, any> => {
    const errors = {} as any;
    if (!values.package_id) {
        errors.package_id = 'ra.validation.required';
    }
    if (!values.create_date) {
        errors.create_date = 'ra.validation.required';
    }
    if (!values.send_date) {
        errors.send_date = 'ra.validation.required';
    }
    if (!values.expected_date) {
        errors.expected_date = 'ra.validation.required';
    }
    if (!values.current_from) {
        errors.current_from = 'ra.validation.required';
    }
    if (!values.from_point_id) {
        errors.from_point_id = 'ra.validation.required';
    }
    if (!values.dest_point_id) {
        errors.dest_point_id = 'ra.validation.required';
    }
    if (!values.current_dest) {
        errors.current_dest = 'ra.validation.required';
    }
    else if (values.current_dest === 'gathering' && values.current_dest !== 'exchanging') {
        errors.current_dest = 'Điểm đến không hợp lệ';
    }
    return errors;
};

const DeliveryCreate = () => {
    const notify = useNotify();
    const redirect = useRedirect();
    const transform = (data: any) => {
        if (data.send_date !== null && data.send_date !== undefined) {
            data.send_date = data.send_date.toLocaleDateString("en-CA");
        }
        return data;
    }
    const onSuccess = (data: any) => {
        localStorage.removeItem("current_package_id");
        localStorage.removeItem("current_exchanging_receiving_point_id");
        localStorage.removeItem("current_receive_name");
        localStorage.removeItem("current_receive_phone");
        localStorage.removeItem("current_receive_address");
        notify(`Tạo đơn hàng thành công. Mã đơn hàng: ${data.delivery_id}`);
        redirect('/gatheringDelivery');
    }
    return (
        <Create
            sx={{
                [`& .RaCreate-main`]: {
                    width: '50%',
                },
            }}
            title='Ghi nhận đơn hàng mới'
            redirect="../"
            transform={transform}
            mutationOptions={{ onSuccess }}

        >

            <SimpleForm
                validate={validateForm}
                defaultValues={{
                    package_id: localStorage.getItem('current_package_id'),
                    create_date: new Date(),
                    send_date: new Date(),
                    expected_date: new Date(),
                    current_dest: 'gathering',
                    current_from: 'gathering',
                    from_point_id: localStorage.getItem('current_point_id') === null ? "" : localStorage.getItem('current_point_id'),
                    dest_point_id: localStorage.getItem('current_exchanging_receiving_point_id') === null ? "" : localStorage.getItem('current_exchanging_receiving_point_id'),
                    receive_name: localStorage.getItem('current_receive_name') === null ? "" : localStorage.getItem('current_receive_name'),
                    receive_phone: localStorage.getItem('current_receive_phone') === null ? "" : localStorage.getItem('current_receive_phone'),
                    receive_address: localStorage.getItem('current_receive_address') === null ? "" : localStorage.getItem('current_receive_address'),
                }}>
                <SectionTitle label="Thông tin chung" />
                <Box display={{ xs: 'block', sm: 'flex', width: '40%' }}>
                    <TextInput label="Mã hàng" source="package_id" isRequired fullWidth disabled />
                </Box>

                <Box display={{ xs: 'block', sm: 'flex', width: '50%' }}>
                    <Box flex={0.5} mr={{ xs: 0, sm: '1em' }}>
                        <DateInput label="Ngày tạo" source="create_date" isRequired disabled />
                    </Box>
                    <Box flex={1} mr={{ xs: 1, sm: '2em' }}>
                        <DateInput label="Ngày gửi" source="send_date" isRequired />
                    </Box>
                    <Box flex={2.5} mr={{ xs: 1, sm: '2em' }}>
                        <DateInput label="Dự kiến" source="expected_date" isRequired />
                    </Box>
                </Box>
                <Box display={{ xs: 'block', sm: 'flex', width: '50%' }}>
                    <SendFromField />
                </Box>
                <Separator />
                <SelectInput
                    label="Chuyển đến"
                    source="current_dest"
                    choices={[
                        { id: 'gathering', name: 'Điểm tập kết' },
                        { id: 'exchanging', name: 'Điểm giao dịch' },
                    ]}
                    validate={required()}
                />
                <SendToField />
                <Separator />
                <Box display={{ width: '100%' }}>
                    <TextInput
                        source="note"
                        multiline
                        fullWidth
                        helperText={false}
                        label='Ghi chú'
                    />
                </Box>
            </SimpleForm>

        </Create>
    );
}

const SendFromField = () => {
    var current_from = useWatch({ name: 'current_from' });
    var from_point_id = useWatch({ name: 'from_point_id' });
    var name = "";
    if (current_from === 'gathering') {
        name = "Điểm tập kết (" + from_point_id + ")";
    }
    return (
        <TextField
            disabled={true}
            label="Chuyển từ"
            variant="filled"
            value={name}
            required={true}
        />
    );
}



const SendToField = () => {
    var current_dest = useWatch({ name: 'current_dest' });
    const [filter, setFilter] = React.useState({ reference_q: '' });

    const { data: choices_gather, isLoading: isLoadingChoices } = useGetList('getAllGatheringPoints', { filter });
    const { data: choices_exchange, isLoading: isLoadingChoices1 } = useGetList('getAllExchangingPoints', { filter });
    if (current_dest === 'gathering') {
        return (
            <Box display={{ xs: 'block', sm: 'flex' }}>
                <AutocompleteInput
                    autoFocus
                    label="Mã điểm tập kết"
                    suggestionLimit={10}
                    openText='Mở'
                    source='dest_point_id'
                    noOptionsText='Không tìm thấy điểm tập kết'
                    choices={choices_gather}
                    optionText="reference"
                    disabled={isLoadingChoices}
                />
            </Box>
        );
    }
    else if (current_dest === 'exchanging') {
        return (
            <Box display={{ xs: 'block', sm: 'flex' }}>
                <AutocompleteInput
                    autoFocus
                    label="Mã điểm giao dịch"
                    suggestionLimit={10}
                    openText='Mở'
                    source='dest_point_id'
                    noOptionsText='Không tìm thấy điểm tập kết'
                    choices={choices_exchange}
                    optionText="reference"
                    disabled={isLoadingChoices1}
                />
            </Box>
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

export default DeliveryCreate;
