import Button from '@mui/material/Button';

import {
    useRecordContext,
    useRedirect,
} from 'react-admin';
import ListAltOutlinedIcon from '@mui/icons-material/ListAltOutlined';
import { selIds } from './PackageList';

const CreateDeliveryButton = () => {
    const record = useRecordContext();
    const redirect = useRedirect();

    const handleClick = () => {
        localStorage.setItem("current_package_id", record.package_id);
        localStorage.setItem("current_receive_name", record.receive_name);
        localStorage.setItem("current_receive_phone", record.receive_phone);
        localStorage.setItem("current_receive_address", record.receive_address);
        localStorage.setItem("current_delivery_mode", "1");
        console.log(record.package_id);
        console.log(record.receive_name);
        console.log(record.receive_phone);
        console.log(record.receive_address);
        redirect("/exchangingDelivery/create");
    }
    
    return record && (record.status === 'received' || record.status === 'Đã nhận' || record.status === 'not-received' || record.status === 'Không nhận được') ? (
        <Button
            variant="outlined"
            color="primary"
            size="small"
            startIcon={
                <ListAltOutlinedIcon />
            }
            onClick={handleClick}
        >
            Tạo đơn hàng
        </Button>
    ) : null;
}

export default CreateDeliveryButton;