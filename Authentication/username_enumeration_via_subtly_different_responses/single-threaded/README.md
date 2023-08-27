# Hack Steps
1. Obtain a valid username via subtly different error messages
2. Brute force password of that valid username

# Run Script
1. change the URL of the lab
2. change the PATH for your usernames list
3. change the PATH for you passwords list
4. Start script
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

✅ Finished in: 0 minutes

[!] Failed users count: 0
[!] Failed users: []

[!] Failed passwords count: 0
[!] Failed passwords: []
```
# Test Samples
This test is done using only 100 users & 100 passwods. What about 10K users & 10K passwords?
Or what about 100K users & 100K passwords?

You can see the comparison I made with these numbers when solving the [Lab: Username enumeration via different responses](https://github.com/elqalawii/portswigger_labs_with_rust/tree/main/Authentication/username_enumeration_via_different_responses) to see the big difference in speed between Rust and Python and also between single-threaded and multi-threaded approaches in Rust.

### Happy Hacking 👾
