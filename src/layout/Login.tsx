import * as React from 'react';
import { useState } from 'react';
import PropTypes from 'prop-types';
import { useLocation } from 'react-router-dom';

import {
    Avatar,
    Button,
    Card,
    CardActions,
    CircularProgress,
} from '@mui/material';
import LockIcon from '@mui/icons-material/Lock';
import {
    Form,
    required,
    TextInput,
    useTranslate,
    useNotify,
    useAuthProvider,
} from 'react-admin';

import Box from '@mui/material/Box';
import { useNavigate } from 'react-router-dom';
const Login = () => {
    const [loading, setLoading] = useState(false);
    const translate = useTranslate();

    const notify = useNotify();
    const navigate = useNavigate();
    const authProvider = useAuthProvider();
    const location = useLocation();
    const [username, setUsername] = React.useState('1');
    const [password, setPassword] = React.useState('1');
    const handleSubmit = (auth: FormValues) => {
        console.log(auth);
        setLoading(true);
        auth.username = username;
        auth.password = password;
        authProvider.login(
            auth
        ).then(res => {
            
            setLoading(false);
            console.log(res);
            if (res.error === 403) {
                notify("Sai tài khoản hoặc mật khẩu", {type: 'error'});
            }
            else {
                notify("Đăng nhập thành công", {type: 'success'});
                navigate("/");
            }
        });
    };

    return (
        <Form onSubmit={handleSubmit} noValidate>
            <Box
                sx={{
                    display: 'flex',
                    flexDirection: 'column',
                    minHeight: '100vh',
                    alignItems: 'center',
                    justifyContent: 'flex-start',
                    background:
                        'url(https://source.unsplash.com/featured/1600x900)',
                    backgroundRepeat: 'no-repeat',
                    backgroundSize: 'cover',
                }}
            >
                <Card sx={{ minWidth: 300, marginTop: '6em' }}>
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
                    <Box
                        sx={{
                            marginTop: '1em',
                            display: 'flex',
                            justifyContent: 'center',
                            color: theme => theme.palette.grey[500],
                        }}
                    >
                    </Box>
                    <Box sx={{ padding: '0 1em 1em 1em' }}>
                        <Box sx={{ marginTop: '1em' }}>
                            <TextInput
                                autoFocus
                                source="username"
                                label={translate('ra.auth.username')}
                                disabled={loading}
                                value={username}
                                fullWidth
                                onChange={(event: React.ChangeEvent<HTMLInputElement>) => {
                                    setUsername(event.target.value);
                                }}
                            />
                        </Box>
                        <Box sx={{ marginTop: '1em' }}>
                            <TextInput
                                source="password"
                                label={translate('ra.auth.password')}
                                type="password"
                                disabled={loading}
                                fullWidth
                                value={password}
                                onChange={(event: React.ChangeEvent<HTMLInputElement>) => {
                                    setPassword(event.target.value);
                                }}
                            />
                        </Box>
                    </Box>
                    <CardActions sx={{ padding: '0 1em 1em 1em' }}>
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
                            {translate('ra.auth.sign_in')}
                        </Button>
                    </CardActions>
                    <CardActions sx={{ padding: '0 1em 1em 1em' }}>
                        <Button
                            variant="outlined"
                            color="primary"
                            disabled={loading}
                            onClick={() => {
                                setLoading(true);
                                var auth: FormValues = {
                                    isGuest: true
                                };
                                authProvider.login(
                                    auth
                                ).then(res => {
                                    navigate("/");
                                });
                            }}
                            fullWidth
                        >
                            {loading && (
                                <CircularProgress size={25} thickness={2} />
                            )}
                            {translate('pos.auth.guest')}
                        </Button>
                    </CardActions>
                </Card>
            </Box>
        </Form>
    );
};

Login.propTypes = {
    authProvider: PropTypes.func,
    previousRoute: PropTypes.string,
};

export default Login;

interface FormValues {
    username?: string;
    password?: string;
    isGuest?: boolean;
}
