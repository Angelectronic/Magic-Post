import * as React from 'react';
import PackageStatistic from './PackageStatistic';
import { useGetList } from 'react-admin';
import RecentDelivery from './RecentDelivery';
import OldPackage from './OldPackage';
const styles = {
    flex: { display: 'flex' },
    flexColumn: { display: 'flex', flexDirection: 'column' },
    leftCol: { flex: 1, marginRight: '0.5em' },
    rightCol: { flex: 1, marginLeft: '0.5em' },
    singleCol: { marginTop: '1em', marginBottom: '1em' },
};

const DashboardExchanging = () => {
    const { data: curPackage, isLoading: isLoading } = useGetList('exchangingPackage');
    if (isLoading) {
        return (
            <></>
        );
    }
    var smallestYear = 9999;
    for (let i = 0; i < curPackage.length; ++i) {
        if ((new Date(curPackage[i].send_date)).getFullYear() < smallestYear) {
            smallestYear = (new Date(curPackage[i].send_date)).getFullYear();
        }
    }
    return (
        <>
            <div style={styles.flex}>
                <RecentDelivery />
                <Spacer />
                <OldPackage />
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

export default DashboardExchanging;