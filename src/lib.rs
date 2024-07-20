//! # md2_digest
//!
//! This crate generates the md2 digest for the bytes that are used as input.
//!
//! This is a `#![no_std]` crate that does not require [alloc] and has no dependencies.
//!
//! [alloc]: <https://doc.rust-lang.org/alloc/index.html>
#![no_std]

const PI_SUB: [u8; 256] = [
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
];

#[derive(Clone, Copy)]
enum Padding { // Using an enum to represent the amount of padding bytes
    Zero,      // saves space as all variants occupy the same amount
    One,       // of space.
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Eleven,
    Twelve,
    Thirteen,
    Fourteen,
    Fifteen,
}

impl Padding {
    #[inline]
    const fn new(input: usize) -> Self {
        let input = 16 - if input > 0 && input < 16 {input} else {input % 16};
        match input {
            0 | 16 => Self::Zero, // Padding is always applied.
            1 => Self::One,
            2 => Self::Two,
            3 => Self::Three,
            4 => Self::Four,
            5 => Self::Five,
            6 => Self::Six,
            7 => Self::Seven,
            8 => Self::Eight,
            9 => Self::Nine,
            10 => Self::Ten,
            11 => Self::Eleven,
            12 => Self::Twelve,
            13 => Self::Thirteen,
            14 => Self::Fourteen,
            15 => Self::Fifteen,
            _ => unreachable!(),
        }
    }
    #[inline]
    const fn len(self) -> usize { // Using a len() method works for this implementation
        match self {              // because the contents of all the padding bytes are
            Self::Zero => 16,     // identical to the length of bytes they would occupy.
            Self::One => 1,       // For example: a padding amount of 5 would be 5 bytes
            Self::Two => 2,       // all with the value 5.
            Self::Three => 3,     // Instead of taking up 5 bytes of space having the same
            Self::Four => 4,      // value repeated 5 times this enum allows you to take
            Self::Five => 5,      // up less space and still get the desired value as if
            Self::Six => 6,       // the actual padding had occurred.
            Self::Seven => 7,
            Self::Eight => 8,
            Self::Nine => 9,
            Self::Ten => 10,
            Self::Eleven => 11,
            Self::Twelve => 12,
            Self::Thirteen => 13,
            Self::Fourteen => 14,
            Self::Fifteen => 15,
        }
    }
}

#[derive(Clone, Copy)]
struct MsgWithPadding<'msg> {
    msg: &'msg [u8],
    padding: Padding,
}

impl<'msg> MsgWithPadding<'msg> {
    #[inline]
    const fn new(msg: &'msg [u8], padding: Padding) -> Self {
        Self {msg, padding}
    }
    #[inline]
    const fn len(&self) -> usize {
        self.msg.len() + self.padding.len()
    }
    #[inline]
    #[allow(clippy::cast_possible_truncation)]
    const fn get_inner(&self, index: usize) -> u8 {
        if index >= self.msg.len() {self.padding.len() as u8} // cast fine, self.padding.len() never exceeds 16
        else {self.msg[index]}
    }
}

/// The md2 digest created from a slice of bytes.
pub struct MD2Digest([u8; 16]);

impl MD2Digest {
    /// Generate a new [`MD2Digest`] from a slice of bytes.
    #[must_use]
    pub const fn new(input: &[u8]) -> Self {
        Self (make_md2(input))
    }
    /// Returns the bytes of the digest in an array.
    #[must_use]
    pub const fn bytes(&self) -> [u8; 16] {
        self.0
    }
}

impl core::fmt::Display for MD2Digest {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{:02x}{:02x}{:02x}{:02x}{:02x}{:02x}{:02x}{:02x}{:02x}{:02x}{:02x}{:02x}{:02x}{:02x}{:02x}{:02x}",
            self.0[0], self.0[1], self.0[2], self.0[3], self.0[4], self.0[5], self.0[6], self.0[7],
            self.0[8], self.0[9], self.0[10], self.0[11], self.0[12], self.0[13], self.0[14], self.0[15])
    }
}

#[allow(clippy::cast_possible_truncation)]
#[inline]
const fn make_md2(input: &[u8]) -> [u8; 16] {
    let msg_with_padding = MsgWithPadding::new(input, Padding::new(input.len()));
    let checksum = checksum(msg_with_padding);
    let msg = MsgWithChecksum::new(msg_with_padding, checksum);
    let mut buffer = [0; 48];
    let mut outer_index = 0;
    while outer_index < (msg.len() / 16) {
        let mut inner_index = 0;
        while inner_index < 16 {
            buffer[16 + inner_index] = msg.get_inner((outer_index * 16) + inner_index);
            buffer[32 + inner_index] = buffer[16 + inner_index] ^ buffer[inner_index];
            inner_index += 1;
        }
        let mut t = 0;
        inner_index = 0;
        while inner_index < 18 {
            let mut innermost_index = 0;
            while innermost_index < 48 {
                t = (buffer[innermost_index]) ^ (PI_SUB[t as usize]);
                buffer[innermost_index] = t;
                innermost_index += 1;
            }
            t = t.wrapping_add(inner_index as u8); // cast fine, inner_index never exceeds 18
            inner_index += 1;
        }
        outer_index += 1;
    }
    let mut digest = [0; 16];
    outer_index = 0;
    while outer_index < 16 {
        digest[outer_index] = buffer[outer_index];
        outer_index += 1;
    }
    digest
}

#[inline]
const fn checksum(msg: MsgWithPadding) -> [u8; 16] {                 // Start with a 16 byte 0 initialised array
    let mut checksum = [0; 16];                                      // and a 0 initialised variable to hold a value.
    let mut l = 0;                                                   // Loop through all input bytes in groups of 16.
    let mut outer_index = 0;                                         // Bitwise xor the input byte with the variable to
    while outer_index < (msg.len() / 16) {                           // get an index into the PI_SUB array.
        let mut inner_index = 0;                                     // Bitwise xor the checksum byte using the value
        while inner_index < 16 {                                     // from the PI_SUB array.
            let c = msg.get_inner((outer_index * 16) + inner_index); // Assign it to the variable ready for the next byte.
            checksum[inner_index] ^= PI_SUB[(c ^ l) as usize];       // After 16 input bytes are processed the checksum
            l = checksum[inner_index];                               // will now be filled with values.
            inner_index += 1;                                        // Those values will then be used and overwritten
        }                                                            // when processing the next 16 bytes.
        outer_index += 1;                                            // Once all input bytes are processed return the
    }                                                                // checksum ready for appending to the input.
    checksum
}

struct MsgWithChecksum<'msg> {
    msg_with_padding: MsgWithPadding<'msg>,
    checksum: [u8; 16],
}

impl<'msg> MsgWithChecksum<'msg> {
    #[inline]
    const fn new(msg_with_padding: MsgWithPadding<'msg>, checksum: [u8; 16]) -> Self {
        Self {msg_with_padding, checksum}
    }
    #[inline]
    const fn len(&self) -> usize {
        self.msg_with_padding.len() + self.checksum.len()
    }
    #[inline]
    const fn get_inner(&self, index: usize) -> u8 {
        if index < self.msg_with_padding.len() {self.msg_with_padding.get_inner(index)} else {
            self.checksum[index - self.msg_with_padding.len()]
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{make_md2, checksum, MsgWithPadding};
    const fn is_identical(first: &[u8], second: &[u8]) -> bool {
        let mut index = 0;
        while index != first.len() {
            if first[index] != second[index] {return false;}
            index += 1;
        }
        true
    }
    #[test]
    const fn empty_md2_checksum() {
        assert!(is_identical(
            &checksum(MsgWithPadding::new(b"", crate::Padding::Zero)),
            &[98, 56, 103, 182, 175, 82, 121, 94, 95, 33, 78, 151, 32, 190, 234, 141]
        ));
        assert!(is_identical(
            &[98, 56, 103, 182, 175, 82, 121, 94, 95, 33, 78, 151, 32, 190, 234, 141],
            &[0x62, 0x38, 0x67, 0xb6, 0xaf, 0x52, 0x79, 0x5e, 0x5f, 0x21, 0x4e, 0x97, 0x20, 0xbe, 0xea, 0x8d]
        ));
    }
    #[test]
    const fn empty_md2() { // see tests in rfc1319
        assert!(is_identical(
            &make_md2(b""),
            &[131, 80, 229, 163, 226, 76, 21, 61, 242, 39, 92, 159, 128, 105, 39, 115]
        ));
        assert!(is_identical(
            &[131, 80, 229, 163, 226, 76, 21, 61, 242, 39, 92, 159, 128, 105, 39, 115],
            &[0x83, 0x50, 0xe5, 0xa3, 0xe2, 0x4c, 0x15, 0x3d, 0xf2, 0x27, 0x5c, 0x9f, 0x80, 0x69, 0x27, 0x73]
        ));
    }
    #[test]
    const fn a_md2_checksum() {
        assert!(is_identical(
            &checksum(MsgWithPadding::new(b"a", crate::Padding::Fifteen)),
            &[25, 115, 156, 173, 163, 186, 40, 22, 147, 52, 142, 157, 37, 111, 255, 49]
        ));
        assert!(is_identical(
            &[25, 115, 156, 173, 163, 186, 40, 22, 147, 52, 142, 157, 37, 111, 255, 49],
            &[0x19, 0x73, 0x9c, 0xad, 0xa3, 0xba, 0x28, 0x16, 0x93, 0x34, 0x8e, 0x9d, 0x25, 0x6f, 0xff, 0x31]
        ));
    }
    #[test]
    const fn a_md2() { // see tests in rfc1319
        assert!(is_identical(
            &make_md2(b"a"),
            &[50, 236, 1, 236, 74, 109, 172, 114, 192, 171, 150, 251, 52, 192, 181, 209]
        ));
        assert!(is_identical(
            &[50, 236, 1, 236, 74, 109, 172, 114, 192, 171, 150, 251, 52, 192, 181, 209],
            &[0x32, 0xec, 0x01, 0xec, 0x4a, 0x6d, 0xac, 0x72, 0xc0, 0xab, 0x96, 0xfb, 0x34, 0xc0, 0xb5, 0xd1]
        ));
    }
    #[test]
    const fn abc_md2_checksum() {
        assert!(is_identical(
            &checksum(MsgWithPadding::new(b"abc", crate::Padding::Thirteen)),
            &[25, 226, 157, 27, 115, 4, 54, 142, 89, 90, 39, 111, 48, 47, 87, 204]
        ));
        assert!(is_identical(
            &[25, 226, 157, 27, 115, 4, 54, 142, 89, 90, 39, 111, 48, 47, 87, 204],
            &[0x19, 0xe2, 0x9d, 0x1b, 0x73, 0x04, 0x36, 0x8e, 0x59, 0x5a, 0x27, 0x6f, 0x30, 0x2f, 0x57, 0xcc]
        ));
    }
    #[test]
    const fn abc_md2() { // see tests in rfc1319
        assert!(is_identical(
            &make_md2(b"abc"),
            &[218, 133, 59, 13, 63, 136, 217, 155, 48, 40, 58, 105, 230, 222, 214, 187]
        ));
        assert!(is_identical(
            &[218, 133, 59, 13, 63, 136, 217, 155, 48, 40, 58, 105, 230, 222, 214, 187],
            &[0xda, 0x85, 0x3b, 0x0d, 0x3f, 0x88, 0xd9, 0x9b, 0x30, 0x28, 0x3a, 0x69, 0xe6, 0xde, 0xd6, 0xbb]
        ));
    }
    #[test]
    const fn message_digest_md2_checksum() {
        assert!(is_identical(
            &checksum(MsgWithPadding::new(b"message digest", crate::Padding::Two)),
            &[86, 214, 81, 87, 222, 223, 205, 117, 167, 177, 232, 45, 151, 14, 236, 75]
        ));
        assert!(is_identical(
            &[86, 214, 81, 87, 222, 223, 205, 117, 167, 177, 232, 45, 151, 14, 236, 75],
            &[0x56, 0xd6, 0x51, 0x57, 0xde, 0xdf, 0xcd, 0x75, 0xa7, 0xb1, 0xe8, 0x2d, 0x97, 0x0e, 0xec, 0x4b]
        ));
    }
    #[test]
    const fn message_digest_md2() { // see tests in rfc1319
        assert!(is_identical(
            &make_md2(b"message digest"),
            &[171, 79, 73, 107, 251, 42, 83, 11, 33, 159, 243, 48, 49, 254, 6, 176]
        ));
        assert!(is_identical(
            &[171, 79, 73, 107, 251, 42, 83, 11, 33, 159, 243, 48, 49, 254, 6, 176],
            &[0xab, 0x4f, 0x49, 0x6b, 0xfb, 0x2a, 0x53, 0x0b, 0x21, 0x9f, 0xf3, 0x30, 0x31, 0xfe, 0x06, 0xb0]
        ));
    }
    #[test]
    const fn alphabet_md2_checksum() {
        assert!(is_identical(
            &checksum(MsgWithPadding::new(b"abcdefghijklmnopqrstuvwxyz", crate::Padding::Six)),
            &[74, 66, 211, 163, 119, 183, 233, 152, 143, 185, 40, 150, 153, 228, 211, 163]
        ));
        assert!(is_identical(
            &[74, 66, 211, 163, 119, 183, 233, 152, 143, 185, 40, 150, 153, 228, 211, 163],
            &[0x4a, 0x42, 0xd3, 0xa3, 0x77, 0xb7, 0xe9, 0x98, 0x8f, 0xb9, 0x28, 0x96, 0x99, 0xe4, 0xd3, 0xa3]
        ));
    }
    #[test]
    const fn alphabet_md2() { // see tests in rfc1319
        assert!(is_identical(
            &make_md2(b"abcdefghijklmnopqrstuvwxyz"),
            &[78, 141, 223, 243, 101, 2, 146, 171, 90, 65, 8, 195, 170, 71, 148, 11]
        ));
        assert!(is_identical(
            &[78, 141, 223, 243, 101, 2, 146, 171, 90, 65, 8, 195, 170, 71, 148, 11],
            &[0x4e, 0x8d, 0xdf, 0xf3, 0x65, 0x02, 0x92, 0xab, 0x5a, 0x41, 0x08, 0xc3, 0xaa, 0x47, 0x94, 0x0b]
        ));
    }
    #[test]
    const fn double_alphabet_numbers_checksum() {
        assert!(is_identical(
            &checksum(MsgWithPadding::new(b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789",
                crate::Padding::Two)),
            &[195, 219, 117, 146, 238, 29, 217, 184, 69, 5, 207, 180, 226, 249, 167, 101]
        ));
        assert!(is_identical(
            &[195, 219, 117, 146, 238, 29, 217, 184, 69, 5, 207, 180, 226, 249, 167, 101],
            &[0xc3, 0xdb, 0x75, 0x92, 0xee, 0x1d, 0xd9, 0xb8, 0x45, 0x05, 0xcf, 0xb4, 0xe2, 0xf9, 0xa7, 0x65]
        ));
    }
    #[test]
    const fn double_alphabet_numbers() { // see tests in rfc1319
        assert!(is_identical(
            &make_md2(b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789"),
            &[218, 51, 222, 242, 164, 45, 241, 57, 117, 53, 40, 70, 195, 3, 56, 205]
        ));
        assert!(is_identical(
            &[218, 51, 222, 242, 164, 45, 241, 57, 117, 53, 40, 70, 195, 3, 56, 205],
            &[0xda, 0x33, 0xde, 0xf2, 0xa4, 0x2d, 0xf1, 0x39, 0x75, 0x35, 0x28, 0x46, 0xc3, 0x03, 0x38, 0xcd]
        ));
    }
    #[test]
    const fn numbers_times_eight_md2_checksum() {
        assert!(is_identical(
            &checksum(MsgWithPadding::new(b"12345678901234567890123456789012345678901234567890123456789012345678901234567890",
                crate::Padding::Zero)),
            &[5, 156, 165, 103, 60, 143, 147, 27, 196, 18, 20, 245, 107, 92, 108, 1]
        ));
        assert!(is_identical(
            &[5, 156, 165, 103, 60, 143, 147, 27, 196, 18, 20, 245, 107, 92, 108, 1],
            &[0x05, 0x9c, 0xa5, 0x67, 0x3c, 0x8f, 0x93, 0x1b, 0xc4, 0x12, 0x14, 0xf5, 0x6b, 0x5c, 0x6c, 0x01]
        ));
    }
    #[test]
    const fn numbers_times_eight_md2() { // see tests in rfc1319
        assert!(is_identical(
            &make_md2(b"12345678901234567890123456789012345678901234567890123456789012345678901234567890"),
            &[213, 151, 111, 121, 216, 61, 58, 13, 201, 128, 108, 60, 102, 243, 239, 216]
        ));
        assert!(is_identical(
            &[213, 151, 111, 121, 216, 61, 58, 13, 201, 128, 108, 60, 102, 243, 239, 216],
            &[0xd5, 0x97, 0x6f, 0x79, 0xd8, 0x3d, 0x3a, 0x0d, 0xc9, 0x80, 0x6c, 0x3c, 0x66, 0xf3, 0xef, 0xd8]
        ));
    }
}
