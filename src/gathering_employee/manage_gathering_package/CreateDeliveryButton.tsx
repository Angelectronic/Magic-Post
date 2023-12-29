import Button from '@mui/material/Button';

import {
    useRecordContext,
    useRedirect,
} from 'react-admin';
import ListAltOutlinedIcon from '@mui/icons-material/ListAltOutlined';


const CreateDeliveryButton = () => {
    const record = useRecordContext();
    const redirect = useRedirect();

    const handleClick = () => {
        localStorage.setItem("current_package_id", record.package_id);
        localStorage.setItem("current_receive_name", record.receive_name);
        localStorage.setItem("current_receive_phone", record.receive_phone);
        localStorage.setItem("current_receive_address", record.receive_address);
        localStorage.setItem("current_exchanging_receiving_point_id", record.receive_point_id)
        console.log(record.package_id);
        console.log(record.receive_name);
        console.log(record.receive_phone);
        console.log(record.receive_address);
        redirect("/exchangingDelivery/create");
    }

    return record && (record.status === 'received' || record.status === 'Đã nhận') &&
        (record.save_dest === 'gathering' && record.dest_point_id === localStorage.getItem("current_point_id")) ? (
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