use core::fmt;

use dyn_clone::DynClone;

pub trait AstVisitor {
    fn visit_program(&mut self, program: &Program);
}

pub trait AstNode: DynClone + fmt::Debug {
    fn apply(&self, visitor: &mut dyn AstVisitor);
}

impl Clone for Box<dyn AstNode> {
    fn clone(&self) -> Self {
        dyn_clone::clone_box(&**self)
    }
}

macro_rules! impl_ast_node {
    ($type:ty, $visit_method:ident) => {
        impl AstNode for $type {
            fn apply(&self, visitor: &mut dyn AstVisitor) {
                visitor.$visit_method(self);
            }
        }
    };
}

#[derive(Clone, Debug)]
pub struct Program {
    children: Vec<Box<dyn AstNode>>,
}

impl Program {
    pub fn new(children: Vec<Box<dyn AstNode>>) -> Self {
        Self { children }
    }
}

impl_ast_node!(Program, visit_program);
