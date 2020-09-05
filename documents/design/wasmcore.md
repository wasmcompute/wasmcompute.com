# Wasm Core

Wasm Core is the software that will run web assembly functions on the webserver.
It will keep all running applications in memory inside of a read write map.

## API Design

At the end of the day, we want a generic design so that it's easy to implement
new logic into our applications. But for now we are just trying to be as quick
to market as we can. So the following is the design we want to end up with:

```rust
// init
let wasm = WasmCore::new(opts);
// new ctx
let path = PathBuf::new();
let ctx = WasmAppContext::new(path)?;
ctx.set_ttl(std::time::Duration::minute * 5);
wasm.add_context(id, ctx).await?;
// prepare
ctx.add_extern_func(module, name, func)?;
ctx.add_env(key, value)?;
ctx.add_arg(key, value)?;
ctx.set_folder_access(path)?;
// execute
ctx.execute(id)?;
```




Though because we are just trying to get our application to work, this is what
we are going to do:

```rust
let wasm = WasmComputeCore::new(opts);
wasm.preload();
wasm.execute(static_folder, static_path)
