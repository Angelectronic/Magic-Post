import * as React from 'react';
import { useRecordContext, TextField, TextFieldProps } from 'react-admin';

const ColoredStateField = (props: TextFieldProps) => {
    const record = useRecordContext(props);
    if (!record || !props.source) {
        return null;
    }
    if (record[props.source] === 'in-transit') {
        record[props.source] = 'Đang chuyển';
        return (
            <TextField {...props} sx={{ color: 'orange' }} />
        );
    }
    else if (record[props.source] === 'received') {
        record[props.source] = 'Đã nhận';
        return (
            <TextField {...props} sx={{ color: 'green' }} />
        );
    }
    else if (record[props.source] === 'not-received') {
        record[props.source] = 'Không nhận được';
        return (
            <TextField {...props} sx={{ color: 'red' }} />
        );
    }
    return (
        <TextField {...props} />
    );
};

export default ColoredStateField;
