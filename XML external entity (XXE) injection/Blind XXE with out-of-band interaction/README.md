## Hack Steps

1. Use an external entity to issue a DNS lookup to burp collaborator
2. Check your burp collaborator for the DNS lookup

## Run Script

1. Change the URL of the lab
2. Change the domain of the burp collaborator
3. Start script

```
~$ cargo run
```

## Expected Output

```
⦗#⦘ Injection point: productId
❯❯ Using an external entity to issue a DNS lookup to burp collaborator.. OK
🗹 Check your burp collaborator for the DNS lookup
🗹 The lab should be marked now as solved
```
