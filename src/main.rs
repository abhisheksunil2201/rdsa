extern crate lazy_static;
pub mod searching;

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use super::searching;
    #[test]
    fn binary_search() {
        let arr = vec!["zoo", "google", "d", "c", "b", "a"];
        let index = searching::binary_search(&"a", &arr);
        assert_eq!(index, Some(5));
    }
}
