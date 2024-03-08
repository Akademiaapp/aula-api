from typing import Union

from fastapi import FastAPI
from fastapi.middleware.cors import CORSMiddleware
from pydantic import BaseModel

from aulaHandler import aulaHandler
from unilogin import unilogin

app = FastAPI()

origins = ["*"]

app.add_middleware(
    CORSMiddleware,
    allow_origins=origins,
    allow_credentials=True,
    allow_methods=["*"],
    allow_headers=["*"],
)

class User(BaseModel):
    username: str
    password: str

@app.post("/getCalendarEventsUsingUnilogin")
def getCalenderEventsUsingUnilogin(user: User):
    session = unilogin(user.username, user.password)
    print(session.cookies)
    handler = aulaHandler(session=session)
    
    return handler.getCalenderEvents()
