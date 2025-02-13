
use std::collections::HashSet;
use std::fmt::Debug;

use crate::interpreter;
use crate::compiler;


pub trait Expression: Debug {
    fn is_mono(&self) -> bool;
    fn used_vars(&self) -> &HashSet<isize>;
    fn compile(&self) -> Vec<interpreter::Instruction>;
}

pub type ExpressionNode = Box<dyn Expression>;

#[derive(Clone, Debug)]
pub struct FractionNode {
    pub const_idx: usize,
    pub used_vars: HashSet<isize>
}

#[derive(Clone, Debug)]
pub struct StringNode {
    pub const_idx: usize,
    pub used_vars: HashSet<isize>
}

#[derive(Debug)]
pub struct ArrayLiteralNode {
    pub items: Vec<ExpressionNode>,
    pub is_mono: bool,
    pub used_vars: HashSet<isize>
}

#[derive(Debug)]
pub struct ArrayRepeatNode {
    pub item: ExpressionNode,
    pub dimensions: ExpressionNode,
    pub is_mono: bool,
    pub used_vars: HashSet<isize>
}

#[derive(Debug)]
pub struct LookupNode {
    pub register: usize,
    pub is_global: bool,
    pub indices: Vec<ExpressionNode>,
    pub is_mono: bool,
    pub var_is_mono: bool,
    pub var_id: isize,
    pub used_vars: HashSet<isize>,
    pub index_used_vars: HashSet<isize>
}

#[derive(Debug)]
pub struct BinopNode {  
    pub lhs: ExpressionNode,
    pub rhs: ExpressionNode,
    pub op: interpreter::Instruction,
    pub is_mono: bool,
    pub used_vars: HashSet<isize>
}

#[derive(Debug)]
pub struct UniopNode {
    pub expr: ExpressionNode,
    pub op: interpreter::Instruction,
    pub is_mono: bool,
    pub used_vars: HashSet<isize>
}



pub trait Statement: Debug {
    fn is_mono(&self) -> bool;
    fn compile(&self) -> compiler::Code;
}

pub type StatementNode = Box<dyn Statement>;

#[derive(Debug)]
pub struct PrintNode {
    pub items: Vec<ExpressionNode>,
    pub newline: bool,
    pub is_mono: bool,
}

#[derive(Debug)]
pub struct LetUnletNode {
    pub is_unlet: bool,
    pub register: usize,
    pub rhs: ExpressionNode,
    pub is_mono: bool
}

#[derive(Debug)]
pub struct RefUnrefNode {
    pub is_unref: bool,
    pub register: usize,
    pub rhs: LookupNode,
    pub is_mono: bool
}

#[derive(Debug)]
pub struct ModopNode {
    pub lookup: LookupNode,
    pub op: interpreter::Instruction,
    pub rhs: ExpressionNode,
    pub is_mono: bool
}

#[derive(Debug)]
pub struct PushPullNode {
    pub is_push: bool,
    pub register: usize,
    pub lookup: LookupNode,
    pub is_mono: bool,
}

#[derive(Debug)]
pub struct IfNode {
    pub fwd_expr: ExpressionNode,
    pub if_stmts: Vec<StatementNode>,
    pub else_stmts: Vec<StatementNode>,
    pub bkwd_expr: ExpressionNode,
    pub is_mono: bool
}

#[derive(Debug)]
pub struct WhileNode {
    pub fwd_expr: ExpressionNode,
    pub stmts: Vec<StatementNode>,
    pub bkwd_expr: Option<ExpressionNode>,
    pub is_mono: bool
}

#[derive(Debug)]
pub struct ForNode {
    pub register: usize,
    pub iterator: LookupNode,
    pub stmts: Vec<StatementNode>,
    pub is_mono: bool
}

#[derive(Debug)]
pub struct DoYieldNode {
    pub do_stmts: Vec<StatementNode>,
    pub yield_stmts: Vec<StatementNode>
}

#[derive(Debug)]
pub struct CatchNode {
    pub expr: ExpressionNode
}

#[derive(Debug)]
pub struct CallNode {
    pub is_uncall: bool,
    pub func_idx: usize,
    pub borrow_args: Vec<LookupNode>,
    pub stolen_args: Vec<usize>,
    pub return_args: Vec<usize>,
    pub is_mono: bool
}


#[derive(Clone, Debug)]
pub struct ParamLink {
    pub is_interior: bool,
    pub link: Option<String>,  // None if unbound link //
    pub linked_borrow: Option<usize>,
    pub linked_io: Option<usize>,
}

#[derive(Clone, Debug, Default)]
pub struct FunctionPrototype {
    pub id: usize,
    pub owned_link_groups: Vec<[Vec<usize>; 3]>,
    pub borrow_params: Vec<Option<ParamLink>>,
    pub steal_params: Vec<Option<ParamLink>>,
    pub return_params: Vec<Option<ParamLink>>
}

#[derive(Debug)]
pub struct FunctionNode {
    pub stmts: Vec<StatementNode>,
    pub consts: Vec<interpreter::Variable>,
    pub num_registers: usize,

    pub borrow_registers: Vec<usize>,
    pub steal_registers: Vec<usize>,
    pub return_registers: Vec<usize>,
}

#[derive(Debug)]
pub struct Module {
    pub functions: Vec<FunctionNode>,
    pub main_idx: Option<usize>,
    pub global_func: FunctionNode
}