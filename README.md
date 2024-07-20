# md2_digest
A 0 dependency, no_std library implementing the md2 hashing function.

## Motivation
I wanted to learn how the md2 hash function works.
It was fun trying work out how to write this in a const context.
Explaining how it works by writing it down in steps aided the understanding of the algorithm.
There can be multiple ways to implement something and alternative approaches can be taken.

## Usage
It is no longer recommended to use md2 hashes for any sensitive information.\
This library is functional but you are very likely better off using a more modern alternative hashing algorithm.

## How md2 works
Input is any number of bytes.\
Output is always exactly 16 bytes.

You can break down creating the md2 digest into steps which themselves can be broken down further.\
Here is a high level overview:
1. Append padding
2. Append checksum
3. Generate digest

Both the checksum and the digest step use a fixed array of 256 bytes that are chosen from the digits of PI to provide a form of randomness.\
Here is the PI based array:
```
[
  41, 46, 67, 201, 162, 216, 124, 1, 61, 54, 84, 161, 236, 240, 6,
  19, 98, 167, 5, 243, 192, 199, 115, 140, 152, 147, 43, 217, 188,
  76, 130, 202, 30, 155, 87, 60, 253, 212, 224, 22, 103, 66, 111, 24,
  138, 23, 229, 18, 190, 78, 196, 214, 218, 158, 222, 73, 160, 251,
  245, 142, 187, 47, 238, 122, 169, 104, 121, 145, 21, 178, 7, 63,
  148, 194, 16, 137, 11, 34, 95, 33, 128, 127, 93, 154, 90, 144, 50,
  39, 53, 62, 204, 231, 191, 247, 151, 3, 255, 25, 48, 179, 72, 165,
  181, 209, 215, 94, 146, 42, 172, 86, 170, 198, 79, 184, 56, 210,
  150, 164, 125, 182, 118, 252, 107, 226, 156, 116, 4, 241, 69, 157,
  112, 89, 100, 113, 135, 32, 134, 91, 207, 101, 230, 45, 168, 2, 27,
  96, 37, 173, 174, 176, 185, 246, 28, 70, 97, 105, 52, 64, 126, 15,
  85, 71, 163, 35, 221, 81, 175, 58, 195, 92, 249, 206, 186, 197,
  234, 38, 44, 83, 13, 110, 133, 40, 132, 9, 211, 223, 205, 244, 65,
  129, 77, 82, 106, 220, 55, 200, 108, 193, 171, 250, 36, 225, 123,
  8, 12, 189, 177, 74, 120, 136, 149, 139, 227, 99, 232, 109, 233,
  203, 213, 254, 59, 0, 29, 57, 242, 239, 183, 14, 102, 88, 208, 228,
  166, 119, 114, 248, 235, 117, 75, 10, 49, 68, 80, 180, 143, 237,
  31, 26, 219, 153, 141, 51, 159, 17, 131, 20
]
```

### 1. Padding
Padding is always applied.\
After appending the padding to the input the total amount of bytes must be divisible by 16.\
If the input is already divisible by 16 before padding then 16 bytes of padding are appended.\
The value of the padded bytes is always the same as the amount of bytes added.\
Minimum padding would be 1 byte of value 1.\
Maximum padding would be 16 bytes of value 16.

### 2. Checksum
The checksum is 16 bytes and is appended to the output of the padding step.\
The input is the result of the previous step (original input + padding) and is divisible by 16.\
To calculate the checksum perform the following steps:
1. Start with a mutable 16 byte array initialised with zeros. This will be used in calculating the values and will become the checksum itself.
2. Declare a mutable variable initialised to zero that will be used to store the result of a calculation. This variable will also be part of an indexing operation into the PI based array.
3. Take 16 bytes from the input.
4. Perform the following steps ensuring you use the current input byte and the corresponding checksum byte until all 16 checksum bytes have been processed:
    1. Using the variable from step 2 perform a bitwise XOR operation on the current input byte. This becomes the index for the PI based array.
    2. Using the value at the index from the PI based array perform a bitwise XOR operation on the current byte of the checksum array. The value in the checksum array must be changed to this result.
    3. Ensure the variable from step 2 now holds the value from step 4.2.
5. Repeat steps 3 and 4 until all input bytes have been processed once. If there are only 16 input bytes then no repetition occurs and you can skip this step.
6. The checksum from step one is now complete and ready to be appended to the input.

In practice the above steps are likely to be implemented as a loop nested inside another loop. The outer loop ensures you deal with 16 bytes of input at a time. The inner loop performs the calculation and populates the checksum.

### 3. Digest
The digest is 16 bytes and the final output of the md2 algorithm.\
The input is the result of the previous step (original input + padding + checksum) and is divisible by 16.\
To calculate the digest perform the following steps:
1. Start with a mutable 48 byte array initialised with zeros. This will be used in calculating the values and the first 16 bytes will become the digest output.
2. Take 16 bytes from the input.
3. Change the middle 16 bytes of the 48 byte array to the same as the 16 input bytes from step 2.
4. Change the last 16 bytes of the 48 byte array to the result of a bitwise XOR of the first 16 bytes and the last 16 bytes of the 48 byte array.
5. Declare a mutable variable initialised to zero to be used in the next step. This variable will be used to index into the PI based array and to change the values in the 48 byte array.
6. Perform this step 18 times.
    1. For each byte in the 48 byte array do the below until all 48 bytes have been processed:
        1. Use the variable from step 5 as an index into the PI based array.
        2. Perform a bitwise XOR using the current byte of the 48 byte array and the value from step 6.1.1. Assign the result to the variable from step 5.
        3. Change the current byte from the 48 byte array to the same value as the result of step 6.1.2.
    2. Change the value of the variable from step 5 ready for the next loop. Add the current loop number (0 up to and including 17) to the current value of the variable from step 5. This addition operation needs to be a wrapping add for an unsigned 8 bit integer so that the value of the variable from step 5 will be between 0 and 255 inclusive. You can perform the same effect with higher integer types by adding the 2 values together and performing modulo 256 (The modulo operation is equivalent to checking if the value is below your target (in this case 256) and if it is use that as your result. If it is equal to or above your target then you are trying to figure out the remainder left over after you have divided your number by the target. Examples: 7 modulo 3 = 1 because 3 fits into 7 twice with 1 left over, 6 modulo 3 = 0 because 3 fits into 6 twice with 0 left over).
7. Repeat steps 2 to 6 until all input bytes have been processed once.
8. The digest is the first 16 bytes from the 48 byte array.

In practice the above steps are likely to be implemented as several nested loops. The outer loop ensures you deal with 16 bytes of input at a time. The first inner loop alters the last 32 bytes of the 48 byte array. The middle 16 bytes are changed to the corresponding input bytes. The last 16 bytes are changed to the middle 16 bytes bitwise XOR'ed by the first 16 bytes. The second inner loop performs an operation 18 times in conjunction with a new variable. The innermost loop (inside the second inner loop) performs a calculation that alters the entire 48 byte array using both the new variable and the contents of the PI based array.

## Acknowledgements
- md2 rfc: https://www.rfc-editor.org/rfc/rfc1319
