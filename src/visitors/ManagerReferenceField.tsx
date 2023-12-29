import * as React from 'react';
import { ReferenceField, FunctionField } from 'react-admin';

import FullNameField from './FullNameField';

const EmptyField = (props: any) => {
    return (<span> </span>);
}
const ManagerReferenceField = () => {
    return (
        <FunctionField
            render={(record: any) => {
                if (record.manager_id !== undefined) {
                    return (
                        <React.Fragment>
                            <ReferenceField
                                source="manager_id"
                                reference="managerAccounts"
                            >
                                <FullNameField />
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


export default ManagerReferenceField;
