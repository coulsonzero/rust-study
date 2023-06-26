# rust-study

## init
create project
```shell
git clone https://github.com/coulsonzero/rust-study.git
cd rust-study
```

clone and push project
```shell
git init
git add .
git commit -m "first commit"
git remote add origin https://github.com/coulsonzero/rust-study.git
git pull origin master
git push -u origin master
```


## mod & use & as

> `mod: import dir`
> 
> `use: simplify the name of the module usage`

```shell
├── src
│   ├── add
│   │   ├── add_one.rs
│   │   └── structs
└── └── m
```

### file-file
```rust
//! string.rs
//! public functions usage
//!     1. mod string;
//!     2. string::hello();

pub fn hello() {
    println!("this is a string.");
}
```

```rust
mod string;

fn main() {
    string::hello();
}
```

### dir-file

```rust
// add/structs
pub mod add_one;
```


```rust
// add/add_one.rs
pub fn plus (base: u32) -> u32 {
    base + 1
}
```
```rust
mod add;

fn main() {
    println!("{}", add::add_one::plus(1));   // output: 2
}
```

**use**

```rust
use add::add_one;

mod add;

fn main() {
    println!("{}", add_one::plus(1));   // output: 2
}
```

```rust
// import *
use add::add_one::*;

mod add;

fn main() {
    println!("{}", plus(1));   // output: 2
}
```

```rust
// only import `plus` 
use add::add_one::plus;

mod add;

fn main() {
    println!("{}", plus(1));   // output: 2
}
```

```rust
// only import `plus` 
use crate::add::add_one::{plus};

mod add;

fn main() {
    println!("{}", plus(1));   // output: 2
}
```

```rust
use add::add_one::plus as p;

mod add;

fn main() {
    println!("{}", p(1));   // output: 2
}
```