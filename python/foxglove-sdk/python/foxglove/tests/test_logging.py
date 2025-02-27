import logging
import unittest

from foxglove import set_log_level


class TestMcap(unittest.TestCase):
    def test_set_log_level_accepts_string_or_int(self) -> None:
        set_log_level("DEBUG")
        set_log_level(logging.DEBUG)
        self.assertRaises(ValueError, set_log_level, "debug")

    def test_set_log_level_clamps_illegal_values(self) -> None:
        set_log_level(-1)
        set_log_level(2**64)
