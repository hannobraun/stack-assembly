# StackAssembly

## Introduction

I need a programming language that I can use as a foundation for my personal research into language design and implementation. To maximize fun, flexibility, and my opportunity for learning, I want full control over that language, which means creating my own.

But I want to achieve this in an incremental manner, with self-contained (and satisfying!) milestones along the way. It would not be acceptable to spend too much time on an initial implementation that does not provide much utility.

In this document I present the design of StackAssembly, a programming language so minimal that I can implement it quickly, but hopefully complete enough to write real code with.

I am fairly confident in that first objective.[^0] But whether this design is complete enough for real code, that remains to be seen. At the very least, I expect it to enable small experiments that can then inform the next steps.

[^0]: [This direct predecessor][predecessor] is reasonably complete and I finished it within a few weeks. It's not well-designed though. One of the control flow primitives, `call_if`, has conditional effects on the operand stack that I can't even imagine how to work with in practice.

[predecessor]: https://github.com/hannobraun/playground/tree/main/archive/2025-10-27_stack-assembly

My purpose here is to put some thought into the language before I implement it, hopefully saving additional time. I'm also going to publish this design, to document my work and in case somebody else finds it interesting. I'm assuming familiarity with basic computer science concepts, but not much else.

Please note that this document is not a complete specification. For the sake of convenience, I'm leaving out many details that I expect to become apparent during the course of the implementation.

## Basic Syntax

Let's start with a basic example:

```stack
1 2 +
```

StackAssembly code is organized into _scripts_,[^1] which are strings of UTF-8 characters. They can be embedded into another context, like the one above which is embedded into this design document. Or they can be dedicated files whose name ends with `.stack`.

[^1]: In this document, I _emphasize_ words that name specific language concepts for the first time.

For the time being, there's no way to reference one script from another. If you need to share code between them, you must copy and paste.

The characters in a script are grouped into _tokens_, which are delimited by whitespace. It doesn't matter how much whitespace, or what kind. As long as there's whitespace between two characters, they belong to different tokens. This means the script above has three tokens

Otherwise, whitespace is ignored. So we could write that script in many other ways without changing its behavior. Like this, for example:

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

Now that we understand its syntax, we can figure out how it works. To make a script do something, we have to _evaluate_ it. We do that by going through the operators, left to right, evaluating them one by one.

Every operator has _inputs_ and _outputs_ (though each of those can be empty). Evaluating an operator consumes the inputs and produces the outputs. For example, the operator `1`, like all integers, has no inputs and one output, the value it represents.

What ties the evaluation of those single operators together, is an implicit _stack_. Outputs are _pushed_ to the top of that stack. So after we've evaluated `1` and `2`, the stack consists of those two numbers, with `2` on top.

Likewise, inputs are _popped_ from the top of the stack. `+` has two inputs and one output, which is the sum of its inputs. So evaluating it pops `2` and `1` from the stack, then pushes `3`

This stack-based evaluation model is where StackAssembly gets the first part of its name from. And it is a key ingredient to making it easy to implement. As you've seen, there is not much to this. And some concepts, like variables and operator precedence, you simply don't have to worry about.

This simplicity comes at a price though. Stack-based code can be hard to follow. But for now, it's the right trade-off. Later on, I can experiment with measures to address this, or move on to another language with a different design.

## Stack Shuffling

With a stack comes the need to access inputs that might not currently be on top. StackAssembly offers two operators to handle this problem: `copy` and `drop`.

`copy` duplicates a value, pushing its copy to the top of the stack. It takes the index of that value as its input. If the index is `0`, it copies the top value; if the index is `1`, it copies the one below that; and so forth.

```stack
3 5 8 1 copy
```

This script finishes evaluation with the values `3`, `5`, `8`, and `5` on the stack.

`drop` removes a value from the stack. Like `copy`, it takes the index of the value to remove as its input.

```stack
3 5 8 1 drop
```

This script finishes evaluation with the values `3` and `8` on the stack.

This is the most minimal yet complete set of operators that I could come up with. I expect that using them is going to be awkward, maybe painful. But before I'm ready to implement a more complex approach, I'd like to see how bad using this one actually is.

## Effects

So far, I've carefully avoided mentioning the possibility of anything going wrong. And yet we've seen multiple examples that could.

For example, what happens if we have an arbitrary identifier in our code, that means nothing to the language? Or what if an operator has more inputs than the stack currently has values? The answer to those and all similar questions is, that any such error condition triggers an _effect_.

Effects pause the evaluation of a script. There are different types of effects, allowing the user to distinguish between different errors.

If that was all there is to effects, we could call them "errors" instead. But while every error triggers an effect, not every effect comes from an error! In fact, effects can be a completely normal part of evaluation, which may even resume after an effect has triggered.

But we'll learn about that later. For now, all we need to know is that any error condition triggers a suitable effect, and usually that means the evaluation is over.

## More Syntax

As I alluded to above, we haven't seen all there is to syntax yet. Let's take a look at this new script:

```stack
loop:
  @loop jump
```

This introduces two new syntactic elements. `loop:` is a _label_, the second type of token, which are distinct from operators. All tokens that end with `:` are labels. `@loop` is a _reference_, the last kind of operator we've been missing. References start with `@`, and are usually paired with labels.

We'll be looking into how they work in a moment. But let's recap first, to make sure we understand the full picture:

- A script is made up of **tokens**.
  - **Operators** are one type of token. They come in three different flavors:
    - **numbers**,
    - **identifiers**,
    - and **references**.
  - **Labels** are the other type of token.

## Control Flow

With all syntax in place, we can now learn about control flow. Here's the previous script again:

```stack
loop:
  @loop jump
```

Let's start with the label, `loop:`. Remember, labels are not operators. Those have inputs and outputs, and we can evaluate them. None of that applies to labels. A label just exists in the code, giving a name to the operator it precedes. That won't do anything, unless you pair the label with a reference.

The reference, `@loop`, is tied to the `loop:` label. References have no inputs and one output, which is the address of the operator that the label names. Since labels name the next operator, which in this case is `@loop`, the output of that is its own address.

Then there's `jump`, which is just a regular identifier, though one we haven't seen before. `jump` has one input, the address of an operator, and no outputs. It moves evaluation to the operator at that address, continuing from there.

Let's put it all together:

1. `loop:` is not an operator and does not evaluate to anything.
   It just tells us the name of the operator it precedes.
2. `@loop` has one output, its own address. It pushes that to the stack.
3. Finally, `jump` pops that address from the stack and jumps back to `@loop`.
   From here, steps 2. and 3. keep repeating indefinitely.

This alone does not get us a Turing-complete programming language yet. We need one more piece, and that's `jump_if`.

```stack
loop:
  1 @loop jump_if
```

This script loops forever, like the one before. Only this time, we're using `jump_if`, which has two inputs, a _condition_ in addition to an address, and again no outputs. If the condition is non-zero, as it is here, it acts exactly like `jump`.

```stack
loop:
  0 @loop jump_if
```

Here we pass `0` as `jump_if`'s condition, which makes it do nothing. As a result, this whole script ends after `jump_if` and leaves no values on the stack.

Control flow is easily the most complex part of this design, and I believe also the one that's easiest to overcomplicate. To counteract that, I made it as simple as I could, using this approach that is inspired by assembly languages.

## Type System

Taking more inspiration from assembly languages, StackAssembly is untyped. This means all values have the same structure, and the language has no concept of what types are.

```stack
3 jump
```

Here we use the integer `3` as the input to `jump`, even though `jump` expects to receive the address of an operand. Nothing in the language tracks or enforces this expectation though, and what this script does is completely dependent on the implementation and how that encodes addresses.

All values are 32-bit words, which seems like a good compromise. It provides enough range for most applications, can be used to represent numbers along with other data like characters, and is well-supported on most modern platforms.

Again, I chose this approach because it the simplest one I can come up with. And it has the additional advantage of incurring no runtime overhead. That makes this solution quite close to a static type system, though without the compile-time protections.

## Memory

While 32-bit integers and a stack can already get you pretty far, we need an escape hatch for non-trivial data structures. A freely addressable, linear _memory_ should do the trick.

Like the stack, that memory is organized into 32-bit words, which are also the smallest units you can address.

```stack
0 read
```

Here we use the `read` operator to read the word at address `0`, the first word in memory, and push it to the stack.

```stack
-1 1 write
```

And here we use `write` to write the value `-1` to the second word in memory, at address `1`.

The more traditional approach of organizing the memory into bytes and providing operators to read/write 8-, 16-, and 32-bit words would have been more flexible. The approach I chose here is simpler though, and should do for now.

## Hosts

I am going to implement StackAssembly as a library in Rust. Doing anything with it will require a Rust application that provides a script and uses the library to evaluate it.

We call this application the _host_. A host drives the evaluation and can communicate with the script throughout. This communication between host and script constitutes the only I/O facility that is available to StackAssembly.

As a result, scripts are sandboxed and only able to affect the outside world through the host, which has full control over this interaction. This makes the language applicable to use cases where access to less restricted I/O would not be acceptable.

Though more importantly, the facility for communication between host and script can be made quite simple (as we'll see). Given the user's ability to implement their own host, this approach combines flexibility with ease of implementation.

While an FFI interface could offer a similar level of flexibility, it would likely be harder to implement. A purpose-built standard library would be limited by the amount of effort I can invest into it.

## I/O

All communication between a script and the outside world goes through the host. To moderate communication between script and host, the `yield` operator exists.

```stack
0 1 yield
```

`yield` has no inputs or outputs. All it does is trigger an effect to transfer control to the host. The host can then inspect stack and memory, and decide how to react. In this example, `0` could define the type of service we request from the host, while `1` could be an argument of that request.

This approach is closely inspired by how system calls work. Together with the already existing language facilities, and the host's access to them, it provides a lightweight channel for communication.

## Valid Identifiers

Unless a token is recognized as something more specific, it ends up as an identifier. But while an identifier can be an arbitrary string of characters, only specific identifiers are valid. Here's the list:

- **Arithmetic**: `+`, `-`, `*` `/`, and `%`
- **Bitwise**:
  `and`, `or`, `xor`, `count_ones`, `leading_zeros`, `trailing_zeros`, `rotate_left`, `rotate_right`, `shift_left`, `shift_right`
- **Comparison**: `=`, `>`, `>=`, `<`, `<=`
- **Control flow**: `jump`, `jump_if`
- **Effects**: `yield`
- **Memory**: `read`, `write`
- **Stack shuffling**: `copy`, `drop`

We've seen some of those already. Those we haven't do what their name suggests, mostly following established conventions from other programming languages. Though there are a few details worth calling out.

Most arithmetic operations wrap on overflow,[^2] as I believe that provides the most flexibility. Other special cases, like divide by zero, trigger a suitable effect. Where that makes a difference, they treat all values as signed (two's complement) integers.

[^2]: Except for `/`, which would overflow if you divide the minimum value by `-1`. I think this is unlikely to be intentional though, so it triggers an effect instead.

There aren't any logical operations for now, as the bitwise ones can do double duty.
