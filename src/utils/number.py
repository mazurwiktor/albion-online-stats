class Number:
    def __init__(self, number):
        self.number = number

    def __eq__(self, other):
        return self.number == other

    def __lt__(self, other):
        if isinstance(other, Number):
            return self.number < other.number
        return self.number < other

    def __truediv__(self, other):
        if isinstance(other, Number):
            return self.number / other.number if other.number else other.number
        return self.number / other if other else other

    def __str__(self):
        if self.number / 1000 < 1:
            return "{0:.2f}".format(self.number)
        elif self.number / 1000000 >= 1:
            return "{0:.2f}M".format(self.number / 1000000)
        elif self.number / 1000 >= 1:
            return "{0:.2f}K".format(self.number / 1000)
        return "{0:.2f}".format(self.number)
