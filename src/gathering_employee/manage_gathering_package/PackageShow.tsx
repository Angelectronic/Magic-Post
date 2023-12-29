import * as React from 'react';
import { Box, Card, CardContent, Grid, Typography } from '@mui/material';
import { ReferenceField, TextField, useRecordContext } from 'react-admin';
import { styled } from '@mui/material/styles';
import {
    Table,
    TableBody,
    TableCell,
    TableHead,
    TableRow,
} from '@mui/material';
import Stack from '@mui/material/Stack';
import ViewReceiptButton from './ViewReceiptButton';
import AcceptButton from './AcceptButton';
import RejectButton from './RejectButton';
import RemoveButton from './RemoveButton';
import CreateDeliveryButton from './CreateDeliveryButton';
import EditPackageButton from './EditPackageButton';
const PackageShow = () => {
    const record = useRecordContext();
    if (!record) return null;
    return (
        <Card sx={{ width: 900, margin: 'auto' }}>
            <CardContent>
                <Grid container spacing={2}>
                    <Grid item xs={6}>
                        <Typography variant="h6" gutterBottom>
                            Người gửi:
                        </Typography>
                    </Grid>
                    <Grid item xs={6}>
                        <Typography variant="h6" gutterBottom align="left">
                            Người nhận:
                        </Typography>
                    </Grid>
                </Grid>
                <Grid container spacing={2}>
                    <Grid item xs={6}>
                        <Typography gutterBottom>
                            Họ và tên: {record.send_name}
                        </Typography>
                    </Grid>
                    <Grid item xs={6}>
                        <Typography gutterBottom align="left">
                            Họ và tên: {record.receive_name}
                        </Typography>
                    </Grid>
                </Grid>
                <Grid container spacing={2}>
                    <Grid item xs={6}>
                        <Typography gutterBottom>
                            SĐT: {record.send_phone}
                        </Typography>
                    </Grid>
                    <Grid item xs={6}>
                        <Typography gutterBottom align="left">
                            SĐT: {record.receive_phone}
                        </Typography>
                    </Grid>
                </Grid>
                <Grid container spacing={2}>
                    <Grid item xs={6}>
                        <Typography gutterBottom>
                            Địa chỉ: {record.send_address}
                        </Typography>
                    </Grid>
                    <Grid item xs={6}>
                        <Typography gutterBottom align="left">
                            Địa chỉ: {record.receive_address}
                        </Typography>
                    </Grid>
                </Grid>
                <Grid container spacing={2}>
                    <Grid item xs={6}>
                        <Typography gutterBottom>
                            Mã bưu chính: {record.send_point_id}
                        </Typography>
                    </Grid>
                    <Grid item xs={6}>
                        <Typography gutterBottom align="left">
                            Mã bưu chính: {record.receive_point_id}
                        </Typography>
                    </Grid>
                </Grid>
                <Box height={20}>&nbsp;</Box>
                <Grid container spacing={2}>
                    <Grid item xs={6}>
                        <Typography variant="h6" gutterBottom align="center">
                            Ngày gửi{' '}
                        </Typography>
                        <Typography gutterBottom align="center">
                            {new Date(record.send_date).toLocaleDateString()}
                        </Typography>
                    </Grid>

                    <Grid item xs={5}>
                        <Typography variant="h6" gutterBottom align="center">
                            Mã bưu gửi
                        </Typography>
                        <Typography gutterBottom align="center">
                            {record.package_id}
                        </Typography>
                    </Grid>
                </Grid>
                <Grid container spacing={2}>
                    <Grid item xs={6}>
                        <Table>
                            <TableHead>
                                <TableRow>
                                    <TableCell>
                                        Vật phẩm
                                    </TableCell>
                                    <TableCellRight>
                                        Số lượng
                                    </TableCellRight>
                                    <TableCellRight>
                                        Giá trị
                                    </TableCellRight>
                                </TableRow>
                            </TableHead>
                            <TableBody>
                                {record.items.map((item: any) => (
                                    <TableRow key={item.item_name}>
                                        <TableCell>
                                            {item.item_name}
                                        </TableCell>
                                        <TableCellRight>{item.quantity}</TableCellRight>
                                        <TableCellRight>
                                            {item.value.toLocaleString(
                                                undefined,
                                                {
                                                    style: 'currency',
                                                    currency: 'VND',
                                                }
                                            )}
                                        </TableCellRight>
                                    </TableRow>
                                ))}
                            </TableBody>
                        </Table>
                    </Grid>
                    <Grid item xs={6}>
                        <Table>
                            <TableHead>
                                <TableRow key={0}>
                                    <TableCell>
                                        Loại cước
                                    </TableCell>
                                    <TableCellRight>
                                        Giá trị
                                    </TableCellRight>
                                </TableRow>
                            </TableHead>
                            <TableBody>
                                <TableRow key={1}>
                                    <TableCell>
                                        Cước chính
                                    </TableCell>
                                    <TableCellRight>
                                        {record.main_cost.toLocaleString(
                                            undefined,
                                            {
                                                style: 'currency',
                                                currency: 'VND',
                                            }
                                        )}
                                    </TableCellRight>
                                </TableRow>
                                <TableRow key={2}>
                                    <TableCell>
                                        Cước phụ
                                    </TableCell>
                                    <TableCellRight>
                                        {record.other_cost.toLocaleString(
                                            undefined,
                                            {
                                                style: 'currency',
                                                currency: 'VND',
                                            }
                                        )}
                                    </TableCellRight>
                                </TableRow>
                                <TableRow key={3}>
                                    <TableCell>
                                        Cước GTGT
                                    </TableCell>
                                    <TableCellRight>
                                        {record.gtgt_cost.toLocaleString(
                                            undefined,
                                            {
                                                style: 'currency',
                                                currency: 'VND',
                                            }
                                        )}
                                    </TableCellRight>
                                </TableRow>
                                <TableRow key={4}>
                                    <TableCell>
                                        Tổng cước
                                    </TableCell>
                                    <TableCellRight>
                                        {(record.total_cost).toLocaleString(
                                            undefined,
                                            {
                                                style: 'currency',
                                                currency: 'VND',
                                            }
                                        )}
                                    </TableCellRight>
                                </TableRow>
                                <TableRow key={5}>
                                    <TableCell>
                                        Thu khác
                                    </TableCell>
                                    <TableCellRight>
                                        {(record.other_service_cost).toLocaleString(
                                            undefined,
                                            {
                                                style: 'currency',
                                                currency: 'VND',
                                            }
                                        )}
                                    </TableCellRight>
                                </TableRow>
                                <TableRow key={6}>
                                    <TableCell>
                                        Tổng thu
                                    </TableCell>
                                    <TableCellRight>
                                        {(record.total_cost + record.other_service_cost).toLocaleString(
                                            undefined,
                                            {
                                                style: 'currency',
                                                currency: 'VND',
                                            }
                                        )}
                                    </TableCellRight>
                                </TableRow>
                            </TableBody>
                        </Table>
                    </Grid>
                </Grid>
                <Stack direction="row" spacing={2}>
                    <EditPackageButton />
                    <ViewReceiptButton />
                    <CreateDeliveryButton />
                    <AcceptButton />
                    <RejectButton />
                    <RemoveButton />
                </Stack>
            </CardContent>
        </Card>
    );
};


const TableCellRight = styled(TableCell)({ textAlign: 'right' });
export default PackageShow;
