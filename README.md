# Krabcake

This is currently demonstration code still very much under construction.

## How to build

Clone this repo including submodules.

Build with `make go -j128`

## UI Tests

UI tests are handled by the `kc` crate.

For example, you can simply use:
```
cargo run
```
inside the `kc` `directory to run all the tests in `kc/tests`.

If you need to bless the output:
```
cargo run -- --bless
```

## What is this Krabcake

(See also presentation from Rust Verification Workshop 2023 https://hackmd.io/@pnkfelix/rkx2G0W7n#/ )

Rust promises that safety violations cannot arise from safe code. But what about unsafe Rust code? Since 2018, Rust users have used Miri, a Rust code interpreter, to validate isolated snippets of unsafe code. Krabcake provides the same safety checks provided by Miri, while making those checks available for large-scale programs that depend on arbitrary C/C++ libraries, or even inline assembly, both of which are unsupported by Miri.

### Miri's limits

If you try to apply Miri to a program that calls an arbitrary foreign function
(for example a tailored sort routine, `mysort`), Miri will issue an error
saying that it cannot invoke that foreign function.

Miri’s architecture fundamentally does not support arbitrary foreign functions.

Therefore, leveraging Miri introduces new constraints on your software architecture:

* Any unsafe code you want to check cannot use inline assembly nor foreign functions, except for those built into Miri ([Standard C shims][], [Unix shims][]).
* Furthermore, one’s test suite must similarly avoid using any functionality unsupported by Miri.

Depending on your application domain and development workflow, either of the two above bulletpoints could represent a severe hindrance.

[Standard C shims]: https://github.com/rust-lang/rust/blob/c50c62d225e004f5b488006d3d205a34363a128c/src/tools/miri/src/shims/foreign_items.rs#L527
[Unix shims]:       https://github.com/rust-lang/rust/blob/c50c62d225e004f5b488006d3d205a34363a128c/src/tools/miri/src/shims/unix/foreign_items.rs#L27

Leveraging Miri requires working around these problems; it is not a “plug-and-play” experience.

### Miri’s limitations are Krabcake’s strengths

Krabcake operates on a compiled binary that is linked to native code. Krabcake carries neither of the two limitations listed above for Miri:

* Krabcake handles inline assembly
* Krabcake handles functions that invoke foreign libraries

## Krabcake design overview

Krabcake builds upon the Valgrind architecture, with a custom Valgrind tool that handles the checks that are specific to Rust's memory model.

The overall long-term picture is this:

1. The developer turns on a flag on the Rust compiler that enables Krabcake Sanitation (KSAN) mode. Enabling Krabcake Santitation mode has no effect on the dynamic semantics of the program apart from injecting special sequences of no-op's (see "Valgrind Client Requests" below). The Rust crate can be linked with non-Rust code in the application being constructed and executed.
2. The developer runs the program atop the Valgrind Krabcake tool. The program is executed, but the tool dynamically rewrites the program to now include dynamic checking of every creation and use of a Rust reference (i.e. `&T` or `&mut T`) to ensure that each reference has permission to access its corresponding memory location at its time of access.

### Valgrind details

Valgrind works by dynamically rewriting appliction binary code (aka the "client machine code") at runtime;
it:

1. parses the client machine code (e.g. x86_64, ARM),
2. translates it to a common intermediate representation (called VEX)
3. allows the specific tool (Krabcake in our case) to perform arbirtrary VEX-to-VEX transformations
4. translates the transformed VEX back to the machine code, and finally
5. resumes execution now via the dynamically generated machine code atop the host machine.

Notably, the steps above are performed not only when the program is first loaded, but also whenever any code is loaded dynamically. In other words, the execution that resumes in step 5 can eventually call a dynamic loader (e.g. `dlopen`) , and that will cause Valgrind to run from step 1 on that dynamically loaded code.

The original machine code can include special machine code sequences, called
"Valgrind Client Requests." These machine code sequences have no semantic effect when the application  is run apart from Valgrind (they are effectively a special sequence of no-op instructions), but when run atop Valgrind, the client requests get converted into calls to procedures that have been defined at the host level in the specific tool (Krabcake in our case).

### Translation details

Krabcake's goal is to validate the program's behavior against the Rust memory model. To do this, we need to extend the standard machine code semantics in two ways:

* Each memory location that is ever borrowed (i.e. is the target of a Rust reference, `&` or `&mut`) needs its own extra state (an  *item-stack* in the [Stacked Borrows][] model)
* Each Rust reference, or pointer value derived from a Rust reference, needs a *tag* (a kind of timestamp) that tracks when that reference was created.

[Stacked Borrows]: https://plv.mpi-sws.org/rustbelt/stacked-borrows/

Note: It is not sufficient to just maintain a global mapping from addresses to tags; the whole point of the Stacked Borrows model is that you may end up
with distinct references that contain *pointers to the same address* but are totally different in terms of where they came from and whether they are currently
allowed to be used.

Luckily, Valgrind's infrastructure makes it feasible to inject code that will build and maintain these two pieces of state.

A large part of Krabcake's VEX-to-VEX transformation is the construction and maintenance of such tag and stack values, represented as *shadow state*.

Any part of the machine state that can carry an address (i.e. a register or a memory cell) is now potentially mapped to a tag (via a partial functional map,
currently represented as a key/value table), and operations that consume those addresses (such as basic mathematical operations which may be performing pointer
arithmetic) likewise pass those tags alongside into their computed results.

Similarly, every memory address is also potentially mapped to an item-stack by a similar key/value table. Maintenance of this state is much simpler than that of tags,
because these stacks do not flow around during computations the same way that tags do.


### Client request details

Krabcake's goal is to validate the program's behavior against the Rust memory model. This requires knowledge of where the operations `&mut` and `&` are performed
as the program runs. However, this presents a problem: A typical Rust application binary, since it has been compiled, has *erased* the distinction between `&mut` versus `&`, turning them both effectively into `&raw` address-computations, with no record of what kind of reference was being constructed.

In order to check the memory model faithfully, Krabcake needs to know for each such address computation whether it was a `&mut`, a `&`, or something else (presumably a native `&raw` address-of operation). 

The Krabcake Sanitation (KSAN) mode of the `rustc` compiler is what provides that knowledge of `&mut`/`&` computations (as well as other operations that are significant to the memory model, such as casts from Rust references to unsafe pointers like `*const` or `*mut`). As an example, for each `&mut` and `&` that we find in the MIR, KSAN injects a Client Request that signals to Valgrind that the given memory location has been the subject of a `&mut`- or `&`-borrow.

### Test Normalization details

In order to write tests, we want output from Krabcake that will remain stable
despite details that change between different hosts and platforms such as the
concrete memory addresses that have been allocated.

Therefore, there is an opt-in normalizing mode that maps each memory address to
a normalized value, where that value is derived from a deterministic counter.

### Project Layout

While Valgrind is very much a C project, the Krabcake developers are Rust programmers. Therefore, we opted to try to put as much of our code into Rust as we could.

Project layout diagram:

```
* Krabcake: the root of the project
|
+-- README: this file
|
+-- krabcake-vg/ (git submodule): fork of Valgrind that carries krabcake tool
|   |
|   +-- coregrind/: C source code for Valgrind's shared core logic
|   |
|   +-- krabcake/: source code for the Valgrind Krabcake tool.
|       |
|       +-- rs_hello/: Rust code statically linked to the Valgrind krabcake tool
|       |
|       +-- kc_main.c: C source for tool
|
+-- kc/: this is a Rust crate that holds our regression tests for Krabcake
|   |
|   +-- src/: source code for invoking runner.py atop the `ui_test` crate
|   |
|   +-- runner.py: the Python code that drives each test invocation
|   |
|   +-- tests/: regression tests for Krabcake; each is compiled atop rustc and then run atop valgrind, checking the normalized output against the provided stderr file.
|   |
|   +-- test_dependencies/: utility macros and methods for interacting with the Valgrind Krabcake tool, such as `kc_borrow_mut!` (see "KSAN is not yet done" below) or `print_tag_of`.
|
|
|              (valgrind build products are installed into the locations below.)
+-- bin/     : `valgrind` is here, as well as other utilities
+-- libexec/ : The architecture-specific tools (e.g. krabcake-amd64-linux, krabcake-x86-linux) are here.
+-- include/ : Various C header files are here
+-- lib/     : Various static archives are here
|
+-- krabcake-rustc/ (not yet done): this is where the fork of rustc that holds the KSAN support will go.
```

The main thing we want to emphasize about the above diagram is that the bulk of development should happen in one of four places:

* `krabcake-vg/krabcake/rs_hello`: this is where the "business logic" resides. It handles the core checks (e.g. that an access via a `&mut`-reference actually has the right to perform that operation). It also handles creating initial shadow state values (and computing new shadow state values from existing ones), and all other Krabcake client request implementations (for example, we currently have operations that allow one to extract the tag associated with a pointer-value, or the item-stack associated with a memory address)
* `krabcake-vg/krabcake/kc_main.c`: contains 1. entry point, 2. the VEX-to-VEX transformation logic (whose generated code sometimes include calls dispatching to functions in `rs_hello/`), and 3. a client request shim that dispatches to routines in `rs_hello/`.
* `kc/test_dependencies/`: the macros and utility methods here provide a way for the regression tests to communicate with the Valgrind Krabcake tool. They follow procotols are shared with the client requests in `rs_hello/`.
* `krabcake-rustc/`: not yet provided (see "KSAN is not yet done" below), but will eventually be a fork of `rustc` that will inject the client request interactions that are today instead provided manual invocations of macros and methods provided by `kc/test_dependencies/`.

### KSAN is not yet done

We have not yet made a fork of `rustc` that carries the Krabcake Sanitation support code.

(It is under development, see e.g.: https://github.com/bryangarza/rust/pull/1 )

So, instead of relying on a compiler to inject all of the necessary client request invocations,
we are instead injecting them ourselves. Namely, our test suite uses macros like `kc_borrow_mut!(expr)`
everywhere that you would expect to see `&mut expr`. This way, we can still make progress on our
tool against some simple test cases, before we tackle the much bigger problem of trying it out
against a fully sanitized build of the Rust standard library.

## Project Tenets

### No false positives

What it means: If the Krabcake tool says your program has undefined behavior, then it does (or you have found a bug in Krabcake).

Why this matters: We want people to treat the output of the tool seriously, and not try to handwave it away with “well this is just catching behavior that is correlated with bugs, but in this case I argue that this behavior is not an issue.”

### Recompile and run

What it means: You do not need to change your source code to leverage Krabcake; you do not even need to disable release mode.

You just do cargo krabcake run, and Krabcake handles the rest (namely: recompiling the program with the appropriate extra rustc flags, and then running it atop valgrind --tool=krabcake).

However, it is certainly expected that Krabcake will catch more instances of undefined behavior from your source code if you do disable optimizations (one might even go further and use -Zmir-opt-level=0, hypothetically?)

Why this matters: A tool that requires people to edit their source code is harder to adopt.

### Embrace incompleteness

What it means: Part of our goal is to operate on programs that have been subject to compiler optimizations. In that context, we cannot provide any guarantees to catch undefined behavior, because a compiler is free to convert a program with undefined behavior into a program that has any behavior at all.

### No unexplored territory

What it means: Every check put into Krabcake has already been prototyped in the context of Miri.

Why this matters: We will be far more confident that the checks provided our tool are the ones that represent real soundness issues if we build atop the work already done in Miri. Furthermore, applying these translated checks to a larger body of "real world" code gives us a big opportunity to test whether these checks are acceptable to those developers out in the "real world."
