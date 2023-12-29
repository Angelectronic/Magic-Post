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

const RecentDelivery = () => {
    const { data: delivery, isLoading: isLoading } = useGetList('exchangingDelivery', {
        filter: { status: "not-resolved" },
        sort: { field: 'create_date', order: 'DESC' },
        pagination: { page: 1, perPage: 10 },
    });

    const display = isLoading ? 'none' : 'block';

    return (
        <CardWithIconTo
            to={{
                pathname: '/exchangingDelivery',
                search: stringify({
                    filter: JSON.stringify({ status: 'not-resolved' }),
                }),
            }}
            icon={ListAltIcon}
            title={'Đơn hàng mới đến'}
            subtitle={""}
        >
            <List sx={{ display }}>
                {delivery?.map((dev: any) => (
                    <ListItem
                        key={dev.delivery_id}
                        component={Link}
                        to={{
                            pathname: "/exchangingDelivery",
                            search: stringify({
                                filter: JSON.stringify({ delivery_id_q: dev.delivery_id, status: 'not-resolved' }),
                            }),
                        }}
                        alignItems="flex-start"
                    >
                        <ListItemText
                            secondary={`${(new Date(dev.create_date)).toLocaleDateString()}: Đơn hàng từ ${dev.from_point_id}`}
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
                to="/exchangingDelivery"
                size="small"
                color="primary"
            >
                <Box p={1} sx={{ color: 'primary.main' }}>
                    Xem tất cả đơn hàng
                </Box>
            </Button>
        </CardWithIconTo>
    );
};

export default RecentDelivery;
