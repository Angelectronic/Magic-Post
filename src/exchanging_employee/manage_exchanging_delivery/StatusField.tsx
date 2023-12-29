import Button from '@mui/material/Button';

import {
    useRecordContext,
    TextField,
} from 'react-admin';


const StatusField = () => {
    const record = useRecordContext();
    if (record.status === 'resolved') {
        record.status1 = 'Đã xử lý';
        return (
            <TextField source='status1' sx={{ color: 'green' }} />
        );
    }
    else if (record.status === 'not-resolved') {
        record.status1 = 'Đang xử lý';
        return (
            <TextField source='status1' sx={{ color: 'orange' }} />
        );
    }
    return (<span></span>);
}

export default StatusField;