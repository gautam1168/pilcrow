# Installation instructions

You need to have rust installed using `rustup`.
You need to install `wasm-pack`
```
cargo install wasm-pack
```

After this you can build the rust part using
```
wasm-pack build
```
in the root directory.

Finally you can start the webapp by running
```
cd www
npm i
npm start
```