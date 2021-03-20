pub trait Node<T> {
    fn get_children(&self) -> Vec<T>;
}
