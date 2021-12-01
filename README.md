# Rust binary integration in Typescript (alpha API)

## Contact
- Eduardo Sánchez<br>


- whipshout@gmail.com<br>


- https://twitter.com/Whipshout <br>


- https://www.linkedin.com/in/eduardo-sanchez-sanchez/ <br>

## Info

- We can integrate **Rust with Node through native addons**, as we would with **C++**. <br>


- To do this, we have to interact with **Typescript through C**. We compile a **Rust library using C interfaces and bindings**. The library is in different formats depending on the operating system.<br>


- To do all the hard work, we use the **crate called NAPI**, which is responsible for making the interfaces and bindings with C and exporting the functions for use in Javascript.

## Performance

Generating **100K uuid** from an **input**

![Performance](./resources/bench.PNG)