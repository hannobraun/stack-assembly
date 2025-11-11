# StackAssembly

## Introduction

I want to resume my personal research into programming language design and implementation,[^0] which requires a suitable starting point. I could use an existing language for that, but to maximize fun and learning, I'm going to build my own.

[^0]: I worked on [Kari] and [Crosscut] before, along with many smaller experiments.

[Kari]: https://github.com/hannobraun/kari
[Crosscut]: https://github.com/hannobraun/crosscut

But I want to achieve this in an incremental manner, with self-contained (and satisfying!) milestones along the way. To that end, I present the design of StackAssembly, a programming language so minimal that I can implement it quickly, but hopefully complete enough to use for real code.

I feel confident about the level of simplicity I achieved, and the speed of implementation that will enable.[^1] Whether this design enables real code though, that remains to be seen. At the very least, I expect it to support small experiments that can then inform the next steps.

[^1]: [This direct predecessor][predecessor] achieved a reasonable level of completeness, and I finished it within a few weeks.

[predecessor]: https://github.com/hannobraun/playground/tree/main/archive/2025-10-27_stack-assembly

In writing this document, I assume that the reader knows basic computer science concepts. Please note that I'm not making this a complete specification. For the sake of convenience, I leave out many details that I expect to become apparent during implementation.

## Basic Syntax

Let's start with a basic example:

```stack
1 2 +
```

StackAssembly code takes the form of _scripts_,[^2] each of which consists of UTF-8 characters. We can embed a script into another context, like I embedded the one above into this document. Or we can dedicate a complete file, whose name should then end with `.stack`.

[^2]: In this document, I _emphasize_ words that name specific language concepts for the first time.

For the time being, we can't reference one script from another. If we need to share code between them, we must copy and paste.

The characters in a script form whitespace-delimited _tokens_. It doesn't matter how much whitespace, or what kind. As long as whitespace separates two characters, they belong to different tokens. This means the script above has three tokens.

The language ignore whitespace otherwise, so we could write that script in different ways without changing its behavior. Like this, for example:

```stack
1
  2
    +
```

All of these tokens are _operators_, which come in different flavors. So far, we've seen _integers_ (`1`, `2`) and _identifiers_ (`+`). Integers consist of base-10 digits and form numbers that represent 32-bit two's complement values. Identifiers consist of arbitrary characters.

Let's move on now, as we learned enough about syntax for the next few sections to make sense. We'll revisit the topic later!

## Evaluation

Here's that first script again:

```stack
1 2 +
```

Now that we understand its syntax, we can figure out how it works. To make a script do something, we have to _evaluate_ it. This means going through the operators, left to right, evaluating each one.

Every operator has (possibly empty) lists of _inputs_ and _outputs_. Evaluating an operator consumes the inputs and produces the outputs. For example, the operator `1`, like all integers, has no inputs and one output, the value it represents.

An implicit _stack_ ties the evaluation of those single operators together. Outputs are _pushed_ to the top of that stack. So after we've evaluated `1` and `2`, the stack consists of those two integers, with `2` on top.

Likewise, inputs are _popped_ from the top of the stack. `+` has two inputs and one output, the sum of its inputs. So evaluating it pops `2` and `1` from the stack, then pushes `3`.

Its simplicity makes this stack-based model a key ingredient in controlling the language's scope. It renders variables, operator precedence rules, or complex syntax redundant, thereby defining the language's flavor along with part of its name.

## Stack Shuffling

With a stack comes the need to access inputs that might not currently sit on top of it. StackAssembly offers two operators to handle this problem: `copy` and `drop`.

`copy` duplicates a value, pushing a replica to the top of the stack. It takes the index of that value as its input. If the index is `0`, it copies the top value; if the index is `1`, it copies the one below that; and so forth.

```stack
3 5 8 1 copy
```

This script finishes evaluation with the values `3`, `5`, `8`, and `5` on the stack.

`drop` removes a value from the stack. Like `copy`, it takes the index of the value to remove as its input.

```stack
3 5 8 1 drop
```

This script finishes evaluation with the values `3` and `8` on the stack.

I could not come up with a more minimal, yet complete set of operators. Though using them may end up feeling awkward, possibly even painful, I don't want to implement a more complex solution before confirming that.

## Effects

So far, I carefully avoided mentioning the possibility of anything going wrong. And yet we've seen multiple examples that could.

For example, what happens if an identifier in our code means nothing to the language? Or what if an operator has more inputs than we currently have values on the stack?

Those and all similar error conditions trigger an _effect_. Effects pause the evaluation of a script. They present as different types depending on the trigger, making it possible to distinguish between them.

Not every effect originates from an error though. They can trigger as a regular part of evaluation, which may even resume afterwards. But we'll learn about that later. For now, we just need to understand that an error condition triggers an effect, which then pauses the evaluation.

## Type System

The simplest way of handling types in a programming language is to not do that at all, making the language untyped. This means that all values have the same structure and the language has no concept of what types are.

All values are 32-bit words, which seems like a good compromise. It provides enough range for most applications and can be used to represent numbers along with other data, like characters. Most modern platforms support 32-bit values well.

```stack
3 5 -1 drop
```

Here we wanted to drop `3` from the stack, but accidentally put a `-` in front of the index. Only unsigned integers make valid indices, so the language treats this integer as unsigned. Since `-1` has the same bit pattern as `4294967295`, that's the index the language sees. It results in an out of bounds error.

This is the simplest approach I could come up with, and it has the additional advantage of not incurring any runtime overhead.

## More Syntax

As I alluded to above, we haven't seen all there is to syntax yet.

```stack
loop:
  @loop jump
```

This script introduces two new syntactic elements:

- `loop:` is a _label_, another type of token distinct from operators.
  All tokens that end with `:` are labels.
- `@loop` is a _reference_, the last kind of operator.
  References start with `@` and are usually paired with labels.

We'll be looking into how they work in a moment. But let's recap first, to make sure we understand the full picture:

- A script consists of **tokens**.
  - **Operators** are one type of token. They come in three different flavors:
    - **integers**,
    - **identifiers**,
    - and **references**.
  - **Labels** are the other type of token.

## Control Flow

With all syntax now in place, we can learn about control flow. Here's the previous script again:

```stack
loop:
  @loop jump
```

Let's start with the label, `loop:`. Remember, labels are not operators. Those have inputs and outputs, and we can evaluate them. None of that applies to labels. A label just exists in the code, giving a name to the operator it precedes. That won't do anything, unless we pair the label with a reference.

The reference, `@loop`, is tied to the `loop:` label. References have no inputs and one output, the address of the operator that the label names. Since labels name the next operator, in this case `@loop`, that has its own address as its output.

Finally, we have `jump`, an identifier that we haven't seen before. `jump` has one input, the address of an operator, and no outputs. It moves evaluation to the operator at that address, so it may continue from there.

Let's put all that together:

1. `loop:` is not an operator and does not evaluate to anything.
   It just tells us the name of the operator it precedes.
2. `@loop` has one output, its own address. It pushes that to the stack.
3. Finally, `jump` pops that address from the stack and jumps back to `@loop`.
   From here, steps 2. and 3. keep repeating indefinitely.

This alone does not get us a Turing-complete programming language yet. We need one more piece, a conditional variant of `jump`.

```stack
loop:
  1 @loop jump_if
```

This script loops forever, like the one before. Only this time, we're using `jump_if`. `jump_if` has two inputs, a _condition_ in addition to an address, and again no outputs. With a non-zero condition, as we have here, it acts exactly like `jump`.

```stack
loop:
  0 @loop jump_if
```

Here we pass `0` as `jump_if`'s condition, which makes it do nothing. As a result, this whole script ends after `jump_if` and leaves no values on the stack.

Control flow is the most complex part of this design, and also one I easily could have overcomplicated. To counteract that, I made it as simple as I could, using an approach inspired by assembly languages. Here, StackAssembly derives the second part of its name.

## Memory

While 32-bit words on a stack can already get us pretty far, we need an escape hatch for non-trivial data structures. A freely addressable, linear _memory_ should do the trick.

Like the stack, the memory consists of 32-bit words. These words are also the smallest units we can address.

For reading from memory, we have the `read` operator.

```stack
0 read
```

Here we read the word at address `0`, the first word in memory, and push it to the stack.

Likewise, for writing to memory, we have `write`.

```stack
-1 1 write
```

This writes the value `-1` to the second word in memory, at address `1`.

I could have gone with the more flexible and traditional approach of organizing the memory into separately addressable bytes and providing operators to read/write 8-, 16-, and 32-bit words. What I went with is simpler though, and should do for now.

## Hosts

I am going to implement StackAssembly as a library in Rust. Doing anything with it will require a Rust application that provides a script and uses the library to evaluate that script. We call this application the _host_.

Every user can write their own host, though they could also use an existing one. The host drives the evaluation and can communicate with the script throughout. This communication between host and script constitutes the only I/O facility that is available to StackAssembly code.

As a result, the host sandboxes scripts and retains full control over their effect on the outside world. This enables use cases that would not tolerate less restricted I/O.

Though more importantly, the facility for communication between host and script can work quite simply (as we'll see). Given the user's ability to bring their own host, this approach combines flexibility with ease of implementation.

While an FFI interface could offer a similar level of flexibility, implementing that would likely require much more work. And a purpose-built standard library would require an investment proportional to the power it provides. Neither would come with sandboxing for free, as the host-based approach does.

## I/O

We've learned that all communication between a script and the outside world goes through the host. Let's take a look at how that works.

The `yield` operator moderates communication between script and host.

```stack
0 1 yield
```

`yield` has no inputs or outputs. It only triggers an effect, transferring control to the host. The host can then inspect stack and memory and decide how to react. In this example, the stack values `0` and `1` could define some type of request that the script makes of the host.

This approach is closely inspired by how system calls work. Together with the already existing language facilities, and the host's access to them, it provides a lightweight channel for communication.

## Valid Identifiers

Any token that the language doesn't recognize as something more specific, ends up as an identifier. But while an identifier can be an arbitrary string of characters, only specific identifiers are valid. Evaluating an invalid identifier is an error and triggers an effect.

Here we have the full list of valid identifiers, grouped by category:

- **Arithmetic**: `+`, `-`, `*` `/`
- **Bitwise**:
  `and`, `or`, `xor`, `count_ones`, `leading_zeros`, `trailing_zeros`, `rotate_left`, `rotate_right`, `shift_left`, `shift_right`
- **Comparison**: `=`, `>`, `>=`, `<`, `<=`
- **Control flow**: `jump`, `jump_if`
- **Effects**: `yield`
- **Memory**: `read`, `write`
- **Stack shuffling**: `copy`, `drop`

We've seen some of those already. All of them do what their name suggests, mostly following established conventions from other programming languages. Though there are a few details worth calling out:

- The arithmetic operations treat all values as signed (two's complement)
  integers, where that makes a difference.
- Most arithmetic operators wrap on overflow,
  as that provides the most flexibility.
- `/` outputs both the result of the division and the remainder.
  This obviates the need for a dedicated modulo/remainder operator.
- `/` triggers suitable effects on divide by zero and on overflow.
  Neither of those cases seem likely to be intentional and we can easily work around them.
- I've avoided adding any logical operations for now,
  as the bitwise ones can do double duty.
