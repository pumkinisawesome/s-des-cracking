# S-DES cracking Rust pseudocode

1. ~~Initialise the encrypted file~~
   1. ~~Load file into Vector of bytes~~
2. ~~Initialise range of possible keys~~

   ~~Repeat for each key in the range:~~
   1. ~~Generate key~~
   2. ~~Generate subkeys from key~~
3. For each key in the range:
    1. Decrypt the file using the key
    2. Calculate the information entropy of the decrypted file
    3. If the calculated entropy is lower than the current lowest entropy:
       1. Update the lowest entropy
       2. Update the best key to the current key
4. Output the best key and the decrypted file