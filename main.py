import zenx


@zenx.page()
def app():
    zenx.title("Hello, Zenx.")
    zenx.paragraph("This is your simple Zenx app.")
    zenx.paragraph(
        "All you needed is just a trusty computer with Python, and that's it."
    )


app.composer()
zenx.Bridge(app.components).start()
