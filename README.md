# S-DES Cracking

## Assignment description

Write a program that inputs a file encrypted by S-DES, and *automatically* determines the S-DES key used, and the cleartext of the file.  Do this by decrypting the file data with each possible key until one produces cleartext.  (There are only 1024 possible 10 bit keys).  BUT, do not require human feedback regarding whether cleartext has been produced.  Instead, determine the information entropy of each "decrypted" version, and use that with the lowest entropy as "cleartext".  This will both make the program independent of human labor, and also able to decrypt different languages.  (Some of the test files may be encryptions of non-English text.)

### Other important points:

1. Implement in Java (or Rust if you're Jamie).

2. Information entropy must be calculated by code you build (possibly with LLM assistance, see below), not a library.

3. You are expected to rely on an LLM to generate initial versions of your program.  (GPT-4 is quite familiar with S-DES, for instance).  However, your final product must be bug-free, clear, and elegantly concise.  Translation: the LLM's output won't be good enough.   Also, you must keep a log of improvements and corrections you made to the LLM output, including bugs fixed and code conciseness improvements.   This log will be a major part of your grade.

Submission takes the form of demoing for me a working program on data sets I'll supply, and talking with me about your code-improvement log and your general process for using the LLM.  Ultimately, you submit a style-correct version of the program and your log, after I've OKed both.