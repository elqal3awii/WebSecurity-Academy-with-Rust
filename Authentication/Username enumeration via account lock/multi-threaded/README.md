# Hack Steps
1. Try all users multiple times until on account is locked
2. Brute force password of that valid username
3. Wait 1 minute every 3 password tries to bypass blocking

# Run Script
1. Change the URL of the lab
2. Change the file path of the username list
3. Change the file path of the password list
4. Change the separator in the split function to \r\n instead of \n if you are still a windows user
5. Start script
```
~$ cargo run
```

# Expected Output
```
[#] Enumerate usernames..
[*] Try number 1 of all users..
Elapsed: 3 minutes || Failed: 0 || Trying (6/101): adam                                              
[#] Brute forcing password..
✅ Valid user: adam
Elapsed: 26 minutes || Failed: 0 || Trying (74/102): joshua                                            
✅ Login successfully:  username: adam, password: joshua
✅ Finished in: 0 minutes
Results was saved to: results
Failed users count: 0
Failed users: [  ]
Failed passwords count: 0
Failed passwords: [  ]
```
