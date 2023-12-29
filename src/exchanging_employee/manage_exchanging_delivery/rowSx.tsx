import green from '@mui/material/colors/green';
import type { SxProps } from '@mui/material';
import orange from '@mui/material/colors/orange';
import red from '@mui/material/colors/red';
import { Identifier } from 'react-admin';

const rowSx = (selectedRow?: Identifier) => (record: any): SxProps => {
    let style = {};
    if (!record) {
        return style;
    }
    if (selectedRow !== undefined && selectedRow !== null && selectedRow === record.id) {
        style = {
            ...style,
            backgroundColor: 'action.selected',
        };
    }
    if (record.status === 'resolved') {
        if (record.is_delivered === 1) {
            return {
                ...style,
                borderLeftColor: green[500],
                borderLeftWidth: 5,
                borderLeftStyle: 'solid',
            };
        }
        else {
            return {
                ...style,
                borderLeftColor: red[500],
                borderLeftWidth: 5,
                borderLeftStyle: 'solid',
            };
        }
    }
    if (record.status === 'not-resolved')
        return {
            ...style,
            borderLeftColor: orange[500],
            borderLeftWidth: 5,
            borderLeftStyle: 'solid',
        };
    return {
        ...style,
        borderLeftColor: 'transparent',
        borderLeftWidth: 5,
        borderLeftStyle: 'solid',
    };

};
export default rowSx;
