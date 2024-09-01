# zenx

```python
import zenx

@zenx.page(title="Todos")
def app(todos: list[str]):
    zenx.markdown("# Todos")
    if (t := zenx.input("Start typing...")):
        todos.append(t)

    for todo in todos:
        zenx.text(todo)
```