import { AuthProvider } from 'react-admin';
import { useLocation } from 'react-router-dom'



const authProvider = {
    login: async (auth: LoginValues) => {
        if (auth.username === undefined) {
            auth.username = 'guest';
        }
        if (auth.isGuest == true) {
            localStorage.setItem('role', 'guest');
            localStorage.setItem('id', 'guest');
            localStorage.setItem('fullName', 'Khách');
        }
        else {

            const request = new Request(`http://localhost:8080/login`, {
                method: 'POST',
                headers: new Headers({ 'Content-Type': 'application/json' }),
                body: JSON.stringify({ username: auth.username, password: auth.password }),
                credentials: "include",
            });
            let response = await fetch(request);

            if (response.status < 200 || response.status >= 300) {
                let body = await response.text()
                return {
                    error: response.status
                };
            }

            let json = await response.json()

            json.error = 0;
            var role = '';
            if (json.position === 'CEO') {
                role = 'admin';
                localStorage.setItem("fullName", "Lãnh đạo");
            }
            else {
                localStorage.setItem("birthday", json.birthday);
                localStorage.setItem("email", json.email);
                localStorage.setItem("id", json.id);
                localStorage.setItem("name", json.name);
                localStorage.setItem("phone", json.phone);
                localStorage.setItem("current_point_id", json.point_id);
                localStorage.setItem("reference", json.reference);
                localStorage.setItem("point_reference", json.point_reference);
                localStorage.setItem("sex", json.sex);
            }
            if (json.position === 'leader') {
                if (json.m_type === "1") {
                    role = 'exchanging_manager';
                    localStorage.setItem("fullName", "Trưởng điểm giao dịch");
                }
                else {
                    role = 'gathering_manager';
                    localStorage.setItem("fullName", "Trưởng điểm tập kết");
                }

            }
            if (json.position === 'subordinate') {
                if (json.point_reference.startsWith("GD")) {
                    role = 'exchanging_employee';
                    localStorage.setItem("fullName", "Nhân viên giao dịch");
                }
                else {
                    role = 'gathering_employee';
                    localStorage.setItem("fullName", "Nhân viên tập kết");
                }
            }
            localStorage.setItem("role", role);
            return json;
            localStorage.setItem('role', auth.username);
            localStorage.setItem('id', 'guest');
            localStorage.setItem('fullName', 'Khách');
            return {
                error: 0
            };
        }
    },
    logout: () => {
        localStorage.removeItem('role');
        localStorage.removeItem("birthday");
        localStorage.removeItem("email");
        localStorage.removeItem("id");
        localStorage.removeItem("name");
        localStorage.removeItem("phone");
        localStorage.removeItem("current_point_id");
        localStorage.removeItem("reference");
        localStorage.removeItem("point_reference");
        localStorage.removeItem("sex");
        localStorage.removeItem("fullName");
        return Promise.resolve();
    },
    checkError: (error) => {
        const status = error.status;
        console.log(status);
        return Promise.resolve();
    },
    checkAuth: (params: any) => {
        if (window.location.hash === '#/search' || (window.location.hash === '#/' && localStorage.getItem("role") === null)) {
            localStorage.setItem('role', 'guest');
            return Promise.resolve();
        }

        if (params.checkRole === undefined) {
            return localStorage.getItem('role') !== null ? Promise.resolve() : Promise.reject();
        }
        return localStorage.getItem('role') === params.checkRole ? Promise.resolve() : Promise.reject();
    },
    getPermissions: () => {
        const role = localStorage.getItem('role');
        return Promise.resolve(role);
    },
    getIdentity: () => {
        var id = localStorage.getItem('id');
        if (id == null) {
            id = 'guest';
        }
        var fullName = localStorage.getItem('fullName');
        if (fullName == null) {
            fullName = 'Khách';
        }
        return Promise.resolve({
            id: id,
            fullName: fullName
        })
    },
    getRole: () => {
        var role = localStorage.getItem('role');
        if (role === null) {
            role = 'guest';
        }
        return role;
    }
};

export default authProvider;

interface LoginValues {
    username?: string;
    password?: string;
    isGuest?: boolean;
}