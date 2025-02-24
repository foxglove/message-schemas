from enum import Enum
from pathlib import Path
from typing import Any, List, Optional, Tuple

class MCAPWriter:
    """
    A writer for logging messages to an MCAP file.

    Obtain an instance by calling :py:func:`open_mcap`.

    This class may be used as a context manager, in which case the writer will
    be closed when you exit the context.

    If the writer is not closed by the time it is garbage collected, it will be
    closed automatically, and any errors will be logged.
    """

    def __new__(cls) -> "MCAPWriter": ...
    def __enter__(self) -> "MCAPWriter": ...
    def __exit__(self, exc_type: Any, exc_value: Any, traceback: Any) -> None: ...
    def close(self) -> None:
        """
        Close the writer explicitly.

        You may call this to explicitly close the writer. Note that the writer
        will be automatically closed whne it is garbage-collected, or when
        exiting the context manager.
        """
        ...

class WebSocketServer:
    """
    A websocket server for live visualization.
    """

    def __new__(cls) -> "WebSocketServer": ...
    def stop(self) -> None: ...
    def clear_session(self, session_id: Optional[str] = None) -> None: ...
    def broadcast_time(self, timestamp_nanos: int) -> None: ...

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
        publish_time: Optional[int] = None,
        log_time: Optional[int] = None,
        sequence: Optional[int] = None,
    ) -> None: ...

class PartialMetadata:
    """
    Structured metadata for use with logging. All fields are optional.
    """

    def __new__(
        cls,
        sequence: Optional[int] = None,
        log_time: Optional[int] = None,
        publish_time: Optional[int] = None,
    ) -> "PartialMetadata":
        """
        :param sequence: The sequence number is unique per channel and allows for ordering of
            messages as well as detecting missing messages. If omitted, a monotonically increasing
            sequence number unique to the channel is used.
        :param log_time: The log time is the time, as nanoseconds from the unix epoch, that the
            message was recorded. Usually this is the time log() is called. If omitted, the
            current time is used.
        :param publish_time: The publish_time is the time at which the message was published. e.g.
            the timestamp at which the sensor reading was taken. If omitted, log time is used.
        """
        ...

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

def open_mcap(path: str | Path, allow_overwrite: bool = False) -> MCAPWriter:
    """
    Creates a new MCAP file for recording.

    :param path: The path to the MCAP file. This file will be created and must not already exist.
    :param allow_overwrite: Set this flag in order to overwrite an existing file at this path.
    :return: A new `MCAPWriter` object.
    """
    ...

def get_channel_for_topic(topic: str) -> BaseChannel:
    """
    Get a previously-registered channel.
    """
    ...
