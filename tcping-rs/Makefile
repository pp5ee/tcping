.PHONY: all build release test clean distclean package-deb package-tar-gz help

# Project configuration
NAME = tcping-rs
VERSION = 0.1.0
TARGET_DIR = target
RELEASE_DIR = $(TARGET_DIR)/release
PACKAGE_DIR = $(TARGET_DIR)/package

# Platform targets
LINUX_TARGET = x86_64-unknown-linux-gnu
MACOS_TARGET = x86_64-apple-darwin

# Package configuration
DEB_PACKAGE_NAME = $(NAME)_$(VERSION)_amd64.deb
TAR_GZ_PACKAGE_NAME = $(NAME)-$(VERSION)-x86_64-apple-darwin.tar.gz

# Default target
all: build

# Build debug version
build:
	cargo build

# Build release version
release:
	cargo build --release

# Run tests
test:
	cargo test

# Clean build artifacts
clean:
	cargo clean

# Clean everything including packages
distclean: clean
	rm -rf $(PACKAGE_DIR)
	rm -f *.deb *.tar.gz

# Build for Linux target
build-linux:
	cargo build --release --target $(LINUX_TARGET)

# Build for macOS target
build-macos:
	cargo build --release --target $(MACOS_TARGET)

# Create Ubuntu .deb package
package-deb: build-linux
	@echo "Building Ubuntu .deb package..."
	@mkdir -p $(PACKAGE_DIR)/deb/DEBIAN
	@mkdir -p $(PACKAGE_DIR)/deb/usr/bin
	@mkdir -p $(PACKAGE_DIR)/deb/usr/share/doc/$(NAME)

	# Copy binary
	cp $(TARGET_DIR)/$(LINUX_TARGET)/release/$(NAME) $(PACKAGE_DIR)/deb/usr/bin/tcping
	chmod 755 $(PACKAGE_DIR)/deb/usr/bin/tcping

	# Create control file
	@echo "Package: $(NAME)" > $(PACKAGE_DIR)/deb/DEBIAN/control
	@echo "Version: $(VERSION)" >> $(PACKAGE_DIR)/deb/DEBIAN/control
	@echo "Section: net" >> $(PACKAGE_DIR)/deb/DEBIAN/control
	@echo "Priority: optional" >> $(PACKAGE_DIR)/deb/DEBIAN/control
	@echo "Architecture: amd64" >> $(PACKAGE_DIR)/deb/DEBIAN/control
	@echo "Maintainer: tcping-rs <noreply@example.com>" >> $(PACKAGE_DIR)/deb/DEBIAN/control
	@echo "Description: A TCP ping tool rewritten in Rust" >> $(PACKAGE_DIR)/deb/DEBIAN/control
	@echo " A tool for measuring network connectivity and latency using TCP probes." >> $(PACKAGE_DIR)/deb/DEBIAN/control

	# Copy documentation
	cp README.md $(PACKAGE_DIR)/deb/usr/share/doc/$(NAME)/

	# Build the package
	dpkg-deb --build $(PACKAGE_DIR)/deb $(DEB_PACKAGE_NAME)
	@echo "Package created: $(DEB_PACKAGE_NAME)"

# Create macOS .tar.gz package
package-tar-gz: build-macos
	@echo "Building macOS .tar.gz package..."
	@mkdir -p $(PACKAGE_DIR)/macos

	# Copy binary and documentation
	cp $(TARGET_DIR)/$(MACOS_TARGET)/release/$(NAME) $(PACKAGE_DIR)/macos/tcping
	chmod 755 $(PACKAGE_DIR)/macos/tcping
	cp README.md $(PACKAGE_DIR)/macos/
	cp LICENSE $(PACKAGE_DIR)/macos/ 2>/dev/null || true

	# Create archive
	cd $(PACKAGE_DIR)/macos && tar -czf ../../../$(TAR_GZ_PACKAGE_NAME) .
	@echo "Package created: $(TAR_GZ_PACKAGE_NAME)"

# Build all packages
package-all: package-deb package-tar-gz

# Install locally (development)
install: release
	sudo cp $(RELEASE_DIR)/$(NAME) /usr/local/bin/tcping

# Uninstall locally
uninstall:
	sudo rm -f /usr/local/bin/tcping

# Show help
help:
	@echo "Available targets:"
	@echo "  build        - Build debug version"
	@echo "  release      - Build release version"
	@echo "  test         - Run tests"
	@echo "  clean        - Clean build artifacts"
	@echo "  distclean    - Clean everything including packages"
	@echo "  build-linux  - Build for Linux target"
	@echo "  build-macos  - Build for macOS target"
	@echo "  package-deb  - Create Ubuntu .deb package"
	@echo "  package-tar-gz - Create macOS .tar.gz package"
	@echo "  package-all  - Create all packages"
	@echo "  install      - Install locally"
	@echo "  uninstall    - Uninstall locally"