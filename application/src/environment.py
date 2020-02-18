import os

TEST_ENV_ENABLED = bool(os.getenv('TESTING'))

class TestType:
    Simple = 1

_test_types = {
    'simple': TestType.Simple
}

if TEST_ENV_ENABLED:
    test_type = os.getenv('TESTING')

    if test_type not in _test_types:
        raise Exception('Unavailable GUI test type, avaliable values: {}'.format(','.join(_test_types.keys())))

    TEST_TYPE = _test_types[os.getenv('TESTING').lower()]