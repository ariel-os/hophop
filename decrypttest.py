# SPDX-FileCopyrightText: Copyright Christian Amsüss <chrysn@fsfe.org>, Silano Systems
# SPDX-License-Identifier: MIT OR Apache-2.0
"""
Checking if we can decrypt things fundamentally.

Captured with the rx example from a dect_shell beacon:

[INFO ] Header details: format 0 length 1 subslots, nid 0x21, from 0x26e5, tx power 10, df_mcs 4 (rx src/bin/rx.rs:34)
[INFO ] PDC: Security: 2, Beacon { network id: 0x876543, transmitter address: 0x70d1776d }, IEs:
    - "MAC Security Info" (6bit, 0x10): Version 0, key index 0, type 0, HPC 17232973
    Rest is encrypted: [6a, b6, 7c, 3a, f4, aa, 79, cd, 6d, 33, a5, b7, f7, c9, 61, f5, f1, c1, f8, 84, 80, 00, ab, 01, bf, d5, 62, a7, 46, c5, 6d, 75, 8e, 0d, be, 71, cc, 16, 67, 3b, 97, 7d, 46, b3, ad, 7e, 99, f4, c0, fd, dd, fb, cf, 56, 9b, e5, 2e, 0c, d0, 05, c6, 34, f1, 82, 12, 2e, c1, 69, 36, b8, ad, d2, 1d, 76, 1b, 83, f1, 75, db, f6, 84, 01, 27, 21, dc, ea, ae, f0, d0, a3, 8e, 3b, e2, ca, b9, 41, 68, 54, a7, 3a, b2, c5] (rx src/bin/rx.rs:87)
"""

ciphertext = bytes.fromhex('6a, b6, 7c, 3a, f4, aa, 79, cd, 6d, 33, a5, b7, f7, c9, 61, f5, f1, c1, f8, 84, 80, 00, ab, 01, bf, d5, 62, a7, 46, c5, 6d, 75, 8e, 0d, be, 71, cc, 16, 67, 3b, 97, 7d, 46, b3, ad, 7e, 99, f4, c0, fd, dd, fb, cf, 56, 9b, e5, 2e, 0c, d0, 05, c6, 34, f1, 82, 12, 2e, c1, 69, 36, b8, ad, d2, 1d, 76, 1b, 83, f1, 75, db, f6, 84, 01, 27, 21, dc, ea, ae, f0, d0, a3, 8e, 3b, e2, ca, b9, 41, 68, 54, a7, 3a, b2, c5'.replace(', ', ''))
# "truncate the MIC to 5 octets"
# (Not looking into the MAC yet because it'd need the full MAC header type and
# common header and probably even the MAC Security Info although Figure 6.3.1-1
# doesn't show that clearly)
ciphertext = ciphertext[:-5]
iv = bytes.fromhex('70d1776d ffffffff') + (17232973).to_bytes(4, 'big') + bytes.fromhex('00000000') # beacon therefore PSN=0x000, block 0x00000
key = b"JustAdefault!!!!"

from cryptography.hazmat.primitives import ciphers

cipher = ciphers.base.Cipher(ciphers.algorithms.AES(key), ciphers.modes.CTR(iv))
decryptor = cipher.decryptor()
plaintext = b""
plaintext += decryptor.update(ciphertext)
plaintext += decryptor.finalize()

# Looks good because there are lots of 00 that indicate padding (I can't read IEs without a decoder yet)
print(plaintext.hex())
