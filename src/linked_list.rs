#[derive(Debug)]
pub struct LinkedList<T>(Option<(T, Box<LinkedList<T>>)>);

impl<T> LinkedList<T> {
    pub fn new() -> Self {
        LinkedList(None)
    }

    pub fn push_front(&mut self, item: T) {
        let temp = self.0.take();

        self.0 = Some((item, Box::new(LinkedList(temp))));
    }

    pub fn push_back(&mut self, item: T) {
        match &mut self.0 {
            None => {
                self.push_front(item);
            }
            Some((_, child)) => {
                child.push_back(item);
            }
        }
    }
}
