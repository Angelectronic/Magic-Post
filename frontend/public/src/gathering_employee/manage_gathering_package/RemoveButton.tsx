import Button from '@mui/material/Button';

import {
    useRecordContext,
} from 'react-admin';
import DeleteOutlineOutlinedIcon from '@mui/icons-material/DeleteOutlineOutlined';

const RemoveButton = () => {
    const record = useRecordContext();
    return record && ((record.status === 'not-received' || record.status === 'Không nhận được') 
    && (record.save_dest === 'gathering') && (record.dest_point_id === localStorage.getItem("current_point_id")))
        ? (
        <Button
            variant="contained"
            color="error"
            size="small"
            startIcon={
                <DeleteOutlineOutlinedIcon />
            }
        >
            Xóa
        </Button>
    ) : null;
}

export default RemoveButton;