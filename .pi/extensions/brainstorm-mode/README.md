# Brainstorm Mode Extension

Interactive exploration mode for brainstorming and design discussions.
The LLM uses the `question` tool to ask the user for decisions,
preferences, and clarifications interactively — instead of outputting
text walls full of questions.

## Features

- **Interactive questioning**: The LLM calls the `question` tool whenever it needs user input — you get a clean option list with ↑↓ navigate, Enter select
- **Read-only tools**: Restricts available tools to read, bash, grep, find, ls, question
- **Bash allowlist**: Only read-only bash commands are allowed
- **Session persistence**: State survives session resume
- **Footer indicator**: 💡 brainstorm shows in the footer when active

## Commands

- `/brainstorm` — Toggle brainstorm mode
- `Ctrl+Alt+B` — Toggle brainstorm mode (shortcut)

## Usage

```
pi --brainstorm "Let's discuss the architecture for the new module"
```

or toggle mid-session:

```
/brainstorm
```

The LLM will then ask you questions one at a time using the interactive
`question` tool. You pick from options or type a custom answer.

```
┌──────────────────────────────────────────┐
│  What kind of component architecture     │
│  do you prefer?                          │
│                                          │
│  > 1. Monolithic (simple, all in one)    │
│    2. Modular (separate crates)          │
│    3. Plugin-based (hot-swappable)       │
│    4. Type something.                    │
│                                          │
│  ↑↓ navigate • Enter select • Esc cancel │
└──────────────────────────────────────────┘
```

## How It Works

### Brainstorm Mode (Interactive)
- Only read-only tools + `question` tool available
- Bash commands filtered through allowlist
- System prompt instructs the LLM to use `question` for every decision
- LLM asks one question at a time via the tool

### Normal Mode
- Full tool access restored when toggled off
- Stale brainstorm context filtered from the prompt

### Command Allowlist

Safe commands (allowed):
- File inspection: `cat`, `head`, `tail`, `less`, `more`
- Search: `grep`, `find`, `rg`, `fd`
- Directory: `ls`, `pwd`, `tree`
- Git read: `git status`, `git log`, `git diff`, `git branch`
- Package info: `npm list`, `npm outdated`, `yarn info`
- System info: `uname`, `whoami`, `date`, `uptime`
- Web: `curl`, `wget`

Blocked commands:
- File modification: `rm`, `mv`, `cp`, `mkdir`, `touch`
- Git write: `git add`, `git commit`, `git push`
- Package install: `npm install`, `yarn add`, `pip install`
- System: `sudo`, `kill`, `reboot`
- Editors: `vim`, `nano`, `code`

## Installing

Add to your `.pi/settings.json` or load directly:

```bash
pi -e /path/to/brainstorm-mode/index.ts "Let's brainstorm"
```
