#[macro_use]
extern crate serde_derive;

mod frame;
mod message;
mod model;
mod session;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
