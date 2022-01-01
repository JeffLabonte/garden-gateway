from datetime import datetime
from threading import Lock, Timer
from time import sleep

import RPi.GPIO as GPIO

POWER_RELAY_PIN = 17

TIME_DATETIME_FORMAT = "%H:%M"

START_LIGHT = "8:00"
END_LIGHT = "20:00"


MAIN_LOOP_LOCK = Lock()

COUNTERS = {
    "light": 0,
    "darkness": 0,
}


def turn_on_lamp():
    GPIO.ouput(POWER_RELAY_PIN, GPIO.LOW)


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


def main():
    start_light_datetime = datetime.strptime(START_LIGHT, TIME_DATETIME_FORMAT)
    end_light_datetime = datetime.strptime(END_LIGHT, TIME_DATETIME_FORMAT)

    now = datetime.now()

    if (
        end_light_datetime.hour > now.hour > start_light_datetime.hour
        and end_light_datetime.minutes > now.minute > start_light_datetime.minute
    ):
        if COUNTERS["light"] == 0:
            turn_on_lamp()
        set_counters(key="light", reset_key="darkness")
        print("Light")
    else:
        if COUNTERS["darkness"] == 0:
            turn_off_lamp()
        set_counters(key="darkness", reset_key="light")
        print("Darkness...")

    print(start_light_datetime)
    print(end_light_datetime)
    MAIN_LOOP_LOCK.release()


if __name__ == "__main__":
    setup_board()
    while True:
        print("******")
        if not MAIN_LOOP_LOCK.locked():
            print("start running")
            MAIN_LOOP_LOCK.acquire()
            timer_thread = Timer(interval=1, function=main)
            timer_thread.start()
            print("Everything Ran")
        sleep(10)
