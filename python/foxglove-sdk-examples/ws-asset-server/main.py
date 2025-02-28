import logging
import time

import foxglove
from foxglove import Capability


def asset_handler(uri: str) -> bytes | None:
    """
    This will respond to "package://" asset requests from Foxglove by reading files from disk.
    This example doesn't do any path validation or upward traversal prevention.
    """
    logging.debug(f"Asset request: {uri}")
    if uri.startswith("package://"):
        filepath = uri.replace("package://", "", 1)
        try:
            with open(filepath, "rb") as file:
                return file.read()
        except FileNotFoundError:
            return None
    return None


def main() -> None:
    foxglove.set_log_level(logging.DEBUG)

    server = foxglove.start_server(
        asset_handler=asset_handler,
    )

    try:
        while True:
            # Send transforms for the model as needed, on a `FrameTransformsChannel`
            print("sleeping")
            time.sleep(1)

    except KeyboardInterrupt:
        server.stop()


if __name__ == "__main__":
    main()
