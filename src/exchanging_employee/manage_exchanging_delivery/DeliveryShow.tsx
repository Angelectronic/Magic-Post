import * as React from 'react';
import {
    ShowBase,
    TextField,
    DateField,
    SimpleForm,
    Labeled,
    useRecordContext,
    TextInput
} from 'react-admin';
import { Box, Grid, Stack, IconButton, Typography } from '@mui/material';
import CloseIcon from '@mui/icons-material/Close';
import ColoredFromField from './ColoredFromField';
import ColoredDestDetailField from './ColoredDestDetailField';

const DeliveryShow = ({ id, onCancel }: any) => {
    return (
        <ShowBase id={id} disableAuthentication>
            <Box pt={5} width={{ xs: '100vW', sm: 400 }} mt={{ xs: 2, sm: 1 }}>
                <Stack direction="row" p={2}>
                    <Typography variant="h6" flex="1">
                        Thông tin đơn hàng
                    </Typography>
                    <IconButton onClick={onCancel} size="small">
                        <CloseIcon />
                    </IconButton>
                </Stack>
                <SimpleForm
                    sx={{ pt: 0, pb: 0 }}
                    toolbar={<></>}
                >
                    <Grid container rowSpacing={1} mb={1}>
                        <Grid item xs={6}>
                            <Labeled>
                                <DateField label='Ngày tạo' source='create_date' />
                            </Labeled>
                        </Grid>
                        <Grid item xs={6}>
                            <Labeled>
                                <DateField label='Ngày gửi' source="send_date" />
                            </Labeled>
                        </Grid>
                        <Grid item xs={6}>
                            <Labeled>
                                <DateField label='Dự kiến' source="expected_date" />
                            </Labeled>
                        </Grid>
                        <ReceivedField />
                    </Grid>
                    <Separator />
                    
                    <Labeled>
                        <ColoredFromField label='Chuyển từ' source='current_from' />
                    </Labeled>
                    <ReceiverDetailField />
                    <TextInput
                        label='Ghi chú'
                        source="note"
                        maxRows={8}
                        multiline
                        fullWidth
                        disabled={true}
                    />
                </SimpleForm>
            </Box>
        </ShowBase>
    );
};

const ReceivedField = () => {
    const record = useRecordContext();
    if (record.is_delivered && record.status === 'resolved') {
        return (
            <Grid item xs={6}>
                <Labeled>
                    <DateField label='Ngày nhận' source="receive_date" />
                </Labeled>
            </Grid>
        );
    }
    record.receive_date123 = 'Chưa nhận'
    return (
        <Grid item xs={6}>
            <Labeled>
                <TextField label='Ngày nhận' source="receive_date123" />
            </Labeled>
        </Grid>
    );
}

const ReceiverDetailField = () => {
    const record = useRecordContext();
    if (record.current_dest === 'receiver') {
        return (
            <>
                <Typography variant="h6">
                    Người nhận hàng
                </Typography>
                <Labeled>
                    <TextField label='Tên' source="receive_name" />
                </Labeled>
                <Labeled>
                    <TextField label='SĐT' source="receive_phone" />
                </Labeled>
                <Labeled>
                    <TextField label='Địa chỉ' source="receive_address" />
                </Labeled>
            </>
        );
    }
    record.receive_date123 = 'Chưa nhận'
    return (
        <>
            <Labeled>
                <ColoredDestDetailField label='Chuyển đến' source='current_dest' />
            </Labeled>
        </>
    );
};
const Separator = () => <Box pt="1em" />;

export default DeliveryShow;
