pub mod first;
pub mod second;

fn add_two(a : &mut i32) {
    add_to(a, 1);
    *a += 1;
}

fn add_to(a : &mut i32, b : i32) {
    *a += b;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn test_add_one() {
        let mut a = 6;
        add_two(&mut a);
        assert_eq!(8, a);
    }
}
