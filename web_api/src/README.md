# web_api

## start web server
```
➜  web_api git:(master) ✗ cargo run
    Finished dev [unoptimized + debuginfo] target(s) in 0.53s
     Running `/Users/kangxiaoning/workspace/rust-exercises/target/debug/web_api`


```

## API
```
➜  ~ curl -s http://localhost:8000/post_feed | python3 -m json.tool
[
    {
        "title": "The First Post",
        "body": "This is the first post in our API",
        "author": "Tensor",
        "datetime": "2020-10-07T08:27:19.325076Z",
        "uuid": "3b7451b8-c7f0-44a7-a07d-a80b0ba617ad"
    },
    {
        "title": "The next post is better",
        "body": "Iron is really cool and Rust is awesome too!",
        "author": "Metalman",
        "datetime": "2020-10-07T08:27:19.327114Z",
        "uuid": "6ccdc383-9c61-416f-bda7-121d56bba6c5"
    }
]
➜  ~
➜  ~
➜  ~ curl -X POST -H "Content-Type: application/json" \
-d '{"title":"abc","body":"This is the abc post","author":"kangxiaoning","datetime":"2020-10-07T08:27:40.327114Z","uuid":"a8098c1a-f86e-11da-bd1a-00112444be1e"}' \
http://localhost:8000/post
{"title":"abc","body":"This is the abc post","author":"kangxiaoning","datetime":"2020-10-07T08:27:40.327114Z","uuid":"a8098c1a-f86e-11da-bd1a-00112444be1e"}
➜  ~
➜  ~ curl -s http://localhost:8000/post_feed | python3 -m json.tool
[
    {
        "title": "The First Post",
        "body": "This is the first post in our API",
        "author": "Tensor",
        "datetime": "2020-10-07T08:27:19.325076Z",
        "uuid": "3b7451b8-c7f0-44a7-a07d-a80b0ba617ad"
    },
    {
        "title": "The next post is better",
        "body": "Iron is really cool and Rust is awesome too!",
        "author": "Metalman",
        "datetime": "2020-10-07T08:27:19.327114Z",
        "uuid": "6ccdc383-9c61-416f-bda7-121d56bba6c5"
    },
    {
        "title": "abc",
        "body": "This is the abc post",
        "author": "kangxiaoning",
        "datetime": "2020-10-07T08:27:40.327114Z",
        "uuid": "a8098c1a-f86e-11da-bd1a-00112444be1e"
    }
]
➜  ~
```
