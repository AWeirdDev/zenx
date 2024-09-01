import inspect


def resolve_page():
    """Gets the nearest page."""
    frames = inspect.getouterframes(inspect.currentframe())
    target_frame = frames[-2]
    return frames[-1].frame.f_locals[target_frame.function]
