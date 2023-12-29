import * as React from 'react';
import { SVGProps } from 'react';
import { useTheme } from '@mui/material/styles';
import { Typography } from '@mui/material';
const Logo = (props: SVGProps<SVGSVGElement>) => {
    const theme = useTheme();
    return (
        <Typography variant="h5" gutterBottom>
            MagicPost
        </Typography>
    );
};

export default Logo;
