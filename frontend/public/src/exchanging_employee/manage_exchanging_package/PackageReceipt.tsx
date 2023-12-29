import './receipt.css';
import React, { useRef, useState } from "react";
import { useReactToPrint } from 'react-to-print';
import QRCode from "qrcode";

import Button from '@mui/material/Button';
import { styled } from '@mui/material/styles';
import Dialog from '@mui/material/Dialog';
import DialogTitle from '@mui/material/DialogTitle';
import DialogContent from '@mui/material/DialogContent';
import DialogActions from '@mui/material/DialogActions';
import IconButton from '@mui/material/IconButton';
import CloseIcon from '@mui/icons-material/Close';
import PrintIcon from '@mui/icons-material/Print';
import Alert from '@mui/material/Alert';
import Collapse from '@mui/material/Collapse';
import Box from '@mui/material/Box';
import { Print } from '@mui/icons-material';
import { useRecordContext } from 'react-admin';
const BootstrapDialog = styled(Dialog)(({ theme }) => ({
    '& .MuiDialogContent-root': {
        padding: theme.spacing(2),
    },
    '& .MuiDialogActions-root': {
        padding: theme.spacing(1),
    },
}));

const PackageReceipt = (data1: any) => {
    const data = data1.data;
    const [open, setOpen] = React.useState(false);
    const [openAlert, setOpenAlert] = React.useState(false);
    const handleClickOpen = () => {
        setOpen(true);
    };
    const handleClickOpenAlert = () => {
        setOpenAlert(true);
    };
    const handleClose = () => {
        setOpen(false);
    };
    const [qrData, setqrData] = useState("");
    const componentRef = useRef();
    const handlePrint = useReactToPrint({
        content: () => componentRef.current,
        documentTitle: 'Giấy biên nhận',
        onAfterPrint: () => handleClickOpenAlert(),
    });
    QRCode.toDataURL(`http://localhost:5173/#/search/${data.package_id}`)
        .then(img => {
            setqrData(img);
        });
    var total = 0;
    for (let i = 0; i < data.items.length; i++) {
        total += data.items[i].quantity * data.items[i].value;
    }
    var tmp = ((data.main_cost + data.other_cost + data.gtgt_cost) * (data.vat + 100) / 100).toFixed(0);
    var total_cost = parseInt(tmp);
    return (
        <React.Fragment>
            <Button startIcon={<PrintIcon />} variant="outlined" onClick={handleClickOpen}>
                In biên nhận
            </Button>
            <BootstrapDialog
                onClose={handleClose}
                aria-labelledby="customized-dialog-title"
                open={open}
                fullWidth={true}
                maxWidth="xl"
            >
                <DialogTitle sx={{ m: 0, p: 2 }} id="customized-dialog-title">
                    Giấy biên nhận
                </DialogTitle>
                <IconButton
                    aria-label="close"
                    onClick={handleClose}
                    sx={{
                        position: 'absolute',
                        right: 8,
                        top: 8,
                        color: (theme) => theme.palette.grey[500],
                    }}
                >
                    <CloseIcon />
                </IconButton>
                <DialogContent dividers>
                    <div
                        className="Giay-bien-nhan"
                        ref={componentRef}
                        style={{ width: '100%' }}
                    >
                        <div style={{ display: "flex" }}>
                            <div style={{ flex: 1, fontWeight: "bold", marginTop: "30px", fontSize: "30px", textAlign: "center" }}>
                                MAGICPOST
                            </div>
                            <div style={{ flex: 1, textAlign: "center" }}>
                                <img src={qrData} />
                            </div>
                        </div>
                        <div style={{ display: "flex" }}>
                            <div style={{ flex: 1, fontWeight: "bold", marginTop: "30px", fontSize: "30px", textAlign: "center" }}>

                            </div>
                            <div style={{ flex: 1, fontWeight: "bold", textAlign: "center" }}>
                                {data.package_id}
                            </div>
                        </div>
                        <br />
                        <div className='special_class'>
                            <table>
                                <tbody>
                                    <tr>
                                        <td >
                                            <div style={{ fontWeight: "bold" }}>
                                                1. Họ tên địa chỉ người gửi
                                            </div>
                                            <div>
                                                {data.send_name}
                                            </div>
                                            <div>
                                                {data.send_address}
                                            </div>
                                            <br />
                                            <div style={{ display: "flex" }}>
                                                <div style={{ flex: 1 }}>
                                                    <b>Điện thoại: </b> {data.send_phone}
                                                </div>
                                                <div style={{ flex: 1 }}>
                                                    <b>Mã bưu chính: </b> {data.send_point_id}
                                                </div>
                                            </div>
                                        </td>
                                        <td colSpan={2}>
                                            <div style={{ fontWeight: "bold" }}>
                                                2. Họ tên địa chỉ người nhận
                                            </div>
                                            <div>
                                                {data.receive_name}
                                            </div>
                                            <div>
                                                {data.receive_address}
                                            </div>
                                            <br />
                                            <div style={{ display: "flex" }}>
                                                <div style={{ flex: 1 }}>
                                                    <b>Điện thoại: </b> {data.receive_phone}
                                                </div>
                                                <div style={{ flex: 1 }}>
                                                    <b>Mã bưu chính: </b> {data.receive_point_id}
                                                </div>
                                            </div>
                                        </td>
                                    </tr>
                                    <tr>
                                        <td style={{ width: "50%" }}>
                                            <div style={{ fontWeight: "bold" }}>
                                                3. Loại hàng gửi
                                            </div>
                                            <div style={{ display: "flex" }}>
                                                <div style={{ flex: 1 }}>
                                                    <input type="checkbox" id="checkbox1" checked={data.package_type === 1} disabled></input>
                                                    <label style={{ marginBottom: "20px" }}>Tài liệu</label>
                                                </div>
                                                <div style={{ flex: 1 }}>
                                                    <input type="checkbox" id="checkbox1" checked={data.package_type === 2} disabled></input>
                                                    <label style={{ marginBottom: "20px" }}>Hàng hóa</label>
                                                </div>
                                            </div>


                                        </td>
                                        <td style={{ width: "30%" }} rowSpan={2}>
                                            <div style={{ fontWeight: "bold" }}>
                                                9. Cước
                                            </div>
                                            <div style={{ display: "flex" }}>
                                                <div style={{ flex: 1, textAlign: 'left' }}>
                                                    a. Cước chính
                                                </div>
                                                <div style={{ flex: 1, textAlign: 'right' }}>
                                                    {data.main_cost.toLocaleString(
                                                        undefined,
                                                        {
                                                            style: 'currency',
                                                            currency: 'VND',
                                                        }
                                                    )}
                                                </div>
                                            </div>
                                            <div style={{ display: "flex" }}>
                                                <div style={{ flex: 1, textAlign: 'left' }}>
                                                    b. Cước phụ
                                                </div>
                                                <div style={{ flex: 1, textAlign: 'right' }}>
                                                    {data.other_cost.toLocaleString(
                                                        undefined,
                                                        {
                                                            style: 'currency',
                                                            currency: 'VND',
                                                        }
                                                    )}
                                                </div>
                                            </div>

                                            <div style={{ display: "flex" }}>
                                                <div style={{ flex: 1, textAlign: 'left' }}>
                                                    c. Cước GTGT
                                                </div>
                                                <div style={{ flex: 1, textAlign: 'right' }}>
                                                    {data.gtgt_cost.toLocaleString(
                                                        undefined,
                                                        {
                                                            style: 'currency',
                                                            currency: 'VND',
                                                        }
                                                    )}
                                                </div>
                                            </div>
                                            <div style={{ display: "flex" }}>
                                                <div style={{ flex: 1, textAlign: 'left' }}>
                                                    {"d. Tổng cước (gồm VAT)"}
                                                </div>
                                                <div style={{ flex: 1, textAlign: 'right' }}>
                                                    {total_cost.toLocaleString(
                                                        undefined,
                                                        {
                                                            style: 'currency',
                                                            currency: 'VND',
                                                        }
                                                    )}
                                                </div>
                                            </div>
                                            <div style={{ display: "flex" }}>
                                                <div style={{ flex: 1, textAlign: 'left' }}>
                                                    {"e. Thu khác"}
                                                </div>
                                                <div style={{ flex: 1, textAlign: 'right' }}>
                                                    {data.other_service_cost.toLocaleString(
                                                        undefined,
                                                        {
                                                            style: 'currency',
                                                            currency: 'VND',
                                                        }
                                                    )}
                                                </div>
                                            </div>
                                            <div style={{ display: "flex" }}>
                                                <div style={{ flex: 1, textAlign: 'left' }}>
                                                    <b>{"f. Tổng thu"}</b>
                                                </div>
                                                <div style={{ flex: 1, textAlign: 'right' }}>
                                                    {(data.other_service_cost + total_cost).toLocaleString(
                                                        undefined,
                                                        {
                                                            style: 'currency',
                                                            currency: 'VND',
                                                        }
                                                    )}
                                                </div>
                                            </div>
                                        </td>
                                        <td style={{ width: "20%" }} >
                                            <div style={{ fontWeight: "bold" }}>
                                                {"10. Khối lượng (kg)"}
                                            </div>
                                            <div style={{ display: "flex" }}>
                                                <div style={{ flex: 2, textAlign: 'left' }}>
                                                    Khối lượng thực tế:
                                                </div>
                                                <div style={{ flex: 1, textAlign: 'right' }}>
                                                    {data.weight}
                                                </div>
                                            </div>
                                        </td>
                                    </tr>
                                    <tr>
                                        <td style={{ width: "50%" }}>
                                            <div style={{ fontWeight: "bold" }}>
                                                4. Nội dung trị giá bưu gửi
                                            </div>
                                            <table style={{ width: "100%" }}>
                                                <tbody>
                                                    <tr>
                                                        <th style={{ backgroundColor: "#f2f2f2" }}>Nội dung</th>
                                                        <th style={{ backgroundColor: "#f2f2f2" }}>Số lượng</th>
                                                        <th style={{ backgroundColor: "#f2f2f2" }}>Trị giá</th>
                                                    </tr>
                                                    {data.items.map(item => (
                                                        <tr key={item.item_name}>
                                                            <td style={{ textAlign: 'center' }}> {item.item_name}</td>
                                                            <td style={{ textAlign: 'center' }}> {item.quantity}</td>
                                                            <td style={{ textAlign: 'center' }}> {item.value.toLocaleString(
                                                                undefined,
                                                                {
                                                                    style: 'currency',
                                                                    currency: 'VND',
                                                                }
                                                            )}</td>
                                                        </tr>
                                                    ))}
                                                    <tr>
                                                        <td style={{ textAlign: 'center' }}> Tổng</td>
                                                        <td style={{ textAlign: 'center' }}> </td>
                                                        <td style={{ textAlign: 'center' }}> {total.toLocaleString(
                                                            undefined,
                                                            {
                                                                style: 'currency',
                                                                currency: 'VND',
                                                            }
                                                        )}</td>
                                                    </tr>
                                                </tbody>
                                            </table>


                                        </td>
                                        <td style={{ width: "20%" }} rowSpan={3}>
                                            <div style={{ fontWeight: "bold" }}>
                                                12. Chú dẫn nghiệp vụ
                                            </div>
                                            {data.note}
                                        </td>
                                    </tr>
                                    <tr>
                                        <td style={{ width: "50%" }}>
                                            <div style={{ fontWeight: "bold" }}>
                                                5. Dịch vụ đặc biệt/Cộng thêm:
                                            </div>
                                            {data.special_service}
                                            <br />
                                        </td>
                                        <td style={{ width: "30%" }} rowSpan={2}>
                                            <div style={{ fontWeight: "bold" }}>
                                                11. Thu của người nhận
                                            </div>
                                            <div style={{ display: "flex" }}>
                                                <div style={{ flex: 1, textAlign: 'left' }}>
                                                    {"COD"}
                                                </div>
                                                <div style={{ flex: 1, textAlign: 'right' }}>
                                                    {(data.cod).toLocaleString(
                                                        undefined,
                                                        {
                                                            style: 'currency',
                                                            currency: 'VND',
                                                        }
                                                    )}
                                                </div>
                                            </div>
                                            <div style={{ display: "flex" }}>
                                                <div style={{ flex: 1, textAlign: 'left' }}>
                                                    {"Thu khác"}
                                                </div>
                                                <div style={{ flex: 1, textAlign: 'right' }}>
                                                    {(data.receive_other_cost).toLocaleString(
                                                        undefined,
                                                        {
                                                            style: 'currency',
                                                            currency: 'VND',
                                                        }
                                                    )}
                                                </div>
                                            </div>
                                            <div style={{ display: "flex" }}>
                                                <div style={{ flex: 1, textAlign: 'left' }}>
                                                    {"Tổng thu"}
                                                </div>
                                                <div style={{ flex: 1, textAlign: 'right' }}>
                                                    {(data.receive_other_cost + data.cod).toLocaleString(
                                                        undefined,
                                                        {
                                                            style: 'currency',
                                                            currency: 'VND',
                                                        }
                                                    )}
                                                </div>
                                            </div>
                                        </td>
                                    </tr>
                                    <tr>
                                        <td style={{ width: "50%" }}>
                                            <div style={{ fontWeight: "bold" }}>
                                                6. Chỉ dẫn của người gửi khi không phát được bưu gửi:
                                            </div>
                                            <div style={{ display: "flex" }}>
                                                <div style={{ flex: 1 }}>
                                                    <input type="checkbox" id="checkbox1" checked={data.instruction_type === 1} disabled></input>
                                                    <label style={{ marginBottom: "20px" }}>Chuyển hoàn ngay</label>
                                                </div>
                                                <div style={{ flex: 1 }}>
                                                    <input type="checkbox" id="checkbox1" checked={data.instruction_type === 2} disabled></input>
                                                    <label style={{ marginBottom: "20px" }}>Gọi điện cho người gửi/BC gửi</label>
                                                </div>
                                                <div style={{ flex: 1 }}>
                                                    <input type="checkbox" id="checkbox1" checked={data.instruction_type === 3} disabled></input>
                                                    <label style={{ marginBottom: "20px" }}>Hủy</label>
                                                </div>
                                            </div>
                                            <div style={{ display: "flex" }}>
                                                <div style={{ flex: 1 }}>
                                                    <input type="checkbox" id="checkbox1" checked={data.instruction_type === 4} disabled></input>
                                                    <label style={{ marginBottom: "20px" }}>Chuyển hoàn trước ngày</label>
                                                </div>
                                                <div style={{ flex: 1 }}>
                                                    <input type="checkbox" id="checkbox1" checked={data.instruction_type === 5} disabled></input>
                                                    <label style={{ marginBottom: "20px" }}>Chuyển hoàn khi hết thời gian lưu trữ</label>
                                                </div>
                                            </div>
                                        </td>

                                    </tr>
                                    <tr>
                                        <td style={{ width: "50%" }}>
                                            <div style={{ fontWeight: "bold" }}>
                                                7. Cam kết của người gửi
                                            </div>
                                            <div >
                                                Tôi chấp nhận các điều khoản tại mặt sau phiếu gửi và cam đoan bưu gửi này không chứa những mặt hàng nguy hiểm, cấm gửi. Trường hợp không phát được hãy thực hiện chỉ dẫn tại mục 6, tôi sẽ trả cước chuyển hoàn.
                                            </div>
                                            <div style={{ display: "flex" }}>
                                                <div style={{ flex: 1 }}>
                                                    <div style={{ fontWeight: "bold", marginTop: "10px" }}>
                                                        8. Ngày giờ gửi
                                                    </div>
                                                </div>
                                                <div style={{ flex: 1 }}>
                                                    <div style={{ fontWeight: "bold", marginTop: "10px" }}>
                                                        Chữ ký người gửi
                                                    </div>

                                                </div>
                                            </div>
                                            <div style={{ display: "flex" }}>
                                                <div style={{ marginLeft: "10px", flex: 1 }}>
                                                    {data.send_date}
                                                </div>
                                                <div style={{ flex: 1 }}>
                                                    <div>
                                                    </div>
                                                </div>
                                            </div>
                                            <br />
                                            <br />
                                            <br />
                                        </td>
                                        <td style={{ width: "30%" }}>
                                            <div style={{ fontWeight: "bold", textAlign: 'center' }}>
                                                13. Bưu cục chấp nhận
                                            </div>
                                            <div style={{ textAlign: 'center' }}>
                                                GDV: {data.gdv}
                                            </div>
                                            <div style={{ textAlign: 'center' }}>
                                                Chữ kí GDV nhận
                                            </div>

                                        </td>
                                        <td style={{ width: "20%" }}>
                                            <div style={{ fontWeight: "bold" }}>
                                                14. Ngày giờ nhận
                                            </div>
                                            <b>....h...../....../...../20....</b>
                                            <div style={{ textAlign: "center" }}>
                                                Người nhận/ Người được ủy quyền nhận
                                            </div>
                                            <div style={{ textAlign: "center" }}>
                                                {"(Ký, ghi rõ họ tên"}
                                            </div>
                                        </td>
                                    </tr>
                                </tbody>
                            </table>
                        </div>

                    </div>
                </DialogContent>
                <DialogActions>
                    <Button startIcon={<PrintIcon />} autoFocus onClick={handlePrint}>
                        In PDF
                    </Button>
                </DialogActions>
                <Box sx={{ width: '100%' }}>
                    <Collapse in={openAlert}>
                        <Alert
                            action={
                                <IconButton
                                    aria-label="close"
                                    color="inherit"
                                    size="small"
                                    onClick={() => {
                                        setOpenAlert(false);
                                    }}
                                >
                                    <CloseIcon fontSize="inherit" />
                                </IconButton>
                            }
                            sx={{ mb: 2 }}
                        >
                            Giấy biên nhận đã in thành công!
                        </Alert>
                    </Collapse>
                </Box>
            </BootstrapDialog>
        </React.Fragment>
    );
}

export default PackageReceipt;