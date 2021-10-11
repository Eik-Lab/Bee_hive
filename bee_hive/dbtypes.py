"""
Generalized database types
"""
import datetime
import uuid

import pytz
from sqlalchemy import DateTime
from sqlalchemy.dialects.postgresql import UUID
from sqlalchemy.types import CHAR, TypeDecorator


class Timestamp(TypeDecorator):
    """
    Based on: https://mike.depalatis.net/blog/sqlalchemy-timestamps.html

    Variation of DateTime type that always stores as UTC
    """

    impl = DateTime
    tz_local = datetime.datetime.utcnow().astimezone().tzinfo

    def process_bind_param(self, value: datetime, dialect):
        if value is None:
            return
        elif value.tzinfo is None:
            value = value.astimezone(self.tz_local)

        return value.astimezone(pytz.utc)

    def process_result_value(self, value, dialect):
        if value is None:
            return

        return value.astimezone(pytz.utc)
