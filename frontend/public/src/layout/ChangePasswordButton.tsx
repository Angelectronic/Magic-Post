import React from "react"
import { useNavigate } from "react-router-dom";
import { IconButton, Tooltip } from '@mui/material';
import PasswordIcon from '@mui/icons-material/Password';
import Button from '@mui/material/Button';
import { styled } from '@mui/material/styles';
import Dialog from '@mui/material/Dialog';
import DialogTitle from '@mui/material/DialogTitle';
import DialogContent from '@mui/material/DialogContent';
import DialogActions from '@mui/material/DialogActions';
import CloseIcon from '@mui/icons-material/Close';
import Alert from '@mui/material/Alert';
import Box from '@mui/material/Box';

import {
    Avatar,
    Card,
    CardActions,
    CircularProgress,
    TextField,
} from '@mui/material';
import LockIcon from '@mui/icons-material/Lock';

import { useNotify } from "react-admin";

const BootstrapDialog = styled(Dialog)(({ theme }) => ({
    '& .MuiDialogContent-root': {
        padding: theme.spacing(2),
    },
    '& .MuiDialogActions-root': {
        padding: theme.spacing(1),
    },
}));

export const ChangePasswordButton = () => {
    const [open, setOpen] = React.useState(false);
    const [loading, setLoading] = React.useState(false);
    const [old_password, setOldPassword] = React.useState("");
    const [new_password, setNewPassword] = React.useState("");
    const [confirm_password, setConfirmPassword] = React.useState("");
    const [ConfirmError, setConfirmError] = React.useState(false);
    const notify = useNotify();
    const handleClickOpen = () => {
        setOpen(true);
    };
    const handleClose = () => {
        setOpen(false);
    };
    const handleSubmit = async (event) => {
        event.preventDefault();

        setConfirmError(false);
        if (new_password !== confirm_password) {
            setConfirmError(true);
            notify("Mật khẩu mới không khớp", { type: 'error' });
        }
    };
    return (
        <>
            <Tooltip title={"Đổi mật khẩu"} enterDelay={300}>
                <IconButton
                    onClick={handleClickOpen}
                    color="inherit"
                    aria-label={"Đổi mật khẩu"}
                >
                    <PasswordIcon />
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
                        Đổi mật khẩu
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
                        <form onSubmit={handleSubmit}>
                            <Box
                                sx={{
                                    display: 'flex',
                                    flexDirection: 'column',
                                    alignItems: 'center',
                                    justifyContent: 'flex-start',
                                }}
                            >
                                <Box
                                    sx={{
                                        margin: '1em',
                                        display: 'flex',
                                        justifyContent: 'center',
                                    }}
                                >
                                    <Avatar sx={{ bgcolor: 'secondary.main' }}>
                                        <LockIcon />
                                    </Avatar>
                                </Box>
                                <Box sx={{ padding: '0 1em 1em 1em' }}>
                                    <Box sx={{ marginTop: '1em', justifyContent: 'center', }}>
                                        <TextField
                                            autoFocus
                                            id="old_pw"
                                            label="Nhập mật khẩu cũ"
                                            variant="filled"
                                            type="password"
                                            value={old_password}
                                            onChange={(event: React.ChangeEvent<HTMLInputElement>) => {
                                                setOldPassword(event.target.value);
                                            }}
                                        />
                                    </Box>
                                    <Box sx={{ marginTop: '1em' }}>
                                        <TextField
                                            id="new_pw"
                                            label="Nhập mật khẩu mới"
                                            variant="filled"
                                            type="password"
                                            required
                                            value={new_password}
                                            onChange={(event: React.ChangeEvent<HTMLInputElement>) => {
                                                setNewPassword(event.target.value);
                                            }}
                                            error={ConfirmError}
                                        />
                                    </Box>
                                    <Box sx={{ marginTop: '1em' }}>
                                        <TextField
                                            id="conf_pw"
                                            label="Nhập lại mật khẩu mới"
                                            variant="filled"
                                            required
                                            value={confirm_password}
                                            type="password"
                                            onChange={(event: React.ChangeEvent<HTMLInputElement>) => {
                                                setConfirmPassword(event.target.value);
                                            }}
                                            error={ConfirmError}
                                        />
                                    </Box>
                                    <Button
                                        variant="contained"
                                        type="submit"
                                        color="primary"
                                        disabled={loading}
                                        fullWidth
                                    >
                                        {loading && (
                                            <CircularProgress size={25} thickness={2} />
                                        )}
                                        {"Đổi mật khẩu"}
                                    </Button>
                                </Box>

                            </Box>
                        </form>
                    </DialogContent>
                </BootstrapDialog>
            </React.Fragment >
        </>
    );
};

export default ChangePasswordButton;
