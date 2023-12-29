import * as React from 'react';
import { SxProps, Typography } from '@mui/material';
import { memo } from 'react';

import { FieldProps, useRecordContext } from 'react-admin';
import AvatarField from './AvatarField';
import { Account } from '../types';

interface Props extends FieldProps<Account> {
    size?: string;
    sx?: SxProps;
}

const FullNameField = (props: Props) => {
    const record = useRecordContext<Account>();
    return record ? (
        <Typography
            variant="body2"
            display="flex"
            flexWrap="nowrap"
            alignItems="center"
            component="div"
            sx={props.sx}
        >
            {record.name}
        </Typography>
    ) : null;
};

FullNameField.defaultProps = {
    source: 'last_name' as const,
    label: 'resources.account.fields.name',
};

export default memo<Props>(FullNameField);
