## Simple TOTP CLI Tool

Command-line TOTP (Time-based One-Time Password) generator for two-factor authentication.

**Platform:** Linux only

### Installation

1. Download Binary
    ```bash
    wget https://github.com/podvoyskiy/totp/releases/latest/download/totp
    ```

2. Make it Executable
    ```bash
    chmod +x totp
    ```

### Usage

#### First Time Setup
```bash
./totp --add
```
Follow the prompts to add your first TOTP service.

#### Interactive Mode
```bash
./totp
```
List and select from available services.

#### Direct Access
```bash
./totp <service_name>
```
Example: ./totp github

### What to Expect

When you run the tool, it will:

    📋 List all available services from ~/.config/totp/

    🔢 Prompt to select a service by number (or use direct access)

    🔐 Ask for your GPG password to decrypt secret

    ⏱️ Display TOTP code with live countdown timer

    🔄 Automatically update code every 30 seconds

### Example

```bash
$ ./totp
Select service:
1 : github
2 : google
> 1
Enter password for decryption:
Decrypting...
Code: 123456 | █████████████░░░░░░░░░░░░░ | Time remaining: 15s
```