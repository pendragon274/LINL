#[allow(dead_code)]
pub struct LinkedArray<T: Sized + Copy, const N: usize> {
    collection: [Option<T>; N]
}

impl<T: Sized + Copy, const N: usize> LinkedArray<T, N> {
    pub fn new() -> Self {
        LinkedArray{
            collection: [None; N]
        }
    }
}