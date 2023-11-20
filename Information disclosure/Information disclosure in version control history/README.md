# Hack Steps

1. Fetch the .git directory
2. Reset to the previous commit
3. Get the administrator password from the admin.conf file
4. Login as administrator
5. Delete carlos

# Run Script

1. Change the domain of the lab
2. Make sure that wget and git are installed on your system
3. Start script

```
~$ cargo run
```

# Expected Output

```
⦗1⦘ Fetching .git directory (wait a minute).. OK
⦗2⦘ Changing current working directory.. OK
⦗3⦘ Resetting to the previous commit.. OK
⦗4⦘ Reading admin.conf file.. OK
⦗5⦘ Extracting the administrator password.. OK => rip9q0tdout2mduxi2og
⦗6⦘ Fetching login page to get a valid session and csrf token.. OK
⦗7⦘ Logging in as administrator.. OK
⦗8⦘ Deleting carlos.. OK
🗹 The lab should be marked now as solved
```
