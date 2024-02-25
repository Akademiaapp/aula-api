# Aula API
A rather simple API for interaction with the Danish school platform Aula. When starting the API it opens endpoints for interacting with Aula's functionalities. Built with Python, FastAPI, and httpx.

## Technologies Used
- Python: Selected Python for backend development due to its simplicity, readability, and extensive library ecosystem.
- FastAPI: FastAPI makes it possible to develop rubust APIs, with rapid iteration and minimal code.
- HTTPX: The python HTTPX library is used to interact with the Aula platform.

## Endpoints
The API currently provides the following endpoints:

### 1. /getCalendarEventsUsingUnilogin
- **Method:** POST
- **Description:** Retrieves calendar events using UniLogin credentials.
- **Request Body:**
  - `username` (string): The UniLogin username.
  - `password` (string): The UniLogin password.
- **Response:**
  - Returns the calendar events for the current week from Aula.

### Todo
- Add more endpoints for more functionality.
- Implement better login handling using sessions for improved speed.
