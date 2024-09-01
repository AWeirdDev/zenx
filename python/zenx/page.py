from typing import Any, Callable, List, Optional

Composer = Callable[..., None]


class Page:
    """Represents a page."""

    route: str
    title: str
    components: List[Any]
    composer: Composer

    def __init__(self, route: str, title: str, composer: Composer):
        self.route = route
        self.title = title
        self.components = []
        self.composer = composer

    def add_component(self, cp: Any):
        self.components.append(cp)


def page(
    route: Optional[str] = None, *, title: Optional[str] = None
) -> Callable[[Composer], Page]:
    """Initializes a new page.

    .. code-block:: python

        @page(title="App")
        def app():
            ...
    """

    def wrapper(fn: Composer):
        page = Page(route or "/", title or "My App", fn)
        return page

    return wrapper
