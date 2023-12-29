import * as React from 'react';
import {
    Box,
    Button,
    List,
    ListItem,
    ListItemText,
    Typography,
} from '@mui/material';
import ListAltIcon from '@mui/icons-material/ListAlt';
import { Link } from 'react-router-dom';

import {
    useGetList,
} from 'react-admin';

import { stringify } from 'query-string';

import CardWithIconTo from './CardWithIconTo';
import InventoryIcon from '@mui/icons-material/Inventory';
const OldPackage = () => {
    const { data: pacs, isLoading: isLoading } = useGetList('exchangingPackage', {
        filter: { status: "received" },
        sort: { field: 'send_date', order: 'ASC' },
        pagination: { page: 1, perPage: 10 },
    });

    const display = isLoading ? 'none' : 'block';

    return (
        <CardWithIconTo
            to={{
                pathname: '/exchangingPackage',
                search: stringify({
                    filter: JSON.stringify({ status: 'received' }),
                }),
            }}
            icon={InventoryIcon}
            title={'Hàng tồn kho lâu'}
            subtitle={""}
        >
            <List sx={{ display }}>
                {pacs?.map((pac: any) => (
                    <ListItem
                        key={pac.package_id}
                        component={Link}
                        to={{
                            pathname: "/exchangingPackage",
                            search: stringify({
                                filter: JSON.stringify({ package_id_q: pac.package_id, status: 'received' }),
                            }),
                        }}
                        alignItems="flex-start"
                    >
                        <ListItemText
                            secondary={`${pac.package_id}: Hàng được gửi từ ${(new Date(pac.send_date)).toLocaleDateString()}`}
                            sx={{
                                overflowY: 'hidden',
                                height: '4em',
                                display: '-webkit-box',
                                WebkitLineClamp: 2,
                                WebkitBoxOrient: 'vertical',
                                paddingRight: 0,
                            }}
                        />
                    </ListItem>
                ))}
            </List>
            <Box flexGrow={1}>&nbsp;</Box>
            <Button
                sx={{ borderRadius: 0 }}
                component={Link}
                to="/exchangingPackage"
                size="small"
                color="primary"
            >
                <Box p={1} sx={{ color: 'primary.main' }}>
                    Xem tất cả hàng
                </Box>
            </Button>
        </CardWithIconTo>
    );
};

export default OldPackage;
