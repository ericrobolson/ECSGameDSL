ECS based compiler/interpreter.
Transpiles to C/JavaScript/C#/CPP.
MAYBE: Interpreter written in Rust to enable quick dev testing.

# Core Features

- ECS based design
- Compiles to primary gamedev languages
- Statically allocated at startup. No dynamic memory allocation during runtime.

# Roadmap
Approach roadmap in 'vertical' slices.
This way you can get generated code as well as a working compiler for each slice.

- [x] Join blocks of comments into one token
- [x] Add parsing of components
- [x] Add parsing of structs
- [x] Add checking of components + tests
- [x] Add in checking of structs + tests
- [x] Add in tests for component checks
- [x] Add in tests for struct checks
- [x] Add in default type for entities
- [x] Add in IR generation of structs
- [x] Add IR generation of components. 
- [x] Add in compilation for C
- [ ] Add in IR generation of component stores. Look into the BitSquid data driven ECS approach for building component stores.
- [ ] Add in some form of constant to know max values in lists/components
- [ ] Add in compilation for JS
- [ ] Add in compilation for C#
- [ ] Add in compilation for CPP
- [ ] Add in compilation of structs/componenets
- [ ] Add in loading of files
- [ ] Add sorting of errors by files in the env module
- [ ] Add in parsing of systems
- [ ] Add in checking of systems
- [ ] Add in generation of systems
- [ ] Add in parsing of world
- [ ] Add in checking of world
- [ ] Add in generation of world
- [ ] Add in parsing of entities
- [ ] Add in checking of entities
- [ ] Add in generation of entities
- [ ] Add in parsing of expressions
- [ ] Add in checking of expressions
- [ ] Add in generation of expressions
- [ ] Add in native expressions (like for entities). Also add in native expressions for component stores, components, and structs
- [ ] Add in comments to code generation and the like. Need to add to Structs, Components, Systems, World, Expr, etc.


# FIN

- [x] determine how you want your language to look. E.g. pipelines? Guards for pipelines? Global world state?

Lexer

- [x] Add in LParen
- [x] Add in RParen
- [x] Add in LSquareBracket
- [x] Add in RSquareBracket
- [x] Add in Period
- [x] Add in =
- [x] Add in !,
- [x] Add in comparison symbols (>, <)
- [x] Add in math symbols (mult, mod, sub, etc.)
- [x] Add in logical symbols (!, |, &)
- [x] Add in/test assign math symbols; maybe 2 tokens is fine and handle in parser? (\*=, %=, -=, /= etc.)
- [x] Add in/test double comparison symbols; maybe 2 tokens is fine and handle in parser? (==, !=, >=, <=, etc.)

Groundwork:

- [x] Add in algebraic data types
- [x] Work on recursive parser generator
- [x] Fix tests
- [x] Replace panic! / todo! with proper error handling
- [x] Add in locations to errors. Replace all Result<_, String> with Result<_, Error>
- [x] Add locations to AST
- [x] Resolve all TODOs
- [x] Build out simple full pass compiler for just text
- [x] Add in comments for original text
- [x] Add parsing of structs

Generation

- [x] Add in a 'output builder' class that handles indentation and the like
- [x] Add in generation of expressions
- [x] Add generation of structs
- [x] Add in a 'checker' module that checks the AST for errors
- [x] Add in compiler for C
- [x] Add in compiler for JS
- [x] Add in compiler for C#
- [x] Add in compiler for CPP
- [x] Add in comments for expressions
- [x] Add in comments for structs
- [x] Add in comments for expression statements
- [x] Add in comments for struct fields
- [x] Adds in comments to generated code
