from typing import Optional

class Duration:
    """
    A duration in seconds and nanoseconds
    """

    def __new__(
        cls,
        sec: int,
        nsec: Optional[int] = None,
    ) -> "Duration": ...

class Timestamp:
    """
    A timestamp in seconds and nanoseconds
    """

    def __new__(
        cls,
        sec: int,
        nsec: Optional[int] = None,
    ) -> "Timestamp": ...
