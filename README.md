# Building Better Interfaces

A set of code that demonstrates how to create interfaces that take advantage of existing Rust naming conventions.

All code was tested on stable Rust. As of this writing, stable Rust version `1.12.1`.

## Workshop Requiresments

* You will need a laptop with Rust installed if they want to participate in the exercises.

## Introduction To Owned and Borrowed Types

[Slides](https://docs.google.com/presentation/d/1kH5xXvHj9CU1_OfYXM4oaeo3VVzAyhUlcW9LNirCQwI/edit?usp=sharing)

## Dicussion

### Owned/Borrowed Types Code Walkthrough

* Introduce `NameString` type. Branch: name-string-1
   * Show how to create one using `NameString::new`.
   * Show how to modify one using `NameString::push`.
* Show `NameString::from_str()`. Branch: name-string-2
* Show `NameString::uppercase()`. Branch: name-string-3
   * The point here is that we can mutate the existing NameString because we own it. We can create a mutable reference too, but we do not own that.
* Introduce `NameStr` type. Branch: name-str-1
* Relate `NameString` to `NameStr`. Branch: relate-1
  * Show how to get one using `NameString::as_name_str` function.
    * Using the `as_*` convention.
  * Show how to get a `&mut NameStr` using `NameString::as_mut_name_str()`
  * Show how to get `NameString` using `&NameStr`.
* Convert a NameString into a `String` and `&str`. Branch: relate-2
  * Show how to create `NameString::into_string()`.
  * Show how to create `NameString::as_str()`.

### Introducing Traits

* Make a more generic version of `NameString::as_name_str`. Branch: name-str-as-ref
  * Show how to create `NameString::as_ref()`.
  * Also, Show how to create `NameString::as_mut()`.
* Create a `Roster` type that can accept `&NameStr` or `&NameString`. Branch: name-string-deref
  * Show how to create Deref.
  * Also, show how to use DerefMut trait if we needed it to be mutable.
  * Now go back and update `NameString::as_ref()` and `NameString::as_name_str()`.
  * Now go back and update `NameString::as_mut()` and `NameString::as_mut_name_str()`.
  * Show how Deref also let's us share methods.
* Convert between `NameString` and `NameStr`. Branch: name_string-from-into
  * Show how to create `NameString::From<NameStr>()`.
  * Show how to create `NameStr::Into<NameString>()`.
  * Show how to create `NameString::From<String>()`.
  * Show how to create `NameString::Into<String>()`.
* Create a `Person` type that can accept `&NameStr` or `NameString`. Branch: person-into
  * Show how to use the `I: Into<NameString>` generic parameter.

### Exercises

* Branch: exercise-1
* play.rust-lang.org link: https://is.gd/BCK4eC

0. Convert a `NameString` into a mutable `str` reference.
  * Answer: `NameString::as_mut_str()`
0. Convert a `NameStr` to a `String` type. Extra credit: Do it using the Into trait.
  * Answer: `NameStr::to_string()`
  * Extra credit: `NameStr::Into<String>()`
0. Create a `Courses` type that can accept `String` or `&str`.

### Copy-on-write (Cow)

* Write a function to uppercase from `NameStr`. Branch: name-str-to-uppercase
  * We need to implement `ToOwned`.
    * Oops! We need to implement `Borrow`.
    * Let us also implement `BorrowMut`.
  * Show how to create `NameStr::to_uppercase() -> Cow<'a, NameStr>`.
* Update our Roster type to require names be uppercase. Branch: roster-require-uppercase
  * Show how to store `Cow<'a, NameStr>` in `Roster`.

### Try, With and Wrapper Types

* Interact with a domain requirement that expects a family (last) name with 1 or more characters. Branch: name-try-into-from
  * Show how to implement `Name::try_from()`. Note the trait is still unstable.
  * Show how to also implement `NameString::try_into()`.
* Create a classroom that can initialized as empty or with a list of names. Branch: classroom-with-names
  * Show how to implement `Classroom::with_names()`.
* Create a `Roster` from a `Classroom`. Branch: classroom-as-roster
  * Show how to create `Classroom::as_roster()`. We use the `as_*` convention here.
* Safely change state of a `Student` from `InClass` to at `Recess`. Branch: student-states
  * Create a `Student` type, an `InClass<T>` type and a `Recess<T>` type.
  * Show how to use Deref to allow the `InClass` and `Recess` types to call methods on `NameStr`.

### Exercises

0. Write a lowercase function for `NameStr`.
0. Implement `Roster::with_names()`.
0. Add a new state `Home` and implement it for Student.
