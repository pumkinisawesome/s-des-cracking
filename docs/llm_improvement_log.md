# Log of LLM interactions
Log of interactions with ChatGPT 3.5 and GitHub Copilot, including suggested
code, structural changes, and corrections and improvements I have made to the
things the LLMs have suggested.

## Vague plan/pseudocode

I started off by asking ChatGPT for some help in generating a vague plan of the
layout of my project, by asking this:

> **Me:**
> I'm going to be making a Rust project to crack the encryption of a file
> encrypted with S-DES encryption. Can you help me with this? I'm relatively
> new to Rust.
>
> I'd just like to start off with some pseudocode. What would be the best way
> of reading in the file?

I used some of its suggestions and some of my own ideas to build up a document
with some pseudocode to help me guide the directio of the project and decide
what to work on next.

## Reading in a file

Relevant ChatGPT chat log:
https://chat.openai.com/share/4e22da96-bc7d-4011-9bef-ede3ac4062ad

ChatGPT suggested a general way of reading a file into a String, and I asked
it if there was any way to read a file into an array or vector of bytes, and
which implementation would be best for the circumstance. It said that while an
array would be faster, it's size has to be known at compile time, and a vector
would be much more suitable given it's size can be determined during runtime.

I then made a test file to help me get the hang of the `std::fs::File` class
and how to use it with `std::io::{Read, Write}`. GitHub Copilot helped out a
little bit with suggestions, but I disabled it for most of the time as I wanted
to see how much I could do on my own at first. I reenabled it after a little,
and it helped me with my error handling for the `"File not found"`
circumstance, as Rust is quite strict with error handling.

I went back to ChatGPT for saving the `Vec<u8>` to a file, and it gave me some
boilerplate code. I used this to figure out the syntax and write my own
implementaton in my `testing_ground` directory.

(Side note, it's really nice how you can use `cargo` to construct Rust
projects, but you can also just create individual `.rs` files and run them as
scratch files, either using `rustc` to compile, or the Code Runner VSCode
extension)

## Information entropy

Relevant ChatGPT chat log:
https://chat.openai.com/share/0c9c5483-a53d-493d-a1b8-2476db34b65b

My next series of questions were about information entropy. I started off
asking ChatGPT if it could explain the mathematics of calculating information
entropy, and it gave me a formula. and a simple example. After a bit of
clarification of the meaning of high vs low entropy, I asked for some "very
general pointers" on how to get started on this in Rust, as I wanted to test my
growing Rust knowledge further.

Armed with these general pointers, I built the `calc_entropy` function with a
little bit of autocomplete from Copilot. I also added some tests that I could
compare against the examples that ChatGPT had given me, which I also double
checked with a calculator.

## Bit shifting and subkey generation

The relevant ChatGPT chat continues in the above log, as I didn't create a new
chat for this section.

After a couple more questions about array vs
vector, I asked it about how I can do bit shifting and rotating in Rust, and it
gave me some more example code.

I created a function to generate subkeys from a given key, and GitHub Copilot
helped me out hugely by providing almost the entire function. I edited some of
the implementation by moving the permutation tables out of the function to
become constants, but other than that, it didn't really need much editing.

## Project organisation and modules

Up until this point, all of my implementation (apart from my file read testing)
was all inside `main.rs`. As it was getting cluttered, I told GitHub Copilot
what I planned to add in the future, and asked if it thought it would make
sense to split my code into modules. As it could access my current files
already, it suggested I split my code as shown in the screenshot found at
`attachments/copilot_module_suggestion.jpg`.s

The screenshot mentioned above also shows how I talked back and forth to
finalise the module structure before implementing it, and how it helped me to
adapt my already written code so it could be moved into the corresponding
files. It tried to put my entropy calculations inside the same module as the
S-DES related code, but after being reminded that the entropy calculations
aren't specific to S-DES, it suggested I move it to a separate module.

## Full key generation

I did all most of this implementation myself, and added a function that would
generate all 1024 possible keys, and return them in a
`Vec<s_des::key_gen::Key>`. I also wrote some code in main to print out every
generated key, and also use `s_des::gen_subkeys()` to demonstrate my code
functioned correctly by using the specific example shown in the S-DES paper.

## File read and write continued

I asked GitHub Copilot for it's opinion on whether it would be a good idea to
put the file read/write code into its own module, and it agreed, mentioning
how Rust is very modular focused. I had a bit of a back and forth with it about
whether I should use relative or absolute module paths, and it gave me some
useful information on the pros and cons of both, and the reasons they're used
by programmers.

I made use of the new Copilot feature where it can answer questions based on
your entire project workspace, file structure and all, to ask it whether it
would suggest my project would be worth publishing on crates.io, and it's
response was interesting:

> **Me**
>
> @workspace Given I don't think I'll ever plan to publish my project as a
> crate, would it be easier to just use `pub` and not worry about it? Or maybe
> I should use `pub(crate)` where relevant, just in case I ever decide to
> publish this as a crate.
>
> With your judgement of this project so far, would you think this is something
> that would be worth publishing as a crate or not?
>
> **GitHub Copilot** used @workspace
>
> The decision to use `pub` or `pub(crate)` depends on how you want to
> structure the visibility of your modules and functions. If you use `pub`, the
> item will be accessible from any code that uses your crate, while
> `pub(crate)` makes the item public within your crate only.
>
> As for whether your project is worth publishing as a crate, it's hard to say
> without more context. The project seems to be a specific implementation of
> S-DES encryption cracking, which might not be broadly applicable as a library
> for other Rust projects. However, if you believe that others could benefit
> from using or learning from your code, it could be worth publishing.
> Remember, publishing a crate also comes with the responsibility of
> maintaining it, addressing issues, and potentially adding features.
>
> In general, it's a good practice to limit the visibility of your code as much
> as possible, and only expose what's necessary. This makes the code easier to
> maintain and understand. So, even if you don't plan to publish this as a
> crate, using `pub(crate)` where relevant could still be beneficial.

I decided that while I probably won't publish this, I'll still practice using
`pub(crate)` where relevant, to get in the habit of writing good code.

## Error handling

My next conversation with GitHub Copilot was about error handling. I
asked it about the best way to handle errors in the subkey generation, as
currently I just caused the program to panic if the key was larger than
10 bits. It suggested I use `Result` and `?` to handle errors, and I
implemented this in my code, and added this to some of my other functions
as well.

## File encryption and decryption

GitHub Copilot helped me out a lot with the implementation of the
`crypt.rs`file to begin with. I implemented the `encrypt` and `decrypt`
functions, and started the `crypt` function myself, then enabled Copilot
auto-complete, which provided the majority of the function. It would usually be
consistent with its formatting, but once or twice I would have to step in and
correct the way it handled the permutation tables, as it would sometimes try to
do them as a match statement instead of following the pattern I started and
saving each permutation table as a constant.

When I got to the S-boxes stage, Copilot attempetd to do them as a big match
statement, but I managed to get it to do them as a 2-d constant array instead,
fitting with my implementation of the permutation tables better.

When I got halfway through the implementation, to the stage where the steps
repeat with the nibbles swapped, Copilot attempted to repeat the steps, but I
opened the chat and asked it if it would be worthwile to separate this out
into a function, to be called twice with the nibbles swapped. It agreed, and
named its function `feistel_round`. When I asked why, it actually taught me
that the name for this was a "Feistel round", named after its inventor, Horst
Feistel.

Unfortunately, it's implementation of the Feistel round had a bug somewhere, so
I spent some time manually going through in the debugger using the example
given in the S-DES paper, and I found and fixed the point it deviated.

## Testing with a real file and entropy analysis... oops

I created a small test file, and did some testing by encrypting and decrypting
this file, and comparing the output to the original file. I confirmed that it
worked correctly. Then, I added in calculating the entropy of the file when it
was plain text, and when it was encrypted. I was expecting the entropy to be
different, but realised that there would be a problem with this method of
determining that the encryption had been a success.

## Shakespeare character frequency analysis experiment

After talking with Clint, the project guidelines were changed, so that we were
now to use character frequency analysis to determine the correct key.

I used GitHub Copilot to write me a program in my `testing_ground` directory to
take in a file, and output the frequency of each character in the file, which
it did so completely correctly, and I guided it to modify the file slightly to
better fit my needs. I fed it Shakespeare's "Romeo and Juliet" to generate a
frequency table for the English language that I could compare my encrypted file
to.

## Redesigning main

I now had all of the pieces I needed to be able to write the whole proram's
functionality, all I had to do was implement the code from character frequency
code from `testing_ground/ascii_frequency` into the main project and combine
all of the pieces together. I decided that I would write the code to combine
all of the pieces together first, so I rewrote main to use my functions in the
way I laid out in `docs/pseudocode_plan.md`. For the character frequency
functions that I hadn't implemented yet, I just called nonexistent functions,
for me to implement later.

## Character frequency analysis

At first, I tried using GitHub Copilot to implement the character frequency
analysis into my project, but it didn't fully understand my project layout and
led to some bugs, so I removed it and rewrote it myself, using the code it
generated in `testing_ground/ascii_frequency` as a reference, and asking
Copilot for syntax help in specific circumstances.

## Testing with a real file and character frequency analysis

I wrote some scratch code in main to encrypt a file with a chosen key (of
course I had to use `keys[42]`...), then removed the scratch code and ran the
project to test decrypting the file. After fixing a little mistake I made with
reading in the Shakespearean character frequency file (instead of reading in
the data stored in that file, I was analyisng the frequency of the characters,
so the frequency distribution I was attempting to match was only made up of
numerical characters and the character ":"), I ran the program and it
successfully decrypted the file in less than a second!

## Redesigning main again

The project was now technically complete, but I wanted to make it much easier
to use and a bit more user friendly. I redesigned main to accept command line
arguments, which would tell it to either generate an encrypted file from
plaintext using a random key, or crack the encryption on an encrypted file and
generate a decrypted file.

The project was finally complete!