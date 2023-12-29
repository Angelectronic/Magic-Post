import * as React from 'react';
import { Card, CardContent } from '@mui/material';
import AccessTimeIcon from '@mui/icons-material/AccessTime';
import LocationOnOutlinedIcon from '@mui/icons-material/LocationOnOutlined';
import NotesOutlinedIcon from '@mui/icons-material/NotesOutlined';
import {
    FilterList,
    FilterListItem,
    FilterLiveSearch,
    SavedQueriesList,
} from 'react-admin';
import {
    endOfYesterday,
    startOfWeek,
    subWeeks,
    startOfMonth,
    subMonths,
} from 'date-fns';


const Aside = () => (
    <Card
        sx={{
            display: {
                xs: 'none',
                md: 'block',
            },
            order: -1,
            flex: '0 0 15em',
            mr: 2,
            mt: 6,
            alignSelf: 'flex-start',
        }}
    >
        <CardContent sx={{ pt: 1 }}>
            <FilterList
                label="Ngày gửi" 
                icon={<AccessTimeIcon />}
            >
                <FilterListItem
                    label="Hôm nay"
                    value={{
                        send_date_gte: endOfYesterday().toISOString(),
                        send_date_lte: undefined,
                    }}
                />
                <FilterListItem
                    label="Tuần này"
                    value={{
                        send_date_gte: startOfWeek(new Date()).toISOString(),
                        send_date_lte: undefined,
                    }}
                />
                <FilterListItem
                    label="Tuần trước"
                    value={{
                        send_date_gte: subWeeks(
                            startOfWeek(new Date()),
                            1
                        ).toISOString(),
                        send_date_lte: startOfWeek(new Date()).toISOString(),
                    }}
                />
                <FilterListItem
                    label="Tháng này"
                    value={{
                        send_date_gte: startOfMonth(new Date()).toISOString(),
                        send_date_lte: undefined,
                    }}
                />
                <FilterListItem
                    label="Tháng trước"
                    value={{
                        send_date_gte: subMonths(
                            startOfMonth(new Date()),
                            1
                        ).toISOString(),
                        send_date_lte: startOfMonth(new Date()).toISOString(),
                    }}
                />
                <FilterListItem
                    label="Thời gian trước"
                    value={{
                        send_date_gte: undefined,
                        send_date_lte: subMonths(
                            startOfMonth(new Date()),
                            1
                        ).toISOString(),
                    }}
                />
            </FilterList>

            <FilterList
                label="Điểm đến"
                icon={<LocationOnOutlinedIcon />}
            >
                <FilterListItem
                    label="Điểm giao dịch"
                    value={{
                        current_dest: 'exchanging',
                    }}
                />
                <FilterListItem
                    label="Điểm tập kết"
                    value={{
                        current_dest: 'gathering',
                    }}
                />
                <FilterListItem
                    label="Người nhận"
                    value={{
                        current_dest: 'receiver',
                    }}
                />
            </FilterList>

            <FilterList
                label="Trạng thái"
                icon={<NotesOutlinedIcon />}
            >
                <FilterListItem
                    label="Đã nhận"
                    value={{
                        status: 'received',
                    }}
                />
                <FilterListItem
                    label="Đang chuyển"
                    value={{
                        status: 'in-transit',
                    }}
                />
                <FilterListItem
                    label="Không nhận được"
                    value={{
                        status: 'not-received',
                    }}
                />
            </FilterList>

        </CardContent>
    </Card>
);

export default Aside;
