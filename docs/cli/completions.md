# Shell Completions

The MTG CLI supports shell completions for enhanced command-line experience. Completions provide automatic suggestions for commands, options, and arguments as you type.

## Available Commands

- `generate <SHELL>` - Generate completion scripts for supported shells

## Supported Shells

The MTG CLI supports completions for the following shells:

- **Bash** - Most common Linux/macOS shell
- **Zsh** - Default macOS shell (macOS 10.15+)
- **Fish** - User-friendly shell with advanced features
- **PowerShell** - Windows PowerShell and PowerShell Core
- **Elvish** - Modern shell with structured data support

## Generating Completions

### Basic Usage

```bash
# Generate completions for your shell
mtg completions generate bash
mtg completions generate zsh
mtg completions generate fish
mtg completions generate powershell
mtg completions generate elvish
```

### Save to File

```bash
# Save completions to a file
mtg completions generate bash > mtg_completions.bash
mtg completions generate zsh > _mtg
mtg completions generate fish > mtg.fish
```

## Installation Instructions

### Bash

#### System-wide Installation

```bash
# Generate and install system-wide
sudo mtg completions generate bash > /etc/bash_completion.d/mtg

# Or for Homebrew users on macOS
mtg completions generate bash > $(brew --prefix)/etc/bash_completion.d/mtg
```

#### User Installation

```bash
# Create completions directory if it doesn't exist
mkdir -p ~/.local/share/bash-completion/completions

# Generate and install for current user
mtg completions generate bash > ~/.local/share/bash-completion/completions/mtg

# Add to your ~/.bashrc if needed
echo 'source ~/.local/share/bash-completion/completions/mtg' >> ~/.bashrc
```

### Zsh

#### Oh My Zsh

```bash
# Generate completion file
mtg completions generate zsh > ~/.oh-my-zsh/completions/_mtg

# Reload completions
compinit
```

#### Manual Installation

```bash
# Create completions directory
mkdir -p ~/.zsh/completions

# Generate completion file
mtg completions generate zsh > ~/.zsh/completions/_mtg

# Add to ~/.zshrc
echo 'fpath=(~/.zsh/completions $fpath)' >> ~/.zshrc
echo 'autoload -U compinit && compinit' >> ~/.zshrc
```

#### System-wide Installation

```bash
# Install system-wide (requires sudo)
sudo mtg completions generate zsh > /usr/local/share/zsh/site-functions/_mtg
```

### Fish

```bash
# Generate and install completions
mtg completions generate fish > ~/.config/fish/completions/mtg.fish

# Completions will be available immediately in new fish sessions
```

### PowerShell

#### Windows PowerShell

```powershell
# Create profile directory if it doesn't exist
New-Item -ItemType Directory -Force -Path (Split-Path -Parent $PROFILE)

# Generate and add to profile
mtg completions generate powershell >> $PROFILE
```

#### PowerShell Core (Cross-platform)

```powershell
# For PowerShell Core
mtg completions generate powershell >> $PROFILE.CurrentUserAllHosts
```

### Elvish

```bash
# Generate completions
mtg completions generate elvish > ~/.config/elvish/lib/mtg-completions.elv

# Add to ~/.config/elvish/rc.elv
echo 'use ./lib/mtg-completions' >> ~/.config/elvish/rc.elv
```

## Verification

After installing completions, verify they work:

```bash
# Start a new shell session or reload your shell configuration
source ~/.bashrc  # For Bash
source ~/.zshrc   # For Zsh

# Test completions by typing and pressing Tab
mtg <TAB>         # Should show: cards, sets, types, completions, mcp
mtg cards <TAB>   # Should show: search, list, get
mtg sets <TAB>    # Should show: list, search, get, booster
```

## Features

### Command Completion

Completions work for all commands and subcommands:

```bash
mtg <TAB>                    # Main commands
mtg cards <TAB>              # Card subcommands
mtg sets <TAB>               # Set subcommands
mtg types <TAB>              # Type subcommands
mtg completions <TAB>        # Completion subcommands
```

### Option Completion

Completions include all available options:

```bash
mtg --<TAB>                  # Global options
mtg cards search --<TAB>     # Search-specific options
mtg sets list --<TAB>        # List-specific options
```

### Value Completion

Some options provide value completions:

```bash
mtg completions generate <TAB>    # Available shells
mtg cards list --rarity <TAB>     # Available rarities (if implemented)
```

## Troubleshooting

### Completions Not Working

1. **Verify Installation**:
   ```bash
   # Check if completion file exists
   ls -la ~/.local/share/bash-completion/completions/mtg  # Bash
   ls -la ~/.zsh/completions/_mtg                         # Zsh
   ls -la ~/.config/fish/completions/mtg.fish             # Fish
   ```

2. **Reload Shell Configuration**:
   ```bash
   source ~/.bashrc    # Bash
   source ~/.zshrc     # Zsh
   # Fish: Start new session
   ```

3. **Check Shell Configuration**:
   ```bash
   # Bash: Ensure bash-completion is installed
   # Ubuntu/Debian: sudo apt install bash-completion
   # macOS: brew install bash-completion

   # Zsh: Ensure compinit is called
   grep compinit ~/.zshrc
   ```

### Permission Issues

```bash
# If you get permission errors for system-wide installation
sudo mtg completions generate bash > /tmp/mtg_completions
sudo mv /tmp/mtg_completions /etc/bash_completion.d/mtg
```

### Outdated Completions

```bash
# Regenerate completions after MTG CLI updates
mtg completions generate bash > ~/.local/share/bash-completion/completions/mtg
```

## Advanced Usage

### Custom Installation Script

Create a script to install completions for multiple shells:

```bash
#!/bin/bash
# install-completions.sh

set -e

echo "Installing MTG CLI completions..."

# Bash
if command -v bash >/dev/null 2>&1; then
    mkdir -p ~/.local/share/bash-completion/completions
    mtg completions generate bash > ~/.local/share/bash-completion/completions/mtg
    echo "✓ Bash completions installed"
fi

# Zsh
if command -v zsh >/dev/null 2>&1; then
    mkdir -p ~/.zsh/completions
    mtg completions generate zsh > ~/.zsh/completions/_mtg
    echo "✓ Zsh completions installed"
fi

# Fish
if command -v fish >/dev/null 2>&1; then
    mkdir -p ~/.config/fish/completions
    mtg completions generate fish > ~/.config/fish/completions/mtg.fish
    echo "✓ Fish completions installed"
fi

echo "Completions installed! Restart your shell or source your configuration."
```

### Integration with Package Managers

For distribution packages, include completion installation:

```bash
# In package post-install script
if [ -d /etc/bash_completion.d ]; then
    mtg completions generate bash > /etc/bash_completion.d/mtg
fi

if [ -d /usr/share/zsh/site-functions ]; then
    mtg completions generate zsh > /usr/share/zsh/site-functions/_mtg
fi
```

## Shell-Specific Features

### Bash

- Works with bash-completion package
- Supports programmable completion
- Compatible with Bash 4.0+

### Zsh

- Integrates with Zsh completion system
- Supports advanced completion features
- Works with Oh My Zsh and other frameworks

### Fish

- Provides rich completion descriptions
- Supports fuzzy matching
- Automatic completion loading

### PowerShell

- Supports both Windows PowerShell and PowerShell Core
- Integrates with PowerShell's completion system
- Works on Windows, macOS, and Linux

### Elvish

- Supports structured completions
- Integrates with Elvish's completion system
- Modern shell features

---

Next: [Configuration](configuration.md) | Back: [Type Commands](types.md)