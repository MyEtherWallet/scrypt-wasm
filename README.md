# scrypt-wasm

Scrypt crypto library in Web Assembly

# Prerequisities

- This project requires Rust 1.30.0 or later.

### Build

```sh
$ make
```

### Usage

```sh
$ npm install -D scrypt-wasm
```

#Javascript

```
import('scrypt-wasm').then((wasm)=>{
    console.log(wasm.scrypt('password in hex', 'salt in hex', n, r, p, dklen))
})
```
