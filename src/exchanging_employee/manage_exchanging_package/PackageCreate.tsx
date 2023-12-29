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
    RadioButtonGroupInput,
    NumberInput,
    ArrayInput,
    DateInput,
    SimpleFormIterator,
} from 'react-admin';
import { Box, Typography, TextField } from '@mui/material';
import { useWatch } from 'react-hook-form';

export const validateForm = (
    values: Record<string, any>
): Record<string, any> => {
    const errors = {} as any;
    if (!values.send_name) {
        errors.send_name = 'ra.validation.required';
    }
    if (!values.send_phone) {
        errors.send_phone = 'ra.validation.required';
    }
    if (!values.send_address) {
        errors.send_address = 'ra.validation.required';
    }
    if (!values.send_point_id) {
        errors.send_point_id = 'ra.validation.required';
    }
    if (!values.send_date) {
        errors.send_date = 'ra.validation.required';
    }
    if (!values.receive_name) {
        errors.receive_name = 'ra.validation.required';
    }
    if (!values.receive_phone) {
        errors.receive_phone = 'ra.validation.required';
    }
    if (!values.receive_address) {
        errors.receive_address = 'ra.validation.required';
    }
    if (!values.receive_point_id) {
        errors.receive_point_id = 'ra.validation.required';
    }
    return errors;
};

const PackageCreate = () => {

    const transform = (data: any) => {
        if (data.send_date !== null && data.send_date !== undefined) {
            data.send_date = data.send_date.toLocaleDateString("en-CA");
        }
        return data;
    }

    return (
        <Create
            sx={{
                [`& .RaCreate-main`]: {
                    width: '100%',
                },
            }}
            title='Ghi nhận hàng mới'
            redirect="../"
            transform={transform}
        >

            <TabbedForm
                validate={validateForm}
                defaultValues={{
                    send_name: '',
                    send_address: '',
                    send_phone: '',
                    send_point_id: '',
                    send_date: new Date(),
                    receive_name: '',
                    receive_address: '',
                    receive_phone: '',
                    receive_point_id: '',
                    weight: 0,
                    items: [],
                    special_service: '',
                    main_cost: 0,
                    other_cost: 0,
                    gtgt_cost: 0,
                    other_service_cost: 0,
                    total_cost: 0,
                    total_every_cost: 0,
                    vat: 10,
                    note: '',
                    cod: 0,
                    receive_other_cost: 0,
                    package_type: 0,
                    instruction_type: 0,
                    gdv: localStorage.getItem("name"),
                    current_dest: 'exchanging',
                    status: 'received',
                }}>
                <TabbedForm.Tab
                    label="Thông tin chung"
                    sx={{ maxWidth: '80em' }}
                >
                    <GeneralFormData />
                </TabbedForm.Tab>
                <TabbedForm.Tab
                    label="Thông tin hàng"
                    sx={{ maxWidth: '80em' }}
                >
                    <PackageFormData />
                </TabbedForm.Tab>
                <TabbedForm.Tab
                    label="Thông tin phí"
                    sx={{ maxWidth: '80em' }}
                >
                    <CostFormData />
                </TabbedForm.Tab>
            </TabbedForm>

        </Create>
    );
}

const GeneralFormData = () => {

    return (
        <>
            <SectionTitle label="Thông tin người gửi" />
            <Box display={{ xs: 'block', sm: 'flex', width: '50%' }}>
                <Box flex={1.5} mr={{ xs: 0, sm: '1em' }}>
                    <TextInput label="Tên người gửi" source="send_name" isRequired fullWidth />
                </Box>
                <Box flex={1} mr={{ xs: 1, sm: '2em' }}>
                    <TextInput label="SĐT" source="send_phone" isRequired fullWidth />
                </Box>
            </Box>
            <Box display={{ width: '50%' }}>
                <TextInput
                    source="send_address"
                    multiline
                    fullWidth
                    helperText={false}
                    label='Địa chỉ người gửi'
                    isRequired
                />
            </Box>

            <Box display={{ xs: 'block', sm: 'flex', width: '50%' }}>
                <Box flex={0.5} mr={{ xs: 0, sm: '1em' }}>
                    <TextInput
                        source="send_point_id"
                        fullWidth
                        helperText={false}
                        label='Mã bưu chính'
                        isRequired
                    />
                </Box>
                <Box flex={1} mr={{ xs: 1, sm: '2em' }}>
                    <DateInput label="Ngày gửi" source="send_date" isRequired />
                </Box>
            </Box>
            <SectionTitle label="Thông tin người nhận" />
            <Box display={{ xs: 'block', sm: 'flex', width: '50%' }}>
                <Box flex={1.5} mr={{ xs: 0, sm: '1em' }}>
                    <TextInput label="Tên người nhận" source="receive_name" isRequired fullWidth />
                </Box>
                <Box flex={1} mr={{ xs: 1, sm: '2em' }}>
                    <TextInput label="SĐT" source="receive_phone" isRequired fullWidth />
                </Box>
            </Box>
            <Box display={{ width: '50%' }}>
                <TextInput
                    source="receive_address"
                    multiline
                    fullWidth
                    helperText={false}
                    label='Địa chỉ người nhận'
                    isRequired
                />
            </Box>
            <Box display={{ width: '15%' }}>
                <TextInput
                    source="receive_point_id"
                    fullWidth
                    helperText={false}
                    label='Mã bưu chính'
                    isRequired
                />
            </Box>

        </>
    );
}

const CostFormData = () => {

    return (
        <>
            <Box display={{ xs: 'block', sm: 'flex', width: '50%' }}>
                <Box flex={1.5} mr={{ xs: 0, sm: '1em' }}>
                    <SectionTitle label="Cước" />
                    <Box display={{ width: '70%' }}>
                        <NumberInput
                            source="main_cost"
                            fullWidth
                            helperText={false}
                            label='Cước chính (VND)'
                            min={0}
                        />
                    </Box>
                    <Box display={{ width: '70%' }}>
                        <NumberInput
                            source="other_cost"
                            fullWidth
                            helperText={false}
                            label='Phụ phí (VND)'
                            min={0}
                        />
                    </Box>
                    <Box display={{ width: '70%' }}>
                        <NumberInput
                            source="gtgt_cost"
                            fullWidth
                            helperText={false}
                            label='Cước GTGT (VND)'
                            min={0}
                        />
                    </Box>
                    <Box display={{ width: '70%' }}>
                        <NumberInput
                            source="vat"
                            fullWidth
                            helperText={false}
                            label='VAT (%)'
                            min={0}
                            max={100}
                        />
                    </Box>
                    <TotalCostField />
                    <Box display={{ width: '70%' }}>
                        <NumberInput
                            source="other_service_cost"
                            fullWidth
                            helperText={false}
                            label='Thu khác'
                            min={0}
                        />
                    </Box>
                    <EveryCostField />
                </Box>
                <Box flex={1.5} mr={{ xs: 0, sm: '1em' }}>
                    <SectionTitle label="Thu của người nhận" />
                    <Box display={{ width: '70%' }}>
                        <NumberInput
                            source="cod"
                            fullWidth
                            helperText={false}
                            label='COD (VND)'
                            min={0}
                        />
                    </Box>
                    <Box display={{ width: '70%' }}>
                        <NumberInput
                            source="receive_other_cost"
                            fullWidth
                            helperText={false}
                            label='Thu khác'
                            min={0}
                        />
                    </Box>
                    <TotalReceiveCostField />
                </Box>
            </Box>

        </>
    );
}

const TotalCostField = () => {
    var vat = useWatch({ name: 'vat' });
    var main_cost = useWatch({ name: 'main_cost' });
    var other_cost = useWatch({ name: 'other_cost' });
    var gtgt_cost = useWatch({ name: 'gtgt_cost' });
    var total = (main_cost + other_cost + gtgt_cost) * (vat + 100) / 100

    return (
        <Box display={{ width: '70%' }}>
            <TextField
                disabled={true}
                label="Tổng thu"
                variant="filled"
                value={total.toFixed(0)}
            />
        </Box>
    );
}

const EveryCostField = () => {
    var vat = useWatch({ name: 'vat' });
    var main_cost = useWatch({ name: 'main_cost' });
    var other_cost = useWatch({ name: 'other_cost' });
    var gtgt_cost = useWatch({ name: 'gtgt_cost' });
    var other_service_cost = useWatch({ name: 'other_service_cost' });
    var total = (main_cost + other_cost + gtgt_cost) * (vat + 100) / 100
    total += other_service_cost;
    return (
        <Box display={{ width: '70%' }}>
            <TextField
                disabled={true}
                label="Tổng cước (gồm VAT)"
                variant="filled"
                value={total.toFixed(0)}
            />
        </Box>
    );
}

const TotalReceiveCostField = () => {
    var cod = useWatch({ name: 'cod' });
    var receive_other_cost = useWatch({ name: 'receive_other_cost' });
    var total = (cod + receive_other_cost);
    return (
        <Box display={{ width: '70%' }}>
            <TextField
                disabled={true}
                label="Tổng thu"
                variant="filled"
                value={total.toFixed(0)}
            />
        </Box>
    );
}


const PackageFormData = () => {

    return (
        <>
            <SectionTitle label="Loại hàng gửi" />
            <RadioButtonGroupInput label=''
                source="package_type"
                choices={[
                    { id: '1', name: 'Tài liệu' },
                    { id: '2', name: 'Hàng hóa' },
                ]} />
            <SectionTitle label="Chỉ dẫn của người gửi khi không phát được bưu gửi" />
            <RadioButtonGroupInput label=''
                source="instruction_type"
                choices={[
                    { id: '1', name: 'Chuyển hoàn ngay' },
                    { id: '2', name: 'Gọi điện cho người gửi/BC gửi' },
                    { id: '3', name: 'Hủy' },
                    { id: '4', name: 'Chuyển hoàn trước ngày' },
                    { id: '5', name: 'Chuyển hoàn khi hết thời gian lưu trữ' },
                ]} />
            <SectionTitle label="Hàng" />
            <Box display={{ width: '15%' }}>
                <NumberInput
                    source="weight"
                    fullWidth
                    helperText={false}
                    label='Khối lượng hàng (kg)'
                    min={0}
                />

            </Box>
            <ArrayInput source="items" label='Vật phẩm'>
                <SimpleFormIterator inline disableReordering>
                    <TextInput
                        label='Tên vật phẩm'
                        source="item_name"
                        helperText={false}

                    />
                    <NumberInput label='Số lượng'
                        source="quantity"
                        min={1}
                        helperText={false}
                        sx={{
                            width: '20%',
                        }}
                    />
                    <NumberInput
                        label='Giá trị (VND)'
                        source="value"
                        min={0}
                        helperText={false}

                    />
                </SimpleFormIterator>
            </ArrayInput>

            <SectionTitle label="Khác" />
            <Box display={{ width: '70%' }}>
                <TextInput
                    source="special_service"
                    multiline
                    fullWidth
                    helperText={false}
                    label='Dịch vụ đặc biệt/Cộng thêm'
                />
            </Box>
            <SectionTitle label='Chú dẫn nghiệp vụ' />
            <Box display={{ width: '50%' }}>
                <TextInput
                    source="note"
                    multiline
                    fullWidth
                    helperText={false}
                    label='Chú dẫn'
                />
            </Box>
        </>
    );
}


const SectionTitle = ({ label }: { label: string }) => {

    return (
        <Typography variant="h6" gutterBottom>
            {label}
        </Typography>
    );
};

const Separator = () => <Box pt="1em" />;

export default PackageCreate;
