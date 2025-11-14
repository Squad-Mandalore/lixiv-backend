## Datastructure

```json
{
  "kind": "Technology",
  "parent": null,
  "data": {
    "id": "string",
    "description": "string",
  }
}
```

```rs
struct Kind<T, K> {
  kind: string,
  parent: Optional<T>,
  data: K
}
```

In this example helmTemplate.app/v2 is a "Technology"
the data field type is derived from the respective technology struct
in this case helmTemplate.app/v2
```json
{
  "kind": "helmTemplate.app/v2",
  "parent": {
    "kind": "Application",
    "name" | "id": "team-rocket-test-ms"
  },
  "data": {
    "name" | "id": "app/v2",
    "description": "",
  }
}
```
