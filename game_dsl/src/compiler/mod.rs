mod c_compiler;
// mod cpp_compiler;
// mod csharp_compiler;
// mod js_compiler;
mod output_builder;

use std::path::PathBuf;

use crate::compiler_ir::*;
pub use c_compiler::CCompiler;
// pub use cpp_compiler::CppCompiler;
// pub use csharp_compiler::CSharpCompiler;
// pub use js_compiler::JSCompiler;
pub use output_builder::OutputBuilder;

#[derive(Debug, Clone, PartialEq)]
pub struct Artifact {
    pub contents: String,
    pub path: PathBuf,
    pub target: Target,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Target {
    C,
    CPP,
    JS,
    CSharp,
}

pub fn build(ir: Vec<IR>) -> Vec<Artifact> {
    let mut artifacts = vec![];

    let compilers: Vec<Box<dyn Compiler>> = vec![
        Box::new(CCompiler),
        // Box::new(CppCompiler),
        // Box::new(CSharpCompiler),
        // Box::new(JSCompiler),
    ];

    for compiler in compilers {
        let mut contents = compiler.compile(ir.clone());
        artifacts.append(&mut contents);
    }

    artifacts
}

pub trait Compiler {
    /// Compiles the given IR into the target language.
    fn compile(&self, ir: Vec<IR>) -> Vec<Artifact> {
        // Break out IR into manageable chunks and sort them for consistency.

        let mut structs = Vec::new();
        let mut expressions = Vec::new();

        for ir in ir {
            match ir {
                IR::Struct(s) => structs.push(s),
                IR::Expression(e) => expressions.push(e),
            }
        }

        structs.sort_by(|a, b| a.value.id.cmp(&b.value.id));
        expressions.sort_by(|a, b| a.value.id.cmp(&b.value.id));

        self.compile_artifacts(structs, expressions)
    }

    /// Compiles the given artifacts into the target langauge.
    fn compile_artifacts(
        &self,
        structs: Vec<Commentable<Struct>>,
        expressions: Vec<Commentable<Expression>>,
    ) -> Vec<Artifact>;
}
