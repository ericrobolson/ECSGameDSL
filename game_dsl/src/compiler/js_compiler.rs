use super::{Artifact, Compiler, OutputBuilder};
use crate::compiler_ir::*;

pub struct JSCompiler;
impl Compiler for JSCompiler {
    fn compile_artifacts(
        &self,
        structs: Vec<Commentable<Struct>>,
        expressions: Vec<Commentable<Expression>>,
    ) -> Vec<Artifact> {
        let mut output = OutputBuilder::new("JavaScript", "\t", "//");

        // Build out expressions
        output.add_section("Expressions");
        for e in expressions.iter() {
            output.add_comments(&e.comments);

            let e = &e.value;

            let id = &e.id;
            let args = e
                .args
                .iter()
                .map(|arg| format!("{}", arg.id))
                .collect::<Vec<_>>()
                .join(", ");
            output.push_line(&format!("function {id}({args}) {{", id = id, args = args));
            output.indent();
            // Body
            {
                for expression in e.body.iter() {
                    if let ExpressionStatement::Comment(comment) = expression {
                        output.add_comment(comment);
                    } else {
                        output.add_indentation();
                        eval_expression(expression, &mut output);
                        output.push(";");
                        output.add_line();
                    }
                }
            }
            output.unindent();
            output.push_line("}");
            output.add_line();
        }

        let text = output.build();
        todo!()
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
