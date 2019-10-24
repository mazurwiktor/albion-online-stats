import pytest

from number import Number

def test_number_equality():
    n = 10
    number = Number(10)

    assert number == n

    other = Number(n)

    assert number == other

def test_less_comparator():
    assert Number(10) < Number(20)
    assert Number(10) < 20

def test_div():
    assert Number(10) / Number(10) == 1
    assert Number(10) / 10 == 1

@pytest.mark.parametrize('num,text', [
    (10, '10.00'),
    (100, '100.00'),
    (100.01, '100.01'),
    (100.01123432123, '100.01'),
    (1000, '1.00K'),
    (10000, '10.00K'),
    (120010.12332, '120.01K'),
    (100000, '100.00K'),
    (5000000, '5.00M')
])
def test_str_repr(num, text):
    assert str(Number(num)) == text