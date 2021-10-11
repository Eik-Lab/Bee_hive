"""
import os
os.chdir(f"{os.getcwd()}/bee_hive")
import typing
from datetime import datetime

from dotenv import load_dotenv
from pydantic import BaseModel
from flask import Flask, request
from . import database
load_dotenv()

app = Flask(__name__)
user = os.getenv("POSTGRES_USER")  # Get user from env variables
password = os.getenv("POSTGRES_PASSWORD")  # Get password from env variable
database_name = os.getenv("POSTGRES_DATABASENAME")
host = os.getenv("POSTGES_HOST")
database_url = f"postgres://{user}:{password}@{host}:5432/{database_name}"  # Format url.
database.create_engine(db_url=database_url)
database.initialize()
conn = database.get_session()
"""

# app.py
from flask import Flask, request, jsonify
app = Flask(__name__)

@app.route('/data/', methods=['POST'])
def post_something():
    json = request.get_json()
    print(json)
    return None



@app.route('/')
def index():
    return "<h1>Welcome to our server !!</h1>"

if __name__ == '__main__':
    # Threaded option to enable multiple instances for multiple user access support
    app.run(threaded=True, port=5000)
