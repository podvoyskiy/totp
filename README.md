## Simple TOTP CLI Tool

Command-line TOTP (Time-based One-Time Password) generator for two-factor authentication.

**Platform:** Linux, Windows, MacOS

**Security:** 
- **Linux**: Uses GPG for secret encryption
- **Windows/MacOS**: Native encryption with PBKDF2 key derivation

### Installation

#### Linux

1. Download Binary
    ```bash
    wget https://github.com/podvoyskiy/totp/releases/latest/download/totp-linux -O totp
    ```

2. Make it Executable
    ```bash
    chmod +x totp
    ```

#### Windows

1. Download `totp-windows.exe` from [Releases](https://github.com/podvoyskiy/totp/releases) 

2. Run in Command Prompt or PowerShell

#### macOS

1. Download Binary
    ```bash
    curl -L -o totp https://github.com/podvoyskiy/totp/releases/latest/download/totp-macos
    ```

2. Make it Executable
    ```bash
    chmod +x totp
    ```

### Usage

| Command | Description |
|---------|-------------|
| `./totp` | Interactive mode |
| `./totp --add` | Add a new TOTP service |
| `./totp --del` | Remove a service from the list |
| `./totp --export` | Export all services to JSON backup |
| `./totp --import` | Import services from JSON backup |

### Example

```bash
$ ./totp
Select service:
1 : github
2 : google
> 1
Enter password:
Decrypting...
Code: 123456 | █████████████░░░░░░░░░░░░░ | Time remaining: 15s
```