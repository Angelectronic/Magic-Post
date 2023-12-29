import * as React from 'react';
import { useRecordContext, TextField, TextFieldProps } from 'react-admin';

const ColoredDestField = (props: TextFieldProps) => {
    const record = useRecordContext(props);
    if (!record || !props.source) {
        return null;
    }
    record.save_dest = record[props.source];
    if (record[props.source] === 'exchanging') {
        record[props.source] = 'Điểm giao dịch (' + record.dest_point_id + ')';

    }
    else if (record[props.source] === 'gathering') {
        record[props.source] = 'Điểm tập kết (' + record.dest_point_id + ')';
    }
    else if (record[props.source] === 'receiver') {
        record[props.source] = 'Người nhận';
    }
    
    return (
        <TextField {...props} sx={{ color: 'blue' }} />
    );
};

export default ColoredDestField;
