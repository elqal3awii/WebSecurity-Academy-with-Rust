# Hack Steps

1. Fetch login page
2. Extract csrf token and session cookie
3. Login as wiener
4. Fetch wiener profile
5. Upload the shell file with the Content-Type changed
6. Fetch the uploaded shell file to read the secret
7. Submit solution 


# Run Script

1. Change the URL of the lab
2. Start script

```
~$ cargo run
```

# Expected Output

```
⦗1⦘ Fetching the login page.. OK
⦗2⦘ Extracting csrf token and session cookie.. OK
⦗3⦘ Logging in as wiener.. OK
⦗4⦘ Fetching wiener profile.. OK
⦗5⦘ Uploading the shell file with the Content-Type changed.. OK
⦗6⦘ Fetching the uploaded shell file to read the secret.. OK
❯ Secret: EbjlyC2Jv4I1VisOiK5WgyvEtQUsrGq1
⦗7⦘ Submitting solution.. OK
🗹 Check your browser, it should be marked now as solved
```