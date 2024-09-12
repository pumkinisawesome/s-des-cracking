# S-DES Cracking

This is a project I worked on for my Cybersecurity class. The assignment was 
to write a brute-force cracking program for the S-DES algorithm, and we were
asked to write it in Java. However, I had already written quite a lot of Java,
and I was teaching myself Rust at the time, so I asked my professor if he
might let me write my program in Rust, and he let me! 

I wrote this program as a double-edged experiment. Firstly, to give myself
an opportunity to try out Rust and see how I like it (spoiler: I love it), and
secondly, to experiment with the use of AI as an assistant to programming.

## Use of AI

I used GitHub Copilot and ChatGPT 3.5 throughout this project, but I
specifically didn't ask them to write code for me. Instead, I would query them
on questions of implementation in Rust. For example:

- I would come up with ideas on how I would like to structure my algorithm,
  and then query Copilot on how what Rust libraries might help me accomplish
  my goals.
- I would ask if there were ways I could refactor my code to fit industry
  standard formatting.
- I would ask Copilot how similar programs are sometimes structured, and then
  figure out myself how I could adapt those structures and make my code more
  modular and easier to read to other people, and keep it easily extendable
  for the future.

A full explanation of my discoveries and experiments with AI in this project
is available in `docs/llm_improvement_log.md`.

## Assignment Overview
Write a program that inputs a file encrypted by S-DES, and \*automatically\*
determines the S-DES key used, and the cleartext of the file.  Do this by
decrypting the file data with each possible key until one produces cleartext.
(There are only 1024 possible 10 bit keys).  BUT, do not require human
feedback regarding whether cleartext has been produced.  Instead, determine
the information entropy of each "decrypted" version, and use that with the
lowest entropy as "cleartext".  This will both make the program independent
of human labor, and also able to decrypt different languages.  (Some of the
test files may be encryptions of non-English text.)

### Other important points:

1. Implement in Java.

2. Information entropy must be calculated by code you build (possibly with LLM
   assistance, see below), not a library.

3. You are expected to rely on an LLM to generate initial versions of your
   program.  (GPT-4 is quite familiar with S-DES, for instance).  However,
   your final product must be bug-free, clear, and elegantly concise.
   Translation: the LLM's output won't be good enough.   Also, you must keep a
   log of improvements and corrections you made to the LLM output, including
   bugs fixed and code conciseness improvements.   This log will be a major
   part of your grade.

## Submission
Submission takes the form of demoing for me a working program on data sets
I'll supply, and talking with me about your code-improvement log and your
general process for using the LLM.  Ultimately, you submit a style-correct
version of the program and your log, after I've OKed both.
