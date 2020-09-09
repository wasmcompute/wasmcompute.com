# Web Server Design

This document contains the design of the web server.

## Futures Research

- Using locking code inside of an async environment, i.e. Why can we not use the standard std::sync::Mutex inside of an async environment? https://users.rust-lang.org/t/std-mutex-vs-futures-mutex-vs-futures-lock-mutex-for-async/41710/4
- Why are we using DashMap inside of our WasmCore? Why do we require it to access a mut refernce to our hashmap and what are the performance implications of using a RwLock? https://www.reddit.com/r/rust/comments/eopxq5/announcing_dashmap_v3_taking_concurrent_hashmaps/

## URL

Once users have uploaded and deployed an application to wasmcompute, they can
access their application by visiting `applicaiton-name`.`github-name`.wasmcompute.com.
All of wasmcompute specific services are under the root domain.

Users who want to be able to debug their application can deploy their application
to debug routes which load a brand new temporary application from scratch that the
user can use. Here the user can test out all of his/her changes. The following
url make look like so, `application-name+branch-name`.`github-name`.wasmcompute.com or
`branch-name`.`application-name`.`github-name`.wasmcompute.com.

## Wasm Project Configuration

Wasmcompute projects must be configured using a wasmcompute file. These files
contain the name of the application you are planning on deploying as well as
what files are needed for it to function. This could include `static files` that
the server should serve, `application files` that the application needs to run
as well as `migrations` needed to be applied to the users database.

The `wasmcompute.yaml` file is the source of truth for all application deployed
to wasmcompute. It should track all the files you are planning on uploading to
the server. Any files you don't declare inside of your `wasmcompute.yaml` file
will not be loaded and not be accessible inside of your project.

An example of a wasmcompute file would look like:

```yaml
application: example
readme: README.md
static_file_base_url: /static
migrations:
  - migrations/2020_09_18-6_38_40-init.sql
static_files:
  - static/script.js
  - static/style.css
application_files:
  - views/index.html
  - views/layout.html
  - views/table.html
```

In this example the application files, migrations, static files and readme would
be packaged up and deployed to wasmcompute.

## Versioning an application on wasmcompute

Wasmcompute allows users to publish multiple versions of their application to the
server. Publishing a version though may come with some down sides like:

- Once a new version is published and deployed, all older versions become `tainted` if they reach the following criteria:
  - A new migration is added to the sql migrations list
  - A static file was deleted or changed
  - A template file was deleted or changed
- `tainted` application versions can not be deployed unless:
  - Users force it to be deployed with the `--unsafe` command
  - Users re-publish the version with the following tainted issues fixed.
- Reverting back to an initial state is impossible.
- All uploaded files are stored in one common directory.

## Routes

- [Web Server Design](#web-server-design)
  - [Futures Research](#futures-research)
  - [URL](#url)
  - [Wasm Project Configuration](#wasm-project-configuration)
  - [Versioning an application on wasmcompute](#versioning-an-application-on-wasmcompute)
  - [Routes](#routes)
  - [Authentication](#authentication)
    - [Login](#login)
    - [Logout](#logout)
    - [Me](#me)
  - [Application](#application)
    - [Prepare](#prepare)
    - [Upload](#upload)
    - [Deploy](#deploy)
  - [Applications](#applications)

## Authentication

Authentication routes deal with making sure the user is authorized to access
the wasmcompute server. It is also used for getting information about the user
so that they can check what type of information we have on them.

### Login

Login uses github as the authenticator. We don't want to be doing authentication
so we pass on the responsibility to github. To login, follow this flow:

1. Go to wasmcompute.com
2. In the top right corner, click on login with github
3. This will put you through the process of logging in using github
4. Once logged in, create a token that can be used with the cli
5. Make sure to copy the token down to a text file to remember it
6. Using the wasmcompute-cli, copy and paste the token to the terminal
7. You are now logged into wasmcompute :)

```bash
wasmcompute-cli login
```

### Logout

1. Run the following command. This will delete the current session that wasmcompute is using

```bash
wasmcompute-cli logout
```

### Me

1. Make request to wasmcompute or use the cli
2. Wasmcompute will take information from github as well as deployments you currently have
3. Present the information to you in json

```bash
wasmcompute-cli me
```

## Application

### Prepare

To prepare for an application to be uploaded, send a post request containing
your wasmcompute.yaml file. Send the post request here to:

```bash
POST https://wasmcompute.com/api/account/:account_name/app/:app_name/prepare/:version
```

Once the application is prepared, the webserver will response back with a 200 OK
response and contain a json object with the request id to use to upload the app.

### Upload

Now that wasmcompute knows you are about to upload an application, it will start
to package up your app into a zip file. Once zipped, the file will be uploaded to
wasmcompute servers and decompressed. Send your request to:

```bash
PUT https://wasmcompute.com/api/account/:account_name/app/:app_name/upload/:version
HEADER X-APP-UPLOAD-ID: <upload-id>
```

Once the application has been successfully uploaded, the webserver will response
back with a 200 OK response and contain a json object with what the server did.
Example would be like, uploaded these files to these locations.

### Deploy

Once a version has been uploaded, deploying it is super easy. Start by hitting
the following url:

```bash
POST https://wasmcompute.com/api/account/:account_name/app/:app_name/deploy/:version
```

Once the deployment has been staged, it will response back with a 200 OK response
and contain the changes that are planned to update from the old version app currently
deployed to the new version that was just deployed.

## Applications

All applications that are deployed to `wasmcompute` has a couple of guarantees.

1. You are given a persistent Key-Value Store (local)
2. You are given a persistent Database (sqlite)
3. You are given a http client to accept web requests
4. You are given a web client to make http requests

All applications that are deployed to `wasmcompute` are given the same folder
structure as well.

```bash
.
└── <account>
    └── <application>
        ├── database/db.sqlite
        ├── filesystem/<...user-file...>.<any>
        ├── logs/<...>.log
        └── versions
            ├── *.*.*-<version>
            ├── *.*.*-<version>
            └── *.*.*-<version>
                ├── static/***
                ├── migrations/***
                ├── applications/***
                ├── README.md
                ├── wasmcompute.yaml
                └── <application>.wasm
```

Inside of the `/database` folder contains the sqlite database that the application
will be connected to.

The `/filesystem` folder is what the user sees as his/her filesystem.

The `/logs` folder contains all the logs that the user has produced.

The `/versions` folder has all of the application the user has uploaded to the
server. Each version can contain a couple of different files.