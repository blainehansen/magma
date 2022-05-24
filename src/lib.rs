use anyhow::Result;
use inkwell::{builder::Builder, context::Context, types::IntType, values::IntValue};
use nom::{
    branch::alt, bytes::complete::tag, character::complete, combinator::map,
    sequence::separated_pair, Finish, IResult,
};
use std::{fs::read, str::from_utf8};

// #[derive(ocmal::IntoValue, ocaml::FromValue)]
pub enum AST {
    Add(Box<AST>, Box<AST>),
    Number(i32),
}

// #[ocaml::func]
pub fn parse(filename: &str) -> Result<AST> {
    parse_bytes(from_utf8(read(filename)?.as_slice())?)
}

fn parse_bytes<'a>(i: &'a str) -> Result<AST> {
    Ok(ast(i).map_err(|err| err.to_owned()).finish().map(|x| x.1)?)
}

fn number(i: &str) -> IResult<&str, AST> {
    map(complete::i32, AST::Number)(i)
}

fn add(i: &str) -> IResult<&str, AST> {
    map(separated_pair(ast, tag("+"), ast), |(a, b)| {
        AST::Add(Box::new(a), Box::new(b))
    })(i)
}

fn ast(i: &str) -> IResult<&str, AST> {
    alt((number, add))(i)
}

// #[ocaml::func]
pub fn render(ast: &AST, to: &str) {
	let context = Context::create();
	let module = context.create_module("lab");
	let builder = context.create_builder();

	let i32_type = context.i32_type();
	let fn_type = i32_type.fn_type(&[], false);
	let function = module.add_function("main", fn_type, None);
	let basic_block = context.append_basic_block(function, "doit");

	builder.position_at_end(basic_block);

    fn build<'ctx>(env: (&Builder<'ctx>, IntType<'ctx>), ast: &AST) -> IntValue<'ctx> {
        match ast {
            AST::Number(x) => env.1.const_int(*x as u64, false),
            AST::Add(a, b) => env.0.build_int_add(build(env, a), build(env, b), "sum"),
        }
    }

    builder.build_return(Some(&build((&builder, i32_type), ast)));
	module.write_bitcode_to_path(&std::path::Path::new(to));
}
