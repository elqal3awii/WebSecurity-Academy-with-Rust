## Hack Steps

1. Inject payload as a query parameter
2. Send multiple request to the main page to cache it with the injected payload

## Run Script

1. Change the URL of the lab
2. Start script

```
~$ cargo run
```

## Expected Output

```
❯❯ Poisoning the main page with the payload as a query parameter (3/10)..
❯❯ Poisoning the main page with the payload as a query parameter (10/10).. OK
🗹 The main page is poisoned successfully
🗹 The lab should be marked now as solved
```
