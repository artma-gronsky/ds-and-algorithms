use std::cell::RefCell;
use std::rc::{Rc, Weak};

#[derive(Debug)]
pub struct DbNode<T> {
    data: T,
    next: Option<Rc<RefCell<DbNode<T>>>>,
    prev: Option<Weak<RefCell<DbNode<T>>>>,
}

impl<T> DbNode<T> {
    pub fn get_data(&self) -> &T {
        &self.data
    }
}

#[derive(Debug)]
pub struct DbList<T> {
    first: Option<Rc<RefCell<DbNode<T>>>>,
    last: Option<Weak<RefCell<DbNode<T>>>>,
}

impl<T> DbList<T> {
    pub fn new() -> Self {
        DbList {
            first: None,
            last: None,
        }
    }

    pub fn push_front(&mut self, new_item: T) {
        match self.first.take() {
            Some(r) => {
                let db_node = DbNode {
                    prev: None,
                    next: Some(r.clone()),
                    data: new_item,
                };

                let new = Rc::new(RefCell::new(db_node));

                let mut r = r.borrow_mut();
                r.prev = Some(Rc::downgrade(&new));
                self.first = Some(new);
            }
            None => {
                let db_node = DbNode {
                    prev: None,
                    next: None,
                    data: new_item,
                };
                let db_node_cell = RefCell::new(db_node);
                let db_node_rc = Rc::new(db_node_cell);
                let db_node_weak = Rc::downgrade(&db_node_rc);
                self.first = Some(db_node_rc);
                self.last = Some(db_node_weak);
            }
        }
    }

    pub fn push_back(&mut self, new_item: T) {
        match self.last.take() {
            Some(mut r) => {
                let db_node = DbNode {
                    next: None,
                    prev: Some(r.clone()),
                    data: new_item,
                };

                let new = Rc::new(RefCell::new(db_node));

                let mut r = Weak::upgrade(&r.clone()).unwrap();
                r.borrow_mut().next = Some(new.clone());
                self.last = Some(Rc::downgrade(&new));
            }
            None => {
                let db_node = DbNode {
                    prev: None,
                    next: None,
                    data: new_item,
                };
                let db_node_cell = RefCell::new(db_node);
                let db_node_rc = Rc::new(db_node_cell);
                let db_node_weak = Rc::downgrade(&db_node_rc.clone());
                self.first = Some(db_node_rc);
                self.last = Some(db_node_weak);
            }
        }
    }

    pub fn get_last(&self) -> Option<&Weak<RefCell<DbNode<T>>>> {
        self.last.as_ref()
    }
}
