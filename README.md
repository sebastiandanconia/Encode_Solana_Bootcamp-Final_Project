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
$ export PATH="$HOME/.local/share/solana/install/active_release/bin:$PATH"
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

$ export PATH="$HOME/.local/share/solana/install/active_release/bin:$PATH"

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

### Building the dApp Scaffold
```
$ cd dapp-scaffold
$ npm install
$ npm run dev

### Wallet setup
A Solana private key looks something like:
```
[197,99, ... 88,77]
```
You can copy it from a Solana-CLI wallet you've set up or a copy-and-paste it from a web browser extension wallet such as Solflare. This private key string should go in
```
~/.config/solana/id.json
```
```
$ solana config set --keypair ~/.config/solana/id.json
$ solana config get
```

### Solana RPC setup (Localhost)
```
$ solana config set --url localhost
```

In a new window, run
```
$ solana-test-validator
```

### Solana RPC setup
Configure Solana RPC endpoint:
```
$ solana config set --url https://api.devnet.solana.com
$ solana config get
```

You may have to configure these environmental variables used by Anchor (assuming you're using bash; the syntax for other shells may vary):
```
# If using Localnet:
$ export ANCHOR_PROVIDER_URL="http://localhost:8899/"
# OR
# If using devnet:
$ export ANCHOR_PROVIDER_URL="https://api.devnet.solana.com"

$ export ANCHOR_WALLET="$HOME/.config/solana/id.json"
```
`ANCHOR_PROVIDER_URL` should be consistent with `provider.cluster` in `Anchor.toml`.

### Building and testing the Anchor project
```
$ cd onchain/
$ anchor build
$ anchor deploy

# Note: `anchor test` runs its own local validator by default, which can cause conflicts.
$ anchor test --skip-local-validator
```
These commands have a slightly complicated relationship. Either see the Anchor documentation or experiment on your own until you're familiar with their behavior. The most conservative way is to run them one at a time in sequence.

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

If you get an error like the following, try running `solana-test-validator` with the `--reset` option:
```
$ anchor deploy
Deploying cluster: http://localhost:8899
Upgrade authority: /home/sebastian/.config/solana/id.json
Deploying program "wen_moon"...
Program path: /home/sebastian/src/Encode_Solana_Bootcamp-Final_Project/wen-moon/target/deploy/wen_moon.so...
============================================================================
Recover the intermediate account's ephemeral keypair file with
`solana-keygen recover` and the following 12-word seed phrase:
============================================================================
...
============================================================================
To resume a deploy, pass the recovered keypair as the
[BUFFER_SIGNER] to `solana program deploy` or `solana program write-buffer'.
Or to recover the account's lamports, pass it as the
[BUFFER_ACCOUNT_ADDRESS] argument to `solana program close`.
============================================================================
Error: Account ... has insufficient funds for spend (2.46738264 SOL) + fee (0.0009 SOL)
There was a problem deploying: Output { status: ExitStatus(unix_wait_status(256)), stdout: "", stderr: "" }.
```


```
Error: AnchorError occurred. Error Code: DeclaredProgramIdMismatch. Error Number: 4100. Error Message: The declared program id does not match the actual program id
```
Check the key Anchor is using for deployment using
```
~/src/Encode_Solana_Bootcamp-Final_Project/onchain$ solana-keygen pubkey target/deploy/onchain-keypair.json
```
This must match (in `Anchor.toml`):
```
[programs.localnet]
onchain = "..."
```
and (in `onchain/programs/onchain/src/lib.rs`):
```
declare_id!("...");
```

`onchain/target/deploy/onchain-keypair.json` may be deleted by `anchor clean`.
