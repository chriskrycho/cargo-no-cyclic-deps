use a::T;

pub struct Y;

impl T for Y {
    fn do_it() {
        println!("implemented in B")
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
