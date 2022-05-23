from typing import Callable, Optional
from threading import Timer

from scheduler.base import BaseSchedule


class ActionSchedule(BaseSchedule):
    """
    Run an action on an interval when the condition is fullfilled
    """
    def __init__(self, action_fn: Callable, is_condition_fullfilled_fn: Callable, interval_check: float) -> None:
        self._action_fn = action_fn
        self._is_condition_fullfilled = is_condition_fullfilled_fn
        self._interval_check = interval_check
        self._running_thread: Optional[Timer] = None

    def run(self, context: dict) -> None:
        self._running_thread = Timer(self._interval_check, self._run_timer, (context,) )
        self._running_thread.start()

    def _run_timer(self, context: dict) -> None:
        if self._is_condition_fullfilled(context=context):
            self._action_fn(context=context)

        self._running_thread = Timer(self._interval_check, self._running_thread, (context,))
        self._running_thread.start()

    def stop(self) -> None:
        self._running_thread.cancel()
