pub fn borrow2_from_vec_mut<'a, T>(
    vec: &'a mut Vec<T>,
    idx1: usize,
    idx2: usize,
) -> (&'a mut T, &'a mut T) {
    if idx1 == idx2 {
        panic!("Attempt to borrow &mut to the same element twice");
    }

    let el1;
    let el2;

    if idx1 < idx2 {
        let (head, tail) = vec.split_at_mut((idx1 + 1) as usize);
        el1 = &mut head[idx1 as usize];
        el2 = &mut tail[(idx2 - idx1 - 1) as usize];
    } else {
        let (head, tail) = vec.split_at_mut((idx2 + 1) as usize);
        el1 = &mut tail[(idx1 - idx2 - 1) as usize];
        el2 = &mut head[idx2 as usize];
    }

    (el1, el2)
}
