use core::fmt;

use dyn_clone::DynClone;

pub trait AstVisitor {
    fn visit_program(&mut self, program: &Program);
    fn visit_type(&mut self, type_value: &Type);
    fn visit_parameter_declaration(&mut self, parameter: &ParameterDeclaration);
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

#[derive(Clone, Debug)]
pub enum Type {
    I8,
    I16,
    I32,
    I64,
    Iptr,
    U8,
    U16,
    U32,
    U64,
    Uptr,
    F32,
    F64,
    Bool,
    Char,
    String,
}

impl_ast_node!(Type, visit_type);

#[derive(Clone, Debug)]
pub struct ParameterDeclaration {
    name: String,
    parameter_type: Box<dyn AstNode>,
}

impl ParameterDeclaration {
    pub fn new(name: String, parameter_type: Box<dyn AstNode>) -> Self {
        Self {
            name,
            parameter_type,
        }
    }
}

impl_ast_node!(ParameterDeclaration, visit_parameter_declaration);
