from dataclasses import dataclass
from typing import List

from ..utils.number import Number


@dataclass
class StandalonePlayerListItem:
    name: str
    items: dict
    value: float
    value_per_second: float


@dataclass
class PlayerListItem:
    name: str
    items: dict
    value: Number
    value_per_second: Number
    best_value: Number
    percentage: Number


def to_player_list_items(standalone_list: List[StandalonePlayerListItem]) -> List[PlayerListItem]:
    best = 0.0
    combined = 0.0

    for item in standalone_list:
        if item.value > best:
            best = item.value
        combined += item.value

    combined_list: List[PlayerListItem] = []

    for item in standalone_list:
        percentage = item.value / combined * 100 if item.value else 0.0
        combined_list.append(PlayerListItem(
            item.name,
            item.items,
            Number(item.value),
            Number(item.value_per_second),
            Number(best),
            Number(percentage),
        ))

    return combined_list
