# CLI (Command line interface)

This document contains the design of the cli and some of the options that it
includes and provides to the user.

## Commands

All commands that users have access to from the cli

- [CLI (Command line interface)](#cli-command-line-interface)
  - [Commands](#commands)
    - [Login](#login)
    - [New](#new)
    - [Push](#push)
    - [Deploy](#deploy)

### Login

For users to be able to publish projects, they must be logged into the cli. If they
try to publish or deploy their app without logging in, we present them with an error.
Logging in is simple, just follow these steps:

1. Type in `wasmcompute-cli login`
2. Visit the website link that pops up. It's probably `https://wasmcompute.com/portal`
3. On the tokens table, type in a new token name and click `generate`
4. The server will generate an CLI token, copy it and paste it in your terminal
5. `wasmcompute-cli` will capture this input and save this to a file in your home directory `~/.wasmcompute/config.yaml`
6. You are now able to interact with the cli!

However if you are already logged in, you will be presented with a message saying
that you are already logged in.

<!-- ### Logout

Logging out is simple, to just run the command `wasmcompute-cli logout`. This will
clear the secret password in your home directory `~/.wasmcompute/config.yaml`. -->

### New

<!-- ### Generate -->

### Push

Push lets you package your application and save it to wasmcompute servers. Packaging
works by looking into your projects [wasmcompute.yaml](./projecfile.md) file and
reading it's contents. Once read it makes sure that every file specified exists
then writes all files into a big `tar.gz` file. Once the files have been compressed,
the cli will then push it up to the server to be decompressed and saved.

Before the upload can happen though, the cli will get the ok from the server if it
is ok. The server requires that the cli sends a `prepare` request before it tries
to upload it's content. If this prepare fails for any reason, then the upload process
is stopped and failed. To prepare the server for your upload, just send the
wasmcompute.yaml file to the server in json form. Using the file, the server will
remember what you are preparing to send and setup anything that needs to be set.
Once the request to `prepare` is complete, you can then push your project.

1. Go to your project (`cd <project-directory>`)
2. Push the project (`wasmcompute-cli push <version>`)

### Deploy

Just because you have pushed a project doesn't mean it's deployed. To deploy the
project, you need to tell wasmcompute what version you want to deploy.

To deploy your project, run the following

```bash
wasmcompute-cli deploy <version>
```

<!-- ### Develop -->

<!-- ### Test -->
