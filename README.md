# StackAssembly

## Introduction

I need a programming language that I can use as a foundation for my personal research into language design and implementation. To maximize flexibility and my opportunity for learning, I want full control over that language, which means creating my own.

But I want to achieve this in an incremental manner, with self-contained milestones along the way. Spending too much time on the initial implementation is not acceptable.

Here I present StackAssembly, a programming language so minimal that I can implement it quickly, but hopefully complete enough to write real code with.

I am fairly confident in that first objective. I worked on previous iterations of the same concept. The most recent one was mostly complete, and despite complicating it more than I needed to, I managed to implement it within a few weeks.[^0]

[^0]: [This direct predecessor][predecessor] is not well-designed though. One of the control flow primitives, `call_if`, has conditional effects on the operand stack that I can't even imagine how to work with in practice.

[predecessor]: https://github.com/hannobraun/playground/tree/main/archive/2025-10-27_stack-assembly

Whether this design is complete enough for real code, that remains to be seen. At the very least, I expect it to enable small experiments that can then inform the next steps.

My purpose here is to put some thought into the language before I implement it, hopefully saving additional time. I'm also going to publish this design, to document my work and in case somebody else finds it interesting. I'm assuming familiarity with basic computer science concepts, but not much else.

Please note that this document is not a complete specification. For the sake of convenience, I'm leaving out many details that I expect to become apparent in the course of the implementation.

## Basic Syntax

Let's start with some basic code.

```stack
1 2 +
```

StackAssembly code is organized into _scripts_,[^1] which are strings of UTF-8 characters. They can be embedded into another context, like the one above which is embedded into this design document. Or they can be dedicated files. In the latter case, the file name should end with `.stack`.

[^1]: In this document, I'm _emphasizing_ words that name specific language concepts for the first time.

For the time being, there's no way to reference one script from another. If you need to share code between them, you must copy and paste.

The characters in a script are grouped into _tokens_, which are delimited by whitespace. It doesn't matter how much whitespace, or what kind. As long as there's whitespace between two characters, they belong to different tokens. Otherwise, whitespace is ignored.

This means the script above has three tokens, and we could format it in many other ways without changing its behavior. Like this, for example:

```stack
1
2
+
```

All of the tokens here are _operators_, which come in different flavors. So far, we've seen _integers_ (`1`, `2`) and _identifiers_ (`+`). Integers are strings of base-10 digits, forming numbers that represent 32-bit two's complement values. Identifiers are arbitrary strings.

There's also another kind of operator and a whole different type of token. But let's not worry about that now. We know all that we need to, for the next step to make sense.

## Evaluation

Here's that first script again:

```stack
1 2 +
```

Now that we understand its syntax, we can figure out how it works. To make a script do something, we have to _evaluate_ it. We do that by going through the operators left to right, evaluating every single one.

Every operator has _inputs_ and _outputs_ (though each of those can be empty). Evaluating an operator consumes its inputs and produces the outputs. For example, the operator `1`, like all integers, has no inputs and one output, the value it represents.

What ties the evaluation of those single operators together, is an implicit _stack_. Outputs are _pushed_ to the top of that stack. So after we've evaluated `1` and `2`, the stack consists of those two numbers, with `2` on top.

Likewise, inputs are _popped_ from the top of the stack. `+` has two inputs and one output, which is the sum of its inputs. So evaluating it pops `2` and `1` from the stack, then pushes `3`

This stack-based evaluation model is where StackAssembly gets part of its name. And it is a key ingredient to enable quick implementation. As you've seen, there is not much to it. Some concepts, like variables and operator precedence, you simply don't have to worry about.

This simplicity comes at a price though. Stack-based code can be hard to follow. But for now, that's the right trade-off. Later on, I can experiment with measures to address this, or move on to another language with a different design.

## Stack Shuffling

With a stack comes the need to access inputs that might not currently be on top. StackAssembly offers two operators to handle this problem.

`copy` duplicates a value, pushing its copy to the top of the stack. It takes the index of that value as its input. If the index is `0`, it copies the top value; if the index is `1`, it copies the one below that; and so forth.

`drop` removes a value from the stack. Like `copy`, it takes the index of the value to remove as its input.

This is the most minimal yet complete set of operators that I could come up with. I expect that using them is going to be awkward, maybe painful. But before I'm ready to implement a more complex approach, I'd like to see how bad this one actually is.

## Effects

So far, I've carefully avoided mentioning the possibility of anything going wrong. And yet we've seen multiple things that could.

For example, what happens if we have an arbitrary identifier in our code, that means nothing to the language? Or what if an operator has more inputs than the stack currently has values? The answer to those and all similar questions is, that any such error condition triggers an _effect_.

Effects pause the evaluation of a script. There are different types of effects, so the user may distinguish between different error conditions.

If that was all there is to effects, we could call them "errors" instead. But while every error condition triggers an effect, not every effect comes from an error! In fact, effects may be a completely normal part of evaluation, which may even resume after an effect has been triggered.

But we'll learn about that later. For now, all we need to know is that any error condition triggers a suitable effect, and usually that means the evaluation is over.

## More Syntax

As I alluded to above, we haven't seen all there is to syntax yet. Let's take a look at this new script:

```stack
loop:
  @loop jump
```

This introduces two new syntactic elements. `loop:` is a _label_, the second type of token which are distinct from operators. All tokens that end with `:` are labels. `@loop` is a _reference_, the last kind of operator we've been missing. References start with `@`, and are usually paired with labels.

We'll be looking into how they work in a moment. But let's recap first, to make sure we understand the full picture:

- A script is made up of **tokens**.
  - **Labels** are one type of token.
  - The other type are **operators**, which come in three different flavors:
    - **references**,
    - **numbers**, and
    - **identifiers**.

## Control Flow

With all syntax in place, we can now learn about control flow. Here's the previous script again:

```stack
loop:
  @loop jump
```

Let's start with the label, `loop:`. Remember, labels are not operators. Those have inputs and outputs, and we can evaluate them. None of that applies to labels. A label just exists in the code, giving a name to the operator it precedes. That won't do anything, unless you pair the label with a reference.

The reference, `@loop` is tied to the `loop:` label. References have no inputs and one output, which is the address of the operator that the label names. Since labels name the next operator, which in this case is `@loop`, the output of that is its own address.

Then there's `jump`, which is just a regular identifier, though we haven't seen that specific one before. `jump` has one input, the address of an operator, and no outputs. It continues evaluation with the operator at that address.

Let's put it all together:

1. `loop:` is not an operator and does not evaluate to anything.
   It just tells us the name of the operator it precedes.
2. `@loop` has one output, its own address. It pushes that to the stack.
3. Finally, `jump` pops that address from the stack and jumps back to `@loop`.
   From here, steps 2. and 3. keep repeating indefinitely.

This alone does not yet make a Turing-complete programming language. We need one more piece, and that's `jump_if`.

```stack
loop:
  1 @loop jump_if
```

This script loops forever, like the one before. Only this time, we're using `jump_if`, which has two inputs, a _condition_ in addition to an address, and again no outputs. If the condition is non-zero (as it is here), it acts exactly like `jump`.

```stack
loop:
  0 @loop jump_if
```

Here we pass `0` as `jump_if`'s condition, which makes it do nothing. As a result, this whole script ends after `jump_if` and leaves no values on the stack.

Control flow is easily the most complicated part of this design, and I believe also the one that's easiest to overcomplicate. To counteract that, I made it as simple as I could, with this approach inspired by assembly languages.

## Type System

Taking more inspiration from assembly languages, StackAssembly is untyped. This means all values have the same structure. The correctness of your script might depend on treating them as one thing or another; a boolean value, an integer, or an address; but nothing in the language tracks or enforces these types.

Values are 32-bit words, which seems like a good compromise. It provides enough range for most applications, can be used to represent numbers along with other data like characters, and is well-supported on most modern platforms.[^2]

[^2]: Even microcontrollers usually support 32 bits these days. Though I'm sure that there's niche hardware that would have trouble with that, I don't think that's going to be relevant for StackAssembly's first version.

Keeping with the overall theme, this approach to the type system is quite simple. It also has the additional advantage of incurring no runtime overhead. That makes this solution quite close to a static type system, though without the compile-time protections.

Due to this lack of protections you can, accidentally or by design, do things lead to unpredictable results. Like passing an arbitrary number to `jump`. For now, my only advice is to be careful and avoid that.

## Memory

While 32-bit integers and a stack can already get you pretty far, we need an escape hatch for non-trivial data structures. A freely addressable, linear memory should do the trick.

Like the stack, that memory is organized into 32-bit words, which are also the smallest units you can address. The first word in memory has address `0`, the second has address `1`, and so forth.

The `read` operator reads a word from memory.

```stack
0 read
```

This reads the word at address `0` and pushes it to the stack.

The `write` function writes a word to memory.

```stack
-1 0 write
```

This writes the value `-1` to address `0`.

This is simpler than the more flexible approach of organizing the memory into bytes, and providing operators to read/write 8-, 16-, and 32-bit words.

## Hosts

StackAssembly will be implemented in Rust as a library. Running a script will require a Rust application that includes this library and provides any scripts to evaluate.

Such an application is called a _host_. The host drives the script's evaluation using the library's API and can communicate with the evaluation script throughout.

The communication between host and evaluation constitutes the only I/O facility that is available to StackAssembly scripts. Consequently, scripts are sandboxed and only able to affect the outside world through the host, which has full control over this interaction.

This sandboxing makes the language applicable to scenarios where a language with access to less restricted I/O would not be, for example if scripts are not fully trusted. Though the more important advantage of this approach, is how it couples flexibility with ease of implementation.

Implementing an FFI interface would provide a similar level of flexibility, though without the ability to sandbox scripts, and requiring a non-trivial amount of work to implement. A purpose-built standard library would only be as flexible as the level of effort invested into it.

The interface between script and host on the other hand, can be made quite simple, as we'll now see.

## I/O

All communication between a script and the outside world goes through the host. To moderate communication between script and host, the `yield` operator exists.

`yield` triggers an effect that exists for the sole purpose of transferring control to the host. Yield itself has no inputs or outputs. But while the host is in control, it can freely access both stack and memory. Afterwards, the host can decide to resume or abort the evaluation.

```stack
0 1 yield
```

Here we push two values to the stack before calling `yield`. The host can freely decide how to interpret these values. For example, it could interpret the `0` as a request to read input from the user, and the `1` as a memory address to write that input to.

This approach is closely inspired by how system calls in operating systems work.

## Valid Identifiers

I mentioned before that there is a fixed list of identifiers that are valid. Here it is:

- **Arithmetic**: `+`, `-`, `*` `/`, and `%`
- **Bitwise**:
  `and`, `or`, `xor`, `count_ones`, `leading_zeros`, `trailing_zeros`, `rotate_left`, `rotate_right`, `shift_left`, `shift_right`
- **Comparison**: `=`, `>`, `>=`, `<`, `<=`
- **Control flow**: `jump`, `jump_if`
- **Effects**: `abort`, `yield`
- **Memory**: `read`, `write`
- **Stack shuffling**: `copy`, `drop`

We've seen some of those already. Those we haven't do what their name suggests, mostly following established conventions from other programming languages. Though there are a few that are worth calling out.

Most arithmetic operations wrap on overflow,[^3] as I believe that provides the most flexibility. Other special cases, like divide by zero, trigger a suitable effect. Where that makes a difference, they treat all values as signed (two's complement) integers.

[^3]: Except for `/`, which would overflow if you divide the minimum value by `-1`. I think this is unlikely to be intentional though, so it triggers an effect instead.

There aren't any logical operations for now, as the bitwise ones can do double duty.

The `abort` operator triggers an effect which signals that the script would like to conclude its evaluation.

Evaluating any identifier that is not on this list triggers an effect.
