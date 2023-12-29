import * as React from 'react';
import { ReferenceField, Labeled, FunctionField, useRecordContext, TextField } from 'react-admin';

import PointNameField from '../manage_point/PointNameField';

const EmptyField = (props: any) => {
    return (<span> </span>);
}

const PointReferenceField = () => {
    return (
        <FunctionField
            render={(record: any) => {
                if (record.point_id !== undefined) {
                    return (
                        <React.Fragment>
                            <ReferenceField
                                source="point_id"
                                reference="points"
                            >
                                <PointNameField />
                            </ReferenceField>
                        </React.Fragment>
                    );
                } else {
                    return (
                        <EmptyField/>
                    )   
                }
            }}
        />
    );
};


export default PointReferenceField;
