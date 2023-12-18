## Hack Steps

1. Store the malicious javascript file on your expoit server
2. Send multiple request to the main page with an unkeyed header pointing to your exploit server

## Run Script

1. Change the URL of the lab
2. Change the DOMAIN of the expoit server
3. Start script

```
~$ cargo run
```

## Expected Output

```
⦗1⦘ Storing the malicious javascript file on your exploit server.. OK
⦗2⦘ Poisoning the main page with an unkeyed header (3/5)..
⦗2⦘ Poisoning the main page with an unkeyed header (5/5).. OK
🗹 The main page is poisoned successfully
🗹 The lab should be marked now as solved
```
