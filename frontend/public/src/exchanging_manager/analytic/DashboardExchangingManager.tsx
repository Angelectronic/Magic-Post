import * as React from 'react';
import PackageStatistic from './PackageStatistic';
import { useGetList } from 'react-admin';
import PackageList from './PackageList';
const styles = {
    flex: { display: 'flex' },
    flexColumn: { display: 'flex', flexDirection: 'column' },
    leftCol: { flex: 1, marginRight: '0.5em' },
    rightCol: { flex: 1, marginLeft: '0.5em' },
    singleCol: { marginTop: '1em', marginBottom: '1em' },
};

const DashboardExchangingManager = () => {
    const { data: curDelivery, isLoading: isLoading } = useGetList('exchangingDelivery');
    if (isLoading) {
        return (
            <></>
        );
    }
    var smallestYear = 9999;
    for (let i = 0; i < curDelivery.length; ++i) {
        if ((new Date(curDelivery[i].begin_date)).getFullYear() < smallestYear) {
            smallestYear = (new Date(curDelivery[i].begin_date)).getFullYear();
        }
    }
    return (
        <>
            <div style={styles.singleCol}>
                <PackageList />
            </div>
            <div style={styles.flexColumn as React.CSSProperties}>
                <div style={styles.singleCol}>
                    <PackageStatistic smallestYear={smallestYear} />
                </div>
            </div>
        </>
    );
};

const Spacer = () => <span style={{ width: '1em' }} />;
const VerticalSpacer = () => <span style={{ height: '1em' }} />;

export default DashboardExchangingManager;