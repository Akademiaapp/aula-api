from typing import Union

from fastapi import FastAPI
from pydantic import BaseModel

from aulaHandler import aulaHandler

from unilogin import unilogin

app = FastAPI()

@app.get("/")
def read_root():
    return {"Hello": "World"}


@app.get("/items/{item_id}")
def read_item(item_id: int, q: Union[str, None] = None):
    return {"item_id": item_id, "q": q}


class User(BaseModel):
    username: str
    password: str

@app.post("/getCalenderEventsUsingUnilogin")
def getCalenderEventsUsingUnilogin(user: User):
    session = unilogin(user.username, user.password)
    print(session.cookies)
    handler = aulaHandler(session=session)
    
    return handler.getCalenderEvents()

@app.post("/login")
def login(user: User):
    return {"username": user.username}
