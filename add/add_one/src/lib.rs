pub fn add(left: usize, right: usize) -> usize {
    left + right
}

pub fn add_one(x: usize) -> usize {
    x + 1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(add_one(3), 4);
    }
}
