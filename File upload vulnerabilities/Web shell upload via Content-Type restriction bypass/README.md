## Hack Steps

1. Fetch login page
2. Extract the csrf token and session cookie
3. Login as wiener
4. Extract the new csrf token from wiener profile
5. Upload the shell file with the Content-Type changed
6. Fetch the uploaded shell file to read the secret
7. Submit the solution 


## Run Script

1. Change the URL of the lab
2. Start script

```
~$ cargo run
```

## Expected Output

```
⦗1⦘ Fetching the login page.. OK
⦗2⦘ Extracting the csrf token and session cookie.. OK
⦗3⦘ Logging in as wiener.. OK
⦗4⦘ Extracting the new csrf token from wiener profile.. OK
⦗5⦘ Uploading the shell file with the Content-Type changed.. OK
⦗6⦘ Fetching the uploaded shell file to read the secret.. OK
❯❯ Secret: EbjlyC2Jv4I1VisOiK5WgyvEtQUsrGq1
⦗7⦘ Submitting the solution.. OK
🗹 The lab should be marked now as solved
```
