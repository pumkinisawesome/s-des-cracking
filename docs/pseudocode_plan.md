# S-DES cracking Rust pseudocode

1. ~~Initialise the encrypted file~~
   1. ~~Load file into Vector of bytes~~
2. ~~Initialise range of possible keys~~

   ~~Repeat for each key in the range:~~
   1. ~~Generate key~~
   2. ~~Generate subkeys from key~~
3. For each key in the range:
    1. Decrypt the file using the key
    2. Calculate the frequency of each byte in the decrypted file
    3. Calculate the euclidean distance between the frequency of each byte in
        the decrypted file and the frequency of each byte in the English
        language
    4. If the euclidean distance is the lowest so far:
       1. Update the lowest euclidean distance
       2. Update the best key to the current key
4. Output the best key and the decrypted file