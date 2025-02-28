import json
from typing import Any, Dict, Optional, Union

from ._foxglove_py import BaseChannel, Schema, channels

JsonSchema = Dict[str, Any]
JsonMessage = Dict[str, Any]


class ChannelSchema:
    """
    A custom schema definition for consumption by Foxglove.

    To use Foxglove well-known schemas, use existing Channel and schema definitions.
    """

    __slots__ = ["message_encoding", "schema"]
    message_encoding: str
    schema: Optional[Schema]

    def __init__(self, *, message_encoding: str, schema: Optional[Schema]):
        self.message_encoding = message_encoding
        self.schema = schema


class Channel:
    """
    A channel that can be used to log binary messages or JSON messages.
    """

    __slots__ = ["base", "message_encoding"]
    base: BaseChannel
    message_encoding: str

    def __init__(
        self,
        topic: str,
        *,
        schema: Union[JsonSchema, ChannelSchema],
    ):
        """
        Create a new channel for logging messages on a topic.

        :param topic: the topic name.
        :param schema: a definition of your schema. Pass a `ChannelSchema` for full control. If a
            dictionary is passed, it will be treated as a JSON schema.

        :raises KeyError: if a channel already exists for the given topic.
        """
        if topic in _channels_by_topic:
            raise ValueError(f"Channel for topic '{topic}' already exists")

        schema = _normalize_schema(schema)

        self.message_encoding = schema.message_encoding

        self.base = BaseChannel(
            topic,
            schema.message_encoding,
            schema.schema,
        )

        _channels_by_topic[topic] = self

    def log(
        self,
        msg: Union[JsonMessage, bytes],
        log_time: Optional[int] = None,
        publish_time: Optional[int] = None,
        sequence: Optional[int] = None,
    ) -> None:
        """
        Log a message on the channel.

        :param msg: the message to log. If the channel uses JSON encoding, you may pass a
            dictionary. Otherwise, you are responsible for serializing the message.
        """
        if isinstance(msg, bytes):
            return self.base.log(msg, log_time, publish_time, sequence)

        if self.message_encoding == "json":
            return self.base.log(
                json.dumps(msg).encode("utf-8"), log_time, publish_time, sequence
            )

        raise ValueError(f"Unsupported message type: {type(msg)}")

    def close(self) -> None:
        """
        Close the channel.

        You do not need to call this unless you explicitly want to remove advertisements from live
        visualization clients. Destroying all references to the channel will also close it.
        """
        self.base.close()


_channels_by_topic: Dict[str, Channel] = {}


def log(topic: str, message: Any) -> None:
    channel: Optional[Channel] = _channels_by_topic.get(topic, None)
    if channel is None:
        schema_name = type(message).__name__
        channel_name = f"{schema_name}Channel"
        channel_cls = getattr(channels, channel_name, None)
        if channel_cls is not None:
            channel = channel_cls(topic)
        if channel is None:
            raise ValueError(
                f"No Foxglove schema channel found for message type {type(message).__name__}"
            )
        _channels_by_topic[topic] = channel
    else:
        # TODO: Check schema compatibility with proto_msg
        pass

    channel.log(message)


def _normalize_schema(
    schema: Union[JsonSchema, ChannelSchema],
) -> ChannelSchema:
    if isinstance(schema, ChannelSchema):
        return schema
    elif isinstance(schema, dict):
        if schema.get("type") != "object":
            raise ValueError("Only object schemas are supported")

        return ChannelSchema(
            message_encoding="json",
            schema=Schema(
                name=schema.get("title", "json_schema"),
                encoding="jsonschema",
                data=json.dumps(schema).encode("utf-8"),
            ),
        )
    else:
        raise ValueError(f"Invalid schema type: {type(schema)}")
