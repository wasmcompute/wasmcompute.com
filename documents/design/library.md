# Wasmcompute lib

For wasmcompute applications to actually do something they need to be able to access
some functions that the server provides. This service could be give applications
access to a database, request/response objects and web requests. Basically anything
that interfaces with another application we will handle because it can be quite
expensive creating that handshake every time an application is loaded and executed.

For the wasmcompute lib to be useful, we need two pieces of code

1. Code inside of the server that can provide the functionality the user requested
2. A library that provide the exported functions

We currently allow the user to access the filesystem using wasi, and using wasi
we could also allow users connection to raw sockets but allowing users to directly
access them maybe unsafe and slow. Therefore, we will provide the needed functionality
that the users need.

Another reason why to use a library is to keep the wasm file as a single threaded
application. This will let users focus on the execution of the program vs. trying
to handle all the different threads being executed pre request.

- [Wasmcompute lib](#wasmcompute-lib)
  - [Library functionality](#library-functionality)
    - [HTTP Server](#http-server)
    - [Database](#database)
    - [Cache](#cache)
    - [Web Requests](#web-requests)
    - [Helper functions](#helper-functions)

## Library functionality

The library needs to be able to provide some basic functionality such as:

1. HTTP Access
2. Database Access
3. Cache Access
4. Web Request Access
5. Filesystem Helper functions

### HTTP Server

For HTTP Access, the user's application could import the following methods

```rust
use wasmcompute::http::{Request, Response, http_serve};

fn handle(mut res: Response, mut req: Request) -> Result<Response, dyn std::error::Error> {
    // ... handle the request

    // let res = Response::redirect("http://google.com")
    let res = Response::new()::builder()
        .header("foo", "bar")
        .header("bar", "foo")
        .body(())
        .status(200);
    Ok(res)
}

fn main() -> std::io::Result<()> {
    http_serve(handle)
}
```

### Database

For Database Access, the user's application could import the following methods.
Note that the database communicates with the users application with json. For
the time being it's the easiest way to implement it. In the future we will try
and allow the user to specify the way the application should communicate with the
database.

```rust
use wasmcompute::db::Sql;

#[serde::Deseralize]
struct Person {
    id: i32,
    name: String,
    age: i32,
}

fn main() -> std::io::Result<()> {
    Sql::query("CREATE TABLE persons(id INTEGER AUTOINCREMENT PRIMARY KEY, name VARCHAR, age INTEGER)").execute()?;
    let rows_changed: i64 = Sql::query("INSERT INTO (age, name) person (?, ?)")
        .bind(10)
        .bind("Alec Divito")
        .execute()?;
    let persons: Vec<Person> = Sql::query("SELECT * FROM person")
        .fetch()?;
    let person: Person = Sql::query("SELECT * FROM person LIMIT ?")
        .bind(1)
        .fetch_one()?;
    let maybe_person: Option<Person> = Sql::query("SELECT * FROM person WHERE id = ?")
        .bind(100)
        .fetch_optional()?;
    Ok(())
}
```

### Cache

For Cache Access, the users application could import the following methods. Note
like databases, we could allow the user to access to the raw socket but for now
that is not possible.

```rust
use wasmcompute::cache::Cache;

fn main() -> std::io::Result<()> {
    let value: () = Cache::set("key", "value")?;
    let value: Option<String> = Cache::get("key")?;
    let value: Option<String> = Cache::delete("key")?;
    Ok(())
}
```

### Web Requests

For web requests, the users application can import the following methods.

```rust
use wasmcompute::web::Request;

fn main() -> std::io::Result<()> {
    Request::post("http://httpbin.org/post")
        .body("this is an example")
        .send()?;
}
```

### Helper functions

Wasmcompute has a couple of opinionated design decisions, this includes where
files are stored and rules on how to access them. To help with this issue, we
provide a couple of macros allowing users to quickly gain access to some of the
files with compile time checking, thus insuring the file they are pointing at
right now will exist when the publish their application to our servers.

```rust
use wasmcompute::template::TemplateFile;
use wasmcompute::util::{app_file, static_file, template_file};

// app_file      - targets /application files. These files are always accessible
// static_file   - targets /static files. These files are always accessible
// template_file - targets /template files. These files are cached on the server
//                 and when you render them, you are actually sending data to the
//                 webserver and it is rendering the page for you and returning
//                 a file descriptor to the rendered file

fn main() -> std::io::Result<()> {
    let file: Option<File> = app_file!("/i_may_exist.txt");
    let file: File = static_file!("/i_do_exist.txt");
    let file: TemplateFile = template_file!("/i_do_exist.html");
}
```
