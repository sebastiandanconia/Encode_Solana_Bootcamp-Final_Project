# Encode Club x Solana Bootcamp Final Project

## Getting Started Developing
### Building the front end project
```
  cd frontend
  npm install
  npm run dev
```
### Wallet setup
A Solana private key looks something like:
```
[197,99, ... 88,77]
```
You can copy it from a Solana-CLI wallet you've set up or a copy-and-paste it from a web browser extension wallet such as Solflare. This private key string should go in
```
~/.config/solana/id.json
```
Then, configure these environmental variables required by Anchor (assuming you're using bash; the syntax for other shells may vary):
```
export ANCHOR_PROVIDER_URL="https://api.devnet.solana.com"
export ANCHOR_WALLET="~/.config/solana/id.json"
```

Configure Solana RPC endpoint:
```
solana config get
solana config set --url https://api.devnet.solana.com
```


### Building and testing the Anchor project
```
cd onchain/
cargo build
```

```cargo test bpf```

or equivalently

```cargo test sbf```
Example output:
```
$ cargo test bpf
   Compiling onchain v0.1.0 (/home/sebastian/src/Encode_Solana_Bootcamp-Final_Project/onchain/programs/onchain)
    Finished test [unoptimized + debuginfo] target(s) in 1.08s
     Running unittests src/lib.rs (target/debug/deps/onchain-acfb36eb85ef3ab6)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 1 filtered out; finished in 0.00s
```

### Related commands and possible errors
The causes of some of the errors in this section are unclear; they may or may not be obstacles.

```
$ cargo build-sbf
error: no such command: `+solana`

	Cargo does not handle `+toolchain` directives.
	Did you mean to invoke `cargo` through `rustup` instead?
```
This is most likely a sign that you're calling `cargo` with incorrect arguments, however some sources suggest you may need to update Rust or Solana-CLI.


```
$ anchor build
info: uninstalling toolchain 'solana'
info: toolchain 'solana' uninstalled
error: no such command: `+solana`

	Cargo does not handle `+toolchain` directives.
	Did you mean to invoke `cargo` through `rustup` instead?
```

```
anchor test
error: no such command: `+solana`

	Cargo does not handle `+toolchain` directives.
	Did you mean to invoke `cargo` through `rustup` instead?
```

```
$ yarn run ts-mocha -p ./tsconfig.json -t 1000000 tests/**/*.ts
yarn run v1.22.22
$ /home/sebastian/src/Encode_Solana_Bootcamp-Final_Project/onchain/node_modules/.bin/ts-mocha -p ./tsconfig.json -t 1000000 'tests/**/*.ts'

Error: target/idl/onchain.json doesn't exist. Did you run `anchor build`?
    at Object.get (/home/sebastian/src/Encode_Solana_Bootcamp-Final_Project/onchain/node_modules/@coral-xyz/anchor/src/workspace.ts:65:15)
    at Suite.<anonymous> (/home/sebastian/src/Encode_Solana_Bootcamp-Final_Project/onchain/tests/onchain.ts:9:36)
    at Object.create (/home/sebastian/src/Encode_Solana_Bootcamp-Final_Project/onchain/node_modules/mocha/lib/interfaces/common.js:148:19)
    at context.describe.context.context (/home/sebastian/src/Encode_Solana_Bootcamp-Final_Project/onchain/node_modules/mocha/lib/interfaces/bdd.js:42:27)
    at Object.<anonymous> (/home/sebastian/src/Encode_Solana_Bootcamp-Final_Project/onchain/tests/onchain.ts:5:1)
    at Module._compile (node:internal/modules/cjs/loader:1256:14)
    at Module.m._compile (/home/sebastian/src/Encode_Solana_Bootcamp-Final_Project/onchain/node_modules/ts-node/src/index.ts:439:23)
    at Module._extensions..js (node:internal/modules/cjs/loader:1310:10)
    at Object.require.extensions.<computed> [as .ts] (/home/sebastian/src/Encode_Solana_Bootcamp-Final_Project/onchain/node_modules/ts-node/src/index.ts:442:12)
    at Module.load (node:internal/modules/cjs/loader:1119:32)
    at Function.Module._load (node:internal/modules/cjs/loader:960:12)
    at Module.require (node:internal/modules/cjs/loader:1143:19)
    at require (node:internal/modules/cjs/helpers:121:18)
    at Object.exports.requireOrImport (/home/sebastian/src/Encode_Solana_Bootcamp-Final_Project/onchain/node_modules/mocha/lib/nodejs/esm-utils.js:60:20)
    at Object.exports.loadFilesAsync (/home/sebastian/src/Encode_Solana_Bootcamp-Final_Project/onchain/node_modules/mocha/lib/nodejs/esm-utils.js:103:20)
    at singleRun (/home/sebastian/src/Encode_Solana_Bootcamp-Final_Project/onchain/node_modules/mocha/lib/cli/run-helpers.js:125:3)
    at Object.exports.handler (/home/sebastian/src/Encode_Solana_Bootcamp-Final_Project/onchain/node_modules/mocha/lib/cli/run.js:374:5)
error Command failed with exit code 1.
info Visit https://yarnpkg.com/en/docs/cli/run for documentation about this command.
```

It's unclear why `anchor test` is not equivalent to
```
yarn run ts-mocha -p ./tsconfig.json -t 1000000 tests/**/*.ts
```
