"""
This implements a parameter server for live visualization.

View and edit parameters from a Parameters panel in Foxglove:
https://docs.foxglove.dev/docs/visualization/panels/parameters
"""

import logging
from typing import Optional
import foxglove
import time


from foxglove import Capability
from foxglove import ParameterType, ParameterValue, Parameter


class ParameterStore(foxglove.ServerListener):
    def __init__(self, parameters: list[Parameter]) -> None:
        # In this example our parameters are unique by name
        self.parameters = {param.name: param for param in parameters}

    # Foxglove server callback
    def on_get_parameters(
        self,
        client: foxglove.Client,
        param_names: list[str],
        request_id: Optional[str] = None,
    ) -> list[Parameter]:
        logging.debug(f"on_get_parameters: {param_names}, {client.id}, {request_id}")
        if not param_names:
            return list(self.parameters.values())
        return [
            self.parameters[name] for name in param_names if name in self.parameters
        ]

    def on_set_parameters(
        self,
        client: foxglove.Client,
        parameters: list[foxglove.Parameter],
        request_id: str | None = None,
    ) -> list[foxglove.Parameter]:
        logging.debug(f"on_set_parameters: {parameters}, {client.id}, {request_id}")
        for changed_param in parameters:
            if changed_param.value is None:
                del self.parameters[changed_param.name]
            else:
                # Add or update
                self.parameters[changed_param.name] = changed_param
        return parameters


def main() -> None:
    foxglove.verbose_on()

    initial_values: list[Parameter] = [
        Parameter(
            "param1",
            value=ParameterValue.Dict(
                {
                    "a": ParameterValue.Number(1),
                    "b": ParameterValue.Bool(True),
                    "c": ParameterValue.Bytes(b"hello"),
                    "arr": ParameterValue.Array(
                        [ParameterValue.Number(1), ParameterValue.Bool(True)]
                    ),
                }
            ),
        ),
        Parameter("param2"),
        Parameter("p3", value=ParameterValue.Number(0.124), type=ParameterType.Float64),
    ]

    store = ParameterStore(initial_values)

    websocket_server = foxglove.start_server(
        server_listener=store,
        capabilities=[Capability.Parameters],
    )

    try:
        while True:
            websocket_server.publish_parameter_values(list(store.parameters.values()))
            time.sleep(10)
    except KeyboardInterrupt:
        websocket_server.stop()


if __name__ == "__main__":
    main()
