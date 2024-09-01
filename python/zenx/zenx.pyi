from typing import Callable

def go(
    handler: Callable[[str], str], port: str = "8787", host: str = "127.0.0.1"
) -> None: ...
