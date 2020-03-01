from . import time_utils


class TimeStats:
    def __init__(self):
        self.time = time_utils.now()

    def stats(self):
        return time_utils.delta(self.time)