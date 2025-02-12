# Rust tools

### Configure

```bash
cargo install --path .
```

### Tools
* my-git-profile - allow to setup git profiles for ssh and gpg configuration

#### my-git-profile
Config file path `~/.config/rust-tools/config.json`

Example:
```json
{
    "sample": {
        "email": "<string>",
        "name": "<string>",
        "ssh_host": "Optional, host to use for repo remote",
        "gpg": "Optional, key id for GPG signature"
    }
}
```
