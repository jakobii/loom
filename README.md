# Loom

Loom is a dead simple in memory key-object database.

The interface is a simple http api. The '/unique/path' is none hiachial.

```
PUT    /unique/path?key=value
PUT    /unique/path <- {JSON}
GET    /unique/path -> {JSON}
DELETE /unique/path
```



