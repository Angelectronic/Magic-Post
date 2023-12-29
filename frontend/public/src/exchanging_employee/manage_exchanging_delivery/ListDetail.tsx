import * as React from 'react';
import {
    Identifier,
    Datagrid,
    DateField,
    TextField,
    ExportButton,
} from 'react-admin';

import rowSx from './rowSx';
import ColoredDestField from './ColoredDestField';
import ColoredFromField from './ColoredFromField';
import FinishStateField from './FinishStateField';
import StatusField from './StatusField';
export interface ListDetailProps {
    selectedRow?: Identifier;
}


const ListDetail = ({ selectedRow }: ListDetailProps) => (
    <Datagrid
        rowClick="edit"
        rowSx={rowSx(selectedRow)}
        optimized
        bulkActionButtons={<></>}
        sx={{
            '& .RaDatagrid-thead': {
                borderLeftColor: 'transparent',
                borderLeftWidth: 5,
                borderLeftStyle: 'solid',
            },
            '& .column-comment': {
                maxWidth: '18em',
                overflow: 'hidden',
                textOverflow: 'ellipsis',
                whiteSpace: 'nowrap',
            },
        }}
    >   
        <TextField source="delivery_id" label="Mã đơn hàng" />
        <DateField source="create_date" label="Ngày tạo" />
        <DateField source="send_date" label="Ngày gửi" />
        <ColoredFromField source='current_from' label='Chuyển từ' />
        <ColoredDestField source='current_dest' label='Chuyển đến' />
        <StatusField label='Trạng thái'/>
        <FinishStateField label='Thành công?' />

    </Datagrid>
);

export default ListDetail;
