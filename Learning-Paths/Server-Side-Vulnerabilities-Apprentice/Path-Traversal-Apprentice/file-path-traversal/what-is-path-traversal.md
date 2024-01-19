## What is path traversal?

Path traversal is also known as directory traversal. These vulnerabilities enable an attacker to read arbitrary files on the server that is running an application. This might include:

   -  Application code and data.
   -  Credentials for back-end systems.
   -  Sensitive operating system files.

In some cases, an attacker might be able to write to arbitrary files on the server, allowing them to modify application data or behavior, and ultimately take full control of the server.


## Reading arbitrary files via path traversal

 Imagine a shopping application that displays images of items for sale. This might load an image using the following HTML:
```html
<img src="/loadImage?filename=218.png">
```

The loadImage URL takes a filename parameter and returns the contents of the specified file. The image files are stored on disk in the location /var/www/images/. To return an image, the application appends the requested filename to this base directory and uses a filesystem API to read the contents of the file. In other words, the application reads from the following file path:
```html
/var/www/images/218.png
```

This application implements no defenses against path traversal attacks. As a result, an attacker can request the following URL to retrieve the /etc/passwd file from the server's filesystem:
```html
https://insecure-website.com/loadImage?filename=../../../etc/passwd
```

This causes the application to read from the following file path:
```shell
/var/www/images/../../../etc/passwd
```

The sequence ../ is valid within a file path, and means to step up one level in the directory structure. The three consecutive ../ sequences step up from /var/www/images/ to the filesystem root, and so the file that is actually read is:
```shell
/etc/passwd
```
On Unix-based operating systems, this is a standard file containing details of the users that are registered on the server, but an attacker could retrieve other arbitrary files using the same technique.

On Windows, both `../` and `..\` are valid directory traversal sequences. The following is an example of an equivalent attack against a Windows-based server:
```html
https://insecure-website.com/loadImage?filename=..\..\..\windows\win.ini
```




## Lab: File path traversal, simple case

 This lab contains a path traversal vulnerability in the display of product images.

To solve the lab, retrieve the contents of the `/etc/passwd` file. 

