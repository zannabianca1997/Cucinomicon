#![feature(const_trait_impl)]
#![feature(result_flattening)]
#![feature(drain_filter)]
#![feature(never_type)]

pub mod book;
pub use book::Book;

pub mod frontends {
    mod tex {}
    mod html {}
    mod plain {}
    mod md {}
}
