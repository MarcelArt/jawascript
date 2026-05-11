use inkwell::{builder::Builder, context::Context, module::Module, values::IntValue};

use crate::ast::{BinaryOp, Expr};

pub struct CodeGen<'ctx> {
    context: &'ctx Context,
    builder: Builder<'ctx>,
    module: Module<'ctx>,
}

impl<'ctx> CodeGen<'ctx> {
    pub fn new(context: &'ctx Context) -> Self {
        let module = context.create_module("main");

        let builder = context.create_builder();

        Self {
            context,
            builder,
            module,
        }
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
                }
            }
        }
    }

    pub fn print_ir(&self) {
        self.module.print_to_stderr();
    }
}