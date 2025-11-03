use std::rc::Rc;

struct Owner{
  name:String
}

struct LinkedListNode {
  value:i32,
  next:Option<Box<LinkedListNode>>
}

