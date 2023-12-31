## Hack Steps

1. Fetch login page
2. Extract the csrf token and session cookie
3. Login as wiener
4. Extract the new csrf token from wiener profile
5. Embed the payload in the image using exiftool
6. Change the extension of the image to .php
7. Read the image with embedded payload
8. Upload the image with the embedded payload
9. Fetch the uploaded image with the embedded payload to read the secret
10. Submit the solution

## Run Script

1. Make sure the exiftool is installed on your system
2. Make sure that an image called 'white.jpg' exists in your root directory
3. Change the 'mv' command to 'move' if you are still a windows user (this should make the script still work)
4. Change the URL of the lab
5. Start script

```
~$ cargo run
```

## Expected Output

```
⦗1⦘ Fetching the login page.. OK
⦗2⦘ Extracting the csrf token and session cookie.. OK
⦗3⦘ Logging in as wiener.. OK
⦗4⦘ Extracting the new csrf token from wiener profile.. OK
⦗5⦘ Embedding the payload in the image using exiftool.. OK
⦗6⦘ Changing the extension of the image to .php.. OK
⦗7⦘ Reading the image with embedded payload.. OK
⦗8⦘ Uploading the image with the embedded payload.. OK
⦗9⦘ Fetching the uploaded image with the embedded payload to read the secret.. OK
❯❯ Secret: qSSeG8IY0ORvH4HTUKG2wVLZ8UHJR8bK
⦗10⦘ Submitting the solution.. OK
⦗#⦘ Changing the image extension back to its original one.. OK
🗹 The lab should be marked now as solved
```