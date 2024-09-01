import pytest
import zenx


def test_sum_as_string():
    assert zenx.sum_as_string(1, 1) == "2"
