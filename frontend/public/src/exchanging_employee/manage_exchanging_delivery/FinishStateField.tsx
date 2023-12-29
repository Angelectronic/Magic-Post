import Button from '@mui/material/Button';

import {
    useRecordContext,
} from 'react-admin';
import CheckIcon from '@mui/icons-material/Check';
import CloseIcon from '@mui/icons-material/Close';


const FinishStateField = () => {
    const record = useRecordContext();
    if (record.status === 'not-resolved') {
        return (
            <span></span>
        );
    }
    if (record.is_delivered) {
        return (
            <span>
                <CheckIcon color='success' />
            </span>
        );
    }
    else {
        return (
            <span>
                <CloseIcon color='error' />
            </span>
        );
    }
}

export default FinishStateField;