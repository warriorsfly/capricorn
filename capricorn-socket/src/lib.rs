#[macro_use]
extern crate serde_derive;

mod message;
mod server;
mod session;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
