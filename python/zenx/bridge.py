import json
from typing import List

from .zenx import go
from .elements import Component


class Bridge:
    def __init__(self, components: List[Component]):
        self.components = components

    def start(self, host: str = "127.0.0.1", port: str = "8787"):
        go(self.handler, host=host, port=port)

    def handler(self, raw: str):
        print(raw)
        data = json.loads(raw)

        if "outline_request" in data["action"]:
            return json.dumps(
                {"action": {"outline": [c.to_dict() for c in self.components]}}
            )

        return "null"
