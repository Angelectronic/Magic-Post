import Button from '@mui/material/Button';

import {
    useRecordContext,
} from 'react-admin';
import CloseOutlinedIcon from '@mui/icons-material/CloseOutlined';

const RejectButton = () => {
    const record = useRecordContext();
    return record && ( (record.status === 'in-transit' || record.status === 'Đang chuyển')
        && (record.save_dest === 'gathering') && (record.dest_point_id === localStorage.getItem("current_point_id"))
    ) ? (
        <Button
            variant="contained"
            color="error"
            size="small"
            startIcon={
                <CloseOutlinedIcon />
            }
        >
            Không nhận được
        </Button>
    ) : null;
}

export default RejectButton;