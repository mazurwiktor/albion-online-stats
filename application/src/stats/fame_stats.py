from dataclasses import dataclass

from ..event_receiver import FameEventReceiver


@dataclass
class FameStats(FameEventReceiver):
    fame: float = 0.0

    def __init__(self):
        self.fame = 0.0

    def on_fame_update(self, value: float):
        self.fame += value

