import PublishIcon from '@mui/icons-material/Publish';
import { Button } from '@mui/material';
import React from "react";
import { styled } from '@mui/material/styles';
import Dialog from '@mui/material/Dialog';
import DialogTitle from '@mui/material/DialogTitle';
import DialogContent from '@mui/material/DialogContent';
import DialogActions from '@mui/material/DialogActions';
import IconButton from '@mui/material/IconButton';
import CloseIcon from '@mui/icons-material/Close';
import Typography from '@mui/material/Typography';
import { useCSVReader } from 'react-papaparse';
import { useDataProvider, useNotify, useRefresh } from 'react-admin';
const styles = {
    csvReader: {
        display: 'flex',
        flexDirection: 'row',
        marginBottom: 10,
        width: "600px",
    } as React.CSSProperties,
    browseFile: {
        width: '20%',
    } as React.CSSProperties,
    acceptedFile: {
        border: '1px solid #ccc',
        height: 45,
        lineHeight: 2.5,
        paddingLeft: 10,
        width: '80%',
    } as React.CSSProperties,
    remove: {
        borderRadius: 0,
        padding: '0 20px',
    } as React.CSSProperties,
    progressBarBackgroundColor: {
        backgroundColor: 'red',
    } as React.CSSProperties,
};
const BootstrapDialog = styled(Dialog)(({ theme }) => ({
    '& .MuiDialogContent-root': {
        padding: theme.spacing(2),
    },
    '& .MuiDialogActions-root': {
        padding: theme.spacing(1),
    },
}));

import { FileInput, SimpleForm, FileField } from 'react-admin';
const ImportButton = () => {
    const provider = useDataProvider();
    const notify = useNotify();
    const { CSVReader } = useCSVReader();
    const [open, setOpen] = React.useState(false);
    const refresh = useRefresh();
    const handleClickOpen = () => {
        setOpen(true);
    };
    const handleClose = () => {
        setOpen(false);
    };
    return (
        <React.Fragment>
            <Button startIcon={<PublishIcon />} onClick={handleClickOpen}>
                Nhập
            </Button>
            <BootstrapDialog
                onClose={handleClose}
                aria-labelledby="customized-dialog-title"
                open={open}
                maxWidth="xl"
            >
                <DialogTitle sx={{ m: 0, p: 2 }} id="customized-dialog-title">
                    Upload file csv
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
                    <CSVReader
                        onUploadAccepted={async (results: any) => {
                            for (let i = 1; i < results.data.length; ++i) {
                                var obj = {};
                                obj.data = {};
                                for (let ii = 0; ii < results.data[0].length; ++ii) {
                                    obj.data[results.data[0][ii]] = results.data[i][ii];
                                }
                                await provider.create("exchangingEmployeeAccounts", obj);
                            }
                            notify("Nhập dữ liệu thành công", {type: 'success'});
                            refresh();
                            handleClose();
                        }}
                    >
                        {({
                            getRootProps,
                            acceptedFile,
                            ProgressBar,
                            getRemoveFileProps,
                        }: any) => (
                            <>
                                <div style={styles.csvReader}>
                                    <button type='button' {...getRootProps()} style={styles.browseFile}>
                                        Chọn File
                                    </button>
                                    <div style={styles.acceptedFile}>
                                        {acceptedFile && acceptedFile.name}
                                    </div>
                                    <button {...getRemoveFileProps()} style={styles.remove}>
                                        Xóa
                                    </button>
                                </div>
                                <ProgressBar style={styles.progressBarBackgroundColor} />
                            </>
                        )}
                    </CSVReader>
                </DialogContent>
            </BootstrapDialog>
        </React.Fragment>
    );
}

export default ImportButton;