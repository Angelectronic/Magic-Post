import { Button } from '@mui/material';

import {
    useRecordContext,
} from 'react-admin';
import { Link } from 'react-router-dom';
import SearchOutlinedIcon from '@mui/icons-material/SearchOutlined';
import { stringify } from 'query-string';

const ViewReceiptButton = () => {
    const record = useRecordContext();
    return (

        <Button
            variant='outlined'
            size="small"
            color="primary"
            component={Link}
            startIcon={
                <SearchOutlinedIcon />
            }
            to={{
                pathname: '/exchangingDelivery',
                search: stringify({
                    filter: JSON.stringify({ package_id_q: record.package_id }),
                }),
            }}
            state={{ _scrollToTop: true }}
        >
            Xem đơn hàng
        </Button>

    );

}

export default ViewReceiptButton;