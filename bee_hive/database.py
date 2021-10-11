import datetime
import typing
import uuid

import sqlalchemy as sa
from sqlalchemy.ext.declarative import declarative_base
from sqlalchemy.orm import scoped_session, sessionmaker

engine = None
Base = declarative_base()


def create_engine(db_url=None, echo=False):
    """
    Creates the engine, defaults to in-memory sqlite for testing

    :param str db_url: Database uri, including credentials and database
    :param bool echo: Echoes database instructions
    """
    db_url = db_url or "sqlite+pysqlite:///:memory:?check_same_thread=False"

    global engine
    engine = sa.create_engine(db_url, echo=echo)
    return engine


def get_session():
    """
    Get database connection
    """
    if engine is None:
        create_engine(echo=True)

    return scoped_session(sessionmaker(autocommit=False, autoflush=False, bind=engine))


def initialize():
    """
    Creates the tables in Base.mapper
    """
    if engine is None:
        create_engine(echo=True)

    Base.metadata.create_all(engine)


class Table:
    pi_id = sa.Column("1", primary_key=True, default=lambda: uuid.uuid4())
    __tablename__ = None

    def __repr__(self):
        return f"<Table<{self.__tablename__}({str(self.id)[:8]})>>"


class Measurements(Table, Base):
    """
    Represents sensor data
    """

    __tablename__ = "measurements"

    def __init__(
        self,
        pi_id: str,
        measurement_time: datetime,
        temp1: float,
        temp2: float,
        temp3: float,
        temp4: float,
        bme_temp1: float,
        bme_temp2: float,
        pressure1: float,
        pressure2: float,
        rh1: float,
        rh2: float,
        image_data: typing.List[float],
    ) -> None:
        self.pi_id = pi_id
        self.measurement_time = measurement_time
        self.temp1 = temp1
        self.temp2 = temp2
        self.temp3 = temp3
        self.temp4 = temp4
        self.bme_temp1 = bme_temp1
        self.bme_temp2 = bme_temp2
        self.pressure1 = pressure1
        self.pressure2 = pressure2
        self.rh1 = rh1
        self.rh2 = rh2
        self.image_data = image_data
