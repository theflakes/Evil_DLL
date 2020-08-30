# Evil_DLL
Simple DLL to test various injection methods.

Will write a file to the directory "c:\pwned" named pwned_{pid}.txt.
- {pid} = pid of process injected.

File content example:
```
[*]          Pid: "7204"
[*]      Process: "C:\\Windows\\system32\\regsvr32.exe"
[*]         Args: [".\\evil_dll.dll"]
[*]         User: "user"
[*]       Domain: "DOMAIN"
[*] Created file: "c:\\pwned\\pwned_7204.txt"
```

# Disclaimer
This tool comes with no warranty or support. If anyone chooses to use it, you accept all responsability and liability.
