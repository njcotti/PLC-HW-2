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
        let mut female_tree = Tree::new();
        female_tree.insert(1, 3, 2);
        female_tree.insert(2,2,3);
        female_tree.insert(3,1,4);
        female_tree.insert(4,2,3);

        let mut male_tree = Tree::new();
        male_tree.insert(1, 3, 2);
        male_tree.insert(2,2,3);
        male_tree.insert(3,1,4);
        male_tree.insert(4,2,3);


        assert_eq!(female_tree.root.is_some(),true);
        println!("{:?}", female_tree);

        assert_eq!(male_tree.root.is_some(),true);
        println!("{:?}", male_tree);
    }
}

fn main() {

}