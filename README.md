# Encode Club x Solana Bootcamp Final Project
Group 4, Q3 2024

## Getting Started Developing
### Quickstart: Set versions for previously-installed tools
If you already have these tools installed, but need to switch versions, use these commands.
```
$ rustup install 1.75.0
$ rustup default 1.75.0
$ rustc --version

$ solana-install init 1.16.24
$ export PATH="~/.local/share/solana/install/active_release/bin:$PATH"
$ solana --version

$ avm install 0.29.0
$ avm use 0.29.0
$ anchor --version
```

### Installing the Toolchain
As of this writing, these are the versions from the Solana Playground Anchor Environment (https://github.com/solana-playground/solana-playground/tree/68569dd9e53357e69ae2ef7caaed2ae8819dd08a). Anchor is tightly coupled to Solana-CLI and Rustup, so if the versions of these utilities aren't compatible, the toolchain breaks.

I may have forgotten to include some instances of `export PATH=...`. Generally, if one of these installation scripts tells you to include something new in your path, you probably should.

#### Install Node.js & Yarn
```
$ curl -fsSL https://deb.nodesource.com/setup_18.x -o nodesource_setup.sh
$ chmod a+x nodesource_setup.sh
$ sudo ./nodesource_setup.sh
$ sudo apt install nodejs -y
$ node --version

$ sudo corepack enable
$ cd ~/src/Encode_Solana_Bootcamp-Final_Project
$ yarn set version 1.22.19
$ yarn install
$ yarn --version
```

#### Install Rust
```
$ curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```
You will be presented with a menu:
```
Current installation options:


   default host triple: x86_64-unknown-linux-gnu
     default toolchain: stable (default)
               profile: default
  modify PATH variable: yes

1) Proceed with standard installation (default - just press enter)
2) Customize installation
3) Cancel installation
```
Enter "2". When you're asked
```
Default toolchain? (stable/beta/nightly/none) [stable]
```
enter "1.75.0".

You can change the active version of Rust later using:
```
$ rustup install 1.75.0
$ rustup default 1.75.0
$ rustc --version
```

#### Install Solana CLI:
```
$ sh -c "$(curl -sSfL https://release.solana.com/v1.16.24/install)"

$ export PATH="~/.local/share/solana/install/active_release/bin:$PATH"

$ solana --version
```

You can change the active version of the Solana tools later using:
```
$ solana-install init 1.16.24
$ solana --version
```

#### Install AVM/Anchor
```
$ cargo install --git https://github.com/coral-xyz/anchor avm --locked --force
$ avm install 0.29.0
$ avm use 0.29.0
$ anchor --version
```

### Building the frontend project
```
$ cd frontend
$ npm install
$ npm run dev
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

### Solana RPC setup (Localhost)
```
$ solana config set --url localhost
```

In a new window, run
```
$ solana-test-validator
```

### Solana RPC setup (devnet)
Configure Solana RPC endpoint:
```
$ solana config set --url https://api.devnet.solana.com
$ solana config get
```

You may have to configure these environmental variables used by Anchor (assuming you're using bash; the syntax for other shells may vary):
```
$ export ANCHOR_PROVIDER_URL="https://api.devnet.solana.com"
$ export ANCHOR_WALLET="~/.config/solana/id.json"
```

### Building and testing the Anchor project
```
$ cd onchain/
$ anchor build
$ anchor deploy
```

```anchor test``` is a superset of ```anchor deploy```.

### Possible errors
If you install Rustup a different way than described above, or if the versions of Rust, Anchor, and Solana CLI are incompatible, you will probably get one of the following errors:

```
$ cargo build-sbf
error: no such command: `+solana`

	Cargo does not handle `+toolchain` directives.
	Did you mean to invoke `cargo` through `rustup` instead?
```

```
$ anchor build
info: uninstalling toolchain 'solana'
info: toolchain 'solana' uninstalled
error: no such command: `+solana`

	Cargo does not handle `+toolchain` directives.
	Did you mean to invoke `cargo` through `rustup` instead?
```

```
$ anchor test
error: no such command: `+solana`

	Cargo does not handle `+toolchain` directives.
	Did you mean to invoke `cargo` through `rustup` instead?
```
