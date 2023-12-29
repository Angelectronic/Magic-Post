import Button from '@mui/material/Button';

import {
    useRecordContext,
} from 'react-admin';
import { useNavigate } from "react-router-dom";
import BuildCircleOutlinedIcon from '@mui/icons-material/BuildCircleOutlined';


const EditPackageButton = () => {
    const record = useRecordContext();
    let navigate = useNavigate();
    const handleClick = () => {
        navigate(record.id.toString());
    };
    return record && (record.status === 'received' || record.status === 'Đã nhận') &&
        (record.save_dest === 'exchanging') ? (
        <Button
            variant="contained"
            color="primary"
            size="small"
            onClick={handleClick}
            startIcon={
                <BuildCircleOutlinedIcon />
            }
        >
            Sửa
        </Button>
    ) : null;
}

export default EditPackageButton;