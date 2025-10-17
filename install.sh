#!/bin/bash

# Passman Installation Script
# Installs the terminal password manager system-wide

set -e

echo "🔐 Passman - Terminal Password Manager Installer"
echo "================================================"

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Function to print colored output
print_status() {
    echo -e "${BLUE}[INFO]${NC} $1"
}

print_success() {
    echo -e "${GREEN}[SUCCESS]${NC} $1"
}

print_warning() {
    echo -e "${YELLOW}[WARNING]${NC} $1"
}

print_error() {
    echo -e "${RED}[ERROR]${NC} $1"
}

# Check if running as root
if [[ $EUID -eq 0 ]]; then
   print_error "This script should not be run as root"
   exit 1
fi

# Get the directory where the script is located
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_DIR="$SCRIPT_DIR"

print_status "Installing Passman from: $PROJECT_DIR"

# Check if Rust is installed
if ! command -v cargo &> /dev/null; then
    print_warning "Rust not found. Installing Rust..."
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
    source ~/.cargo/env
    print_success "Rust installed successfully"
else
    print_success "Rust is already installed"
fi

# Check for system dependencies
print_status "Checking system dependencies..."

# Check for clipboard tools
if command -v xclip &> /dev/null; then
    print_success "xclip found (X11 clipboard support)"
elif command -v wl-copy &> /dev/null; then
    print_success "wl-copy found (Wayland clipboard support)"
else
    print_warning "No clipboard tool found. Installing xclip..."
    sudo apt update
    sudo apt install -y xclip
    print_success "xclip installed"
fi

# Build the application
print_status "Building Passman..."
cd "$PROJECT_DIR"

# Clean previous builds
cargo clean 2>/dev/null || true

# Build the TUI version
print_status "Building TUI version..."
cargo build --bin keytui-tui --release

# Build the CLI version
print_status "Building CLI version..."
cargo build --bin passman --release

print_success "Build completed successfully"

# Create the wrapper script
print_status "Creating wrapper script..."
cat > passman-wrapper.sh << 'EOF'
#!/bin/bash
# Passman wrapper script - launches the TUI interface
exec "/home/$(whoami)/workspace/cortex/projects/passman/target/release/keytui-tui"
EOF

# Update the wrapper script with the correct path
sed -i "s|/home/\$(whoami)/workspace/cortex/projects/passman|$PROJECT_DIR|g" passman-wrapper.sh

chmod +x passman-wrapper.sh
print_success "Wrapper script created"

# Install system-wide
print_status "Installing system-wide..."

# Copy to /usr/local/bin
sudo cp passman-wrapper.sh /usr/local/bin/passman
sudo chmod +x /usr/local/bin/passman

# Create symlink in user bin for PATH priority
mkdir -p ~/.local/bin
ln -sf /usr/local/bin/passman ~/.local/bin/passman

print_success "Installed to /usr/local/bin/passman"
print_success "Created symlink in ~/.local/bin/passman"

# Test the installation
print_status "Testing installation..."
if command -v passman &> /dev/null; then
    print_success "Passman command is available"
else
    print_error "Passman command not found in PATH"
    print_warning "You may need to restart your terminal or run: source ~/.bashrc"
fi

# Create .passman directory
mkdir -p ~/.passman
print_success "Created ~/.passman directory"

# Test clipboard functionality
print_status "Testing clipboard functionality..."
if echo "test" | xclip -selection clipboard 2>/dev/null && xclip -selection clipboard -o 2>/dev/null | grep -q "test"; then
    print_success "Clipboard functionality working"
else
    print_warning "Clipboard test failed - you may need to install xclip or wl-clipboard"
fi

echo ""
print_success "🎉 Installation completed successfully!"
echo ""
echo "📖 Usage:"
echo "  passman          # Launch the TUI interface"
echo "  passman add gmail # Add a new password (CLI mode)"
echo "  passman list     # List all passwords (CLI mode)"
echo ""
echo "📚 Documentation:"
echo "  README.md        # Project overview"
echo "  SETUP_GUIDE.md   # Detailed setup instructions"
echo ""
echo "🔧 Troubleshooting:"
echo "  If 'passman' command not found, restart your terminal"
echo "  If clipboard doesn't work, install: sudo apt install xclip"
echo ""
print_success "Happy password managing! 🔐"