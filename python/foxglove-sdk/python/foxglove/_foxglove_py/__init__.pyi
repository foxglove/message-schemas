from enum import Enum
from typing import Any, List, Optional, Tuple

class MCAPWriter:
    """
    A writer for logging messages to an MCAP file. Obtain an instance by calling `record_file`, or
    the context-managed `new_mcap_file`.

    If you're using `record_file`, you must maintain a reference to the returned writer until you
    are done logging. The writer will be closed automatically when it is garbage collected, but you
    may also `close()` it explicitly.
    """

    def __new__(cls) -> "MCAPWriter": ...
    def close(self) -> None:
        """
        Close the writer explicitly.
        """
        ...

class StatusLevel(Enum):
    Info = ...
    Warning = ...
    Error = ...

class WebSocketServer:
    """
    A websocket server for live visualization.
    """

    def __new__(cls) -> "WebSocketServer": ...
    def stop(self) -> None: ...
    def clear_session(self, session_id: Optional[str] = None) -> None: ...
    def broadcast_time(self, timestamp_nanos: int) -> None: ...
    def publish_status(
        self, message: str, level: "StatusLevel", id: Optional[str] = None
    ) -> None: ...
    def remove_status(self, ids: list[str]) -> None: ...

class BaseChannel:
    """
    A channel for logging messages.
    """

    def __new__(
        cls,
        topic: str,
        message_encoding: str,
        schema_name: Optional[str] = None,
        schema_encoding: Optional[str] = None,
        schema_data: Optional[bytes] = None,
        metadata: Optional[List[Tuple[str, str]]] = None,
    ) -> "BaseChannel": ...
    def log(
        self,
        msg: bytes,
        log_time: Optional[int] = None,
        publish_time: Optional[int] = None,
        sequence: Optional[int] = None,
    ) -> None: ...

class Capability(Enum):
    """
    A capability that the websocket server advertises to its clients.
    """

    Time = ...
    ClientPublish = ...

class Client:
    """
    A client that is connected to a running websocket server.
    """

    id = ...

class ClientChannelView:
    """
    Information about a client channel.
    """

    id = ...
    topic = ...

def start_server(
    name: Optional[str] = None,
    host: Optional[str] = "127.0.0.1",
    port: Optional[int] = 8765,
    capabilities: Optional[List[Capability]] = None,
    server_listener: Any = None,
    supported_encodings: Optional[List[str]] = None,
) -> WebSocketServer:
    """
    Start a websocket server for live visualization.
    """
    ...

def enable_logging(level: str) -> None:
    """
    Forward SDK logs to python's logging facility.
    """
    ...

def disable_logging() -> None:
    """
    Stop forwarding SDK logs.
    """
    ...

def shutdown() -> None:
    """
    Shutdown the running websocket server.
    """
    ...

def record_file(path: str) -> MCAPWriter:
    """
    Create a new MCAP file at ``path`` for logging.
    """
    ...

def get_channel_for_topic(topic: str) -> BaseChannel:
    """
    Get a previously-registered channel.
    """
    ...
