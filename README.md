# Aula API
A rather simple API for interaction with the Danish school platform [Aula](https://aulainfo.dk/). When starting the API it opens endpoints for interacting with Aula's functionalities.


## Technologies Used
- [Rust](https://www.rust-lang.org/) - A systems programming language that runs blazingly fast, prevents segfaults, and guarantees thread safety.
- [Cargo](https://doc.rust-lang.org/cargo/) - Package manager for Rust.
- [Actix](https://actix.rs/) - A powerful, pragmatic, and extremely fast web framework for Rust. Used for the APi endpoints.
- [reqwest](https://docs.rs/reqwest/0.11.4/reqwest/) - An ergonomic, batteries-included HTTP Client for Rust.

# API Documentation


## Running the API

To run the API, you will need to have Rust and Cargo installed on your machine. If you don't have them installed, you can download them from [here](https://www.rust-lang.org/tools/install).

Once you have Rust and Cargo installed, you can clone the repository and navigate to the project directory:

```bash
git clone https://github.com/Akademiaapp/aula-api
cd aula-api
```

Then, you can run the API using Cargo. By default, the API will run on port 8080. If you want to specify a different port, you can do so by providing it as a command line argument:

```bash
cargo run <port>
```

For example, to run the API on port 3000, you would use the following command:

```bash
cargo run 3000
```

The API will now be running and listening for requests on the specified port.

## Endpoints
The API currently provides the following endpoints:

### `/login`
- Method: POST
- Description: This endpoint is used to authenticate a user and initiate a session.
- Request Body:
  - `username` (String): The username of the user trying to log in.
  - `password` (String): The password of the user trying to log in.
- Response:
  - Returns a JSON object containing the login information of the user.

### `/getCalenderEvents`
- Method: POST
- Description: This endpoint is used to retrieve calendar events for a user within a specified date range.
- Request Body:
  - `login_info` (LoginInfo): The login information of the user.
  - `start` (String): The start date of the range in which to retrieve events.
  - `end` (String): The end date of the range in which to retrieve events.
- Response:
  - Returns a JSON object containing the events within the specified date range.

### `/getNotifications`
- Method: POST
- Description: This endpoint is used to retrieve notifications for a user.
- Request Body:
  - `login_info` (LoginInfo): The login information of the user.
- Response:
  - Returns a JSON object containing the notifications for the user.

### Todo
- Add more endpoints for more functionality.
- Implement better login handling using sessions for improved speed.
