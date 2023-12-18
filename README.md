![Static Badge](https://img.shields.io/badge/Developed%20on-GNU%20Linux-red)
![Static Badge](https://img.shields.io/badge/rustc-v1.73.0--nightly-bright)
![Static Badge](https://img.shields.io/badge/License-GPLv3-green)
![Static Badge](https://img.shields.io/badge/Topics-18-red)
![Static Badge](https://img.shields.io/badge/Labs-152-blue)
![Static Badge](https://img.shields.io/badge/Multi--threaded%20Labs-13-yellow)

![WebScurity Academy](./web-security-rust.png)

## Disclaimer

This repository is for educational purposes only, and I am not responsible for using any of the techniques described here for illegal usage.

## Using Burp Suite for Labs

You can, of course, solve the labs using Burp Suite, but this repository is for those who want to take their scripting skills up to a higher level.

This repository assumes that you already know how to solve the labs and want to solve them using scripts in order to practice writing robust ones.
You may not find a detailed description of how the labs should be solved but you will find a detailed description of how the scripts were written.

## Trivial Labs

There are some labs that you may find trivial in their solutions and don't necessarily require a script. In fact, solving them without a script might be faster and easier. I have only written scripts for these labs for the completeness of this repository. Feel free to skip them if you prefer.

## Error Handling

Since this repository is intended for educational purposes and not for production, I have omitted some error handling, which I believe will not significantly impact your testing of the script. This decision represents a trade-off between addressing every possible scenario and prioritizing simplicity.

## Reporting Issues

If you encounter any issues or have suggestions for improvement while working with these scripts, feel free to open an issue.
Your feedback is valuable, and I appreciate your contributions to enhance the learning experience for everyone.

## Why Rust? 🦂

Rust gives you speed (as if you write C code) and a high level of interfaces and API (as if you write Python code).

Due to its power and speed, it evolved rapidly in much areas. Big companies now are migrating to Rust (If they are not alreay did).
We, penetration testers, will also make it evolves in our field leveraging its power and making our tests more efficient than ever. That's why I created this repositroy.

Although learning Rust can be challenging due to its steep learning curve, with time, you will develop a deep appreciation for the language and become proficient in writing code with it. You will find that it is not a difficult task to write a script, and it will be far faster than Python.

If you are not interested in speeding up automation as much as possible, then this repository is not for you.

## Multi-threaded Programming 🚀

You can leverage multi-threading to achieve significantly higher speed in your tests. In fact, you can write a script that is **10** times faster than a single-threaded one. That is why I have written a multi-threaded version for the labs that require the use of brute force technique to solve them.

These multi-threaded scripts are not perfectly written as final code for a thread-safe program due to concurrent issues; they may fail at times due to these issues. However, in most cases, they will work as you expect. I believe this level of performance is sufficient for you as a penetration tester.

If you find the scripts too hard, it is advisable to stick with single-threaded ones. Even with a single thread, they will still be faster than using Python.

## Rust Alternatives

At the begining, you may find that it so difficult to write Rust code and you will want to get back to python. Don't give up easly!

If you insist and don't get comfrotable with Rust, you can check out [WebSecurity Academy with Python](https://github.com/elqal3awii/WebSecurity-Academy-with-Python) repository in which I have solved the same labs using Python.

## Support & Star ✨

If you appreciate the work and find it valuable, please consider giving this repository a star. Your support is greatly appreciated and helps to showcase the popularity and significance of the project. Thank you for your interest and support!

## Resources

- [Jim Blandy, Jason Orendorﬀ, and Leonora F.S. Tindall. (2021). _Programming Rust_. O’Reilly.](https://www.goodreads.com/book/show/25550614-programming-rust?ref=nav_sb_ss_2_16)
- [Steve Klabnik, Carol Nichols. (2018). _The Rust Programming Language_. no strach press.](https://doc.rust-lang.org/book/title-page.html)
- [Rust Documentation](https://doc.rust-lang.org/beta/)
