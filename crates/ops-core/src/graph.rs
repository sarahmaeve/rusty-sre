use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug)]
pub struct Component {
    pub name: String,
    pub parent: RefCell<Option<Rc<Component>>>,
    pub children: RefCell<Vec<Rc<Component>>>,
}

impl Component {
    pub fn new(name: impl Into<String>) -> Rc<Self> {
        Rc::new(Self {
            name: name.into(),
            parent: RefCell::new(None),
            children: RefCell::new(Vec::new()),
        })
    }

    pub fn attach(parent: &Rc<Self>, child: &Rc<Self>) {
        *child.parent.borrow_mut() = Some(Rc::clone(parent));
        parent.children.borrow_mut().push(Rc::clone(child));
    }
}
