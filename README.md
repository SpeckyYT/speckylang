# SpeckyLang

One of the programming languages ever.

## Functionality

### Pointer

SpeckyLang is a pointer-based language, it only has one pointer which you can change to any value and read or write to/from that pointer.

By default, the pointer is set to the `null` value.
To change the pointer, you need to use the `|< (expression)` operation.
The pointer can be any possible value.

### Expressions

Expressions are quite minimalist in SpeckyLang.
They can only be `(any amout (>= 0) of '§')(value)`.

The `§` symbol is a "reader", it reads the value, and replaces it with the content of the pointer as that value.
Every `§` you add will repeat the process.

In other programming languages, this is known as "dereferencing".

```specky
|< a <= b   # sets `a` as the current pointer, and assigns `b` to the current pointer (`a`)
|< §a       # now the pointer is `b`, since it's the value at the `a` pointer.
|< §§a      # the pointer is now `null`, since `b` has nothing assigned to it.
```

### Values

SpeckyLang has various kind of values.

```specky
# Symbol
abc_abc

# Boolean
true
false

# Integer
69420133791142666 # Integers use BigInts, therefore these numbers can be arbitrarily big

# Float
0.1273 # Floats use BigFloats, so these numbers can have any precision imaginable

# Text
/sussy baka/

# Time
µ # represents the current time if you do `<= µ`

# JumpAddress
[<] sussy # can be any value. this is as if it was doing `|< sussy <= (current_statement index)`

# Null
null
```

### Operations / Statements

Statements can only have three forms:

- Binary operations (e.g. `<= (expression)`)
- Unary operations (e.g. `{%}`)
- Sequential operations (e.g. `?????`, `!!!`, `°°`, `$`)

All statements will be executed in order, without any possible statement grouping / priority.

For example, in the code `<= 1 + 2`, as the first step, `1` is assigned to the current pointer, and then `2` is added to it.
It's technically not the same as `<= 3`, but in this case they do the same thing.

#### Main

These are the main operations, they are quite important.

```specky
|< value    # sets the current pointer to `value`
<= value    # assigns the value of the current pointer to `value`
=> value    # sets the current pointer to `value`
<=> value   # swaps the values of the current pointer and `value`
~ value     # indexes the value at the current pointer with `value`
```

#### Loops

Loops are quite important in any language, so here they are!

You can define jump addresses and then jump back to them.
The code needs to execute the definition line for it to be defined.
You cannot jump to addresses you never defined.

Jump addresses are saved in the same memory where the variables are stored.
You should use an unique name / value for loops if you don't want to accidentally overwrite them, which results into being unable to jump back.

```specky
[<] loop    # defines a jump address to the current line, with the name `loop`
[>] loop    # jumps to the `loop` address
```

#### Math

Math operations always get beformed with the value of the pointer as the left operand, and the following expression as the right operand.
The output of the operation will be inserted into the current pointer value.

If the inputs are invalid for the specific operation, it will output `null`.

Note that it will literally take the expression as the right operand.
For example, if you want to exponentiate `a` by `a`, you need to do `|< a ^ §a`, since you want to use the underlaying value, not literally `a`.

```specky
+ value # this increments the current pointer by `value` (literally), if you want to increment it by the value of the `value` pointer, you should do `§value`
- value # decrements the current pointer
* value # multiplies the current pointer
\ value # divides the current pointer
% value # sets the remainder to the current pointer (`pointer = pointer % value`)
^ value # exponentiates the current pointer
```

#### Comparisons

Comparisons are similar to the [math operations](#math) with one difference: the output is usually a boolean (it can still also be `null` if the inputs aren't included in the operation)

```specky
>< value    # unequal
= value     # equal
< value     # less than
=< value    # less than or equal
> value     # greater than
>= value    # greater than or equal
```

#### Binary

```specky
& value     # and
| value     # or
>-< value   # xor
```

#### Conditions

Conditions are funny in SpeckyLang, they simply check if the value of the current pointer is truthy/falsy/existing/null, and if it's not then it will skip the next `n` statements, where `n` is the amount of condition characters you put in a row.

```specky
?   # truthy
!   # falsy
$   # exists    (if it's not null)
°   # null      (if it's null)
```

```specky
|< a <= false
???             # will only run the next 3 statements if it's truthy (otherwise those lines will be skipped)

|< a {@} {%}    # (these are 3 statements) should print 'a \n false', but since the value of `a` (the current pointer) is falsy, this won't be executed.
```

#### Logging

Logging is overcomplicated in SpeckyLang, but it act as if that wasn't the case!

Logging statements are delimited by `{` and `}`, and may contain some of the following characters.

```specky
# value print
# you can have none or one of these two active, if you input both, the last one will get used.
@   # prints the current pointer
%   # prints the value at the current pointer

# value reader
§   # reads the value (fun fact: doing `{@§}` is the same as `{%}`)

# assigning
<   # assigns the output print to the value at current pointer (it can be any of type depending on the output), it won't print to the terminal

# special printing
$   # prints the value in a special/alterate way
~   # reverses the output
^   # prints vertically

# formatting
\   # removes the final newline
°   # adds additional spaces to the output (doing `{°°°\}` will add 3 spaces without a newline)
```

## Examples

### Factorial

```specky
|< number <= 10 # this represents the `n` in `n!`
|< total <= 1   # this will be the output number

[<] factorial       # define 
|< total * §number  # this will do `total *= memory[number]`
|< number - 1 ?     # this decrements `number` and checks if it is truthy
[>] factorial       # if it is truthy, it will loop back to `factorial`

|< total {%}    # print the `total` variable
```

### For other examples, check the `test/` folder

## Where can I join the SpeckyLang religion?

[Here](https://discord.gg/4EecFku).
