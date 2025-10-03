pub trait List<T> {
    fn read(&self, index: usize) -> Option<&T>;
    fn size(&self) -> usize;
    fn reverse(&self) -> Self;
    fn find(&self, val: T) -> Option<usize>;
    fn map(&self, f: impl Fn(&T) -> T) -> Self;
    fn filter(&self, f: impl Fn(&T) -> bool) -> Self;
}
