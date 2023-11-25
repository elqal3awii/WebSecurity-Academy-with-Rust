## Hack Steps

1. Fetch the administrator page via URL id parameter
2. Extract the password from the source code
3. Fetch the login page to get a valid session and the csrf token
4. Login as administrator
5. Delete carlos

## Run Script

1. Change the URL of the lab
2. Start script

```
~$ cargo run
```

## Expected Output

```
⦗1⦘ Fetching the administrator profile page.. OK
⦗2⦘ Extracting password from the source code.. OK => o5t0q3q6l9r4ly948g4t
⦗3⦘ Fetching the login page to get a valid session and the csrf token.. OK
⦗4⦘ Logging in as administrator.. OK
⦗5⦘ Deleting carlos.. OK
🗹 The lab should be marked now as solved
```
