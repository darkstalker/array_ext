pub trait Array<T>
{
    // taken from the slice impl
    fn len(&self) -> usize;
    fn is_empty(&self) -> bool;
    fn first(&self) -> Option<&T>;
    fn first_mut(&mut self) -> Option<&mut T>;
    fn last(&self) -> Option<&T>;
    fn last_mut(&mut self) -> Option<&mut T>;
    fn get(&self, index: usize) -> Option<&T>;
    fn get_mut(&mut self, index: usize) -> Option<&mut T>;
    fn as_ptr(&self) -> *const T;
    fn as_mut_ptr(&mut self) -> *mut T;
    // convenience methods
    fn as_slice(&self) -> &[T];
    fn as_mut_slice(&mut self) -> &mut [T];
    fn map<F>(self, f: F) -> Self where T: Copy, F: FnMut(T) -> T;
    fn fold<A, F>(self, acc: A, f: F) -> A where T: Copy, F: FnMut(A, T) -> A;
}

// for arrays with 1+ elements
macro_rules! impl_array
{
    (@replace $t:tt $sub:expr) => ($sub);

    (@count $($e:expr)*) => { 0usize $(+ impl_array!(@replace $e 1usize))* };

    ($($idx:expr)+) => {
        impl <T> ::Array<T> for [T; impl_array!(@count $($idx)+)]
        {
            fn len(&self) -> usize { impl_array!(@count $($idx)+) }
            fn is_empty(&self) -> bool { false }
            fn first(&self) -> Option<&T> { Some(&self[0]) }
            fn first_mut(&mut self) -> Option<&mut T> { Some(&mut self[0]) }
            fn last(&self) -> Option<&T> { Some(&self[impl_array!(@count $($idx)+) - 1]) }
            fn last_mut(&mut self) -> Option<&mut T> { Some(&mut self[impl_array!(@count $($idx)+) - 1]) }
            fn get(&self, index: usize) -> Option<&T> { if index < impl_array!(@count $($idx)+) { Some(&self[index]) } else { None } }
            fn get_mut(&mut self, index: usize) -> Option<&mut T> { if index < impl_array!(@count $($idx)+) { Some(&mut self[index]) } else { None } }
            fn as_ptr(&self) -> *const T { &self[0] }
            fn as_mut_ptr(&mut self) -> *mut T { &mut self[0] }
            fn as_slice(&self) -> &[T] { self }
            fn as_mut_slice(&mut self) -> &mut [T] { self }
            fn map<F>(self, mut f: F) -> Self where T: Copy, F: FnMut(T) -> T { [$(f(self[$idx])),+] }
            fn fold<A, F>(self, mut acc: A, mut f: F) -> A where T: Copy, F: FnMut(A, T) -> A { $(acc = f(acc, self[$idx]);)+ acc }
        }
    };
}

// workaround to not being able to cast `&mut [T; 0]` to `*mut T` directly
fn get_mut_ptr<T>(a: &mut [T; 0]) -> *mut T { a.as_mut_ptr() }

// special case for the empty array
impl <T> Array<T> for [T; 0]
{
    fn len(&self) -> usize { 0 }
    fn is_empty(&self) -> bool { true }
    fn first(&self) -> Option<&T> { None }
    fn first_mut(&mut self) -> Option<&mut T> { None }
    fn last(&self) -> Option<&T> { None }
    fn last_mut(&mut self) -> Option<&mut T> { None }
    fn get(&self, _index: usize) -> Option<&T> { None }
    fn get_mut(&mut self, _index: usize) -> Option<&mut T> { None }
    fn as_ptr(&self) -> *const T { self as _ }
    fn as_mut_ptr(&mut self) -> *mut T { get_mut_ptr(self) }
    fn as_slice(&self) -> &[T] { self }
    fn as_mut_slice(&mut self) -> &mut [T] { self }
    fn map<F>(self, _f: F) -> Self where T: Copy, F: FnMut(T) -> T { self }
    fn fold<A, F>(self, acc: A, _f: F) -> A where T: Copy, F: FnMut(A, T) -> A { acc }
}

impl_array!(0);
impl_array!(0 1);
impl_array!(0 1 2);
impl_array!(0 1 2 3);
impl_array!(0 1 2 3 4);
impl_array!(0 1 2 3 4 5);
impl_array!(0 1 2 3 4 5 6);
impl_array!(0 1 2 3 4 5 6 7);
impl_array!(0 1 2 3 4 5 6 7 8);
impl_array!(0 1 2 3 4 5 6 7 8 9);
impl_array!(0 1 2 3 4 5 6 7 8 9 10);
impl_array!(0 1 2 3 4 5 6 7 8 9 10 11);
impl_array!(0 1 2 3 4 5 6 7 8 9 10 11 12);
impl_array!(0 1 2 3 4 5 6 7 8 9 10 11 12 13);
impl_array!(0 1 2 3 4 5 6 7 8 9 10 11 12 13 14);
impl_array!(0 1 2 3 4 5 6 7 8 9 10 11 12 13 14 15);
impl_array!(0 1 2 3 4 5 6 7 8 9 10 11 12 13 14 15 16);


#[cfg(test)]
mod tests
{
    use super::Array;
    use std::ops::Add;

    #[test]
    fn methods()
    {
        let mut arr: [i32; 0] = [];
        assert_eq!(arr.len(), 0);
        assert_eq!(arr.is_empty(), true);
        assert_eq!(arr.first(), None);
        assert_eq!(arr.first_mut(), None);
        assert_eq!(arr.last(), None);
        assert_eq!(arr.last_mut(), None);
        assert_eq!(arr.get(1), None);
        assert_eq!(arr.get_mut(1), None);
        assert_eq!(arr.map(|a| a * 2), []);
        assert_eq!(arr.fold(0, |a, n| a + n), 0);

        let mut arr = [1, 2, 3, 4];
        assert_eq!(arr.len(), 4);
        assert_eq!(arr.is_empty(), false);
        assert_eq!(arr.first(), Some(&1));
        assert_eq!(arr.first_mut(), Some(&mut 1));
        assert_eq!(arr.last(), Some(&4));
        assert_eq!(arr.last_mut(), Some(&mut 4));
        assert_eq!(arr.get(1), Some(&2));
        assert_eq!(arr.get(42), None);
        assert_eq!(arr.get_mut(1), Some(&mut 2));
        assert_eq!(arr.get_mut(42), None);
        assert_eq!(arr.map(|a| a * 2), [2, 4, 6, 8]);
        assert_eq!(arr.fold(0, |a, n| a + n), 10);
    }

    fn sum<T, V>(arr: T, val: V) -> T
        where T: Array<V>, V: Add<Output=V> + Copy
    {
        arr.map(|n| n + val)
    }

    #[test]
    fn generics()
    {
        assert_eq!(sum([], 1), []);
        assert_eq!(sum([1], 10), [11]);
        assert_eq!(sum([1, 2], 20), [21, 22]);
        assert_eq!(sum([1, 2, 3], 30), [31, 32, 33]);
        assert_eq!(sum([1, 2, 3, 4], 40), [41, 42, 43, 44]);
    }

    #[test]
    fn slice()
    {
        assert_eq!([1, 2, 3].as_slice(), &[1, 2, 3]);
        assert_eq!([4, 5, 6].as_mut_slice(), &mut [4, 5, 6]);

        let mut arr = [1, -5, 8, 42, 33];
        arr.as_mut_slice().sort();
        assert_eq!(arr, [-5, 1, 8, 33, 42]);
    }
}