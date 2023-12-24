mod compiler;
/// Intermediate represation of all primitives for a compiler.
mod compiler_ir;
mod env;
mod error;
mod ir_builder;
mod lexer;
mod location;
mod parser;
mod unchecked_env;

use std::path::Path;

use location::Location;

fn main() {
    let input = r#"
    # Structs can be empty with a semicolon.
    struct Empty;
    
    # They can wrap primitives.
    struct Dollar(i32);
    
    # Lists can be wrapped.
    struct Title([char 64]);
    
    # Structs can no properties.
    struct Empty2 {}
    
    # Structs can have any number of properties
    struct Aabb {
        i32 width
        i32 height
    }
    
    # Structs can have arrays. All arrays are statically allocated.
    struct Name {
        [char 10] name
    }
    
    # Structs can be properties.
    struct Person {
        i32 age
        Name name
    }    

    # This is an example of a 'tag' component. It has no data.
    component IsAlive;
    
    # This is an example of a 'tag' single component. It has no data.
    single_component GameOver;
    
    # This is an example of a 'value' component. It only has one field.
    component Hp(i32);
    
    # This is an example of a 'value' component with arrays.
    # Arrays are statically allocated.
    component Collisions([Entity 256]);
    
    # Tag components can also be empty structs.
    component IsDead {}
    
    # This is an example of a struct component.
    component Position {
        i32 x
        i32 y
    }
    
    # Components can contain structs
    component Person {
        Name name
        i32 age
    }
    
    # This is an example of a struct component.
    single_component WorldState {
        i32 frame
        i32 deltaT
    }
    
    # This is an example of a struct component. Prop values can also be arrays.
    component HitBoxes {
        i32 x
        i32 y
        [Aabb 256] boxes
    }
    
    "#;

    let ast = parser::parse(input, Location::default()).unwrap();
    let unchecked_env = unchecked_env::build(ast);
    let env = match env::build(unchecked_env) {
        Ok(env) => env,
        Err(errors) => {
            for error in errors {
                println!("{}", error.message);
            }
            return;
        }
    };
    let compiler_ir = ir_builder::build(env).unwrap();
    let artifacts = compiler::build(compiler_ir);

    // Save to file and compile
    std::fs::remove_dir_all(Path::new("../_generated/")).unwrap();
    for artifact in artifacts.iter() {
        println!("Compiling {:?}...", artifact.target);
        let mut path = std::path::PathBuf::from("../_generated/");
        path.push(&format!("{:?}", artifact.target));

        let src_dir = path.clone();
        std::fs::create_dir_all(&path).unwrap();

        path.push(&artifact.path);
        std::fs::write(&path, &artifact.contents).unwrap();

        let cmd = match artifact.target {
            compiler::Target::C => "gcc",
            _ => todo!(),
        };

        let mut cmd = std::process::Command::new(cmd);
        cmd.arg(&path);
        cmd.arg("-o");
        cmd.arg(format!("{}/a.out", src_dir.to_str().unwrap().to_string()));

        let output = cmd.output().unwrap();
        println!("{}", String::from_utf8_lossy(&output.stdout));
        println!("{}", String::from_utf8_lossy(&output.stderr));
        let mut exe_path = src_dir.clone();
        exe_path.push("./a.out");
        let mut cmd = std::process::Command::new(exe_path);
        let output = cmd.output().unwrap();
        println!("{}", String::from_utf8_lossy(&output.stdout));
        println!("{}", String::from_utf8_lossy(&output.stderr));
        if output.status.success() {
            println!("Success!");
        } else {
            panic!("Failure!");
        }
    }
}
