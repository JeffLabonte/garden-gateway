import RPi.GPIO as GPIO

POWER_RELAY_PIN = 17


class RelayPowerBar:

    def __init__(self, pin: int = POWER_RELAY_PIN):
        self._pin = pin
        self._setup_board()

    def _setup_board(self):
        GPIO.setmode(GPIO.BCM)
        GPIO.setup(self._pin, GPIO.OUT)

    def set_relay_on(self):
        """
        Turn on Normal ON
        """
        GPIO.output(POWER_RELAY_PIN, GPIO.LOW)

    def set_relay_off(self):
        """
        Normal ON goes OFF and normal ON goes ON
        """
        GPIO.output(POWER_RELAY_PIN, GPIO.HIGH)
