import * as React from 'react';
import {
    Create,
    SimpleForm,
    TextInput,
    email,
    DateInput,
    PasswordInput,
    SelectInput,
    required,
} from 'react-admin';
import { Box, Typography } from '@mui/material';

export const validateForm = (
    values: Record<string, any>
): Record<string, any> => {
    const errors = {} as any;
    if (!values.name) {
        errors.name = 'ra.validation.required';
    }
    if (!values.sex) {
        errors.sex = 'ra.validation.required';
    }
    if (!values.username) {
        errors.username = 'ra.validation.required';
    }
    if (!values.email) {
        errors.email = 'ra.validation.required';
    } else {
        const error = email()(values.email);
        if (error) {
            errors.email = error;
        }
    }
    if (values.password && values.password !== values.confirm_password) {
        errors.confirm_password =
            'resources.customers.errors.password_mismatch';
    }
    return errors;
};

const EmployeeAccountCreate = () => (
    <Create
        sx={{
            [`& .RaCreate-main`]: {
                width: '45%',
            },
        }}
        title='Tạo tài khoản nhân viên giao dịch'
        redirect='../'
    >
        <SimpleForm
            sx={{ maxWidth: 500 }}
            defaultValues={{
                name: '',
                email: '',
                username: '',
                password: '12345678',
                confirm_password: '12345678',
                birthday: new Date(),
                create_date: new Date(),
            }}
            validate={validateForm}
        >
            <SectionTitle label="Thông tin chung" />
            <Box display={{ xs: 'block', sm: 'flex', width: '100%' }}>
                <Box flex={1} mr={{ xs: 0, sm: '1em' }}>
                    <TextInput autoFocus label="Tên" source="name" isRequired fullWidth />
                </Box>
                <Box flex={1} mr={{ xs: 0, sm: '0.2em' }}>
                    <SelectInput
                        label="Giới tính"
                        source="sex"
                        choices={[
                            { id: 'male', name: 'Nam' },
                            { id: 'female', name: 'Nữ' },
                        ]}
                        validate={required()}
                    />
                </Box>
            </Box>
            <TextInput type="email" source="email" isRequired fullWidth />
            <DateInput label="Ngày sinh" source="birthday" />
            <Separator />
            <SectionTitle label="Tài khoản" />
            <TextInput label="Tên đăng nhập" source="username" isRequired fullWidth />
            <Box display={{ xs: 'block', sm: 'flex' }}>
                <Box flex={1} mr={{ xs: 0, sm: '0.5em' }}>
                    <PasswordInput label="Mật khẩu" source="password" isRequired fullWidth helperText={'Mật khẩu mặc định là 12345678'}/>
                </Box>
                <Box flex={1} ml={{ xs: 0, sm: '0.5em' }}>
                    <PasswordInput label="Nhập lại mật khẩu" source="confirm_password" isRequired fullWidth />
                </Box>
            </Box>
        </SimpleForm>
    </Create>
);

const SectionTitle = ({ label }: { label: string }) => {

    return (
        <Typography variant="h6" gutterBottom>
            {label}
        </Typography>
    );
};

const Separator = () => <Box pt="1em" />;

export default EmployeeAccountCreate;
