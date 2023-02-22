use core::fmt;

use dyn_clone::DynClone;

pub trait AstVisitor {
    fn visit_list(&mut self, list: &[Box<dyn AstNode>]);
    fn visit_type(&mut self, type_value: &Type);
    fn visit_parameter_declaration(&mut self, parameter: &ParameterDeclaration);
    fn visit_function_definition(&mut self, function: &FunctionDefinition);
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

impl_ast_node!(Vec<Box<dyn AstNode>>, visit_list);

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

#[derive(Clone, Debug)]
pub struct FunctionDefinition {
    name: String,
    parameters: Vec<Box<dyn AstNode>>,
    return_type: Box<dyn AstNode>,
    body: Box<dyn AstNode>,
}

impl FunctionDefinition {
    pub fn new(
        name: String,
        parameters: Vec<Box<dyn AstNode>>,
        return_type: Box<dyn AstNode>,
        body: Box<dyn AstNode>,
    ) -> Self {
        Self {
            name,
            parameters,
            return_type,
            body,
        }
    }
}

impl_ast_node!(FunctionDefinition, visit_function_definition);
