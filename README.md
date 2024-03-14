# Aula API
A rather simple API for interaction with the Danish school platform [Aula](https://aulainfo.dk/). When starting the API it opens endpoints for interacting with Aula's functionalities.

## Technologies Used
- Python: Selected Python for backend development due to its simplicity, readability, and extensive library ecosystem.
- FastAPI: FastAPI makes it possible to develop rubust APIs, with rapid iteration and minimal code.
- HTTPX: The python HTTPX library is used to interact with the Aula platform.

# API Documentation

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
