## Hack Steps

1. Read usernames and passwords lists
2. Try to find a valid username via different error messages
3. Brute force the password of that valid username
4. Login with the valid credentials

## Run Script

1. Change the URL of the lab
2. Make sure the passwords and usernames files exist in the root directory (Authentication directory) or change its path accordingly
3. Start script

```
~$ cargo run
```

## Expected Output

```
⦗1⦘ Reading usernames list.. OK
⦗2⦘ Reading password list.. OK
⦗3⦘ Trying to find a valid username.. 
❯❯ Elapsed: 4  seconds || Trying (71/101): access
🗹 Valid username: agenda
⦗4⦘ Brute forcing password..
❯❯ Elapsed: 5  seconds || Trying (12/101): charlie
🗹 Valid username: agenda
🗹 Valid password: 123456
⦗5⦘ Logging in.. OK
🗹 Finished in: 6 seconds
🗹 The lab should be marked now as solved
```

## Test Samples

### Objectives

See how much time the script will take to find a valid credentials

### How to test?

1. Obtain a valid username & password using Burp Suite or by runnig this script with the usernames and passwords lists provided on the Burp Suite Academy.
2. put the valid credentials at the bottom of both lists (simulating the worst case).

### Results

When running this script on a Core i7, 4th generation laptop with 16G RAM, I obtained the following results:


**1K users & 1K password**

- It toke approximately only **2** minutes!

**10K users & 10K passwords**

- It toke approximately only **13** minutes!

**100K users & 100K passwords**

- It toke approximately only **2.5** hours!
