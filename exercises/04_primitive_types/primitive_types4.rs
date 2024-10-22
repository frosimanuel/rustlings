fn main() {
    // You can optionally experiment here.
    let a = [1, 2, 3, 4, 5];
    println!("a has {} elements", a.len());
    let slice = &a[1..3];
    println!("a slice has {} elements", slice.len());
}

#[cfg(test)]
mod tests {
    #[test]
    fn slice_out_of_array() {
        let a = [1, 2, 3, 4, 5];

        // TODO: Get a slice called `nice_slice` out of the array `a` so that the test passes.
        let nice_slice = a.get(1..4).unwrap();

        assert_eq!([2, 3, 4], nice_slice);
    }
}
//crabs
