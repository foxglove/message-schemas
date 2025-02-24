import time
import unittest

from foxglove import Capability, Service, ServiceSchema, StatusLevel, start_server


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

    def test_services_interface(self) -> None:
        server = start_server(
            capabilities=[Capability.Services],
            supported_encodings=["json"],
            services=[
                Service(
                    name="test",
                    schema=ServiceSchema(name="test-schema"),
                    handler=lambda _client, _request: b"{}",
                ),
            ],
        )
        server.stop()
