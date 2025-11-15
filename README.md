# hearbeat
This is a Rust web service using Actix Web to manage and monitor a list of uri endpoints.

The server starts at http://127.0.0.1:8080/.


## Add Resource (Monitoring)

Adds a new resource URI to the application's list of monitored items.

| Detail            | Specification      |
|-------------------|--------------------|
| **Method**        | `POST`             |
| **Path**          | `/add-resource`    |
| **Content-Type**  | `application/json` |


Request Body:
```json
{
    "uri": "https://google.com"
}
```



## List Resources Status

Retrieves the full list of monitored resources and their current heartbeat status.

| Detail            | Specification      |
|-------------------|--------------------|
| **Method**        | `Get`              |
| **Path**          | `/show-resource`   |
| **Content-Type**  | `application/json` |

The response is a JSON object where keys are the resource URIs and values are their boolean status.

    true: Resource is currently Up/Healthy (heartbeat successful).

    false: Resource is currently Down/Unhealthy (heartbeat failed).


Example response:
```json
{
  "https://google.com": true
}
```

