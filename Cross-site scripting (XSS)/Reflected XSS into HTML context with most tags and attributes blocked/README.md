## Hack Steps

1. Craft an iframe that, when loaded, will change the body width, causing the onresize event handler to be invoked
2. Deliver the exploit to the victim
3. The print() function will be called after they trigger the exploit

## Run Script

1. Change the URL of the lab
2. Change the URL of the exploit server
3. Start script

```
~$ cargo run
```

## Expected Output

```
❯❯ Delivering the exploit to the victim.. OK
🗹 The print() function will be called after they trigger the exploit
🗹 The lab should be marked now as solved
```
