from .zenx import go


class Bridge:
    def start(self, host: str = "127.0.0.1", port: str = "8787"):
        go(self.handler, host=host, port=port)

    def handler(self, data: str):
        return '"wow!"'
