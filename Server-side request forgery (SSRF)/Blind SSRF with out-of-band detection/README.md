## Hack Steps

1. Inject payload into the Referer header to cause an HTTP request to the burp collaborator
2. Check your burp collaborator for the HTTP request

## Run Script

1. Change the URL of the lab
2. Change the domain of the burp collaborator
3. Start script

```
~$ cargo run
```

## Expected Output

```
⦗#⦘ Injection point: Referer header
❯❯ Injecting payload to cause an HTTP request to the burp collaborator.. OK
🗹 Check your burp collaborator for the HTTP request
🗹 The lab should be marked now as solved
```
