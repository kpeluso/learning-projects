# Small projects to learn Rust

List from http://www.cplusplus.com/forum/beginner/3473/

To run project from repo root: `sh r.sh p1`

If external crates are required, run: `sh r2.sh p6`

## Beginner

1. [X] Write a program which finds the factorial of a number entered by the user. (check for all conditions).
2. [X] Develop a program to convert currency X to currency Y and vice versa.
3. [X] Write a program that prints out a triangle from largest to smallest; user inputs the largest number. Eg:

```
***
**
*
```

4. [X] Write a program that prints out a triangle from smallest to largest; user inputs bottom number. Eg:

```
*
**
***
```

5. [X] Print out a triangle from smallest to largest, skipping even rows. User inputs largest number, eg:

```
*
***
*****
```

6. [X] Develop a program that uses a randomly generated number to select 1 of 3 (or more) functions to show the user.
7. [X] Guessing game. ask the user to guess a number between 1 and a 100. If you guessed correctly, it will say you win. If you're too high or too low it will also let you know.
8. [X] Create a program which generates Fibonacci series til a number 'n', where 'n' is entered by the user. Eg if the user enters 10 then the output would be: `1 1 2 3 5 8`
9. [X] Given a string, determine how many of the characters are vowels and how many are consonants. Terminate the string when the input character encountered is non-alphabetic.
10. [X] Find the Fifth root of the sum of the squares of the first 100 ODD numbers only.
11. [X] Write a program to simulate a simple calculator. It should accept two numbers from the user along with the required operation to be performed. Addition, subtraction, division and multiplication are the basic operations that should be implemented. Feel free to implement other operations. Bonus points for splitting the calculation functions into a separate module.

## Intermediate

12. [ ] Simple file encryption (using something simple like ROT13).
13. [ ] Read XHTML, remove the tags, then print out the remaining text.
14. [X] Write a program which performs addition, subtraction, multiplication of matrices. The dimensions of both the matrices would be specified by the user (dynamic memory allocation required). Use of structure or a class to define the matrix would be a good idea.
15. [ ] Create a sophisticated linked list class. You should be able to insert and delete nodes anywhere in the list, and the nodes should have pointers to nodes both in front and behind them.
16. [ ] Create a command-line todo list with [Clap](https://crates.io/crates/clap). Users should be able to add, complete and delete items. Bonus: use a database (eg SQLite) to persist todo items between program runs.

## Expert

17. [X] Make a Markov chain generator. Read text from a source, create a histogram and allow different prefix lengths. See [Think Python](http://greenteapress.com/thinkpython2/html/thinkpython2014.html#sec159) for info.
18. [ ] Create a binary tree which has search and sorting functions.
19. [ ] Create a Quine, (a program that prints out its own source code).

My Rust resources:
- [play with rust](https://play.rust-lang.org/)
- [rust by example](https://doc.rust-lang.org/stable/rust-by-example/)
- [rust book](https://doc.rust-lang.org/book/)
- [cargo book](https://doc.rust-lang.org/cargo/)
- [cargo](https://crates.io/)
- [docstrings 1](https://deterministic.space/machine-readable-inline-markdown-code-cocumentation.html)
- [docstrings 2](https://docs.rs/docstrings/0.1.1/docstrings/)

Recommended resources for Rust:
- https://github.com/kud1ing/awesome-rust
- https://github.com/ctjhoa/rust-learning
- https://github.com/cis198-2016s
- http://fredrik.anderzon.se/2016/05/10/rust-for-node-developers-part-1-introduction/
- http://www.arewewebyet.org/#getting-started
- http://cglab.ca/~abeinges/blah/too-many-lists/book/
- https://www.reddit.com/r/rust/comments/5d4h8l/school_vacation_starts_tomorrow_i_want_to_know/da1zdiz/
