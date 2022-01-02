import logging

from datetime import datetime
from threading import Lock, Timer
from time import sleep

import RPi.GPIO as GPIO

logging.basicConfig(filename="/var/log/garden-gateway.log", level=logging.INFO)

POWER_RELAY_PIN = 17

TIME_DATETIME_FORMAT = "%H:%M"

START_LIGHT = "8:00"
END_LIGHT = "20:00"


MAIN_LOOP_LOCK = Lock()

COUNTERS = {
    "light": 0,
    "darkness": 0,
}

TIMER_THREAD_INTERVAL_MINUTE = 5 * 60


def turn_on_lamp():
    """
    Turn on Normal ON
    """
    GPIO.output(POWER_RELAY_PIN, GPIO.LOW)


def turn_off_lamp():
    """
    Turn off lamp (Normally ON) but turns off heater (Normally OFF)
    """
    GPIO.output(POWER_RELAY_PIN, GPIO.HIGH)


def setup_board():
    GPIO.setmode(GPIO.BCM)
    GPIO.setup(POWER_RELAY_PIN, GPIO.OUT)


def set_counters(key: str, reset_key: str):
    COUNTERS[key] = COUNTERS[key] + 1
    COUNTERS[reset_key] = 0


def run_schedule():
    start_light_datetime = datetime.strptime(START_LIGHT, TIME_DATETIME_FORMAT)
    end_light_datetime = datetime.strptime(END_LIGHT, TIME_DATETIME_FORMAT)

    now = datetime.now()
    logging.info(f"# Current Time: {now.hour}: {now.minute}")

    if end_light_datetime.hour > now.hour > start_light_datetime.hour or (
        (start_light_datetime.hour == now.hour)
        and now.minute > start_light_datetime.minute
    ):
        if COUNTERS["light"] == 0:
            turn_on_lamp()
        set_counters(key="light", reset_key="darkness")
        logging.info("Light")
    else:
        if COUNTERS["darkness"] == 0:
            turn_off_lamp()
        set_counters(key="darkness", reset_key="light")
        logging.info("Darkness...")

    MAIN_LOOP_LOCK.release()


def main():
    setup_board()
    while True:
        if not MAIN_LOOP_LOCK.locked():
            logging.info("start running")
            MAIN_LOOP_LOCK.acquire()
            timer_thread = Timer(
                interval=TIMER_THREAD_INTERVAL_MINUTE,
                function=run_schedule,
            )
            timer_thread.start()
            logging.info(f"Running Timer in {TIMER_THREAD_INTERVAL_MINUTE}")
        sleep(10)


if __name__ == "__main__":
    main()
