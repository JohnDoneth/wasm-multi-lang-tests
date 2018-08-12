# wasm-multi-lang-tests

For fun I made this project to compile a bunch of different programming languages to [WebAssembly](https://webassembly.org/) and run all of the generated WASM using the [wasmi](https://github.com/paritytech/wasmi) library.

Each language test generates a standalone WASM binary that has a shared WASM function that can be called from [wasmi](https://github.com/paritytech/wasmi) by the test runner.

---

*Disclaimer* 

This project is not associated with the [wasmi](https://github.com/paritytech/wasmi) project and is merely for researching how [WebAssembly](https://webassembly.org/) can be just another abstraction in any project. 

[Parity Technologies](https://github.com/paritytech) are developing/using [WebAssembly ](https://webassembly.org/)with [wasmi](https://github.com/paritytech/wasmi) as a target for their smart cryptocurrency contracts but [WebAssembly ](https://webassembly.org/)potentially has other uses cases than cryptocurrency or the web.

## Supported Languages

The languages currently supported are the following.

- C
- C++
- Rust

Any language that supports compiling to WebAssembly should be able to complete the tests.

## Building WebAssembly Test modules

### Rust

For Rust WebAssembly can be generated using nightly Rust with the following command.

```
cargo  +nightly build --target wasm32-unknown-unknown --release
```

For further assistance with setting up the Rust environment for compiling WebAssembly see this old but functional [walkthrough](https://www.hellorust.com/setup/wasm-target/) 


### C / C++

Install [Emscripten](https://kripken.github.io/emscripten-site/docs/getting_started/downloads.html) to install the C/C++ -> WASM tools and then run `make` in the respective folder for each language tests.

## Tests

**Add** 
Simple addition function with the following form
``` pseudocode 
a + b 
```
**Factorial**
Factorial function with the following form
``` pseudocode
if n > 1 {
	return n * factorial(n - 1);
} else {
	return 1;
} 
```

### Running the built WebAssembly modules

The wasm-runner cargo project in the repository root when executed will recursively find any wasm modules in the 'generated' folder and, if they are named appropriately, run the matching test on it.

For Example:

`generated/rust/factorial.wasm` will be executed by test function `factorial_test` in the runner

`generated/rust/add.wasm` will be executed by test function `add_test` in the runner