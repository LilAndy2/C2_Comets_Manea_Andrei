fn fill_vec(vec: Vec<i32>) -> Vec<i32> {
    let mut vec = vec;
    //  ^^^ added

    vec.push(88);

    vec
}

fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn move_semantics1() {
        let vec0 = vec![22, 44, 66];
        let vec1 = fill_vec(vec0);
        // `vec0` can't be accessed anymore because it is moved to `fill_vec`.
        assert_eq!(vec1, vec![22, 44, 66, 88]);
    }
}