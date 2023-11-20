# Hack Steps

1. Fetch /login page
2. Extract the session and the csrf token
3. Login as wiener
4. Extract the new session
5. Bypass admin access using custom header
6. Delete carlos

# Run Script

1. Change the URL of the lab
2. Start script

```
~$ cargo run
```

# Expected Output

```
⦗1⦘ Fetching /login page.. OK
⦗2⦘ Getting session and csrf token.. OK
⦗3⦘ Logging in as wiener.. OK
⦗4⦘ Getting a new session as wiener .. OK
⦗5⦘ Bypassing admin access using custom header.. OK
⦗6⦘ Deleting carlos.. OK
🗹 The lab should be marked now as solved
```
