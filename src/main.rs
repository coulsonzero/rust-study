/* ================ main.rs ================ */
#![allow(dead_code)]            // warning: struct `Rectangle` is never constructed, warning: function `run` is never used
#![allow(unused_imports)]       // unused import: `crate::template::template_impl as ti`
#![allow(unused_variables)]     // unused variable: `integer`
#![allow(unused_doc_comments)]  // warning: unused doc comment
#![allow(unused_mut)]           // warning: variable does not need to be mutable
#![allow(deprecated)]           // warning: use of deprecated function
#![allow(unused)]


//! simplify the usage of module
use add::add_one::plus;
use crate::basic::output;
use crate::basic::input;
use crate::basic::datatype;
use crate::basic::vars;
use crate::control::while_loop;
use crate::core::string;
use crate::core::tuple;
use crate::core::vector;
use crate::core::hashmap;


/// manage modules
mod add;
mod basic;
mod control;
mod core;
mod template;
mod functions;
mod macros;
mod depends;





/// note: `#[warn(dead_code)]` on by default
fn main() {
    /* module usage example */
    // println!("{}", plus(1));

    /* basic */
    // output::run();
    // input::example_str();
    // input::example_num();
    // datatype::run();
    // vars::run();
    // basic::vars_more::run();
    // basic::vars_more::let_example();

    /* control */
    // control::for_loop::run();
    // while_loop::run();


    /* core */
    // string::hi("John Smith".to_string());
    // string::run();
    // tuple::run();
    // vector::run();
    // hashmap::run();
    // core::str::run();


    /* functions */
    // functions::fun::run();
    // functions::rand::run();

    // structs
    // structs::struct_func::run();
    // structs::struct_impl::run();
    // structs::struct_impl_for::run();
    // sif::run();




    /* template */



    /* macros */
    // macros::macro_r::run();
    // macros::type_of::example();
    // macros::color::example();
    // macros::color::example();


    /* depends */
    // depends::depend::run();



}

// output:



