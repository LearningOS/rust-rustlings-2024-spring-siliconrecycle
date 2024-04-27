/*
sort
This problem requires you to implement a sorting algorithm
you can use bubble sorting, insertion sorting, heap sorting, etc.
*/

fn sort<T: std::cmp::PartialOrd>(array: &mut [T]){
    //TODO
    // a, b, c, d, e
    // left -> right: a*, b
    // ?: a* > b,
    // t=> swap(a, b), left -> right: a*, c
    // f=>
    // right -> left: a.prev, a**
    // ?: a** < a.prev
    // t=>swap(a, a.prev), right -> left: a.prev.prev, a**
    // f=>
    // left -> right: a*

    // 37, 73, 57, 75, 91, 19, 46, 64
    //
    for i in 0..array.len()-1 {
        // left -> right
        if array[i] > array[i + 1] {
            array.swap(i, i + 1);
        };
        let mut to_right = i;
        // right -> left
        while to_right > 0 {
            if array[to_right] <= array[to_right - 1] {
                array.swap(to_right, to_right - 1);
            };
            to_right -= 1;
        };
    };
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sort_1() {
        let mut vec = vec![37, 73, 57, 75, 91, 19, 46, 64];
        sort(&mut vec);
        assert_eq!(vec, vec![19, 37, 46, 57, 64, 73, 75, 91]);
    }
    #[test]
    fn test_sort_2() {
        let mut vec = vec![1];
        sort(&mut vec);
        assert_eq!(vec, vec![1]);
    }
    #[test]
    fn test_sort_3() {
        let mut vec = vec![99, 88, 77, 66, 55, 44, 33, 22, 11];
        sort(&mut vec);
        assert_eq!(vec, vec![11, 22, 33, 44, 55, 66, 77, 88, 99]);
    }
}
