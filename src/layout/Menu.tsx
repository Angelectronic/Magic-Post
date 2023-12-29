import * as React from 'react';
import { useState } from 'react';
import Box from '@mui/material/Box';
import LabelIcon from '@mui/icons-material/Label';

import {
    useTranslate,
    DashboardMenuItem,
    MenuItemLink,
    MenuProps,
    useSidebarState,
} from 'react-admin';

import visitors from '../visitors';
import orders from '../orders';
import invoices from '../invoices';
import products from '../products';
import categories from '../categories';
import reviews from '../reviews';
import SubMenu from './SubMenu';
import HomeIcon from '@mui/icons-material/Home';
import SearchIcon from '../guest/search/SearchIcon';
import EditLocationAltIcon from '../admin/manage_point/ManagePointIcon'
import ManageAccountsIcon from '../admin/manage_account/ManageAccountIcon';
import ListAltIcon from '@mui/icons-material/ListAlt';
import InventoryIcon from '@mui/icons-material/Inventory';
import { useAuthProvider } from 'react-admin';
type MenuName = 'menuCatalog' | 'menuSales' | 'menuCustomers';

const Menu = ({ dense = false }: MenuProps) => {
    const translate = useTranslate();
    const [open] = useSidebarState();
    const auth = useAuthProvider();
    const role = auth.getRole();

    const menuRoleSwitch = (param: string) => {
        switch (param) {
            case 'guest':
                return (
                    <>
                        <MenuItemLink
                            to="/"
                            state={{ _scrollToTop: true }}
                            primaryText={translate('resources.home')}
                            leftIcon={<HomeIcon />}
                            dense={dense}
                        />
                        <MenuItemLink
                            to="/search"
                            state={{ _scrollToTop: true }}
                            primaryText={translate('resources.search')}
                            leftIcon={<SearchIcon />}
                            dense={dense}
                        />
                    </>
                );
            case 'admin':
                return (
                    <>
                        <DashboardMenuItem />
                        <MenuItemLink
                            to="/points"
                            state={{ _scrollToTop: true }}
                            primaryText={translate('resources.manage_point')}
                            leftIcon={<EditLocationAltIcon />}
                            dense={dense}
                        />
                        <MenuItemLink
                            to="/managerAccounts"
                            state={{ _scrollToTop: true }}
                            primaryText={translate('resources.manage_account')}
                            leftIcon={<ManageAccountsIcon />}
                            dense={dense}
                        />
                    </>
                );
            case 'exchanging_manager':
                return (
                    <>
                        <DashboardMenuItem />
                        <MenuItemLink
                            to="/exchangingEmployeeAccounts"
                            state={{ _scrollToTop: true }}
                            primaryText={"Quản lý tài khoản"}
                            leftIcon={<ManageAccountsIcon />}
                            dense={dense}
                        />
                    </>
                )
            case 'exchanging_employee':
                return (
                    <>
                        <DashboardMenuItem />
                        <MenuItemLink
                            to="/exchangingPackage"
                            state={{ _scrollToTop: true }}
                            primaryText={"Quản lý hàng"}
                            leftIcon={<InventoryIcon />}
                            dense={dense}
                        />
                        <MenuItemLink
                            to="/exchangingDelivery"
                            state={{ _scrollToTop: true }}
                            primaryText={translate('Quản lý đơn hàng')}
                            leftIcon={<ListAltIcon />}
                            dense={dense}
                        />
                    </>
                );
            case 'gathering_employee':
                return (
                    <>
                        <MenuItemLink
                            to="/gatheringPackage"
                            state={{ _scrollToTop: true }}
                            primaryText={"Quản lý hàng"}
                            leftIcon={<InventoryIcon />}
                            dense={dense}
                        />
                        <MenuItemLink
                            to="/gatheringDelivery"
                            state={{ _scrollToTop: true }}
                            primaryText={translate('Quản lý đơn hàng')}
                            leftIcon={<ListAltIcon />}
                            dense={dense}
                        />
                    </>
                );
                case 'gathering_manager':
                    return (
                        <>
                            <DashboardMenuItem />
                            <MenuItemLink
                                to="/gatheringEmployeeAccounts"
                                state={{ _scrollToTop: true }}
                                primaryText={"Quản lý tài khoản"}
                                leftIcon={<ManageAccountsIcon />}
                                dense={dense}
                            />
                        </>
                    )
        }
    };
    return (
        <Box
            sx={{
                width: open ? 200 : 50,
                marginTop: 1,
                marginBottom: 1,
                transition: theme =>
                    theme.transitions.create('width', {
                        easing: theme.transitions.easing.sharp,
                        duration: theme.transitions.duration.leavingScreen,
                    }),
            }}
        >
            {menuRoleSwitch(role)}
        </Box>
    );

};

export default Menu;
