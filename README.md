# Emailang
Emailang version 1.0.0

A programming language based on emails.

This is the perfect programming language for people who are tired of using fax
machines and Cobol, and think that emails are a great place to get started in
the modern age.

Never has there been a programming language more enterprise than this; The
officialness of email combined with the enterprise solutions that come with
object-oriented design, all while discarding useless, dated conventions found
in other popular languages such as 'states,' 'variables,' and 'functions,' and
replaces them with a system of the modern era: emails.

## About
Emailang is a programming language based on sending emails from one object to
another. Is it object oriented? Is it a functional language? I don't know. It
just is.

## Basic syntax
This section is to explain how to use Emailang.
All statements must end with a semicolon `;`.

### Types
Like many languages, Emailang includes a basic type system. The basic
types are as so:

* String - A sequence of characters, e.g. `"foo"`
* Tuple - A sequence of other data types, e.g. `("Foo", "Bar", "Baz")`
* User - A name and domain which refers to a user, e.g. `<foo@bar.com>`

### Creating servers and users
In order to use Emailang, a user must first be created. To create a user, a
server for that user to exist in must also be defined.

To define something, the `!` symbol is used.

Creating a server can be done as so (The .com isn't necessary, but is reccomended for convention):
```
!servername.com;
```

And to create a user (note that the server MUST exist in order to do this):
```
!<username@servername.com>;
```

However, this defines an empty user which doesn't do a whole lot. Sending an
email to this user would do nothing. In order for a user to do something, it
needs to be able to receive emails. to do this, we put a block after the
definition of the user like so:
```
!<username@servername.com>{
	"^foo$" {

	};
	"^bar$" {

	};
};
```

Now this user can receive emails with the subjects 'foo' and 'bar'. Note that
subjects are matched by Regular Expression, not just simple comparison. It
doesn't do anything upon receiving these emails, but it does receive them at
this point.

### Sending mail
Now that we have a user, how do we send mail with it?

Easy, we use the `>` symbol to send mail.

In order to send mail to a user, we can do it like so:
```
("subject", "content", "attachment1", "attatchment2") > <username@servername.com>;
```

Note that emails are 'constructed' from tuples. The `>` operator takes a tuple
as a draft, adds some extra metadata, and sends it to the user.

This does nothing to our current existing user because our user does not handle
subjects with the name 'subject'; it only handles the subjects 'foo' and 'bar'.
If we want to send message our user can handle, we instead must use one of these
valid subjects, like so:
```
("foo", "Have a great day!") > <username@servername.com>;
("bar", "Important document", "Nuclear launch code: 12345") > <username@servername.com>;
("foo", "How was your weekend?") > <username@servername.com>;
```

Our user is now receiving this mail, but it isn't doing anything with it yet.
We still aren't getting any messages printed to the screen.

### Standard domain
In order to print something to the screen, we need to be able to use the
standard library. All standard functions are defined in the server 'std.com'.
Users in the standard domain represent different submodules of the standard
domain, for example `<io@std.com>` handles input/output based functions.

To use the standard library, an email must be sent to the standard library. The
subject of email refers to the specific function in the library to use. To
print something to the terminal, an email with the subject "print" must be sent
to the user `<io@std.com>`, for example:
```
# Tell the io standard library to print "Hello, World!" to the screen.
("println", "Hello,", "World!") > <io@std.com>;
```

Note that attachments will also be printed.

### Adding functionality to a user
Now that we know how to print things to the screen, we can now implement the
standard library into our user.

A quick note before we get started, the blocks of code inside our user have what
is called an 'environment.' The environment is what contains all of the
information sent to the user in the email.

Data can be retrieved from the environment using the `@` symbol,
e.g. `@content` will retrieve the content of the message. The following environment variables can also be accessed:

* `@subject` - The subject of the email
* `@content` - The content of the email
* `@attachments` - Tuple containing all attachments in the email
* `@self` - Userpath referring to the user containing it.
* `@sender` - Userpath referring to the user who sent the email.

So now that we can retrieve information from an email, we can finally implement
this into our user:
```
!<username@servername.com>{
	"^foo$" {
		("println", "Received mail:", @content) > <io@std.com>;
	};
	"^bar$" {
		("println", "Received classified information!") > <io@std.com>;
	};
};
```

Our user can now receive and prints information when the subject is "foo", and
tells us that some classified information was received when the subject is
"bar".

### Conclusion
Our final code will now look like this:
```
!servername.com;
!<username@servername.com>{
	"^foo$" {
		("println", "Received mail:", @content) > <io@std.com>;
	};
	"^bar$" {
		("println", "Received classified information!") > <io@std.com>;
	};
};
("foo", "Have a great day!") > <username@servername.com>;
("bar", "Important document", "Nuclear launch code: 12345") > <username@servername.com>;
("foo", "How was your weekend?") > <username@servername.com>;
```

And when we run it in the terminal:
```
Received mail: Have a great day!
Received classified information!
Received mail: How was your weekend?
```
