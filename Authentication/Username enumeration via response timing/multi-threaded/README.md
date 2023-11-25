## Hack Steps

1. Read usernames and passwords lists
2. Change X-Forwarded-For header to a random IP in every request to bypass blocking
3. Try to find a valid username via response timing
4. Brute force the password of that valid username
5. Login with the valid credentials

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
❯❯ Elapsed: 7  seconds || Trying (96/101): ajax
🗹 Valid username: at
⦗4⦘ Brute forcing password..
❯❯ Elapsed: 14 seconds || Trying (56/101): 000000
🗹 Valid username: at
🗹 Valid password: harley
⦗5⦘ Logging in.. OK
🗹 Finished in: 15 seconds
🗹 The lab should be marked now as solved
```

## Test Samples

This test is done using only 100 users & 100 passwods. What about 10K users & 10K passwords?
Or what about 100K users & 100K passwords?

You can see the comparison I made with these numbers when solving the [Lab: Username enumeration via different responses](https://github.com/elqal3awii/WebSecurity-Academy-with-Rust/tree/main/Authentication/Username%20enumeration%20via%20different%20responses) to see the big difference in speed between Rust and Python and also between single-threaded and multi-threaded approaches in Rust.
