import os
import typing
from datetime import datetime

from dotenv import load_dotenv
from fastapi import FastAPI
from pydantic import BaseModel

from . import database

app = FastAPI()
load_dotenv()
user = os.getenv("POSTGRES_USER")  # Get user from env variables
password = os.getenv("POSTGRES_PASSWORD")  # Get password from env variable
database_name = os.getenv("POSTGRES_DATABASENAME")
host = os.getenv("POSTGES_HOST")
database_url = f"postgres://{user}:{password}@{host}:5432/{database_name}"  # Format url.
database.create_engine(db_url=database_url)
database.initialize()
conn = database.get_session()


class Data(BaseModel):
    pi_id: str
    measurement_time: datetime
    temp1: float
    temp2: float
    temp3: float
    temp4: float
    bme_temp1: float
    bme_temp2: float
    pressure1: float
    pressure2: float
    rh1: float
    rh2: float
    image_data: typing.List[float]


@app.get("/")
async def read_root():
    return {"Hello": "World"}


@app.post("/data")
async def post_data(data: Data):
    print(data)
    print("Fuck")
    return data
