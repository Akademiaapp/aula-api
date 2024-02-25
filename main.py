from typing import Union

from fastapi import FastAPI
from pydantic import BaseModel

from aulaHandler import aulaHandler
from unilogin import unilogin


app = FastAPI()

class User(BaseModel):
    username: str
    password: str

@app.post("/getCalendarEventsUsingUnilogin")
def getCalenderEventsUsingUnilogin(user: User):
    session = unilogin(user.username, user.password)
    print(session.cookies)
    handler = aulaHandler(session=session)
    
    return handler.getCalenderEvents()