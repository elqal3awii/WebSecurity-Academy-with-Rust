## Hack Steps

1. Fetch a product page
2. Extract the debug path
3. Fetch the debug path
4. Extract the secret key
5. Submit the solution

## Run Script

1. Change the URL of the lab
2. Start script

```
~$ cargo run
```

## Expected Output

```
⦗1⦘ Checking the source code.. OK
⦗2⦘ Extracting the debug path.. OK => /cgi-bin/phpinfo.php
⦗3⦘ Fetching the debug page.. OK
⦗4⦘ Extracting the secret key.. OK => 8ewgag7yl0vf8dfbti11d0gy6rr1ie37
⦗5⦘ Submitting the solution.. OK
🗹 The lab should be marked now as solved
```
