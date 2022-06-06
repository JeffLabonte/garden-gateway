class BaseSchedule:
    """
    The function expected for a schedule
    """

    def run(self, context: dict) -> None:
        raise NotImplementedError("You need to implement the run function")

    def stop(self):
        raise NotImplementedError("You need to implement the stop function")
