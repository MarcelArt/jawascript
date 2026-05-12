
use inkwell::context::Context;

use crate::{codegen::CodeGen, lexer::Lexer, parser::Parser};

mod ast;
mod lexer;
mod parser;
mod codegen;

fn main() {
    let source = "
        let x = 10;

        if x > 5 {
            x + 1
        }
    ";

    let lexer = Lexer::new(source);

    let mut parser = Parser::new(lexer);

    let program = parser.parse_program();

    println!("{:#?}", program);

    let context = Context::create();

    let mut codegen = CodeGen::new(&context);

    codegen.compile_program(&program);

    codegen.print_ir();
}

// fn main() {
//     let source = "1 + 2 * 3";

//     let lex = lexer::Lexer::new(source);

//     let mut parser = Parser::new(lex);

//     let ast = parser.parse();

//     println!("{:#?}", ast);

//     let context = Context::create();

//     let mut codegen = CodeGen::new(&context);

//     codegen.compile(&ast);

//     codegen.print_ir();

//     // let tokens = lex.tokenize();

//     // for token in tokens {
//     //     println!("{:?}", token);
//     // }
// }

// fn main() {
//     let context = Context::create();

//     let module = context.create_module("jawascript");

//     let builder = context.create_builder();

//     let i32_type = context.i32_type();

//     let fn_type = i32_type.fn_type(&[], false);

//     let function = module.add_function("main", fn_type, None);

//     let basic_block = context.append_basic_block(function, "entry");

//     builder.position_at_end(basic_block);

//     let value = i32_type.const_int(0, false);

//     builder.build_return(Some(&value)).unwrap();

//     module.print_to_stderr();
//     module.print_to_file("output.ll").unwrap();
// }
