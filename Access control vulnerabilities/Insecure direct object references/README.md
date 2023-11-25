## Hack Steps

1. Fetch the 1.txt log file
2. Extract carlos password from the log file
3. Fetch the login page to get a valid session and the csrf token
4. Login as carlos

## Run Script

1. Change the URL of the lab
2. Start script

```
~$ cargo run
```

## Expected Output

```
⦗1⦘ Fetching the 1.txt log file.. OK
⦗2⦘ Extracting password from the log file.. OK => g85h50jv195a84egtzlr
⦗3⦘ Fetching the login page to get a valid session and the csrf token.. OK
⦗4⦘ Logging in as carlos.. OK
⦗5⦘ Fetching carlos profile.. OK
🗹 The lab should be marked now as solved
```
