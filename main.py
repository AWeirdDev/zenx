import python.zenx


def impl(payload: str):
    print(payload)
    return "\"asdf\""


python.zenx.go(impl)
