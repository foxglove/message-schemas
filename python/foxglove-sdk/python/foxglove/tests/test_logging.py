import logging
import unittest

from foxglove import verbose_on


class TestMcap(unittest.TestCase):
    def test_verbose_on_accepts_string_or_int(self) -> None:
        verbose_on("DEBUG")
        verbose_on(logging.DEBUG)
        self.assertRaises(ValueError, verbose_on, "debug")

    def test_verbose_on_clamps_illegal_values(self) -> None:
        verbose_on(-1)
        verbose_on(2**64)
