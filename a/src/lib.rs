use b::Y;

pub trait T {
    fn do_it();
}

impl T for Y {
    fn do_it() {
        println!("Implemented in A");
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
