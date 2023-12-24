# Intro

This is a data oriented language primarily targeted for soft real time systems. The goals are no allocations past initialization and no garbage collection. The language is statically typed and compiled to C, C#, JavaScript or C++ with the intent of being embedded as a library. An ECS (entity/component/system) architecture is used to achieve these goals.

Due to the desire for no runtime memory allocation past initialization, some oddities are present. There are some seemingly strange design decisions such as no strings. Only arrays of characters are allowed, for example. This is to ensure that the language can be embedded in other languages without having to worry about memory allocation.

As this is the initial version, many features found in other languages aren't present. These will be added in a backwards compatible fashion, meaning code written in this language should always be able to compile in the future. With that said, the host language is not guaranteed to be backwards compatible. Effort will be taken to ensure that the host language implementation is backwards compatible, but it is not guaranteed.

# Primitives
- `u32` - 32 bit unsigned integer.
- `u64` - 64 bit unsigned integer.
- `i32` - 32 bit signed integer.
- `i64` - 64 bit signed integer.
- `f32` - 32 bit floating point.
- `f64` - 64 bit floating point.
- `bool` - boolean. Can be `true` or `false`.
- `char` - a single character.

# Arrays
- `[]` - a slice of a type. E.g. `[i32]` is a slice of 32 bit integers.
- `[char 64]` - an array of characters of size 64.

# Comments
- `#` - a single line comment.

# Structs

- A basic class representation.
- Can have 0..n fields.

Example:

```
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

# TODO:
# Structs can have methods.
NEED TO DETERMINE
```

# Components

- A basic component representation.
- Data only. No methods.
- Can have 0..n fields.
- Can have singletons. Declared with `single_component ComponentName`.

Example:

```
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

# This is an example of a struct component. Prop values can also be arrays.
single_component Graphics {
    i32 frame
    [Sprite 256] sprites
}
```


#

# TODO: parsing

#

# Expressions

- A basic function representation.
- Can have 0..n parameters.
- Must return a value.
- Last statement is always returned.
- Can have a `void` return type.

Example:

```
i32 add(i32 a, i32 b) {
    a + b
}

i32 min(i32 a, i32 b) {
    if a < b {
        a
    } else {
        b
    }
}

void do_nothing() {
    # Do nothing
}

void add_one(i32 a) {
    a += 1
}
```


#

# TODO: implement

#

# Systems

- A basic system representation.
- No generics.
- Components are declared then expressions are declared.
- Has writeble and readable components. Readable and writeable components are arrays of 0..n components.
- Can also have optional components.
- Can do a `for entity with [component list]` to iterate over all entities with the given components.
- Can also do a `world_state = single WorldState` to get the first component of a type. Typically used for singletons.
- Can include optional entities in the iteration by marking components with a `?`.
- Checks components to make sure `read` components are not mutated.
- Checks components to make sure they're declared as `read` or `write` before accessing.
- `read` and `write` are only required if the system is accessing components.
- Can create and delete components. E.g. `entity.add Component` and `delete entity.Component`.

Example:

```
system ChangeStatus{
    read [Hp]
    write [Status IsAlive]

    for entity with Hp, Status, IsAlive? {
        # Deletes the given component if it exists.
        # Does not error if it doesn't.
        delete entity.IsAlive

        if entity.Hp == 0 {
            entity.Status = "Dead"
        } else {
            # Adds the given component to the entity
            entity.add IsAlive

            if entity.Hp < 10 {
                entity.Status = "Critical"
            } else if entity.Hp < 40 {
                entity.Status = "Warning"
            } else {
                entity.Status = "OK"
            }
        }
    }
}

system Controllables {
    read [Controller SpeedModifier]
    write [Position]

    # Allows optional speed modifiers
    for e with Controller, Position, SpeedModifier? {
        movespeed = 1 + (e.SpeedModifier ?? 0)

        delta = Vector2(0, 0)

        if e.Controller.upHeld {
            delta.y -= 1
        } else if e.Controller.downHeld {
            delta.y += 1
        }

        if e.Controller.leftHeld {
            delta.x -= 1
        } else if e.Controller.rightHeld {
            delta.x += 1
        }

        e.Position += delta
    }
}

system Collisions {
    read [Aabb Position]
    write [Collisions]

    for e with Aabb, Position, Collisions {
        e.Collisions.clear()

        for other with Aabb, Position {
            if e != other {
                if e.Position.x < other.Position.x + other.Aabb.width &&
                   e.Position.x + e.Aabb.width > other.Position.x &&
                   e.Position.y < other.Position.y + other.Aabb.height &&
                   e.Position.y + e.Aabb.height > other.Position.y {
                    collisions.push(other)
                }
            }
        }
    }
}

system WorldStateExample {
    # read and write are optional
    read [WorldState]

    world_state = single WorldState

    if world_state.frame == 0 {
        print("Hello World!")
    }

    if world_state.frame % 2 == 0{
        print("Is even!")
    } else {
        print("Is odd!")
    }
}

system WorldStateMutateExample {
    write [WorldState]

    world_state = single WorldState

    world.frame += 1
}
```

# TODO: parsing

# World

- Contains all systems to be dispatched in pipelines.
- Checks systems to ensure that they are sequenced properly. For example, if a pipeline has two systems, it will ensure that they don't both read and write the same component at the same time.
- Dispatches pipelines in parallel.
- Only one world may exist.
- Has a `init` method that is called once on initialization.

Example:

```
world {
    #
    init {
        # create is a keyword that takes a array of components to initialize
        create [
            WorldState {
                frame = 0
                deltaT = 0
            }
        ]

        entity1 = create [
            Position(10, 10)
            Aabb(10, 10)
            Hp(100)
            Controller()
        ]

        entity1 = create [
            Position(10, 10)
            Aabb(10, 10)
            Hp(100)
            Controller()
        ]

    }

    # Runs ChangeStatus and Gravity at the same time as they don't mutate and read the same components
    [ChangeStatus Gravity]

    # Moves the entities after Gravity as they would collide with read/writes
    [Controllables WorldStateExample]

    # Execute collisions after Controllables as they would collide with read/writes
    # Systems can also be ran multiple times in a dispatch
    [Collisions WorldStateMutateExample]
}
```

# TODO: parsing

# Entities

- A basic entity representation.
- Can have 0..n components.
- Can be created and killed.
- Can add and remove components.

Example:

```
create [
    WorldState {
        frame = 0
        deltaT = 0
    }
]

entity1 = create [
    Position(10, 10)
    Aabb(10, 10)
    Hp(100)
    Controller()
]

entity2 = create [
    Aabb(10, 10)
    Hp(100)
    Controller()
]

remove entity2.Controller
add entity2 Position(10, 10)

kill entity2
```

# TODO: Implement

# External Systems

This is a system that is implemented by the host language.
The code will be generated by the compiler and left to the user to implement.
Each target language will have a different implementation, and expects a single callback function to be implemented.
It is not type checked by the compiler due to it being externally implemented. As a result, no read components are allowed as this compiler can not check if the component is being mutated.

A stub will be generated for each file if the host language doesn't already have an implementation.

```
external_system Graphics {
    write [Graphics]

    # TODO: Need to determine if it's better to use headers
    # This specifies that the 'graphics.c' file is used for C.
    target_c_impl "graphics.c"

    target_js_impl "graphics.js"

    target_csharp_impl "graphics.cs"

    target_cpp_impl "graphics.cpp"
}
```

# Future Goals

## Closures
- MAYBE: Add ability to add closures to functions. Need to determine how useful this actually would be.

## Structs

- Member expressions. This includes operator overloading.
- Generics
- Static methods

## Expressions

- Generics
- Pattern matching

## Algebraic Data Types

- Generics
- Pattern matching
- Member expressions. This includes operator overloading.

## Interfaces

- Interfaces for structs
- Interfaces for ADTs
- Default implementations

