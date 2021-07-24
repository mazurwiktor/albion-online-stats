from dataclasses import dataclass

from ..event_receiver import FameEventReceiver
from .statistics import Stats


@dataclass
class FameStats(FameEventReceiver, Stats):
    fame: float = 0.0

    @classmethod
    def new(self):
        return FameStats()

    @classmethod
    def from_other(other: Stats):  # type: ignore
        return FameStats(fame=other.fame)  # type: ignore

    def update(self, other: Stats):  # type: ignore
        self.fame += other.fame  # type: ignore

    def on_fame_update(self, value: float):
        self.fame += value

    def stats(self):
        return {
            'fame': self.fame,
            'fame_per_hour': 0.0  # TODO: remove, added because backward compatibility
        }
