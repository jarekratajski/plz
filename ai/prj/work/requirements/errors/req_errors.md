# Goal

Sometimes command that is created by agent does not work on a local system.
Example:
```
Asking Claude how to: list the newest file in this folder and describe what it is but do not print it

Proposed command:
  ls -lt --time=style=locale | head -2 && echo "---" && file "$(ls -t | head -1)"

Risk: Safe (read or common local file operation)

ls: invalid argument ‘style=locale’ for ‘--time’
Valid arguments are:
  - ‘atime’, ‘access’, ‘use’
  - ‘ctime’, ‘status’
  - ‘birth’, ‘creation’
Try 'ls --help' for more information.

```

In that case read error, and ask agent to create other solution (passing error message as detail).

Retry max 3 times.

When sending information about error to agent take only max first 512 characters of error response.



