from dataclasses import field
from dataclasses import dataclass

from . import time_utils
from .statistics import Stats


@dataclass
class TimeStats(Stats):
    time: float = field(default_factory=lambda: time_utils.now())

    @classmethod
    def new(self):
        return TimeStats()

    def stats(self):
        return {
            'seconds_in_game': time_utils.as_seconds(time_utils.delta(self.time))
        }

    @classmethod
    def from_other(other):
        return TimeStats(time=other.time)

    def update(self, other):
        if other.time < self.time:
            self.time = other.time
