# StackAssembly

## Introduction

This is the design for StackAssembly, a programming language so minimal that I can implement it quickly, but hopefully complete enough to write real code with.

I'm fairly confident in the first objective. I worked on previous iterations of the same concept. The most recent one was mostly complete, and despite complicating that more than I needed to, I managed to implement it within a few weeks.[^0]

[^0]: [This direct predecessor][predecessor] is not well-designed though. One of the control flow primitives, `call_if`, has conditional effects on the operand stack that I can't even imagine how to work with in practice.

[predecessor]: https://github.com/hannobraun/playground/tree/main/archive/2025-10-27_stack-assembly

Whether this design is complete enough for real code, that remains to be seen. At the very least, I hope it can serve as my foundation for further exploration into programming language design.

My purpose here is to put some thought into the language before I implement it, hopefully saving additional time. I'm also going to publish it, in case somebody else finds it interesting and to document my work. I'm assuming familiarity with basic computer science concepts, but not much else.

Please note that this document is not a complete specification. For the sake of convenience, I'm leaving out many details that I expect to become apparent during the implementation.

## Basic Syntax

Let's start by looking at some basic StackAssembly code and understanding its syntax.

```stack
1 2 +
```

StackAssembly code is organized into _scripts_,[^1] which are strings of UTF-8 characters. They could be embedded into some other context, like the one above which is embedded into this design document, or they could be dedicated files. In the latter case, the file name should end with `.stack`.

[^1]: Here and in the rest of this document, I'm _emphasizing_ words that name specific language concepts for the first time.

For the time being, there's no way to reference one script from another. If you need to share code between scripts, you must copy and paste.

The characters in a script are grouped into _tokens_, which are delimited by whitespace. It doesn't matter how much, or what kind. As long as there's whitespace between two characters, they belong to different tokens. Otherwise, whitespace is ignored.

This means the script from above has three tokens, and we could format it in many other ways without changing its behavior. Like this, for example:

```stack
1
2
+
```

All of the tokens here are _operators_, which come in different flavors. So far, we've seen _integers_ and _identifiers_. Integers are strings of base-10 digits, forming numbers that represent 32-bit two's complement values. Identifiers are arbitrary strings.

There's also another kind of operator and a whole different type of token. But let's not worry about that now. We know all that we need to, for the next step to make sense.

## Stack-Based Evaluation

Now we understand the syntax of a basic StackAssembly script. It's not complicated! Here it is again:

```stack
1 2 +
```

To figure out what the script does, we have to _evaluate_ it. That's not complicated either! We just go left to right, evaluating every single operator we encounter.

Each operator can have _inputs_ and _outputs_. Integers are pretty simple: they have no inputs and one output, the value that the respective integer represents. This means the output of the operator `1` is the value `1`, and the output of `2` is `2`.

What do we do with those outputs? We push them to the top of a stack! So by the time we arrive at `+`, that stack consists of the values `1` at the bottom and `2` on top.

The operator `+` has two inputs and one output, which is the sum of its inputs. We always pop inputs from the top of the stack. So after we've taken `1` and `2` and pushed the output, the stack consists of the value `3`.

This stack-based evaluation model is where StackAssembly gets the first part of its name.

I have worked on programming languages before, both stack-based and otherwise. In my experience, nothing beats the simplicity and ease of implementation of stack-based languages. There are many things you just don't have to worry about, including variables and operator precedence.

This simplicity comes at a price. Stack-based code can become hard to follow. But for now, that's the right trade-off. Later on, I can experiment with measures to address this, or implement another language with a different design.

## Stack Shuffling

With a stack comes the need to access inputs that might not currently be on top. StackAssembly offers two operators to address this problem.

`copy` duplicates a value, pushing its copy to the top of the stack. It takes the index of that value as its input. If the index is `0`, it removes the top value; if the index is `1`, it removes the one below that; and so forth.

`drop` removes a value from the stack. Like `copy`, it takes the index of the value to remove as its input.

This is the most minimal yet complete set of operators that I could come up with. I expect that using them is going to be awkward, maybe painful. But before I'm ready to implement a more complex approach, I'd like to see how bad this one actually is.

## Effects

So far, I've carefully avoided mentioning the possibility of anything going wrong. And yet we've seen multiple things that could.

For example, what happens if we have an arbitrary identifier that means nothing to the language? Or what if an operator has more inputs than the stack currently has values? The answer to those and all similar questions is, that any such error condition triggers an _effect_.

An effect pauses the evaluation of a script. There are different types of effects, so the user may distinguish between the different error conditions.

Why then, if every error condition triggers an effect, do we call it "effect" and not "error"? That's because not every effect is an error, and in fact, a script's evaluation may resume after being paused by an effect.

But we'll learn about that later. For now, all we need to know is that any error condition triggers a suitable effect, and usually that means the evaluation is done.

## More Syntax

There's a bit more to syntax that we haven't seen yet. Let's take a look at another script:

```stack
loop:
  @loop jump
```

This introduces two new syntactic elements that we haven't seen so far. `loop:` is a _label_, the second type of token that are distinct from operators. All tokens that end with `:` are labels, so for example `a:`, `b:`, and `c:` are all labels.

`@loop` is a _reference_, the last kind of operator we've been missing. References all start with `@`, so the references that match the aforementioned labels would be `@a`, `@b`, and `@c`.

We'll be looking into what all this actually does in a moment. But let's recap first, to make sure we understand the full picture:

- A script is made up of **tokens**.
  - **Labels** are one type of token.
  - The other type are **operators**, which come in three different flavors:
    - **References**
    - **Numbers**
    - **Identifiers**

## Control Flow

It's time to put that new syntax we just learned about into action and look into control flow. Here's the previous script again:

```stack
loop:
  @loop jump
```

Let's start with the label, `loop:`. Remember, labels are not operators. Those have inputs and outputs, and we can evaluate them. None of that applies to labels. They just exist in the code, to give a name to the next operator. But they alone won't do anything, unless you pair them with a reference.

The reference, `@loop` is tied to the `loop:` label. References have no inputs and one output, which is the address of the operator that the label names. Since labels name the next operator in the code, which in this case is `@loop`, the output of that is its own address.

Then we have `jump`, which syntactically is not new. It's just an operator, specifically an identifier, though we haven't seen that specific one before. `jump` has one input, the address of an operator, and no outputs. It "jumps" to that operator, so that evaluates next.

Let's put it all together:

1. `loop:` is not an operator and does not evaluate to anything.
   It just tells us that the following operator, `@loop`, has the name "loop".
2. `@loop` has one output, its own address. It pushes that to the stack.
3. Finally, `jump` pops that address from the stack and jumps back to `@loop`.
   From here, steps 2. and 3. keep repeating forever.

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

Here we pass `0` as `jump_if`'s condition, which makes it do nothing. As a result, this whole script ends after `jump_if` and leaves no output on the stack.

All of this is inspired by assembly languages and about as simple as I could make it. Control flow is easily the most complicated part of this design, and I believe also the one that's easiest to overcomplicate. I kept it simple to counteract that.

## Type System

In keeping with this theme of simplicity, StackAssembly goes with the simplest possible type system. Neither static nor dynamic, but none. The language is untyped.

All values are 32-bit words, which seems like a good compromise. It provides enough range for most applications, can be used to represent numbers along with other data like characters, and is well-supported on most modern platforms.[^2]

[^2]: Even microcontrollers usually have 32 bits these days. Though I'm sure that there's niche hardware that would have trouble with that, I don't think that's going to be relevant for StackAssembly's first version.

Different operators may treat values differently. For example, `+` treats its inputs as numbers, while `jump` treats its input as the address of an operand. So there are types in that sense, but nothing enforces their correct use.

This means that a user could, accidentally or by design, pass an arbitrary number to `jump` or do other things that could lead to unpredictable results. For now, the only advice I can give is to be careful and not do that.

In addition to being simple, no type system means no runtime overhead, which means I have the option to later add a static type system without changing the runtime behavior.[^3]

[^3]: It would also be possible to add a dynamic type system instead, but I'm fairly certain that I'm not going to do that. I want StackAssembly to be widely applicable, and the overhead of a dynamic type system would limit that.

## Memory

While 32-bit integers and an operand stack can already get you pretty far, StackAssembly needs an escape hatch for non-trivial data structures. A freely addressable, linear memory should do the trick.

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

This is simpler than the more flexible approach of organizing the memory into bytes, and providing operators to read/write 8-, 16-, and 32-bit words. Although that sounds like something that later versions could introduce.

## Hosts

We've seen most of the language by now, but we've been missing one important thing: an I/O facility. Or in other words, anything that can affect the outside world in any way. There are multiple ways to provide such a capability.

One way would be a standard library, but this would limit the available use cases to what the standard library covers. In terms of the ratio of capability for the user to work invested on my side, this wouldn't be a good option.

An alternative would be a foreign function interface, which could be C-compatible, or integrate with any other established platform. This would allow users to write their own libraries, but it could still be a non-trivial amount of work for me. Possibly more than the whole rest of the language.

Fortunately there is a third way. By implementing the language as a library and requiring the user to provide a host that embeds that and drives its execution, I can provide as much flexibility as I would with an FFI, while significantly reducing the level of complexity that the actual interface requires.[^4]

[^4]: And since I intend to implement the language in Rust, which has a C-compatible FFI interface, it should be possible to use this library from pretty much every language in existence, on any platform that Rust supports. I don't intend to write the necessary C API myself though.

Whenever the evaluation triggers an effect, control yields to the host, which then gets to decide what to do. Since most effects signal catastrophic (as far the the script is concerned) error conditions, the host should mostly react by aborting the evaluation. But there is one case where it doesn't need to.

The `yield` operator triggers a special effect whose only purpose is to yield control to the host. This is the primary means by which the script moderates communication with the host.

The host can then decide, based on the contents of stack and memory, what should happen. In turn, it can modify stack and memory before resuming the script. This way, `yield` can be used as a generic, host-specific call mechanism by which the script can request a service or notify of a condition.

In addition to the aforementioned advantages, this architecture also means that all StackAssembly scripts run completely sandboxed, making the language usable in situation where another language with access to a full FFI interface wouldn't be acceptable.

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

Most arithmetic operations wrap on overflow,[^5] as I believe that provides the most flexibility. Other special cases, like divide by zero, trigger a suitable effect. Where that makes a difference, they treat all values as signed (two's complement) integers.[^6]

[^5]: Except for `/`, which would overflow if you divide the minimum value by `-1`. I think this is unlikely to be intentional though, so it triggers an effect instead.

[^6]: Later versions could introduce additional variants for unsigned and floating-point values. But for now, I'd like to keep it simple.

There aren't any logical operations for now, as the bitwise ones can do double duty.

The `abort` operator triggers an effect which signals that the script would like to conclude its evaluation.

Evaluating any identifier that is not on this list triggers an effect.
