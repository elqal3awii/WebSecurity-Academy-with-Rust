## Hack Steps

1. Fetch the robots.txt file
2. Search for hidden paths
3. Fetch the hidden path
4. Extract the path to the backup file
5. Fetch the backup file
6. Extract key
7. Submitt the solution

## Run Script

1. Change the URL of the lab
2. Start script

```
~$ cargo run
```

## Expected Output

```
⦗1⦘ Fetching the robots.txt file.. OK
⦗2⦘ Searching for hidden paths.. OK => /backup
⦗3⦘ Fetching the hidden path.. OK
⦗4⦘ Extracting the path to the backup file.. OK => /backup/ProductTemplate.java.bak
⦗5⦘ Fetching the backup file.. OK
⦗6⦘ Extracting key .. OK => xydew2o4wwjnyn3z444f8rn3pdad1ld2
⦗7⦘ Submitting the solution.. OK
🗹 The lab should be marked now as solved
```
