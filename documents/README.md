# WasmCompute

This document covers the why and how we are creating wasm compute. It also will
go over design and document my processes into creating the website. I plan on
using this as my thesis so I want to be able to point the history of the project
to give my thesis a deeper meaning.

## Thesis

Is Webassembly a good binary language to target to support being deployed to a
multi tenant system?

Is Webassembly a technology to make docker container obsolete for new green field
projects?

Can we make multi tenant systems that share resources better to help customers
get the same compute power they would get using lambda functions but save them
money at the same time?

## Why

The current state of software development calls for developers to package their
programs into docker containers to be then pulled into orchestration systems like
Kubernetes that would then plan their deployment. Container's themselves aren't
a huge problem but at times they are I/O bound and can't use all of the system
resources that they are promised.

## What is WasmCompute

WasmCompute is a Serverless Webassembly framework thats built on top of the
wasmtime runtime. It allows users to upload and deploy Webassembly applications
that are built with the wasmcompute framework to be quickly deploy and executed.
The framework gives it basic access to operating system system calls to allow
the program to use the disk and other os functions.

These programs are applications that are stored in memory that are passed web
requests and execute their function. Each program (a webassembly binary) is safely
sandboxed, thus allowing for multi-tenancy.

## Goal

The goal of wasmcompute is to create a platform that allows any web application
that can be compiled down to web assembly to be runnable inside of it's environment
with the correct configuration.

## Overview

- [Hello World](#hello-world)

## Hello World

We start by creating a new project using the [wasmcompute-cli](https://github.com/wasmcompute/wasmcompute-cli).

```bash
wasmcompute-cli new example-app
```

```bash
tree example-app
.
├── Cargo.lock
├── Cargo.toml
├── README.md
├── wasmcompute.yaml
├── src
│   └── lib.rs
├── static
│   ├── script.js
│   └── style.css
└── views
    ├── index.html
    └── layout.html
```

This will create a basic rust project containing a simple web server and
configuration file. If all we wanted to do was deploy this, running some more
commands using the cli will do the trick.

```bash
wasmcompute-cli publish 0.0.1
wasmcompute-cli deploy 0.0.1
```

The following 2 commands will first build the application and upload it to
wasmcompute's servers. `Publish` only saves the application there with a version
tag. Once on the server though, we can then trigger a deployment. 

## Components

### Functions

### Gateways

### Datastores

### Files

### Syscalls
