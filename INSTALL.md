# Installation Instructions for macOS

## Prerequisites

- macOS 10.12 or later
- Intel x86_64 or Apple Silicon (ARM64) architecture

## Installation Methods

### Method 1: Using Homebrew (Recommended)

If this package is available via Homebrew:

```bash
brew install tcping-rs
```

### Method 2: Manual Installation

1. Download the latest release: `tcping-rs-0.1.0-x86_64-apple-darwin.tar.gz`

2. Extract the archive:
   ```bash
   tar -xzf tcping-rs-0.1.0-x86_64-apple-darwin.tar.gz
   ```

3. Move the binary to a directory in your PATH:
   ```bash
   # For system-wide installation (requires sudo)
   sudo mv tcping /usr/local/bin/
   
   # For user-only installation
   mkdir -p ~/bin
   mv tcping ~/bin/
   echo 'export PATH="$HOME/bin:$PATH"' >> ~/.zshrc
   ```

4. Verify installation:
   ```bash
   tcping --version
   ```

## Security Considerations

On recent macOS versions, you may need to allow the binary to run:

1. Go to System Preferences > Security & Privacy
2. Click "Allow" next to the tcping application
3. Alternatively, run:
   ```bash
   sudo spctl --add /usr/local/bin/tcping
   ```

## Uninstallation

To remove tcping-rs:

```bash
# Remove the binary
sudo rm /usr/local/bin/tcping

# Or if installed in user directory
rm ~/bin/tcping
```