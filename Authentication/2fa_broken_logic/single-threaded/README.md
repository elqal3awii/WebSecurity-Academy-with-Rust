# Hack Steps
1. Get a valid session using valid credentials
2. GET /login2 page
3. Brute force the mfa-code

# Run Script
1. change the URL of the lab
2. Start script
```
~$ cargo run
```

# Expected Output
```
1. Obtaining a valid session ..☑️
2. GET /login2 page ..☑️
3. Start brute forcing mfa-code ..
[*] 1467 => Incorrect
[*] 1468 => Correct
✅ Finished in: 4 minutes
```

