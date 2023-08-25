# # Hack Steps
1. Enumerate a valid username via different error messages
2. Brute force password of that valid username

# Run Script
1. change the URL of the lab
2. change the PATH for your usernames list
3. change the PATH for you passwords list
4. Start script
```
~$ cargo run
```
# Test Samples
#### Objective
See how much time the script will take to find a valid credentials

#### How to test?
1. Obtain a valid username & password using Burp Suite or by runnig this script with the username & passwords lists provided on the Burp Suite Academy.
2. put the valid credentials at the bottom of both lists (simulating the worst case).

### Run tests
When running this script on a Core i7, 4th generation laptop with 16G RAM, I obtain the following results:
### 1000 users & 1000 password
It toke approximately only **2** minutes!

### 10K users & 10K passwords
It toke approximately only **13** minutes!

