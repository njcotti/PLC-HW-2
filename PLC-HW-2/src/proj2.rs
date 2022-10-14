#[derive(Debug)]
struct Tree{
    root: Option<Box<Node>>, //Box is a pointer and defines a size for node and allows for it to compile
}


#[derive(Debug)]
struct Node{
    id: i32,
    age: i32,
    size: i32,
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,

}

impl Node {
    fn new(id: i32, age:i32, size: i32) -> Self  {
        Node{
            id,
            age,
            size,
            left: None,
            right: None,
        }
    }
}

impl From<Node> for Option<Box<Node>>{
    fn from(node: Node) -> Self {
        Some(Box::new(node))
    }
}

impl Tree {
    fn new() -> Self {
        Tree {
            root: None
        }
    }
    fn insert(&mut self, id: i32, age: i32, size: i32){
        match &mut self.root {
            None => {
                self.root = Node::new(id, age, size).into();
            },
            Some(node) => {
                Tree::insert_recursive(node,id, age, size);
            }
        }
    }
    fn insert_recursive(node: &mut Box<Node>, id: i32, age: i32, size:i32){
        if age > node.age {
            match &mut node.right {
                None => {
                    node.right = Node::new(id, age, size).into();
                },
                Some(n) => {
                    Tree::insert_recursive(n, id, age, size);
                }
            }
        } else if age < node.age {
            match &mut node.left {
                None => {
                    node.left = Node::new(id, age, size).into();
                },
                Some(n) => {
                    Tree::insert_recursive(n, id, age , size);
                }
            }
        } else if age == node.age {
            if size > node.size{
                match &mut node.right {
                    None => {
                        node.right = Node::new(id, age, size).into();
                    },
                    Some(n) => {
                        Tree::insert_recursive(n, id, age, size);
                    }
                }
            } else if size < node.size{
                match &mut node.left {
                    None => {
                        node.left = Node::new(id, age, size).into();
                    },
                    Some(n) => {
                        Tree::insert_recursive(n, id, age, size);
                    }
                }
            } else if size == node.size {
                if id > node.id {
                    match &mut node.right {
                        None => {
                            node.right = Node::new(id, age, size).into();
                        },
                        Some(n) => {
                            Tree::insert_recursive(n, id, age, size);
                        }
                    }
                } else if id < node.id {
                    match &mut node.left {
                        None => {
                            node.left = Node::new(id, age, size).into();
                        },
                        Some(n) => {
                            Tree::insert_recursive(n, id, age, size);
                        }
                    }

                    }
                }
            }
        }
    }


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn works_build_tree() {
        let mut tree = Tree::new();
        tree.insert(8);
        tree.insert(10);
        tree.insert(3);
        tree.insert(1);
        tree.insert(6);
        tree.insert(4);

        assert_eq!(tree.root.is_some(),true);
        println!("{:?}", tree);
    }
}

fn main() {

}