import tempfile
import unittest
from pathlib import Path

from foxglove import open_mcap
from foxglove.channel import Channel


class TestMcap(unittest.TestCase):
    chan: Channel
    dir: tempfile.TemporaryDirectory
    path: Path

    @classmethod
    def setUpClass(cls) -> None:
        cls.chan = Channel("test", schema={"type": "object"})

    def setUp(self) -> None:
        self.dir = tempfile.TemporaryDirectory()
        self.path = Path(self.dir.name) / "test.mcap"

    def tearDown(self) -> None:
        self.dir.cleanup()

    def test_open_with_str(self) -> None:
        open_mcap(str(self.path))

    def test_overwrite(self) -> None:
        self.path.touch()
        with self.assertRaises(FileExistsError):
            open_mcap(self.path)
        open_mcap(self.path, allow_overwrite=True)

    def test_explicit_close(self) -> None:
        mcap = open_mcap(self.path)
        for ii in range(20):
            self.chan.log({"foo": ii})
        size_before_close = self.path.stat().st_size
        mcap.close()
        self.assertGreater(self.path.stat().st_size, size_before_close)

    def test_context_manager(self) -> None:
        with open_mcap(self.path):
            for ii in range(20):
                self.chan.log({"foo": ii})
            size_before_close = self.path.stat().st_size
        self.assertGreater(self.path.stat().st_size, size_before_close)
