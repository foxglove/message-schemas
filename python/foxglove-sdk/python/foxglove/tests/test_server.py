import time
import unittest

from foxglove import (
    Client,
    ClientChannelView,
    Parameter,
    ParameterValue,
    ServerListener,
    StatusLevel,
    start_server,
)


class TestServer(unittest.TestCase):
    def test_server_interface(self) -> None:
        """
        Exercise the server interface; will also be checked with mypy.
        """
        server = start_server()
        server.publish_status("test message", StatusLevel.Info, "some-id")
        server.broadcast_time(time.time_ns())
        server.remove_status(["some-id"])
        server.clear_session()
        server.stop()

    def test_server_listener_provides_default_implementation(self) -> None:

        class DefaultServerListener(ServerListener):
            pass

        listener = DefaultServerListener()

        listener.on_message_data(Client(), ClientChannelView(), b"")

        listener.on_parameters_subscribe(["test"])
        listener.on_parameters_unsubscribe(["test"])
        self.assertEqual([], listener.on_get_parameters(Client(), ["test"], None))
        self.assertEqual(
            [],
            listener.on_set_parameters(
                Client(), [Parameter(name="test", value=ParameterValue.Number(1))], None
            ),
        )
