"""
This module provides interfaces for logging messages to Foxglove.

See :py:mod:`foxglove.schemas` and :py:mod:`foxglove.channels` for working with well-known Foxglove
schemas.
"""

import atexit
import logging
from typing import List, Optional, Protocol, Union

from ._foxglove_py import (
    Capability,
    ChannelView,
    Client,
    MCAPWriter,
    Parameter,
    ParameterType,
    ParameterValue,
    StatusLevel,
    WebSocketServer,
    disable_logging,
    enable_logging,
    open_mcap,
    shutdown,
)
from ._foxglove_py import start_server as _start_server
from .channel import Channel, SchemaDefinition, log

atexit.register(shutdown)


class ServerListener(Protocol):
    """
    A mechanism to register callbacks for handling client message events.
    """

    def on_subscribe(self, client: Client, channel: ChannelView) -> None:
        """
        Called by the server when a client subscribes to a channel.

        :param client: The client (id) that sent the message.
        :param channel: The channel (id, topic) that the message was sent on.
        """
        return None

    def on_unsubscribe(self, client: Client, channel: ChannelView) -> None:
        """
        Called by the server when a client unsubscribes from a channel.

        :param client: The client (id) that sent the message.
        :param channel: The channel (id, topic) that the message was sent on.
        """
        return None

    def on_client_advertise(self, client: Client, channel: ChannelView) -> None:
        """
        Called by the server when a client advertises a channel.

        :param client: The client (id) that sent the message.
        :param channel: The channel (id, topic) that the message was sent on.
        """
        return None

    def on_client_unadvertise(self, client: Client, channel: ChannelView) -> None:
        """
        Called by the server when a client unadvertises a channel.

        :param client: The client (id) that sent the message.
        :param channel: The channel (id, topic) that the message was sent on.
        """
        return None

    def on_message_data(
        self, client: Client, channel: ChannelView, data: bytes
    ) -> None:
        """
        Called by the server when a message is received from a client.

        :param client: The client (id) that sent the message.
        :param channel: The channel (id, topic) that the message was sent on.
        :param data: The message data.
        """
        return None

    def on_get_parameters(
        self,
        client: Client,
        param_names: List[str],
        request_id: Optional[str] = None,
    ) -> List["Parameter"]:
        """
        Called by the server when a client requests parameters.

        You must advertise the `Parameters` capability.
        """
        return []

    def on_set_parameters(
        self,
        client: Client,
        parameters: List["Parameter"],
        request_id: Optional[str] = None,
    ) -> List["Parameter"]:
        """
        Called by the server when a client sets parameters.
        Note that only `parameters` which have changed are included in the callback, but the return
        value must include all parameters.

        You must advertise the `Parameters` capability.
        """
        return parameters

    def on_parameters_subscribe(
        self,
        param_names: List[str],
    ) -> None:
        """
        Called by the server when a client subscribes to one or more parameters for the first time.

        You must advertise the `Parameters` capability.
        """
        return None

    def on_parameters_unsubscribe(
        self,
        param_names: List[str],
    ) -> None:
        """
        Called by the server when the last client subscription to one or more parameters has been
        removed.

        You must advertise the `Parameters` capability.
        """
        return None


def start_server(
    name: Optional[str] = None,
    host: Optional[str] = "127.0.0.1",
    port: Optional[int] = 8765,
    capabilities: Optional[List[Capability]] = None,
    server_listener: Optional[ServerListener] = None,
    supported_encodings: Optional[List[str]] = None,
) -> WebSocketServer:
    """
    Start a websocket server for live visualization.

    :param name: The name of the server.
    :param host: The host to bind to.
    :param port: The port to bind to.
    :param capabilities: A list of capabilities to advertise to clients.
    :param server_listener: A Python object that implements the :py:class:`ServerListener` protocol.
    :param supported_encodings: A list of encodings to advertise to clients.
    """
    return _start_server(
        name=name,
        host=host,
        port=port,
        capabilities=capabilities,
        server_listener=server_listener,
        supported_encodings=supported_encodings,
    )


def verbose_on(level: Union[int, str] = logging.INFO) -> None:
    """
    Enable SDK logging.
    """
    # For convenience in quick scripts, set basic config if the user hasn't already.
    # This will raise a ValueError for invalid levels.
    logging.basicConfig(level=level, format="%(asctime)s [%(levelname)s] %(message)s")

    if isinstance(level, str):
        level_map = logging.getLevelNamesMapping()
        level = level_map[level]
    else:
        level = max(0, min(2**32 - 1, level))

    enable_logging(level)


def verbose_off() -> None:
    """
    Disable SDK logging.
    """
    disable_logging()


__all__ = [
    "Capability",
    "Channel",
    "MCAPWriter",
    "Parameter",
    "ParameterType",
    "ParameterValue",
    "SchemaDefinition",
    "ServerListener",
    "StatusLevel",
    "WebSocketServer",
    "log",
    "open_mcap",
    "start_server",
    "verbose_off",
    "verbose_on",
]
