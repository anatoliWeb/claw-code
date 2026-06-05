# Recoverable Tool And Model Errors In REPL

Interactive REPL turns should keep the session alive when a local tool, plugin
tool, or post-tool model continuation fails in a recoverable way.

Recoverable examples:

- A tool or plugin tool reports an execution error.
- A tool or plugin tool completes with empty output.
- The model stream stops with no final assistant content after tool execution.

In these cases Claw should show the user-facing warning or tool error, preserve
the session, and return to the prompt so the user can continue.

Fatal examples remain fatal:

- Configuration or authentication boot failures.
- CLI startup failures.
- Workspace access failures.
- Explicit `/exit` or `/quit`.
- Non-interactive command failures where callers rely on the process exit code.

