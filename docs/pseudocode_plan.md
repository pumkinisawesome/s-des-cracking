# S-DES cracking Rust pseudocode

1. Initialise the encrypted file and range of possible keys
2. For each key in the range:
    1. Decrypt the file using the key
    2. Calculate the information entropy of the decrypted file
    3. If the calculated entropy is lower than the current lowest entropy:
       1. Update the lowest entropy
       2. Update the best key to the current key
3. Output the best key and the decrypted file