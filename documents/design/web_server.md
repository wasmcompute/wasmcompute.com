# Web Server Design

This document contains the design of the web server.

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

### Upload

### Deploy