import abc


class Stats(abc.ABC):
    @abc.abstractmethod
    def new(self):
        pass

    @abc.abstractclassmethod
    def from_other(other):
        pass

    @abc.abstractmethod
    def update(self, other):
        pass

    @abc.abstractmethod
    def stats(self):
        pass

    def combined(self, other):
        stats = self.new()
        stats.update(self)
        stats.update(other)

        return stats
