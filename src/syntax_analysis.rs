use crate::lexer::Token;
use std::{ cell::RefCell, rc::Rc };

pub struct SyntaxTreeNode {
    val: Token,
    left: Option<TreeNodeRef>,
    right: Option<TreeNodeRef>,
}

type TreeNodeRef = Rc<RefCell<SyntaxTreeNode>>;

impl SyntaxTreeNode {
    fn new(val: Token) -> Self {
        return SyntaxTreeNode {
            val,
            left: None,
            right: None,
        };
    }
}

pub fn analyse_syntax(tokens: Vec<Token>) -> () {
    let mut root = SyntaxTreeNode::new(Token::value(String::from("1")));
    let left = SyntaxTreeNode::new(Token::value(String::from("2")));
    let right = SyntaxTreeNode::new(Token::value(String::from("3")));

    root.left = Some(Rc::new(RefCell::new(left)));
    root.right = Some(Rc::new(RefCell::new(right)));
}
