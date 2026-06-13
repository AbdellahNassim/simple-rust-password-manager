# Password Manager

A command-line password manager written in Rust. Credentials are stored locally in a SQLite database, protected by a master password. Passwords are encrypted at rest with AES-256-GCM, and the master password is hashed with Argon2.

## Features

- **Vault setup** — Create a master password to protect your vault
- **Credential storage** — Save, retrieve, list, and delete credentials by service name
- **Encryption** — Stored passwords are encrypted; the master password is required to decrypt them
- **Password generator** — Generate random passwords with configurable length and optional symbol exclusion
- **Clipboard support** — Copy generated passwords directly to the clipboard

## Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) (stable toolchain with `cargo`)

No separate database server is required. The app uses SQLite and stores data in a local `vault.db` file in the current working directory.

## Build

Clone the repository and build the project:

```bash
git clone https://github.com/AbdellahNassim/simple-rust-password-manager
cd simple-rust-password-manager
cargo build
```

For an optimized release build:

```bash
cargo build --release
```

The binary will be at:

- Debug: `target/debug/password-manager`
- Release: `target/release/password-manager`

## Usage

Run commands with `cargo run --` during development, or invoke the built binary directly.

### 1. Set up the vault (first time only)

Before adding credentials, create your vault and choose a master password:

```bash
cargo run -- setup
```

You will be prompted to enter a master password. This step can only be performed once per vault.

### 2. Manage credentials

Most commands require your master password to unlock the vault.

**Add a credential**

```bash
cargo run -- add <service> <username>
```

You will be prompted for the password to store (input is hidden).

**Get a credential**

```bash
cargo run -- get <service>
```

**List all credentials**

```bash
cargo run -- list
```

Shows service names and usernames only (not passwords).

**Delete a credential**

```bash
cargo run -- delete <service>
```

### 3. Generate a password

Generate a random password without unlocking the vault:

```bash
cargo run -- generate <length>
```

Options:

- `--no-symbols` — Exclude special characters from the generated password
- `--copy` — Copy the generated password to the clipboard

Minimum length is 8 characters.

Example:

```bash
cargo run -- generate 16 --copy
cargo run -- generate 24 --no-symbols
```

## Example workflow

```bash
# First-time setup
cargo run -- setup

# Add credentials
cargo run -- add github myusername
cargo run -- add gmail user@example.com

# List saved services
cargo run -- list

# Retrieve a password
cargo run -- get github

# Generate a new password
cargo run -- generate 20 --copy
```

## Testing

Run the test suite:

```bash
cargo test
```

Integration tests use a separate `test.db` file and do not affect your vault.

## Project structure

| Path | Description |
|------|-------------|
| `src/main.rs` | CLI entry point |
| `src/data/` | SQLite repository and database setup |
| `src/auth/` | Master password authentication |
| `src/crypto.rs` | AES-GCM encryption and decryption |
| `src/services/` | Command handlers and password generator |
| `migrations/` | SQLx database migrations (applied automatically on startup) |

## Security notes

- Keep your master password safe. It cannot be recovered if lost.
- The `vault.db` file contains encrypted credentials and the master password hash. Do not commit it to version control (it is listed in `.gitignore`).
- Run the application from a trusted environment, since master and credential passwords are entered via the terminal.
