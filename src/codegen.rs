use std::collections::HashMap;

use inkwell::{IntPredicate, builder::Builder, context::Context, module::Module, values::{IntValue, PointerValue}};

use crate::ast::{BinaryOp, Expr, Statement};

pub struct CodeGen<'ctx> {
    context: &'ctx Context,
    builder: Builder<'ctx>,
    module: Module<'ctx>,

    variables: HashMap<String, PointerValue<'ctx>>,
}

#[allow(unused)]
impl<'ctx> CodeGen<'ctx> {
    pub fn new(context: &'ctx Context) -> Self {
        let module = context.create_module("main");

        let builder = context.create_builder();

        Self {
            context,
            builder,
            module,
            variables: HashMap::new(),
        }
    }

    pub fn compile_program(&mut self, statements: &[Statement]) {
        let i64_type = self.context.i64_type();

        let fn_type = i64_type.fn_type(&[], false);

        let function = self.module.add_function("main", fn_type, None);

        let block = self.context.append_basic_block(function, "entry");

        self.builder.position_at_end(block);

        let mut last_value = i64_type.const_int(0, false);

        for stmt in statements {
            last_value = self.compile_statement(stmt);
        }

        self.builder.build_return(Some(&last_value)).unwrap();
    }

    pub fn compile(&mut self, expr: &Expr) {
        let i64_type = self.context.i64_type();

        let fn_type = i64_type.fn_type(&[], false);

        let function = self.module.add_function("main", fn_type, None);

        let block = self.context.append_basic_block(function, "entry");

        self.builder.position_at_end(block);

        let result = self.compile_expr(expr);

        self.builder.build_return(Some(&result)).unwrap();
    }

    fn compile_expr(&self, expr: &Expr) -> IntValue<'ctx> {
        match expr {
            Expr::Number(value) => {
                self.context.i64_type().const_int(*value as u64, false)
            },
            Expr::Binary { left, op, right } => {
                let lhs = self.compile_expr(left);

                let rhs = self.compile_expr(right);

                match op {
                    BinaryOp::Add => {
                        self.builder.build_int_add(lhs, rhs, "addtmp").unwrap()
                    },
                    BinaryOp::Sub => {
                        self.builder.build_int_sub(lhs, rhs, "subtmp").unwrap() 
                    },
                    BinaryOp::Mul => {
                        self.builder.build_int_mul(lhs, rhs, "multmp").unwrap()
                    },
                    BinaryOp::Div => {
                        self.builder.build_int_signed_div(lhs, rhs, "divtmp").unwrap()
                    },
                    BinaryOp::Greater => {
                        let result = self.builder.build_int_compare(IntPredicate::SGT, lhs, rhs, "cmptmp").unwrap();
                        self.builder.build_int_z_extend(result, self.context.i64_type(), "booltmp").unwrap()
                    }
                    BinaryOp::Less => {
                        let result = self.builder.build_int_compare(IntPredicate::SLT, lhs, rhs, "cmptmp").unwrap();
                        self.builder.build_int_z_extend(result, self.context.i64_type(), "booltmp").unwrap()
                    }
                    _ => todo!(),
                }
            },
            Expr::Variable(name) => {
                let ptr = self.variables
                    .get(name)
                    .expect("Undefined variable");

                self.builder
                    .build_load(self.context.i64_type(), *ptr, name)
                    .unwrap()
                    .into_int_value()
            }
            _ => panic!("Unexpected expression"),
        }
    }

    fn compile_statement(&mut self, stmt: &Statement) -> IntValue<'ctx> {
        match stmt {
            Statement::Let { name, value } => {
                let value = self.compile_expr(value);

                let ptr = self.builder.build_alloca(self.context.i64_type(), name).unwrap();

                self.builder.build_store(ptr, value).unwrap();

                self.variables.insert(name.clone(), ptr);

                value
            },
            Statement::Expr(expr) => {
                self.compile_expr(expr)
            },
            Statement::If { condition, then_branch } => {
                let parent = self.builder
                    .get_insert_block()
                    .unwrap()
                    .get_parent()
                    .unwrap();

                let then_block = self.context.append_basic_block(parent, "then");

                let merge_block = self.context.append_basic_block(parent, "merge");

                let condition_value = self.compile_expr(condition);

                let zero = self.context.i64_type().const_int(0, false);

                let condition_bool = self.builder.build_int_compare(IntPredicate::NE, condition_value, zero, "ifcond").unwrap();

                self.builder.build_conditional_branch(condition_bool, then_block, merge_block).unwrap();

                self.builder.position_at_end(then_block);

                let mut last = self.context.i64_type().const_int(0, false);

                for stmt in then_branch {
                    last = self.compile_statement(stmt);
                }

                self.builder.build_unconditional_branch(merge_block).unwrap();

                self.builder.position_at_end(merge_block);

                last
            }
            _ => todo!(),
        }
    }

    pub fn print_ir(&self) {
        self.module.print_to_stderr();
    }
}