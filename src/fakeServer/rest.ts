import FakeRest from 'fakerest';
import fetchMock from 'fetch-mock';
import generateData from 'data-generator-retail';

export default () => {
    var data = generateData({ serializeDate: true });
    data = {
        ...data,
        points: [
            {id: 0, type: 0, reference: 'AAAAA', name: 'Điểm A', address: 'BBBB', city: 'Hà Nội', zipcode: '84', manager_id: 0, manager_reference: 'ABC', has_manager: 1},
            {id: 1, type: 0, reference: 'AAAAB', name: 'Điểm B', address: 'BBBB', city: 'Hà Nội', zipcode: '84', manager_id: 0, manager_reference: '', has_manager: 0},
        ],
        invoice: [        
            {id: 0, date: '2019-06-01T15:09:29.886Z', command_id: 346, customer_id: 410, total_ex_taxes: 58.46}

        ],
        managerAccounts: [
            {id: 0, type: 1, reference: 'ABC', name: 'Hùng', email: 'hung@gmail.com', birthday:'03/11/1993', password: 'test', point_reference: 'AAAAA', point_id: 0, is_managing: 1},
            {id: 1, type: 2, reference: 'ABD', name: 'Tuấn', email: 'tuan@gmail.com', birthday:'01/08/1992', password: 'test', point_reference: 'AAAAB', point_id: 0, is_managing: 0},
        ],
    }
    const restServer = new FakeRest.FetchServer('http://localhost:4000');
    if (window) {
        window.restServer = restServer; // give way to update data in the console
    }
    console.log(data);
    restServer.init(data);
    restServer.toggleLogging(); // logging is off by default, enable it
    fetchMock.mock('begin:http://localhost:4000', restServer.getHandler());
    return () => fetchMock.restore();
};
