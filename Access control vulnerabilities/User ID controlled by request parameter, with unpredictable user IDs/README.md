## Hack Steps

1. Fetch a post published by carlos
2. Extract the GUID of carlos from the response body
3. Fetch carlos profile using his GUID
4. Extract the API key from the response body
5. Submit the solution

## Run Script

1. Change the URL of the lab
2. Change the postId to that of Carlos's post
3. Start script

```
~$ cargo run
```

## Expected Output

```
⦗1⦘ Fetching a post published by carlos.. OK
⦗2⦘ Extracting the GUID of carlos from the response body.. OK
⦗3⦘ Fetching carlos profile page.. OK
⦗4⦘ Extracting the API key from the response body.. OK
⦗5⦘ Submitting the solution.. OK
🗹 The lab should be marked now as solved
```
