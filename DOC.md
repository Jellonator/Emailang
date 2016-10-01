Document of things in Emailaing:

### Data types
* Null (Not a value)
* String (String of characters)
* Tuple (List of values)
* User (a Username and a Domain)
* Expression (evaluates an instruction)

A tuple with a single value can be constructed by adding a single comma after
the value, e.g. `("foo",)`. An empty tuple can be constructed with `(,)`.

### Operators
The following operators exist:

`>` - Send an email. Uses a tuple or string on the left as a draft for the
email, and sends it to the user on the right.

`+` - Concatenation. If both the left and right values are strings, it will
concatenate these two strings together. If one or both of them are a tuple, it
will concatenate them as a tuple. E.g., `"a" + ("b", "c")` results
in `("a", "b", "c")`

`@` - Technically an operator. Takes the identifier to the right and retrieves
the value of the environment variable of that name. If instead of an identifier
a tuple is given, this operator will return a tuple with all of the values of
the identifiers in the environment, e.g. `@("message", "subject")` will evaluate
to `(email message, email subject)`.

### Internals
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
