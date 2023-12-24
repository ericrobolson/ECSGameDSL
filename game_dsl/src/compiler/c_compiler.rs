use super::{Artifact, Compiler, OutputBuilder};
use crate::{
    compiler::Target,
    compiler_ir::*,
    parser::{ListType, Listable, Primitive, PrimitiveType},
};

pub struct CCompiler;

impl Compiler for CCompiler {
    fn compile_artifacts(
        &self,
        structs: Vec<Commentable<Struct>>,
        expressions: Vec<Commentable<Expression>>,
    ) -> Vec<Artifact> {
        let mut output = OutputBuilder::new("C", "\t", "//");

        // Build out includes
        output.add_section("Includes");

        output.push_line("#include <stdint.h>");
        output.push_line("#include <stdbool.h>");
        output.push_line("#include <stdio.h>");

        output.add_line();

        // Build out forward declarations for structs
        output.add_section("Forward declarations");

        for s in structs.iter() {
            output.add_comments(&s.comments);
            let s = &s.value;
            output.push_line(&format!("struct {};", s.id));
        }

        output.add_line();

        // // Build out forward declarations for expressions
        // for e in expressions.iter() {
        //     output.add_comments(&e.comments);

        //     let e = &e.value;
        //     let ret_type = primitive_type(&e.return_type);
        //     let id = &e.id;
        //     let args = e
        //         .args
        //         .iter()
        //         .map(|arg| format!("{} {}", primitive_type(&arg.ty), arg.id))
        //         .collect::<Vec<_>>()
        //         .join(", ");
        //     output.push_line(&format!("{ret_type} {id}({args});"));
        //     output.add_line();
        // }

        output.add_line();

        // Build out structs
        output.add_section("Structs");

        for s in structs.iter() {
            output.add_comments(&s.comments);

            let s = &s.value;

            output.push_line(&format!("typedef struct {} {{", s.id));
            output.indent();
            for field in s.fields.iter() {
                let mut ptr_value = String::default();
                let array_type = {
                    match &field.ty {
                        ListType::List { ty, max_size } => {
                            if !ty.is_identifier() {
                                ptr_value = "*".to_string();
                            }
                            format!("[{}]", max_size)
                        }
                        ListType::Single(_) => "".to_string(),
                    }
                };

                output.push_line(&format!(
                    "{ty}{ptr_value} {id}{array_type};",
                    ty = primitive_type(&field.ty),
                    ptr_value = ptr_value,
                    id = field.id,
                    array_type = array_type
                ));
            }
            output.unindent();
            output.push_line(&format!("}} {};", s.id));
            output.add_line();
        }

        output.add_line();

        // // Build out expressions
        // output.add_section("Expressions");
        // for e in expressions.iter() {
        //     output.add_comments(&e.comments);
        //     let e = &e.value;

        //     let ret_type = primitive_type(&e.return_type);
        //     let id = &e.id;
        //     let args = e
        //         .args
        //         .iter()
        //         .map(|arg| format!("{} {}", primitive_type(&arg.ty), arg.id))
        //         .collect::<Vec<_>>()
        //         .join(", ");
        //     output.push_line(&format!(
        //         "{ret_type} {id}({args}) {{",
        //         ret_type = ret_type,
        //         id = id,
        //         args = args
        //     ));
        //     output.indent();
        //     // Body
        //     {
        //         for expression in e.body.iter() {
        //             if let ExpressionStatement::Comment(comment) = expression {
        //                 output.add_comment(comment);
        //             } else {
        //                 output.add_indentation();
        //                 eval_expression(expression, &mut output);
        //                 output.push(";");
        //                 output.add_line();
        //             }
        //         }
        //     }
        //     output.unindent();
        //     output.push_line("}");
        //     output.add_line();
        // }

        // TODO: remove
        {
            // Add simple 'main' for compilation purposes
            output.push_line("int main() { printf(\"Hello, world!\"); }");
        }

        let text = output.build();

        vec![Artifact {
            target: Target::C,
            path: "main.c".into(),
            contents: text,
        }]
    }
}

fn eval_expression(expression: &ExpressionStatement, output: &mut OutputBuilder) {
    match expression {
        ExpressionStatement::Literal(literal) => match literal {
            Literal::Int(i) => output.push(&format!("{}", i)),
            Literal::Bool(b) => output.push(&format!("{}", b)),
            Literal::String(s) => output.push(&format!("\"{}\";", s)),
            Literal::Identifier(id) => output.push(&format!("{}", id)),
        },
        ExpressionStatement::Return(expression) => {
            output.push("return ");
            eval_expression(expression, output);
        }
        ExpressionStatement::Call { id, args } => {
            output.push(&format!("{}(", id));
            for (i, arg) in args.iter().enumerate() {
                eval_expression(arg, output);
                if i != args.len() - 1 {
                    output.push(", ");
                }
            }
            output.push(")");
        }
        ExpressionStatement::NativeExpression(native) => {
            //
            match native {
                NativeExpression::Add { lhs, rhs } => {
                    output.push("(");
                    eval_expression(lhs, output);
                    output.push(" + ");
                    eval_expression(rhs, output);
                    output.push(")");
                }
                NativeExpression::Multiply { lhs, rhs } => {
                    output.push("(");
                    eval_expression(lhs, output);
                    output.push(" * ");
                    eval_expression(rhs, output);
                    output.push(")");
                }
            }
        }
        ExpressionStatement::Comment(_) => {
            // Do nothing
        }
    }
}

fn base_primitive(primitive_ty: &PrimitiveType) -> String {
    match primitive_ty {
        PrimitiveType::U32 => "uint32_t".to_string(),
        PrimitiveType::U64 => "uint64_t".to_string(),
        PrimitiveType::I32 => "int32_t".to_string(),
        PrimitiveType::I64 => "int64_t".to_string(),
        PrimitiveType::F32 => todo!(),
        PrimitiveType::F64 => todo!(),
        PrimitiveType::Bool => "bool".to_string(),
        PrimitiveType::Char => "char".to_string(),
        PrimitiveType::Identifier(s) => format!("struct {}*", s),
    }
}

fn primitive_type(primitive: &ListType<Primitive>) -> String {
    match primitive {
        ListType::List { ty, max_size } => base_primitive(&ty.primitive_type),
        ListType::Single(ty) => base_primitive(&ty.primitive_type),
    }
}
