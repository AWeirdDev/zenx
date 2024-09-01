![nasa-yZygONrUBe8-unsplash](https://github.com/user-attachments/assets/556c8a25-a1b7-4ced-be10-768b7889273d)

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
