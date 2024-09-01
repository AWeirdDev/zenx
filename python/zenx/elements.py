from typing import Dict, Optional, Union, overload

from .page import Page
from .resolve import resolve_page


class Component:
    vault: Optional[str]
    tag: str
    text_content: Optional[str]
    maps: Optional[Dict[str, str]]

    def __init__(
        self,
        tag: str,
        text_content: Optional[str] = None,
        maps: Optional[Dict[str, str]] = None,
        abstract: Optional[str] = None,
    ):
        self.tag = tag
        self.text_content = text_content
        self.maps = maps
        self.vault = None
        self.abstract = abstract

    def __repr__(self):
        return self.abstract or "Component()"


@overload
def title(a: int, b: str) -> Component:
    """Represents an ``<hX>`` tag, where X is determined by the value of `a`.

    .. code-block:: python

        title(1, "Wow!")  # <h1>Wow!</h1>
        title(6, "smol")  # <h1>smol</h1>
    """


@overload
def title(a: str) -> Component:
    """Title.

    .. code-block:: python

        title("Hi, everyone!")
    """


def title(a: Union[str, int], b: Optional[str] = None) -> Component:
    if isinstance(a, int) and b:
        assert a >= 1 and a <= 6, "Invalid title tag (valid range: h1~h6)"
        component = Component(f"h{a}", text_content=b, abstract=f"h{a}(...)")

    elif isinstance(a, str):
        component = Component("h1", text_content=a, abstract="h1(...)")

    else:
        raise UnboundLocalError("Invalid overload for title()")

    page: Page = resolve_page()
    page.add_component(component)

    return component


def paragraph(text: str) -> Component:
    component = Component("p", text_content=text, abstract="p(...)")
    page: Page = resolve_page()
    page.add_component(component)
    return component
