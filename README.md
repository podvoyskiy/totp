## Simple TOTP CLI Tool

Command-line TOTP (Time-based One-Time Password) generator for two-factor authentication.

**Platform:** Linux only

### Installation & Usage

1. Create configuration directory
    ```bash
    mkdir -p ~/.config/totp
    ```

2. Add Your TOTP Secret (Encrypted)
    ```bash
    # Use two spaces to prevent the command from being saved in shell history
      echo "your_base32_secret_here" | gpg -c > ~/.config/totp/your_service.gpg
    ```

    Replace:
    + `"your_base32_secret_here"` with your actual TOTP secret key
    + `your_service.gpg` with a descriptive name (e.g., `github.gpg`, `google.gpg`)

3. Download Binary
    ```bash
    wget https://github.com/podvoyskiy/totp/releases/latest/download/totp
    ```

4. Make it Executable
    ```bash
    chmod +x totp
    ```

4. Run app
    ```
    ./totp
    ```

### What to Expect

When you run the tool, it will:

    📋 List all available services from ~/.config/totp/

    🔢 Prompt you to select a service by number

    🔐 Ask for your GPG password to decrypt the secret

    ⏱️ Display the TOTP code with a live countdown timer

    🔄 Automatically update the code every 30 seconds

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