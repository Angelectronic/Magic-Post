import * as React from 'react';
import { useRecordContext, TextField, TextFieldProps } from 'react-admin';

const ColoredDestDetailField = (props: TextFieldProps) => {
    const record = useRecordContext(props);
    if (!record || !props.source) {
        return null;
    }
    if (record[props.source] === 'exchanging') {
        record[props.source] = 'Điểm giao dịch (' + record.dest_point_id + ')';

    }
    else if (record[props.source] === 'gathering') {
        record[props.source] = 'Điểm tập kết (' + record.dest_point_id + ')';
    }
    else if (record[props.source] === 'receiver') {
        record[props.source] = 'Người nhận (' + record.receive_address + ')';
    }
    return (
        <TextField {...props} sx={{ color: 'blue' }} />
    );
};

export default ColoredDestDetailField;
