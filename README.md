# magicpost

## Installation

Install the application dependencies by running:

```sh
npm install
```

## Development

Start the application in development mode by running:

```sh
npm run dev
```

## Production

Build the application in production mode by running:

```sh
npm run build
```

## DataProvider

The included data provider use [ra-data-json-server](https://github.com/marmelab/react-admin/tree/master/packages/ra-data-json-server). It fits REST APIs powered by [JSON Server](https://github.com/typicode/json-server), such as [JSONPlaceholder](https://jsonplaceholder.typicode.com/).

You'll find an `.env` file at the project root that includes a `VITE_JSON_SERVER_URL` variable. Set it to the URL of your backend. By default, we set it to targets [JSONPlaceholder](https://jsonplaceholder.typicode.com/).

Frontend: Phạm Vũ Duy & Đỗ Mạnh Dũng

# Backend
Phí Minh Hiếu - 21020200:
- Tạo, thiết kế và duy trì, chỉnh sửa CSDL
- Lập trình Backend theo mô hình MVC sử dụng ngôn ngữ lập trình Rust

First import magicpost1.sql in backend into database name 'magic_post' in Mysql (We recommend use xampp)
Run backend local: (You must have install Rust)
```sh
cd backend
cargo run
```

