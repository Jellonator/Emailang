# Complete Emailang Document

## Data types
* Null (Not a value)
* String (String of characters)
* Tuple (List of values)
* User (a Username and a Domain)
* Expression (evaluates an instruction)

A tuple with a single value can be constructed by adding a single comma after
the value, e.g. `("foo",)`. An empty tuple can be constructed with `(,)`.

Alphanumeric words, e.g. `foo`, `bar_baz` and `123`, will all be treated as a
string, provided they contain no whitespace or unexpected characters.

## Operators
The following operators exist:

### Send mail
`>` - Send an email. Uses a tuple or string on the left as a draft for the
email, and sends it to the user on the right.

### Concatenation
`+` - If both the left and right values are strings, it will
concatenate these two strings together. If one or both of them are a tuple, it
will concatenate them as a tuple. E.g., `"a" + ("b", "c")` results
in `("a", "b", "c")`

### Environment variable getter.
`@` - Takes the identifier to the right and retrieves
the value of the environment variable of that name. If instead of an identifier
a tuple is given, this operator will return a tuple with all of the values of
the identifiers in the environment, e.g. `@("message", "subject")` will evaluate
to `(email message, email subject)`. Note that `@content` and `@"content"` are
the same, as well as `@(content, subject)` and `@("content", "subject")`. It is
possible to chain retrieval operators, e.g.
`foo = bar; bar = "Hello, World!"; ("print", @@foo) > <io@std.com>` will
print out `"Hello, World!"`.

### Indexing
`[n]` - Can get an element from a tuple, or a character from a string.

### Slicing
`[n:m]` - When used on a tuple, it returns a new tuple with elements in
the range [n, m). When used on a string, it returns a substring from n inclusive
to m exclusive. Examples: `"hello"[1:4]` returns `"ell"`, and
`("a", "b", "c", "d")[1:3]` returns `("b", "c")`.

### Assignment
`=` - Assigns a variable in the user's environment to the value on
the right hand side. Note that the variable being assigned to should be a
string or identifier, NOT a retrieval operator, e.g. `@foo = "bar"` might not
work; instead, use `foo = "bar"`. It is, however, possible to use the retrieval
operator on the left side like this:
`foo = "Hello!"; bar = "foo"; @bar = "World!";("print", @foo) > <io@std.com>;`,
which will print `"World!"`

### Modifier
`|` - Takes the value on the left and modifies it based on the value
to the right. More on modifiers later in this document.

## User definition
When defining a user, typically a block is placed after the username that is
used to give functionality to a user. This block contains a list of subject
regexes and blocks of code. When an email is sent to a user, its subject is
tested, from top to bottom, against every regex in the user's definition. When
a match is found, it executes the block of code in that match. If none are
found, it fails silently.

## Modifiers
A modifier is an operator used to take a value and transform it into another
value. Modifiers take the form `value|modifier`, where 'value' is what is going
to be modified, and 'modifier' is how the value will be modified.

A modifier can be either a string or a tuple, while the value being modified
can be any type accepted by the given modifier.

### Chars
The 'chars' modifier takes a string and turns it into a tuple of characters.
For example, `"foo"|chars` results in `("f", "o", "o")`.

### Merge
The 'merge' modifier takes a tuple and merges it into a single string.
For example, `("foo", "bar", "baz")|merge` results in "foobarbaz".

### Filter
The 'filter' modifier takes a tuple of strings and keeps only strings that match
a given regex expression. For example,
`("foo", "bar", "baz")|(filter, "a")` will result in `("bar", "baz")`.

Note that the filter modifier takes a single additional argument. In this case,
the modifier MUST be a tuple, and nothing else.

### Chaining modifiers
Modifiers can also be chained together, for example
`"Hello, World!"|chars|(filter, "[^aeiou]")|merge` will result in "Hll, Wrld!".

## Standard Domain Library
The standard domain, `std.com`, contains many useful users who can perform
important functions.

### Input/Output
The user `<io@std.com>` contains functions for input and output.

#### Print
`print` - prints out the given message and all given attachments, separated by
a space.
For example,
```
(print, "Hello,", "World!") > <io@std.com>;
```
will print `Hello, World!`.

### Comparing
The user `<cmp@std.com>` contains functions used for different types of loops.

`eq` - Used to test if two values are equivalent.
`neq` - Used to test if two values are not equivalent.

### Math
Since Emailang does not (And can not by design!) operate on numbers, the math
library does it instead. The user `<math@std.com>` can be messaged in order to
perform basic mathematic operations. Math functions can only operate on
integers, floating point numbers such as '0.2' or '1e-5' will NOT work.

Every math function takes the form
`(operator, callback, op1, op2) > <math@std.com>`.

`add` - Adds two numbers together.

`mul` - Multiplies two numbers together.

`div` - Divides one number by another.

### Looping
The user `<loop@std.com>` contains functions for looping.

#### Iterate
`iterate` - Iterates through all attachments. For every attachment given, it
will send an email back to the sender with the same subject as the message that
was received.

Example:
```
!bar;
!<foo@bar> {
	"start" {
		("iterate", "each") + @content + @attachments > <loop@std.com>;
	}
	"each" {
		("print", @content) > <io@std.com>;
	}
};
("start", "A", "B", "C") > <foo@bar>;
```
Outputs:
```
A
B
C
```

## Internals
In general, the following set of operations are carried out every frame:

1. Add servers
2. Add users
3. Send emails

If an email is sent during a frame, said email will not be received until the
next frame.

The first frame, and the first frame only, will execute all of the code within
the main block. This is so that servers and users can be defined, and
initialization emails can be sent.

Emails that are sent consecutively one after another in the same block can be
assumed to be received in order; however, emails sent from different blocks at
the same time can not fall under the same assumption, even if you know the order
that these blocks are executed and are sure they are being executed in the same
frame.

Currently they are received in the order they are sent, but in the future,
multi-threading may be implemented which would break this.
