from sched import scheduler
from time import time, sleep
from datetime import datetime
from typing import Callable
from scheduler.base import BaseSchedule


class TimeSchedule(BaseSchedule):
    def __init__(self, action_fn: Callable, run_on_date: datetime, interval_sec: int) -> None:
        self._scheduler = scheduler(time, sleep)
        self._inteval_sec = interval_sec

    def run(self, context: dict) -> None:
        pass

    def _compute_time(self):
        pass
