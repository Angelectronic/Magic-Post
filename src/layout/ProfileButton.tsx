import React from "react";
import { useNavigate } from "react-router-dom";
import { IconButton, Tooltip } from '@mui/material';
import AccountBoxIcon from '@mui/icons-material/AccountBox';

import Button from '@mui/material/Button';
import { styled } from '@mui/material/styles';
import Dialog from '@mui/material/Dialog';
import DialogTitle from '@mui/material/DialogTitle';
import DialogContent from '@mui/material/DialogContent';
import DialogActions from '@mui/material/DialogActions';
import CloseIcon from '@mui/icons-material/Close';
import Alert from '@mui/material/Alert';
import Box from '@mui/material/Box';
const BootstrapDialog = styled(Dialog)(({ theme }) => ({
    '& .MuiDialogContent-root': {
        padding: theme.spacing(2),
    },
    '& .MuiDialogActions-root': {
        padding: theme.spacing(1),
    },
}));



export const ProfileButton = () => {
    const [open, setOpen] = React.useState(false);
    const handleClickOpen = () => {
        setOpen(true);
    };
    const handleClose = () => {
        setOpen(false);
    };
    return (
        <>
            <Tooltip title={"Thông tin hồ sơ"} enterDelay={300}>
                <IconButton
                    onClick={handleClickOpen}
                    color="inherit"
                    aria-label={"Thông tin hồ sơ"}
                >
                    <AccountBoxIcon />
                </IconButton>
            </Tooltip>
            <React.Fragment>
                <BootstrapDialog
                    onClose={handleClose}
                    aria-labelledby="customized-dialog-title"
                    open={open}
                    fullWidth={true}
                    maxWidth="xs"
                >
                    <DialogTitle sx={{ m: 0, p: 2 }} id="customized-dialog-title">
                        Thông tin hồ sơ
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
                    <DialogContent >
                        <div style={{ display: "flex" }}>
                            <div style={{ flex: 1, fontWeight: "bold", textAlign: "left" }}>
                                Họ và tên:
                            </div>
                            <div style={{ flex: 1, textAlign: "left" }}>
                                {localStorage.getItem("name")}
                            </div>
                        </div>
                        <div style={{ display: "flex" }}>
                            <div style={{ flex: 1, fontWeight: "bold", textAlign: "left" }}>
                                Ngày sinh:
                            </div>
                            <div style={{ flex: 1, textAlign: "left" }}>
                                {localStorage.getItem("birthday")}
                            </div>
                        </div>
                        <div style={{ display: "flex" }}>
                            <div style={{ flex: 1, fontWeight: "bold", textAlign: "left" }}>
                                Giới tính:
                            </div>
                            <div style={{ flex: 1, textAlign: "left" }}>
                                {localStorage.getItem("sex") === 'male' ? 'Nam' : 'Nữ'}
                            </div>
                        </div>
                        <div style={{ display: "flex" }}>
                            <div style={{ flex: 1, fontWeight: "bold", textAlign: "left" }}>
                                Email: 
                            </div>
                            <div style={{ flex: 1, textAlign: "left" }}>
                                {localStorage.getItem("email")}
                            </div>
                        </div>
                        <div style={{ display: "flex" }}>
                            <div style={{ flex: 1, fontWeight: "bold", textAlign: "left" }}>
                                Mã nhân viên:
                            </div>
                            <div style={{ flex: 1, textAlign: "left" }}>
                                {localStorage.getItem("reference")}
                            </div>
                        </div>
                        <div style={{ display: "flex" }}>
                            <div style={{ flex: 1, fontWeight: "bold", textAlign: "left" }}>
                                Vị trí:
                            </div>
                            <div style={{ flex: 1, textAlign: "left" }}>
                                {localStorage.getItem("fullName")}
                            </div>
                        </div>
                        <div style={{ display: "flex" }}>
                            <div style={{ flex: 1, fontWeight: "bold", textAlign: "left" }}>
                                Mã điểm:
                            </div>
                            <div style={{ flex: 1, textAlign: "left" }}>
                                {localStorage.getItem("point_reference")}
                            </div>
                        </div>
                    </DialogContent>
                </BootstrapDialog>
            </React.Fragment >
        </>
    );
};

export default ProfileButton;
