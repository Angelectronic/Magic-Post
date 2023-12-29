import FakeRest from 'fakerest';
import { DataProvider } from 'ra-core';
import generateData from 'data-generator-retail';
/* eslint-disable no-console */
function log(type, resource, params, response) {
    if (console.group) {
        // Better logging in Chrome
        console.groupCollapsed(type, resource, JSON.stringify(params));
        console.log(response);
        console.groupEnd();
    } else {
        console.log('FakeRest request ', type, resource, params);
        console.log('FakeRest response', response);
    }
}

/**
 * Respond to react-admin data queries using a local JavaScript object
 *
 * Useful for debugging and testing - do not use in production.
 *
 * @example
 *
 * import fakeDataProvider from 'ra-data-fakerest';
 * const dataProvider = fakeDataProvider({
 *   posts: [
 *     { id: 0, title: 'Hello, world!' },
 *     { id: 1, title: 'FooBar' },
 *   ],
 *   comments: [
 *     { id: 0, post_id: 0, author: 'John Doe', body: 'Sensational!' },
 *     { id: 1, post_id: 0, author: 'Jane Doe', body: 'I agree' },
 *   ],
 * })
 */
export default (data, loggingEnabled = false): DataProvider => {
    const restServer = new FakeRest.Server();
    restServer.init(data);
    if (typeof window !== 'undefined') {
        // give way to update data in the console
        (window as any).restServer = restServer;
    }

    async function removePoint(id: any) {
        const request = new Request(`http://localhost:8080/delete/point/${id}`, {
            method: 'DELETE',
            headers: new Headers({ 'Content-Type': 'application/json' }),
            credentials: "include",
        });
        let response = await fetch(request);
        if (response.status < 200 || response.status >= 300) {
            throw new Error(response.statusText);
        }
    }

    async function removeManager(id: any) {
        const request = new Request(`http://localhost:8080/delete/leader/${id}`, {
            method: 'DELETE',
            headers: new Headers({ 'Content-Type': 'application/json' }),
            credentials: "include",
        });
        let response = await fetch(request);
        if (response.status < 200 || response.status >= 300) {
            throw new Error(response.statusText);
        }

    }

    async function removeEmployee(id: any) {
        const request = new Request(`http://localhost:8080/delete_employee/${id}`, {
            method: 'DELETE',
            headers: new Headers({ 'Content-Type': 'application/json' }),
            credentials: "include",
        });
        let response = await fetch(request);
        if (response.status < 200 || response.status >= 300) {
            throw new Error(response.statusText);
        }

    }
    function initField(json) {
        for (var key in json) {
            if (json[key] === undefined) {
                json[key] = '';
            }
        }
        return json;
    }
    async function fetchPoint() {
        const request = new Request('http://localhost:8080/points/all', {
            method: 'GET',
            headers: new Headers({ 'Content-Type': 'application/json' }),
            credentials: "include",
        });
        let response = await fetch(request);
        if (response.status < 200 || response.status >= 300) {
            throw new Error(response.statusText);
        }
        let json = await response.json();
        for (let i = 0; i < json.length; i++) {
            let obj = json[i];
            obj.p_type = parseInt(obj.p_type);
        }
        var data_points = {
            points: json
        };
        restServer.init(data_points);

    }

    async function fetchEmployee1() {
        const request = new Request('http://localhost:8080/leader/view_employees', {
            method: 'GET',
            headers: new Headers({ 'Content-Type': 'application/json' }),
            credentials: "include",
        });
        let response = await fetch(request);
        if (response.status < 200 || response.status >= 300) {
            throw new Error(response.statusText);
        }
        let json = await response.json();
        var data_em = {
            exchangingEmployeeAccounts: json
        };
        restServer.init(data_em);

    }

    async function fetchEmployee2() {
        const request = new Request('http://localhost:8080/leader/view_employees', {
            method: 'GET',
            headers: new Headers({ 'Content-Type': 'application/json' }),
            credentials: "include",
        });
        let response = await fetch(request);
        if (response.status < 200 || response.status >= 300) {
            throw new Error(response.statusText);
        }
        let json = await response.json();
        var data_em = {
            gatheringEmployeeAccounts: json
        };
        restServer.init(data_em);

    }

    async function fetchPackage1(point_id: any) {
        const request1 = new Request(`http://localhost:8080/points/all`, {
            method: 'GET',
            headers: new Headers({ 'Content-Type': 'application/json' }),
            credentials: "include",
        });
        let response1 = await fetch(request1);
        if (response1.status < 200 || response1.status >= 300) {
            throw new Error(response1.statusText);
        }

        const request = new Request(`http://localhost:8080/packages/to/${point_id}`, {
            method: 'GET',
            headers: new Headers({ 'Content-Type': 'application/json' }),
            credentials: "include",
        });
        let response = await fetch(request);
        if (response.status < 200 || response.status >= 300) {
            throw new Error(response.statusText);
        }
        let json = await response.json();

        const request2 = new Request(`http://localhost:8080/packages/at/${point_id}`, {
            method: 'GET',
            headers: new Headers({ 'Content-Type': 'application/json' }),
            credentials: "include",
        });
        let response2 = await fetch(request2);
        if (response2.status < 200 || response2.status >= 300) {
            throw new Error(response2.statusText);
        }
        let json1 = await response2.json();
        let json2 = json.concat(json1);
        console.log(json2);
        var data_package = {
            exchangingPackage: json2
        };
        restServer.init(data_package);

    }

    async function fetchPackage2(point_id: any) {
        const request1 = new Request(`http://localhost:8080/points/all`, {
            method: 'GET',
            headers: new Headers({ 'Content-Type': 'application/json' }),
            credentials: "include",
        });
        let response1 = await fetch(request1);
        if (response1.status < 200 || response1.status >= 300) {
            throw new Error(response1.statusText);
        }

        const request = new Request(`http://localhost:8080/packages/to/${point_id}`, {
            method: 'GET',
            headers: new Headers({ 'Content-Type': 'application/json' }),
            credentials: "include",
        });
        let response = await fetch(request);
        if (response.status < 200 || response.status >= 300) {
            throw new Error(response.statusText);
        }
        let json = await response.json();

        const request2 = new Request(`http://localhost:8080/packages/at/${point_id}`, {
            method: 'GET',
            headers: new Headers({ 'Content-Type': 'application/json' }),
            credentials: "include",
        });
        let response2 = await fetch(request2);
        if (response2.status < 200 || response2.status >= 300) {
            throw new Error(response2.statusText);
        }
        let json1 = await response2.json();
        let json2 = json.concat(json1);
        console.log(json2);
        var data_package = {
            gatheringPackage: json2
        };
        restServer.init(data_package);

    }

    async function fetchLeader() {
        const request = new Request('http://localhost:8080/leaders/all', {
            method: 'GET',
            headers: new Headers({ 'Content-Type': 'application/json' }),
            credentials: "include",
        });
        let response = await fetch(request);
        if (response.status < 200 || response.status >= 300) {
            throw new Error(response.statusText);
        }
        let json = await response.json();
        for (let i = 0; i < json.length; i++) {
            let obj = json[i];
            obj.m_type = parseInt(obj.m_type);
        }
        var data_leader = {
            managerAccounts: json
        };
        restServer.init(data_leader);
    }
    async function getResponse(type, resource, params) {
        switch (type) {
            case 'getList': {
                const { page, perPage } = params.pagination;
                const { field, order } = params.sort;
                const query = {
                    sort: [field, order],
                    range: [(page - 1) * perPage, page * perPage - 1],
                    filter: params.filter,
                };
                return {
                    data: restServer.getAll(resource, query),
                    total: restServer.getCount(resource, {
                        filter: params.filter,
                    }),
                };
            }
            case 'getOne':

                return {
                    data: restServer.getOne(resource, params.id, { ...params }),
                };
            case 'getMany':
                return {
                    data: params.ids.map(
                        id => restServer.getOne(resource, id),
                        { ...params }
                    ),
                };
            case 'getManyReference': {
                const { page, perPage } = params.pagination;
                const { field, order } = params.sort;
                const query = {
                    sort: [field, order],
                    range: [(page - 1) * perPage, page * perPage - 1],
                    filter: { ...params.filter, [params.target]: params.id },
                };
                return {
                    data: restServer.getAll(resource, query),
                    total: restServer.getCount(resource, {
                        filter: query.filter,
                    }),
                };
            }
            case 'update':
                if (resource === 'points') {
                    params.data.p_type = params.data.p_type.toString();
                    params.data = initField(params.data);
                    const request = new Request(`http://localhost:8080/update/point/${params.data.id}`, {
                        method: 'PUT',
                        headers: new Headers({ 'Content-Type': 'application/json' }),
                        body: JSON.stringify(params.data),
                        credentials: "include",
                    });
                    let response = await fetch(request);
                    if (response.status < 200 || response.status >= 300) {
                        throw new Error(response.statusText);
                    }

                }
                if (resource === 'managerAccounts') {
                    params.data = initField(params.data);
                    const request = new Request(`http://localhost:8080/update/leader/${params.data.id}`, {
                        method: 'PUT',
                        headers: new Headers({ 'Content-Type': 'application/json' }),
                        body: JSON.stringify(params.data),
                        credentials: "include",
                    });
                    let response = await fetch(request);
                    if (response.status < 200 || response.status >= 300) {
                        throw new Error(response.statusText);
                    }
                }
                if (resource === 'exchangingEmployeeAccounts' || resource === 'gatheringEmployeeAccounts') {
                    params.data = initField(params.data);
                    const request = new Request(`http://localhost:8080/leader/update_employee/${params.data.id}`, {
                        method: 'PUT',
                        headers: new Headers({ 'Content-Type': 'application/json' }),
                        body: JSON.stringify(params.data),
                        credentials: "include",
                    });
                    let response = await fetch(request);
                    if (response.status < 200 || response.status >= 300) {
                        throw new Error(response.statusText);
                    }
                }

                if (resource === 'exchangingPackage' ) {
                    params.data = initField(params.data);
                    const request = new Request(`http://localhost:8080/subordinate/update/${params.data.id}`, {
                        method: 'PUT',
                        headers: new Headers({ 'Content-Type': 'application/json' }),
                        body: JSON.stringify(params.data),
                        credentials: "include",
                    });
                    let response = await fetch(request);
                    if (response.status < 200 || response.status >= 300) {
                        throw new Error(response.statusText);
                    }
                }
                return {
                    data: restServer.updateOne(resource, params.id, {
                        ...params.data,
                    }),
                };
            case 'updateMany':
                params.ids.forEach(id =>
                    restServer.updateOne(resource, id, {
                        ...params.data,
                    })
                );
                return { data: params.ids };
            case 'create':
                if (resource === 'points') {
                    params.data.p_type = params.data.p_type.toString();
                    params.data = initField(params.data);
                    const request = new Request('http://localhost:8080/add/point', {
                        method: 'POST',
                        headers: new Headers({ 'Content-Type': 'application/json' }),
                        body: JSON.stringify(params.data),
                        credentials: "include",
                    });
                    let response = await fetch(request);
                    if (response.status < 200 || response.status >= 300) {
                        throw new Error(response.statusText);
                    }
                }

                if (resource === 'managerAccounts') {
                    params.data = initField(params.data);
                    const request = new Request('http://localhost:8080/add/leader', {
                        method: 'POST',
                        headers: new Headers({ 'Content-Type': 'application/json' }),
                        body: JSON.stringify(params.data),
                        credentials: "include",
                    });
                    let response = await fetch(request);
                    if (response.status < 200 || response.status >= 300) {
                        throw new Error(response.statusText);
                    }
                }

                if (resource === 'exchangingEmployeeAccounts' || resource === 'gatheringEmployeeAccounts') {
                    params.data = initField(params.data);
                    const request = new Request('http://localhost:8080/leader/add_employee', {
                        method: 'POST',
                        headers: new Headers({ 'Content-Type': 'application/json' }),
                        body: JSON.stringify(params.data),
                        credentials: "include",
                    });
                    let response = await fetch(request);
                    if (response.status < 200 || response.status >= 300) {
                        throw new Error(response.statusText);
                    }
                }

                if (resource === 'exchangingPackage') {
                    params.data = initField(params.data);
                    const request = new Request('http://localhost:8080/subordinate/add_package', {
                        method: 'POST',
                        headers: new Headers({ 'Content-Type': 'application/json' }),
                        body: JSON.stringify(params.data),
                        credentials: "include",
                    });
                    let response = await fetch(request);
                    if (response.status < 200 || response.status >= 300) {
                        throw new Error(response.statusText);
                    }
                }
                console.log(params.data);
                return {
                    data: restServer.addOne(resource, { ...params.data }),
                };
            case 'delete':
                if (resource === 'points') {
                    removePoint(params.id);
                }
                if (resource === 'managerAccounts') {
                    removeManager(params.id);
                }
                if (resource === 'exchangingEmployeeAccounts' || resource === 'gatheringEmployeeAccounts') {
                    removeEmployee(params.id);
                }
                return { data: restServer.removeOne(resource, params.id) };
            case 'deleteMany':
                params.ids.forEach(id => {
                    if (resource === 'points') {
                        removePoint(id);
                    }
                    if (resource === 'managerAccounts') {
                        removeManager(id);
                    }
                    restServer.removeOne(resource, id)
                });
                if (resource === 'points') {
                    await fetchPoint();
                }
                return { data: params.ids };
            default:
                return false;
        }
    }

    /**
     * @param {String} type One of the data Provider methods, e.g. 'getList'
     * @param {String} resource Name of the resource to fetch, e.g. 'posts'
     * @param {Object} params The data request params, depending on the type
     * @returns {Promise} The response
     */
    const handle = async (type, resource, params): Promise<any> => {
        if (resource === 'points') {
            await fetchPoint();
        }
        if (resource === 'managerAccounts') {
            await fetchLeader();
        }
        if (resource === 'exchangingPackage') {
            await fetchPackage1(localStorage.getItem("point_id"));
        }
        if (resource === 'gatheringPackage') {
            await fetchPackage2(localStorage.getItem("point_id"));
        }
        if (resource === 'exchangingEmployeeAccounts') {
            await fetchEmployee1();
        }
        if (resource === 'gatheringEmployeeAccounts') {
            await fetchEmployee2();
        }
        const collection = restServer.getCollection(resource);
        if (!collection && type !== 'create') {
            const error = new UndefinedResourceError(
                `Undefined collection "${resource}"`
            );
            error.code = 1; // make that error detectable
            return Promise.reject(error);
        }
        let response;
        try {
            response = getResponse(type, resource, params);
        } catch (error) {
            console.error(error);
            return Promise.reject(error);
        }
        if (loggingEnabled) {
            log(type, resource, params, response);
        }
        return Promise.resolve(response);
    };

    return {
        getList: (resource, params) => handle('getList', resource, params),
        getOne: (resource, params) => handle('getOne', resource, params),
        getMany: (resource, params) => handle('getMany', resource, params),
        getManyReference: (resource, params) =>
            handle('getManyReference', resource, params),
        update: (resource, params) => handle('update', resource, params),
        updateMany: (resource, params) =>
            handle('updateMany', resource, params),
        create: (resource, params) => handle('create', resource, params),
        delete: (resource, params) => handle('delete', resource, params),
        deleteMany: (resource, params) =>
            handle('deleteMany', resource, params),
    };
};

class UndefinedResourceError extends Error {
    code: number;
}
