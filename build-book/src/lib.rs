#![feature(const_trait_impl)]
#![feature(result_flattening)]
#![feature(drain_filter)]
#![feature(never_type)]
#![feature(iterator_try_collect)]
#![feature(is_some_and)]
#![feature(result_option_inspect)]

pub mod book;
pub use book::Book;

pub(crate) mod parsers;

pub mod frontends {
    mod tex {}
    mod html {}
    mod plain {}
    mod md {}
}
