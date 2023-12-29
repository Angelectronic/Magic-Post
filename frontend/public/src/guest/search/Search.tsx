import * as React from 'react';
import {
    TextField,
    Box,
    Button,
    CircularProgress,
} from '@mui/material';
import { useTranslate, Title, useNotify, useDataProvider } from 'react-admin';
import SearchIcon from './SearchIcon'

const Search = () => {
    const [loading, setLoading] = React.useState(false);
    const [billID, setBillID] = React.useState('');
    const notify = useNotify();
    const translate = useTranslate();
    const data = useDataProvider();
    
    const handleClick = async () => {
        var errorString = '';
        setLoading(true);
        try {
            const test = await data.getList(
                'invoices',
                {
                    filter: {},
                    pagination: { page: 1, perPage: 10 },
                    sort: { field: 'date', order: 'DESC' }
                }
            );
        } catch (error) {
            errorString = translate('resources.search_message.error');
        }
        setLoading(false);

        if (errorString !== '') {
            notify(
                errorString,
                {
                    type: 'error',
                    messageArgs: {
                        _:
                            errorString,
                    },
                }
            );
        }
    };
    return (
        <Box
            sx={{
                '& > :not(button):not(style)': { m: 1, width: '100ch' },
                '& > :is(button):not(style)': {
                    m: 1,
                },
            }}
        >
            <TextField
                autoFocus
                id="bill_id"
                label="Mã bưu gửi"
                variant="outlined"
                value={billID}
                helperText="Mã bưu gửi (tra nhiều bill thêm dấu phẩy giữa các bill VD: EB125966888VN, EI125556888VN)"
                onChange={(event: React.ChangeEvent<HTMLInputElement>) => {
                    setBillID(event.target.value);
                }}
            />
            <Button
                variant="contained"
                endIcon={loading ? (<CircularProgress size={15} thickness={2} />) : (<SearchIcon />)}
                onClick={handleClick}
                disabled={loading}
            >
                Tra cứu
            </Button>
        </Box>
    );
};

export default Search;
