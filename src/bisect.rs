/// Locate the insertion point for x in a to maintain sorted order. The parameters lo and hi may be used to specify a subset of the list which should be considered; by default the entire list is used. If x is already present in a, the insertion point will be before (to the left of) any existing entries. The return value is suitable for use as the first parameter to list.insert() assuming that a is already sorted.
///
/// The returned insertion point i partitions the array a into two halves so that all(val < x for val in a[lo:i]) for the left side and all(val >= x for val in a[i:hi]) for the right side.
pub fn bisect_left<T: PartialOrd>(x: T, a: Vec<T>) -> usize {
    // naive implementation.
    for i in 0..a.len() {
        if a[i] >= x {
            return i
        }
    }
    a.len()
}

//pub fn bisect_left<T>(x: T, a: Vec<T>, lo: usize, hi: usize) -> usize {
//    unimplemented!()
//}
//
pub fn bisect_right<T>(x: T, a: Vec<T>) -> usize {
    unimplemented!()
}

//pub fn bisect_right<T>(x: T, a: Vec<T>, lo: usize, hi: usize) -> usize {
//    unimplemented!()
//}
//
pub fn bisect<T>(x: T, a: Vec<T>) -> usize {
    unimplemented!()
}

//pub fn bisect<T>(x: T, a: Vec<T>, lo: usize, hi: usize) -> usize {
//    unimplemented!()
//}
//
