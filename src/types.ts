import { Identifier, RaRecord } from 'react-admin';

export type ThemeName = 'light' | 'dark';

export interface Category extends RaRecord {
    name: string;
}

export interface Product extends RaRecord {
    category_id: Identifier;
    description: string;
    height: number;
    image: string;
    price: number;
    reference: string;
    stock: number;
    thumbnail: string;
    width: number;
}

export interface Point extends RaRecord {
    name: string;
    reference: string;
    address: string;
    city: string;
    zipcode: string;
    manager_reference: string;
}

export interface ManagerAccount extends RaRecord {
    name: string;
    reference: string;
    email: string;
    point_reference: string;
}

export type OrderStatus = 'ordered' | 'delivered' | 'cancelled';

export interface Order extends RaRecord {
    status: OrderStatus;
    basket: BasketItem[];
    date: Date;
    total: number;
    total_ex_taxes: number;
    delivery_fees: number;
    tax_rate: number;
    taxes: number;
    customer_id: Identifier;
    reference: string;
}

export type BasketItem = {
    product_id: Identifier;
    quantity: number;
};

export interface Invoice extends RaRecord {
    date: Date;
}

export type ReviewStatus = 'accepted' | 'pending' | 'rejected';

export interface Review extends RaRecord {
    date: Date;
    status: ReviewStatus;
    customer_id: Identifier;
    product_id: Identifier;
    comment: string;
}

declare global {
    interface Window {
        restServer: any;
    }
}
