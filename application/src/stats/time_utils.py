from datetime import datetime


def now():
    return datetime.now()


def delta(time):
    delta = datetime.now() - time
    return (delta.seconds * 1000.0 + delta.microseconds / 1000.0)


def as_seconds(time):
    return time * 1000.0