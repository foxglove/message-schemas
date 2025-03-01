from enum import Enum
from pathlib import Path
from typing import Any, Callable, List, Optional, Tuple, Union

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

class StatusLevel(Enum):
    Info = ...
    Warning = ...
    Error = ...

class WebSocketServer:
    """
    A websocket server for live visualization.
    """

    def __new__(cls) -> "WebSocketServer": ...
    @property
    def port(self) -> int: ...
    def stop(self) -> None: ...
    def clear_session(self, session_id: Optional[str] = None) -> None: ...
    def broadcast_time(self, timestamp_nanos: int) -> None: ...
    def publish_parameter_values(self, parameters: List["Parameter"]) -> None: ...
    def publish_status(
        self, message: str, level: "StatusLevel", id: Optional[str] = None
    ) -> None: ...
    def remove_status(self, ids: list[str]) -> None: ...
    def add_services(self, services: list["Service"]) -> None: ...
    def remove_services(self, names: list[str]) -> None: ...
    def publish_connection_graph(self, graph: "ConnectionGraph") -> None: ...

class BaseChannel:
    """
    A channel for logging messages.
    """

    def __new__(
        cls,
        topic: str,
        message_encoding: str,
        schema: Optional["Schema"] = None,
        metadata: Optional[List[Tuple[str, str]]] = None,
    ) -> "BaseChannel": ...
    def log(
        self,
        msg: bytes,
        publish_time: Optional[int] = None,
        log_time: Optional[int] = None,
        sequence: Optional[int] = None,
    ) -> None: ...
    def close(self) -> None: ...

class Capability(Enum):
    """
    An enumeration of capabilities that the websocket server can advertise to its clients.
    """

    ClientPublish = ...
    """Allow clients to advertise channels to send data messages to the server."""

    Connectiongraph = ...
    """Allow clients to subscribe and make connection graph updates"""

    Parameters = ...
    """Allow clients to get & set parameters."""

    Services = ...
    """Allow clients to call services."""

    Time = ...
    """Inform clients about the latest server time."""

class Client:
    """
    A client that is connected to a running websocket server.
    """

    id: int = ...

class ChannelView:
    """
    Information about a channel.
    """

    id: int = ...
    topic: str = ...

class Parameter:
    """
    A parameter.
    """

    name: str
    type: Optional["ParameterType"]
    value: Optional["AnyParameterValue"]

    def __init__(
        self,
        name: str,
        *,
        type: Optional["ParameterType"] = None,
        value: Optional["AnyParameterValue"] = None,
    ) -> None: ...

class ParameterType(Enum):
    """
    The type of a parameter.
    """

    ByteArray = ...
    """A byte array."""

    Float64 = ...
    """A decimal or integer value that can be represented as a `float64`."""

    Float64Array = ...
    """An array of decimal or integer values that can be represented as `float64`s."""

class ParameterValue:
    """
    The value of a parameter.
    """

    class Bool:
        """A boolean value."""

        def __new__(cls, value: bool) -> "ParameterValue.Bool": ...

    class Number:
        """A decimal or integer value."""

        def __new__(cls, value: float) -> "ParameterValue.Number": ...

    class Bytes:
        """A byte array."""

        def __new__(cls, value: bytes) -> "ParameterValue.Bytes": ...

    class Array:
        """An array of parameter values."""

        def __new__(
            cls, value: List["AnyParameterValue"]
        ) -> "ParameterValue.Array": ...

    class Dict:
        """An associative map of parameter values."""

        def __new__(
            cls, value: dict[str, "AnyParameterValue"]
        ) -> "ParameterValue.Dict": ...

AnyParameterValue = Union[
    ParameterValue.Bool,
    ParameterValue.Number,
    ParameterValue.Bytes,
    ParameterValue.Array,
    ParameterValue.Dict,
]

AssetHandler = Callable[[str], Optional[bytes]]

class ServiceRequest:
    """
    A websocket service request.
    """

    service_name: str
    client_id: int
    call_id: int
    encoding: str
    payload: bytes

ServiceHandler = Callable[["ServiceRequest"], bytes]

class Service:
    """
    A websocket service.
    """

    name: str
    schema: "ServiceSchema"
    handler: "ServiceHandler"

    def __new__(
        cls,
        *,
        name: str,
        schema: "ServiceSchema",
        handler: "ServiceHandler",
    ) -> "Service": ...

class ServiceSchema:
    """
    A websocket service schema.
    """

    name: str
    request: Optional["MessageSchema"]
    response: Optional["MessageSchema"]

    def __new__(
        cls,
        *,
        name: str,
        request: Optional["MessageSchema"] = None,
        response: Optional["MessageSchema"] = None,
    ) -> "ServiceSchema": ...

class MessageSchema:
    """
    A service request or response schema.
    """

    encoding: str
    schema: "Schema"

    def __new__(
        cls,
        *,
        encoding: str,
        schema: "Schema",
    ) -> "MessageSchema": ...

class Schema:
    """
    A schema for a message or service call.
    """

    name: str
    encoding: str
    data: bytes

    def __new__(
        cls,
        *,
        name: str,
        encoding: str,
        data: bytes,
    ) -> "Schema": ...

class ConnectionGraph:
    """
    A graph of connections between clients.
    """

    def __new__(cls) -> "ConnectionGraph": ...
    def set_published_topic(self, topic: str, publisher_ids: List[str]) -> None: ...
    def set_subscribed_topic(self, topic: str, subscriber_ids: List[str]) -> None: ...
    def set_advertised_service(self, service: str, provider_ids: List[str]) -> None: ...

def start_server(
    *,
    name: Optional[str] = None,
    host: Optional[str] = "127.0.0.1",
    port: Optional[int] = 8765,
    capabilities: Optional[List[Capability]] = None,
    server_listener: Any = None,
    supported_encodings: Optional[List[str]] = None,
    services: Optional[List["Service"]] = None,
    asset_handler: Optional["AssetHandler"] = None,
) -> WebSocketServer:
    """
    Start a websocket server for live visualization.
    """
    ...

def enable_logging(level: int) -> None:
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
    :rtype: :py:class:`MCAPWriter`
    """
    ...

def get_channel_for_topic(topic: str) -> BaseChannel:
    """
    Get a previously-registered channel.
    """
    ...
