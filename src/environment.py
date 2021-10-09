import os

TEST_ENV_ENABLED = bool(os.getenv('TESTING'))


class TestType:
    Simple = 'simple'
    ZoneChange = 'zone_change'
    Healing = 'healing'


_test_types = {
    TestType.Simple: TestType.Simple,
    TestType.ZoneChange: TestType.ZoneChange,
    TestType.Healing: TestType.Healing
}

if TEST_ENV_ENABLED:
    test_type = os.getenv('TESTING')

    if test_type not in _test_types:
        raise Exception(f'Unavailable GUI test type: {test_type}, avaliable values: {",".join(_test_types.keys())}')

    TEST_TYPE = _test_types[test_type.lower()]
