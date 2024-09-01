import zenx


@zenx.page()
def app():
    zenx.title("Hello, world!")
    zenx.paragraph("mr. beast!")


zenx.Bridge().start()
