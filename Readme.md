# Readme:

## Goal:
Goal of project was to:
1. Read text from file
1. Change text to bytes
1. Generating kay of proper length
1. Encrypting loaded text using generated key
1. Decrypting loaded text using generated key
1. Check if compatibility of files

## Functions:
 *  `Load Message from file` => Loading text from file, and caching that message.
 *  `Turn Message to bytes` => Turining text to bytes just for show, as it was required `¯\_(ツ)_/¯`.
 *  `Generate key` => Generating random char vector of length of cached message and caching key.
 *  `Encrpyt Loaded Message` => Using generated key to encrypt message.
 *  `Decrypt Loaded Message` => Using generated key to decrypt message.
 *  `Check compatibility of two files using key` => reading from two files and checking their content using key.
 *  `Readme` => Show readme on console.
 *  `Clean cache` => Cleaning cached values that maybe loaded message or key.
 *  `Finish` => Closing program.


## Example:
We have file with name `to_encrypt.txt`. 
### Encrypting:
```
1. Load Message from file
2. Turn Message to bytes
3. Generate key
4. Encrpyt Loaded Message
5. Decrypt Loaded Message
6. Check compatibility of 2 files using key
7. Readme
8. Clean cache
0. Finish
1
Type filename of 'Message'.
to_encrypt.txt
Message is 'Confidential information'
1. Load Message from file
2. Turn Message to bytes
3. Generate key
4. Encrpyt Loaded Message
5. Decrypt Loaded Message
6. Check compatibility of 2 files using key
7. Readme
8. Clean cache
0. Finish
3
Generating Key
Key is '941812061486415425166671818921433945016181204244391315479165'
1. Load Message from file
2. Turn Message to bytes
3. Generate key
4. Encrpyt Loaded Message
5. Decrypt Loaded Message
6. Check compatibility of 2 files using key
7. Readme
8. Clean cache
0. Finish
4
Type filename of result file of encrypting.
encrypted.txt
Succesfully encrypted
```
### Decrypting:
```
1. Load Message from file
2. Turn Message to bytes
3. Generate key
4. Encrpyt Loaded Message
5. Decrypt Loaded Message
6. Check compatibility of 2 files using key
7. Readme
8. Clean cache
0. Finish
1
Type filename of 'Message'.
encrypted.txt
Message is `↔ڠ�)��,7{ܺ☺7\vھ�F�_ �`
1. Load Message from file
2. Turn Message to bytes
3. Generate key
4. Encrpyt Loaded Message
5. Decrypt Loaded Message
6. Check compatibility of 2 files using key
7. Readme
8. Clean cache
0. Finish
5
Type filename of result file of decrypting.
decrypted.txt
Confidential information
Succesfully decrypted
```
### Checking compatibility:
```

1. Load Message from file
2. Turn Message to bytes
3. Generate key
4. Encrpyt Loaded Message
5. Decrypt Loaded Message
6. Check compatibility of 2 files using key
7. Readme
8. Clean cache
0. Finish
6
Type filenames of 'encrypted_file decrypted_file'.
encrypted.txt decrypted.txt
Files are matching
1. Load Message from file
2. Turn Message to bytes
3. Generate key
4. Encrpyt Loaded Message
5. Decrypt Loaded Message
6. Check compatibility of 2 files using key
7. Readme
8. Clean cache
0. Finish
0
```