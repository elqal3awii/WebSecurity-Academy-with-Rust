# # Hack Steps
1. Enumerate a valid username via different error messages
2. Brute force password of that valid username

# Run Script
1. Change the URL of the lab
2. Change the PATH for your usernames list
3. Change the PATH for you passwords list
4. Change the the list splitter to \r\n instead of \n if you still a windows user
5. Start script
```
~$ cargo run
```

# Expected Output
```
[#] Enumerate usernames..
Elapsed:   0 minutes || Failed:   0 || Trying (36/101): ae                                                
[#] Brute forcing password..
✅ Valid user: ae
Elapsed:   0 minutes || Failed:   0 || Trying (47/102): robert                                            

✅ Login successfully: username: ae, password: robert

Restults was saved to: results

✅ Finished in: : 0 minutes

[!] Failed users count: 0
[!] Failed passwords: []

[!] Failed passwords count: 0
[!] Failed passwords: []
```

# Test Samples
### Objective
See how much time the script will take to find a valid credentials

### How to test?
1. Obtain a valid username & password using Burp Suite or by runnig this script with the username & passwords lists provided on the Burp Suite Academy.
2. put the valid credentials at the bottom of both lists (simulating the worst case).

### Run tests
When running this script on a Core i7, 4th generation laptop with 16G RAM, I obtain the following results:
#### 1K users & 1K password
It toke approximately only **2** minutes!

#### 10K users & 10K passwords
It toke approximately only **13** minutes!

#### 100K users & 100K passwords
It toke approximately only **2.5** hours!