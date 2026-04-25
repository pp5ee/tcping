# Installation Instructions for macOS

## Prerequisites

- macOS 10.12 or later
- Intel x86_64 or Apple Silicon (ARM64) architecture

## Important Note: macOS Build Requirements

**The current macOS package is incomplete and requires building on macOS hardware.**

Due to Apple's licensing restrictions, cross-compilation from Linux to macOS requires the macOS SDK and Apple toolchain, which are only available on macOS systems. The current package only contains documentation files.

## Installation Methods

### Method 1: Build from Source on macOS (Recommended)

1. Clone the repository on a macOS system:
   ```bash
   git clone <repository-url>
   cd tcping-rs
   ```

2. Build the binary:
   ```bash
   ./build-macos.sh
   ```

3. Install the binary:
   ```bash
   tar -xzf tcping-rs-0.1.0-x86_64-apple-darwin.tar.gz
   sudo cp tcping /usr/local/bin/
   ```

### Method 2: Using Homebrew (If Available)

If this package becomes available via Homebrew:

```bash
brew install tcping-rs
```

### Method 3: Manual Installation (When Complete Package Available)

1. Download the complete release package when available

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