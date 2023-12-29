import * as React from 'react';
import { SxProps, Typography } from '@mui/material';
import { memo } from 'react';

import { FieldProps, useRecordContext } from 'react-admin';
import { Point } from '../../types'

interface Props extends FieldProps<Point> {
    size?: string;
    sx?: SxProps;
}

const PointNameField = (props: Props) => {
    const { size } = props;
    const record = useRecordContext<Point>();
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

export default memo<Props>(PointNameField);
