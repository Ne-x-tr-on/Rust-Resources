from fastapi import FastAPI
from pydantic import BaseModel

app = FastAPI()

class User(BaseModel):
    name:str

@app.get("/")
def home():
    return {
        "Message": "Welcome to FastApi Server"
    }

@app.post("/user")
def create_user(user:User):
    return{
        "message":f"Hello, {user.name}!"
    }