#!/bin/bash
#
# Install Git Hooks Script
# 
# This script installs git hooks for the MTG CLI project to ensure
# code quality and consistency across all commits.
#

set -e  # Exit on any error

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Function to print colored output
print_info() {
    echo -e "${BLUE}ℹ️  $1${NC}"
}

print_success() {
    echo -e "${GREEN}✅ $1${NC}"
}

print_warning() {
    echo -e "${YELLOW}⚠️  $1${NC}"
}

print_error() {
    echo -e "${RED}❌ $1${NC}"
}

# Function to check if we're in a git repository
check_git_repo() {
    if [ ! -d ".git" ]; then
        print_error "Not in a git repository. Please run this script from the project root."
        exit 1
    fi
}

# Function to check if cargo is available
check_cargo() {
    if ! command -v cargo >/dev/null 2>&1; then
        print_error "Cargo is not installed or not in PATH. Please install Rust and Cargo first."
        exit 1
    fi
}

# Function to check if we're in the correct project
check_project() {
    if [ ! -f "Cargo.toml" ]; then
        print_error "No Cargo.toml found. Please run this script from the MTG CLI project root."
        exit 1
    fi
    
    # Check if this is the MTG project by looking for workspace or package name
    if ! (grep -q '\[workspace\]' Cargo.toml 2>/dev/null || grep -q 'name = "mtg"' Cargo.toml 2>/dev/null); then
        print_warning "This doesn't appear to be the MTG CLI project, but continuing anyway..."
    fi
}

# Function to backup existing hooks
backup_existing_hooks() {
    local hooks_dir=".git/hooks"
    local backup_dir=".git/hooks.backup.$(date +%Y%m%d_%H%M%S)"
    
    if [ -d "$hooks_dir" ]; then
        # Check if there are any existing hooks (not just samples)
        existing_hooks=$(find "$hooks_dir" -name "*" -not -name "*.sample" -type f 2>/dev/null | wc -l)
        
        if [ "$existing_hooks" -gt 0 ]; then
            print_info "Backing up existing git hooks to $backup_dir"
            cp -r "$hooks_dir" "$backup_dir"
            print_success "Existing hooks backed up"
        fi
    fi
}

# Function to install the pre-commit hook
install_pre_commit_hook() {
    local hook_file=".git/hooks/pre-commit"
    
    print_info "Installing pre-commit hook..."
    
    cat > "$hook_file" << 'EOF'
#!/bin/sh
#
# Pre-commit hook that calls the standalone pre-commit checks script
#

# Get the project root directory (where this script should be run from)
PROJECT_ROOT=$(git rev-parse --show-toplevel)

# Check if the pre-commit checks script exists
CHECKS_SCRIPT="$PROJECT_ROOT/scripts/pre-commit-checks.sh"
if [ ! -f "$CHECKS_SCRIPT" ]; then
    echo "Error: Pre-commit checks script not found at $CHECKS_SCRIPT"
    exit 1
fi

# Make sure the script is executable
if [ ! -x "$CHECKS_SCRIPT" ]; then
    echo "Error: Pre-commit checks script is not executable: $CHECKS_SCRIPT"
    exit 1
fi

# Run the pre-commit checks script
exec "$CHECKS_SCRIPT"
EOF

    # Make the hook executable
    chmod +x "$hook_file"
    print_success "Pre-commit hook installed"
}

# Function to test the installed hooks
test_hooks() {
    print_info "Testing installed hooks..."
    
    # Test pre-commit hook
    if [ -x ".git/hooks/pre-commit" ]; then
        print_info "Testing pre-commit hook..."
        if .git/hooks/pre-commit; then
            print_success "Pre-commit hook test passed"
        else
            print_warning "Pre-commit hook test failed (this might be expected if there are formatting issues)"
        fi
    fi
}

# Function to show usage information
show_usage() {
    echo "Usage: $0 [OPTIONS]"
    echo ""
    echo "Install git hooks for the MTG CLI project."
    echo ""
    echo "Options:"
    echo "  -h, --help     Show this help message"
    echo "  -f, --force    Force installation even if hooks already exist"
    echo "  -t, --test     Test hooks after installation"
    echo "  -q, --quiet    Quiet mode (less verbose output)"
    echo ""
    echo "Examples:"
    echo "  $0                 # Install hooks with default settings"
    echo "  $0 --force         # Force reinstall hooks"
    echo "  $0 --test          # Install and test hooks"
}

# Main installation function
main() {
    local force_install=false
    local test_after_install=false
    local quiet_mode=false
    
    # Parse command line arguments
    while [[ $# -gt 0 ]]; do
        case $1 in
            -h|--help)
                show_usage
                exit 0
                ;;
            -f|--force)
                force_install=true
                shift
                ;;
            -t|--test)
                test_after_install=true
                shift
                ;;
            -q|--quiet)
                quiet_mode=true
                shift
                ;;
            *)
                print_error "Unknown option: $1"
                show_usage
                exit 1
                ;;
        esac
    done
    
    if [ "$quiet_mode" = false ]; then
        echo "🔧 MTG CLI Git Hooks Installer"
        echo "=============================="
        echo ""
    fi
    
    # Run checks
    check_git_repo
    check_cargo
    check_project
    
    # Check if hooks already exist
    if [ -f ".git/hooks/pre-commit" ] && [ "$force_install" = false ]; then
        print_warning "Pre-commit hook already exists. Use --force to overwrite."
        echo ""
        print_info "To reinstall hooks, run: $0 --force"
        exit 0
    fi
    
    # Backup existing hooks if not in force mode
    if [ "$force_install" = false ]; then
        backup_existing_hooks
    fi
    
    # Install hooks
    install_pre_commit_hook
    
    # Test hooks if requested
    if [ "$test_after_install" = true ]; then
        echo ""
        test_hooks
    fi
    
    if [ "$quiet_mode" = false ]; then
        echo ""
        print_success "Git hooks installation completed!"
        echo ""
        print_info "The following hooks have been installed:"
        echo "  • pre-commit: Runs the following checks:"
        echo "    - cargo fmt: Ensures consistent code formatting"
        echo "    - cargo check: Verifies code compiles without errors"
        echo "    - cargo test: Runs all tests to ensure they pass"
        echo ""
        print_info "These hooks will now run automatically on git operations."
        print_info "To test the hooks manually, run: $0 --test"
    fi
}

# Run main function with all arguments
main "$@"