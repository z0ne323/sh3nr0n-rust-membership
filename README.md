# sh3nr0n-rust-membership ?

This project is intented to be a simple implementation of the "membership" side of Shodan Rest API (non-enterprise) and Shodan Streaming API (non-enterprise) in Rust.

It's not at all intended to be something exhaustive with everything from the Shodan API. If you see something missing, or not done the *proper* way (still learning Rust tbh) and would like me to add something or if you'd like to contribute, feel free to open a PR or an issue request!

### What do I mean by "membership" side of Shodan API?

You'll need a paid plan with your shodan account to make this project work. 

You have different options at hand that you'll find on the **Choose Your Plan** page on Shodan just [here](https://account.shodan.io/billing). 

The main requirement for this project is to get an API key after either getting a one time membership on Shodan or paying a subscription with them.

Before starting to build the project, don't forget to put the absolute path of your API Key file in `main.rs` (at the top), if you don't do it, project will not run.

# What's in there ?

You'll find different things in this repo:
- `cargo.lock` & `cargo.toml`, necessary configuration file no matter what you're doing with this project...
- `src/` folder contains various files:
    - `main.rs` -> Like the name state, main file of the project, calling functions and handling some logic
    - `helpers.rs` -> Helpers file, storing generic functions
    - `shodan.rs` -> probably most important file of this project, storing all the functions interacting with Shodan API and called in main !

(Side note: In this project, `main.rs` is just used to show how to call every functions and provide examples about Shodan API. This file should not be considered to be an actual part of your project, `shodan.rs` and `helpers.rs` as **modules** should be **THE** part of my project you'll use in your actual `main.rs` file)

# Links ?

## If you'd like... 
- To take a tour of all services proposed by Shodan (API and more): https://account.shodan.io/billing/tour
- To take a look at the pricing options: https://account.shodan.io/billing
- To check the Shodan Developer overview page: https://developer.shodan.io/
- To check the Rest API page: https://developer.shodan.io/api
