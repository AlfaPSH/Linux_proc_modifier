# proc_editor-rs

A dynamic variable and memory editor for Android processes via `/proc`, with an interactive shell UI. Requires root access.

## Features
- Select Android process by package name
- Detect and modify environment variables, memory maps, and direct memory
- Interactive menu-driven CLI (with automation via flags)
- Logging and change history
- Automated mode for scripting

## Usage Example
```
[proc_editor-rs] > Enter package name: com.example.target
Detecting PID... found: 1423
[1] Environment Variables
[2] Open File Descriptors
[3] Memory Maps
[4] Direct Memory Modification
Select an option: 1
Showing environment variables:
- TARGET_VAR=1234
- DEBUG_MODE=true
Select variable to edit: TARGET_VAR
Current value: 1234
Enter new value: 9999
Confirm change of TARGET_VAR → 9999 ? (y/N): y
✔️ Variable modified successfully.
```

## Security
- Uses ptrace for safe memory modification
- Backs up memory segments before overwriting
- Logs changes to `/data/local/tmp/`

## Dependencies
- nix
- dialoguer
- regex
- procfs
- colored
- clap
- memmap2
- log
- env_logger

## License
GNU
