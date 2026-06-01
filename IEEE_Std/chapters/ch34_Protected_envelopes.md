# Chapter 34: Protected envelopes

## 34. Protected envelopes
### 34.1 General
This clause describes the following:
— Processing protected envelopes
— Protect pragma directives
— Protect pragma keywords
### 34.2 Overview
Protected envelopes specify a region of text that shall be transformed prior to analysis by the source
language processor. These regions of text are structured to provide the source language processor with the
specification of the cryptographic algorithm, key, envelope attributes, and textual design data.
All information that identifies a protected envelope is introduced by the protect pragma (see 22.11). This
pragma is reserved by this standard for the description of protected envelopes and is the prefix for specifying
the regions and processing specifications for each protected envelope. Additional information is associated
with the pragma by appending pragma expressions. The pragma expressions of the protect pragma are
evaluated in sequence from left to right. Interpretation of protected envelopes shall not be altered based on
whether the sequence of pragma expressions occurs in a single protect pragma directive or in a sequence
of protect pragma directives. In this clause, unless otherwise specified, pragma directives, pragma
keywords, and pragma expressions shall refer to occurrences of protect pragma directives and their
associated pragma keywords and pragma expressions.
Envelopes may be defined for either of two modes of processing. Encryption envelopes specify the pragma
expressions for encrypting source text regions. An encryption envelope begins in the source text when a
begin pragma expression is encountered. The end of the encryption envelope occurs at the point where an
end pragma expression is encountered. The end pragma expression is said to close the envelope and shall be
associated with the most recent begin pragma expression.
Decryption envelopes specify the pragma expressions for decrypting encrypted text regions. A decryption
envelope begins in the source text when a begin_protected pragma expression is encountered. The end
of the decryption envelope occurs at the point where an end_protected pragma expression is encountered.
The end_protected pragma expression is said to close the envelope and shall be associated with the most
recent begin_protected that has not already been closed. Decryption envelopes may contain other
envelopes within their enclosed data block. The number of nested decryption envelopes that can be
processed is implementation-specific; however, that number shall be no less than 8. Code that is contained
within a decryption envelope is said to be protected.
Pragma expressions that precede begin or begin_protected are designated as envelope keywords.
Pragma expressions that follow the begin/begin_protected keywords and precede the associated end/
end_protected keywords are designated as content keywords. Content keywords are pragma expressions
that are within the region of text that is processed during encryption or decryption of a protected envelope.
### 34.3 Processing protected envelopes
Two modes of processing are defined for protected envelopes. Envelope encryption is the process of
recognizing encryption envelopes in the source text and transforming them into decryption envelopes.
Envelope decryption is the process of recognizing decryption envelopes in the input text and transforming
them into the corresponding cleartext for the compilation step that follows.

<!-- Page 951 -->

IEEE Std Tools that process SystemVerilog source code shall perform envelope decryption for all decryption
envelopes contained in the source text, where the proper key is supplied by the user. Tools that perform
envelope encryption shall only be required to process the protect pragma directives and shall apply no other
interpretation to text that is not part of a protect pragma directive.
#### 34.3.1 Encryption
SystemVerilog tools that provide encryption services shall transform source text containing encryption
envelopes by replacing each encryption envelope with a decryption envelope formed by encrypting the
source text of the encryption envelope according to the specified pragma expressions.
Source text that is not contained in an encryption envelope shall not be modified by the encrypting language
processor, unless otherwise specified.
Decryption envelopes are formed from encryption envelopes by transforming the specified encryption
envelope pragma expressions into decryption envelope pragma expressions and decryption content pragma
expressions. The body of the encryption envelope is encrypted using the specified key, referred to as the
exchange key, and is recorded in the decryption envelope as a data_block.
Encryption algorithms that use the same key to encrypt cleartext and decrypt the corresponding ciphertext
are said to be symmetric. Algorithms that require different keys to encrypt and decrypt are said to be
asymmetric. This description may be applied to both the algorithm and the key.
Tools that provide encryption services may support session keys to limit exposure to the exchange key that is
specified by the IP author using the encryption envelope pragma expressions. A session key is created in an
unspecified manner to encrypt the data from the encryption envelope. A copy of the session key is encrypted
using the exchange key and is recorded in a key_block in the decryption envelope. Next, the body of the
encryption envelope is encrypted using the session key and is recorded in the decryption envelope as a
data_block.
The following example shows the use of the protect pragma to specify encryption of design data. The
encryption method is a simple substitution cipher where each alphabetic character is replaced with the 13th
character in alphabetic sequence, commonly referred to as “rot13.” Nonalphabetic characters are not
substituted. The following design data contain an encryption envelope that specifies the desired protection.
NOTE 1—The `pragma protect runtime_license line is actually a single long line, but it wraps over to the
following line on the printed page.
module secret (a, b);
input a;
output b;
`pragma protect encoding=(enctype="raw")
`pragma protect data_method="x-caesar", data_keyname="rot13", begin
`pragma protect
runtime_license=(library="lic.so",feature="runSecret",entry="chk", match=42)
logic b;
initial
begin
b = 0;
end
always
begin
#5 b = a;
end

<!-- Page 952 -->

IEEE Std `pragma protect end
endmodule // secret
After encryption processing, the following design data are produced. The decryption envelope is written
with a “raw” encoding to make the substitution encryption directly visible.
NOTE 2—The `pragma protect line that ends with begin_protected and the encoded line beginning
`centzn are actually single long lines, but they wrap over to the following lines on the printed page.
module secret (a, b);
input a;
output b;
`pragma protect encoding=(enctype="raw")
`pragma protect data_method="x-caesar", data_keyname="rot13",
begin_protected
`pragma protect encoding=(enctype="raw", bytes=190), data_block
`centzn cebgrpg ehagvzr_yvprafr=(yvoenel="yvp.fb",srngher="ehaFrperg",
ragel="pux",zngpu=42)
ert o;
vavgvny
ortva
o = 0;
raq
nyjnlf
ortva
#5 o = n;
raq
`pragma protect end_protected
`pragma reset protect
endmodule // secret
NOTE 3—Products that include cryptographic algorithms may be subject to government laws and regulations, including
possible restrictions or limitations on transfer or use in many jurisdictions. Users of this standard are advised to seek the
advice of competent counsel to determine all applicable legal and regulatory obligations.
#### 34.3.2 Decryption
SystemVerilog tools that support decrypting compilation shall transform source text containing decryption
envelopes by replacing each decryption envelope with the decrypted source text from the data_block,
according to the specified pragma expressions. The substituted text may contain usages of text macros,
which shall be substituted after replacement of the decryption envelope. The substituted text may also
contain decryption envelopes, which shall be decrypted and substituted after replacement of their enclosing
decryption envelope.
### 34.4 Protect pragma directives
Protected envelopes are lexical regions delimited by protect pragma directives. The effect of a particular
protect pragma directive is specified by its pragma expressions. This standard defines the pragma
keyword names listed in Table34-1 for use with the protect pragma. These pragma keywords are defined
in 34.5 with a specification of how each participates in the encryption and decryption processing modes.

<!-- Page 953 -->

IEEE Std Table34-1—protect pragma keywords
Pragma keyword Description
begin Opens a new encryption envelope
end Closes an encryption envelope
begin_protected Opens a new decryption envelope
end_protected Closes a decryption envelope
author Identifies the author of an envelope
author_info Specifies additional author information
encrypt_agent Identifies the encryption service
encrypt_agent_info Specifies additional encryption service information
encoding Specifies the coding scheme for encrypted data
data_keyowner Identifies the owner of the data encryption key
data_method Identifies the data encryption algorithm
data_keyname Specifies the name of the data encryption key
data_public_key Specifies the public key for data encryption
data_decrypt_key Specifies the data session key
data_block Begins an encoded block of encrypted data
digest_keyowner Identifies the owner of the digest encryption key
digest_key_method Identifies the digest encryption algorithm
digest_keyname Specifies the name of the digest encryption key
digest_public_key Specifies the public key for digest encryption
digest_decrypt_key Specifies the digest session key
digest_method Specifies the digest computation algorithm
digest_block Specifies a message digest for data integrity
key_keyowner Identifies the owner of the key encryption key
key_method Specifies the key encryption algorithm
key_keyname Specifies the name of the key encryption key
key_public_key Specifies the public key for key encryption
key_block Begins an encoded block of key data
decrypt_license Specifies licensing constraints on decryption
runtime_license Specifies licensing constraints on simulation
comment Uninterpreted documentation string
reset Resets pragma keyword values to default
viewport Modifies scope of access into decryption envelope

<!-- Page 954 -->

IEEE Std The scope of protect pragma directives is completely lexical and not associated with any declarative
region or declaration in the source text itself. This lexical scope may cross file boundaries and included files.
In protected envelopes where a specific pragma keyword is absent, the SystemVerilog tool shall use the
default value. SystemVerilog tools that perform encryption should explicitly output all relevant pragma
keywords for each envelope in order to avoid unintended interpretations during decryption. Further
robustness can be achieved by appending a reset pragma keyword after each envelope.
### 34.5 Protect pragma keywords
#### 34.5.1 begin
34.5.1.1 Syntax
begin
34.5.1.2 Description
ENCRYPTION INPUT: The begin pragma expression is used in the input text to indicate to an encrypting
tool the point at which encryption shall begin.
Nesting of pragma begin-end blocks shall be an error. There may be begin_protected-
end_protected blocks containing previously encrypted content inside such a block. They are simply
treated as a byte stream and encrypted as if they were text.
ENCRYPTION OUTPUT: The begin pragma expression is replaced in the encryption output stream by the
begin_protected pragma expression. Following begin_protected, all pragma expressions required as
encryption output shall be generated prior to the end_protected pragma expression. Protected envelopes
should be completely self-contained to avoid any undesired interaction when multiple encrypted models
exist in the decryption input stream. The data_block and key_block pragma expressions introduce the
encrypted data or keys and will always be found within a begin_protected-end_protected envelope.
All text, including comments and other protect pragmas, occurring between the begin pragma expression
and the corresponding end pragma expression shall, unless otherwise specified, be encrypted and placed in
the encryption output stream using the data_block pragma expression. An unspecified length of arbitrary
comment text may be added by the encrypting tool to the beginning and end of the input text in order to
prevent known text attacks on the encrypted content of the data_block.
DECRYPTION INPUT: none
#### 34.5.2 end
34.5.2.1 Syntax
end
34.5.2.2 Description
ENCRYPTION INPUT: The end pragma expression is used in the input cleartext to indicate the end of the
region that shall be encrypted.
ENCRYPTION OUTPUT: The end pragma expression is replaced in the encryption output stream by the
end_protected pragma expression.

<!-- Page 955 -->

IEEE Std DECRYPTION INPUT: none
#### 34.5.3 begin_protected
34.5.3.1 Syntax
begin_protected
34.5.3.2 Description
ENCRYPTION INPUT: When a begin_protected-end_protected block is found in an input file
during encryption, its contents are treated as input cleartext. This allows a previously encrypted model to be
reencrypted as a portion of a larger model. Any other protect pragmas inside the begin_protected-
end_protected block shall not be interpreted and shall not override pragmas in effect. Nested encryption
shall not corrupt pragma values in the current encryption in process.
ENCRYPTION OUTPUT: The begin_protected pragma expression, and the entire content of the
protected envelope up to the corresponding end_protect pragma expression, shall be encrypted into the
current data_block as specified by the current method and keys.
DECRYPTION INPUT: The begin_protected pragma expression begins a previously encrypted region.
A decrypting tool shall accumulate all the pragma expressions in the block for use in decryption of the
block.
#### 34.5.4 end_protected
34.5.4.1 Syntax
end_protected
34.5.4.2 Description
ENCRYPTION INPUT: This pragma expression indicates the end of a previous begin_protected block.
This indicates that the block is complete, and subsequent pragma expression values will be accumulated for
the next envelope.
ENCRYPTION OUTPUT: The end_protected pragma expression following the corresponding
begin_protected pragma expression shall be encrypted into the current data_block as specified by the
current method and keys.
DECRYPTION INPUT: The end_protected pragma expression indicates the end of a set of pragmas that
are sufficient to decrypt the current block.
#### 34.5.5 author
34.5.5.1 Syntax
author = <string>
34.5.5.2 Description
ENCRYPTION INPUT: The author pragma expression specifies a string that identifies the name of the IP
author. It is distinct from the comment pragma expression so that this information can be recognized without
need for parsing of a comment string value.

<!-- Page 956 -->

IEEE Std ENCRYPTION OUTPUT: If present in the encryption envelope, the author pragma expression shall be
placed in a pragma directive enclosed within the protected envelope, but shall not be encrypted into the
data_block. Otherwise, it is copied without change into the output stream.
DECRYPTION INPUT: none
#### 34.5.6 author_info
34.5.6.1 Syntax
author_info = <string>
34.5.6.2 Description
ENCRYPTION INPUT: The author_info pragma expression specifies a string that contains additional
information provided by the IP author. It is distinct from the comment pragma expression so that this
information can be recognized without need for parsing of a comment string value.
ENCRYPTION OUTPUT: If present in the encryption envelope, the author_info pragma expression
shall be placed in a pragma directive enclosed within the protected envelope, but shall not be encrypted into
the data_block. Otherwise, it is copied without change into the output stream.
DECRYPTION INPUT: none
#### 34.5.7 encrypt_agent
34.5.7.1 Syntax
encrypt_agent = <string>
34.5.7.2 Description
ENCRYPTION INPUT: none
ENCRYPTION OUTPUT: The encrypt_agent pragma expression specifies a string that identifies the
name of the encrypting tool. The encrypting tool shall generate this pragma expression and place it in a
pragma directive enclosed within the protected envelope, but shall not encrypt it into the data_block.
DECRYPTION INPUT: none
#### 34.5.8 encrypt_agent_info
34.5.8.1 Syntax
encrypt_agent_info = <string>
34.5.8.2 Description
ENCRYPTION INPUT: none
ENCRYPTION OUTPUT: The encrypt_agent_info pragma expression specifies a string that contains
additional information provided by the encrypting tool. If provided, the encrypt_agent_info pragma
expression shall be placed within a pragma directive enclosed within the protected envelope, but shall not be
encrypted into the data_block.

<!-- Page 957 -->

IEEE Std DECRYPTION INPUT: none
#### 34.5.9 encoding
34.5.9.1 Syntax
encoding = ( enctype = <string> [ , line_length = <number> ]
[ , bytes = <number> ] )
34.5.9.2 Description
ENCRYPTION INPUT: The encoding pragma expression specifies how the data_block,
digest_block, and key_block content shall be encoded. This encoding allows all binary data produced
in the encryption process to be treated as text. If an encoding pragma expression is present in the input
stream, it specifies how the output shall be encoded.
The following subkeywords are defined for the value of the encoding pragma expression:
enctype=<string> The method for calculating the encoding. This standard specifies the identifiers
in Table34-2 as string values for the enctype subkeyword. These identifiers are
associated with their respective encoding algorithms. The required methods are
standard in every implementation. Optional identifiers are implementation-
specific, but are required to use these identifiers for the corresponding encoding
algorithm. Additional identifier values and their corresponding encoding
algorithms are implementation-defined.
Table34-2—Encoding algorithm identifiers
Required/
enctype Encoding algorithm
optional
uuencode Required IEEE Std 1003.1 (uuencode historical algorithm)
base64 Required IETF RFC 2045 [also IEEE Std 1003.1 (uuencode -m)]
quoted-printable Optional IETF RFC 2045
raw Optional Identity transformation; no encoding shall be performed,
and the data may contain nonprintable characters.
line_length=<number>
The maximum number of characters (after any encoding) in a single line of the
data_block. Insertion of line breaks in the data_block after encryption and
encoding allows the generated text files to be usable by commonly available text
tools.
bytes=<number> The number of bytes in the original block of data before any encoding or the
addition of line breaks. This encoding keyword shall be ignored in the encryption
input.
ENCRYPTION OUTPUT: The encoding directive shall be output in each begin_protected–
end_protected block to explicitly specify the encoding used by the encrypt_agent. A tool may choose to
encode the data even if no encoding pragma expression was found in the input stream and shall output the
corresponding encoding pragma expression. The tool shall generate an encoding descriptor that specifies in
the bytes keyword the number of bytes in the original block of data.

<!-- Page 958 -->

IEEE Std The data_block, data_public_key, data_decrypt_key, digest_block, key_block, and
key_public_key are all encoded using this encoding. If separate encoding is desired for each of these
fields, then multiple encoding pragma expressions can be given in the input stream prior to each of the
above pragma expressions. The bytes value is added by the encrypting tool for each block that it encrypts.
DECRYPTION INPUT: During decryption, the encoding directive is used to find the encoding algorithm
used and the size of actual data.
#### 34.5.10 data_keyowner
34.5.10.1 Syntax
data_keyowner = <string>
34.5.10.2 Description
ENCRYPTION INPUT: The data_keyowner specifies the legal entity or tool that provided the keys used
for encryption and decryption of the data. This pragma keyword permits use of a third-party key, distinct
from one associated with either author or encrypt_agent. The data_keyowner value is used by the
encrypting tool to select the key used to encrypt the data_block. The values for data_keyname,
data_decrypt_key, and data_public_key shall be unique for the specified data_keyowner.
ENCRYPTION OUTPUT: The data_keyowner shall be unchanged in the output file, except where a
digital signature is used, in which case it is encrypted with the key_method and placed in a key_block.
DECRYPTION INPUT: During decryption, the data_keyowner is combined with the data_keyname or
data_public_key to determine the appropriate secret/private key to use during decryption of the
data_block.
#### 34.5.11 data_method
34.5.11.1 Syntax
data_method = <string>
34.5.11.2 Description
ENCRYPTION INPUT: The data_method pragma expression specifies the encryption algorithm that shall
be used to encrypt subsequent begin-end blocks. The encryption method is an identifier that is commonly
associated with a specific encryption algorithm. If the specified encryption algorithm uses a cipher-block
chaining (CBC) technique that requires an Initialization Vector (IV), it is recommended that the IV be
randomly generated for each use of the cipher.
This standard specifies the identifiers in Table34-3 as string values for the data_method pragma
expression. These identifiers are associated with their respective encryption types. The required methods are
standard in every implementation. Optional identifiers are implementation-specific, but are required to use
these identifiers for the corresponding cipher. Additional identifier values and their corresponding ciphers
are implementation-defined.

<!-- Page 959 -->

IEEE Std Table34-3—Encryption algorithm identifiers
Required/
Identifier Encryption algorithm
optional
des-cbc Required Data Encryption Standard (DES) in CBC mode, see FIPS46-3.
3des-cbc Optional Triple DES in CBC mode, see FIPS 46-3; ANSI X9.52-1998.
aes128-cbc Optional Advanced Encryption Standard (AES) with 128-bit key, see FIPS197.
aes256-cbc Optional AES in CBC mode, with 256-bit key.
aes192-cbc Optional AES with 192-bit key.
blowfish-cbc Optional Blowfish in CBC mode, see Schneier (Blowfish).
twofish256-cbc Optional Twofish in CBC mode, with 256-bit key, see Schneier (Twofish).
twofish192-cbc Optional Twofish with 192-bit key.
twofish128-cbc Optional Twofish with 128-bit key.
serpent256-cbc Optional Serpent in CBC mode, with 256-bit key, see Anderson, et al.
serpent192-cbc Optional Serpent with 192-bit key.
serpent128-cbc Optional Serpent with 128-bit key.
cast128-cbc Optional CAST-128 in CBC mode, see IETF RFC 2144.
rsa Optional RSA, see IETF RFC 2437.
elgamal Optional ElGamal, see ElGamal.
pgp-rsa Optional OpenPGP RSA key, see IETF RFC 2440.
ENCRYPTION OUTPUT: The data_method shall be unchanged in the output file, except where a digital
signature is used, in which case it is encrypted with the key_method and placed in a key_block.
DECRYPTION INPUT: The data_method specifies the algorithm that should be used to decrypt the
data_block.
#### 34.5.12 data_keyname
34.5.12.1 Syntax
data_keyname = <string>
34.5.12.2 Description
ENCRYPTION INPUT: The data_keyname pragma expression specifies the name of the key, or key pair,
for an asymmetric encryption algorithm, that should be used to decrypt the data_block. It shall be an error
to specify a data_keyname that is not a member of the list of keys known for the given data_keyowner.
ENCRYPTION OUTPUT: When a data_keyname is provided in the input, it indicates the key that should
be used for encrypting the data. The encrypting tool shall combine this pragma expression with the
data_keyname and determine the key to use. The data_keyname itself shall be output as cleartext in the

<!-- Page 960 -->

IEEE Std output file except where a digital envelope is used. For a digital envelope mechanism, the data_keyname is
encrypted using key_method and key_keyname/key_public_key and encoded in the key_block.
DECRYPTION INPUT: The data_keyname value is combined with the data_keyowner to select a single
key that shall be used to decrypt the data_block from the protected envelope.
#### 34.5.13 data_public_key
34.5.13.1 Syntax
data_public_key
34.5.13.2 Description
ENCRYPTION INPUT: The data_public_key pragma expression specifies that the next line of the file
contains the encoded value of the public key to be used to encrypt the data. The encoding is specified by the
encoding pragma expression that is currently in effect. If both data_public_key and data_keyname
are present, then they shall refer to the same key.
ENCRYPTION OUTPUT: The data_public_key pragma expression shall be output in each protected
block for which it is used, followed by the encoded value. The data_method and data_public_key can
be combined to fully specify the required encryption.
DECRYPTION INPUT: The data_keyowner and data_method can be combined with the
data_public_key to determine whether the decrypting tool knows the corresponding private key to
decrypt a given data_block. If the decrypting tool can compute the required key, the model can be
decrypted (if licensing allows it).
#### 34.5.14 data_decrypt_key
34.5.14.1 Syntax
data_decrypt_key
34.5.14.2 Description
ENCRYPTION INPUT: The data_decrypt_key indicates that the next line contains the encoded value of
the key that will decrypt the data_block. This pragma expression should only be used when digital
signatures are used. An IP author can generate a key and use it to encrypt the cleartext. This encrypted text is
then stored in the output file as the data_block. Then the data_method and data_decrypt_key are
encrypted using the key_method and stored in the output file as the contents of the key_block. The
data_block itself is not reencrypted; only the information about the data key is.
ENCRYPTION OUTPUT: The data_decrypt_key is output as part of the encrypted content of the
key_block. The value is encoded as specified by the encoding pragma expression.
DECRYPTION INPUT: Upon determining that a digital signature was in use for a given protected region,
the decrypting tool shall decrypt the key_block to find the data_decrypt_key and data_method that in
turn can be used to decrypt the data_block.

<!-- Page 961 -->

IEEE Std #### 34.5.15 data_block
34.5.15.1 Syntax
data_block
34.5.15.2 Description
ENCRYPTION INPUT: It shall be an error if a data_block is found in an input file unless it is contained
within a previously generated begin_protected–end_protected block, in which case it is ignored.
ENCRYPTION OUTPUT: The data_block pragma expression indicates that a data block begins on the
next line in the file. The encrypting tool shall take each begin-end block, encrypt the contents as specified
by the data_block pragma expression, and then encode the block as specified by the encoding pragma
expression. The resultant text shall be output. If the data_method specifies a CBC encryption algorithm
that requires an IV, then the IV cipher-block shall be prepended to the encrypted data before encoding is
performed.
DECRYPTION INPUT: The data_block is first read in the encoded form. The encoding shall be reversed,
and then the block shall be internally decrypted. If the data_method specifies a CBC encryption algorithm
that requires an IV, then the first cipher-block of the decoded data_block shall be removed for use as the
IV. The remainder of the data_block shall be internally decrypted.
#### 34.5.16 digest_keyowner
34.5.16.1 Syntax
digest_keyowner = <string>
34.5.16.2 Description
ENCRYPTION INPUT: The digest_keyowner specifies the legal entity or tool that provided the keys
used for encryption and decryption of the data. This pragma keyword permits use of a third-party key,
distinct from one associated with either author or encrypt_agent. The digest_keyowner value is used
by the encrypting tool to select the key used to encrypt the digest_block. The values for digest_keyname,
digest_decrypt_key, and digest_public_key shall be unique for the specified digest_keyowner.
If no digest_keyowner is specified in the input, then the default value of digest_keyowner shall be the
current value of data_keyowner.
ENCRYPTION OUTPUT: The digest_keyowner shall be unchanged in the output file, except where a
digital signature is used, in which case it is encrypted with the digest_key_method and placed in a
digest_key_block.
DECRYPTION INPUT: During decryption, the digest_keyowner is combined with the
digest_keyname or digest_public_key to determine the appropriate secret/private key to use during
decryption of the digest_block.
#### 34.5.17 digest_key_method
34.5.17.1 Syntax
digest_key_method = <string>

<!-- Page 962 -->

IEEE Std 34.5.17.2 Description
ENCRYPTION INPUT: The digest_key_method pragma expression indicates the encryption algorithm
that shall be used to encrypt subsequent digest_block contents. The values specified for
digest_key_method to identify encryption algorithms are the same as those specified for data_method.
If no digest_key_method is specified in the input, then the default value of digest_key_method shall
be the current value of data_method.
ENCRYPTION OUTPUT: The digest_key_method shall be unchanged in the output file, except where a
digital signature is used, in which case it is encrypted with the key_method algorithm and uses the key
found in the key_block.
DECRYPTION INPUT: The digest_key_method indicates the algorithm that shall be used to decrypt the
digest_block.
#### 34.5.18 digest_keyname
34.5.18.1 Syntax
digest_keyname = <string>
34.5.18.2 Description
ENCRYPTION INPUT: The digest_keyname pragma expression provides the name of the key, or key
pair for an asymmetric encryption algorithm, that shall be used to decrypt the digest_block. It shall be an
error to specify a digest_keyname that is not a member of the list of keys known for the given
digest_keyowner. If no digest_keyname is specified in the input, then the default value of
digest_keyname shall be the current value of data_keyname.
ENCRYPTION OUTPUT: When a digest_keyname is provided in the input, it indicates the key that shall
be used for encrypting the data. The encrypting tool shall combine this pragma expression with the
digest_keyowner and determine the key to use. The digest_keyname itself shall be output as cleartext
in the output file except where a digital envelope is used. For a digital envelope mechanism, the
digest_keyname is encrypted using key_method and key_keyname/key_public_key and encoded in
the key_block.
DECRYPTION INPUT: The digest_keyname value is combined with the digest_keyowner to select a
single key that shall be used to decrypt the digest_block from the protected envelope.
#### 34.5.19 digest_public_key
34.5.19.1 Syntax
digest_public_key
34.5.19.2 Description
ENCRYPTION INPUT: The digest_public_key pragma expression indicates that the next line of the
file contains the encoded value of the public key used to encrypt the digest. The encoding is specified by the
encoding pragma expression that is currently in effect. If both digest_public_key and
digest_keyname are present, then they shall refer to the same key. If no digest_public_key is
specified in the input, then the default value of digest_public_key shall be the current value of
data_public_key.

<!-- Page 963 -->

IEEE Std ENCRYPTION OUTPUT: The digest_public_key pragma expression shall be output in each protected
block for which it is used, followed by the encoded value. The digest_key_method and
digest_public_key can be combined to fully specify the required encryption.
DECRYPTION INPUT: The digest_keyowner and digest_key_method can be combined with the
digest_public_key to determine whether the decrypting tool knows the corresponding private key to
decrypt a given digest_block. If the decrypting tool can compute the required key, the model can be
decrypted (if licensing allows it).
#### 34.5.20 digest_decrypt_key
34.5.20.1 Syntax
digest_decrypt_key
34.5.20.2 Description
ENCRYPTION INPUT: The digest_decrypt_key indicates that the next line contains the encoded value
of the key that will decrypt the digest_block. This pragma expression should only be used when digital
signatures are used. An IP author can generate a key and use it to encrypt the digest. This encrypted text is
then stored in the output file as the digest_block. Then the digest_key_method and
digest_decrypt_key are encrypted using the key_method and stored in the output file as the contents of
the key_block. The digest_block itself is not reencrypted; only the information about the digest key is.
If no digest_decrypt_key is specified in the input, then the default value of digest_decrypt_key
shall be the current value of data_decrypt_key.
ENCRYPTION OUTPUT: The digest_decrypt_key is output as part of the encrypted content of the
key_block. The value is encoded as specified by the encoding pragma expression.
DECRYPTION INPUT: Upon determining that a digital signature was in use for a given protected region,
the decrypting tool shall decrypt the key_block to find the digest_decrypt_key and
digest_key_method that in turn can be used to decrypt the digest block.
#### 34.5.21 digest_method
34.5.21.1 Syntax
digest_method = <string>
34.5.21.2 Description
ENCRYPTION INPUT: The digest_method pragma expression specifies the message digest algorithm
that shall be used to generate message digests for subsequent data_block and key_block output. The
string value is an identifier commonly associated with a specific message digest algorithm.
This standard specifies the values in Table34-4 for the digest_method pragma expression. Additional
identifier values are implementation-defined.

<!-- Page 964 -->

IEEE Std Table34-4—Message digest algorithm identifiers
Required/
Identifier Message digest algorithm
optional
sha1 Required Secure Hash Algorithm 1 (SHA-1), see FIPS 180-2.
md5 Required Message Digest Algorithm 5, see IETF RFC 1321.
md2 Optional Message Digest Algorithm 2, see IETF RFC 1319.
ripemd-160 Optional RIPEMD-160, see ISO/IEC 10118-3:2004.
ENCRYPTION OUTPUT: The digest_method shall be unchanged in the output file, except where a
digital signature is used, in which case it is encrypted with the key_method and placed in a key_block.
DECRYPTION INPUT: The digest_method indicates the algorithm that shall be used to generate the
digest from the data_block.
#### 34.5.22 digest_block
34.5.22.1 Syntax
digest_block
34.5.22.2 Description
ENCRYPTION INPUT: If a digest_block pragma expression is found in an input file (other than in a
begin_protected–end_protected block), it shall be treated by the encrypting tool as a request to
generate a message digest in the output file.
ENCRYPTION OUTPUT: A message digest is used to verify that the encrypted data have not been
modified. The encrypting tool generates the message digest (a fixed-length, computationally unique
identifier corresponding to a set of data) using the algorithm specified by the digest_method pragma
expression and encrypts the message digest as specified by the digest_key_method pragma keyword
using the key specified by digest_keyname, digest_key_keyowner, digest_public_key, and
digest_decrypt_key. If digest_key_method is not specified for the encryption envelope, then the
current data_method encryption key shall be used. If the digest_key_method, or in its absence the
current data_method, specifies a CBC encryption algorithm that requires an IV, then the IV cipher-block
shall be prepended to the encrypted digest before encoding is performed.
This digest shall then be encoded using the current encoding pragma expression and output on the next line
of the output file following the digest_block pragma expression. A digest_block shall be generated for
each key_block and data_block that are generated in the encryption process and shall immediately
follow the key_block or data_block to which it refers.
DECRYPTION INPUT: In order to authenticate the message, the consuming tool will decrypt the encrypted
data, generate a message digest from the decrypted data, decrypt the message digest in the digest_block
with the specified key, and compare the two message digests. If the two digests do not match, then either the
digest_block or the encrypted data has been altered since the input data was encrypted. The message
digest for a key_block or data_block shall be contained in a digest_block immediately following the
key_block or data_block. If the digest_key_method, or in its absence the current data_method,
specifies a CBC encryption algorithm that requires an IV, then the first cipher-block of the decoded

<!-- Page 965 -->

IEEE Std digest_block shall be removed for use as the IV. The remainder of the digest_block shall be internally
decrypted.
#### 34.5.23 key_keyowner
34.5.23.1 Syntax
key_keyowner = <string>
34.5.23.2 Description
ENCRYPTION INPUT: The key_keyowner specifies the legal entity or tool that provided the keys used
for encryption and decryption of the key information. The value of the key_keyowner also has the same
constraints specified for the data_keyowner values.
ENCRYPTION OUTPUT: The key_keyowner shall be unchanged in the output file.
DECRYPTION INPUT: During decryption, the key_keyowner can be combined with the key_keyname
or key_public_key to determine the appropriate secret/private key to use during decryption of the
key_block.
#### 34.5.24 key_method
34.5.24.1 Syntax
key_method = <string>
34.5.24.2 Description
ENCRYPTION INPUT: The key_method pragma expression indicates the encryption algorithm that shall
be used to encrypt the keys used to encrypt the data_block. The values specified for key_method to
identify encryption algorithms are the same as those specified for data_method.
ENCRYPTION OUTPUT: The key_method shall be unchanged in the output file.
DECRYPTION INPUT: The key_method indicates the algorithm that shall be used to decrypt the
key_block.
#### 34.5.25 key_keyname
34.5.25.1 Syntax
key_keyname = <string>
34.5.25.2 Description
ENCRYPTION INPUT: The key_keyname pragma expression provides the name of the key, or key pair
for an asymmetric encryption algorithm, that shall be used to decrypt the key_block. It shall be an error to
specify a key_keyname that is not a member of the list of keys known for the given key_keyowner.
ENCRYPTION OUTPUT: When a key_keyname is provided in the input, it indicates the key that shall be
used for encrypting the data encryption keys. The encrypting tool shall combine this pragma expression with
the key_keyowner and determine the key to use. The key_keyname itself shall be output as cleartext in the
output file.

<!-- Page 966 -->

IEEE Std DECRYPTION INPUT: The key_keyname value is combined with the key_keyowner to select a single
key that shall be used to decrypt the data_block from the protected envelope.
#### 34.5.26 key_public_key
34.5.26.1 Syntax
key_public_key
34.5.26.2 Description
ENCRYPTION INPUT: The key_public_key pragma expression indicates that the next line of the file
contains the encoded value of the public key to be used to encrypt the key data. The encoding is specified by
the encoding pragma expression that is currently in effect. If both a key_public_key and key_keyname
are present, then they shall refer to the same key.
ENCRYPTION OUTPUT: The key_public_key pragma expression shall be output in each protected
block for which it is used, followed by the encoded value. The key_method and key_public_key can be
combined to fully specify the required encryption of data keys.
DECRYPTION INPUT: The key_keyowner and key_method can be combined with the
key_public_key to determine whether the decryption tool knows the corresponding private key to decrypt
a given key_block. If the decrypting tool can compute the required key, the data keys can be decrypted.
#### 34.5.27 key_block
34.5.27.1 Syntax
key_block
34.5.27.2 Description
ENCRYPTION INPUT: It shall be an error if a key_block is found in an input file unless it is contained
within a previously generated begin_protected–end_protected block, in which case it is ignored.
ENCRYPTION OUTPUT: The key_block pragma expression indicates that a key block begins on the next
line in the file. When requested to use a digital signature, the encrypting tool shall take any of the
data_method, data_public_key, data_keyname, data_decrypt_key, and digest_block to form a
text buffer. This buffer shall then be encrypted with the appropriate key_public_key, and then the
encrypted region shall be encoded using the encoding pragma expression in effect. The output of this
encoding shall be generated as the contents of the key_block. If the key_method specifies a CBC
encryption algorithm that requires an IV, then the IV cipher-block shall be prepended to the encrypted key
before encoding is performed. Note that encrypting keys with a symmetric cipher is not a common use case.
Where more than one key_block pragma expression occurs within a single begin-end block, the
generated key blocks shall all encode the same data decryption key data. It shall be an error if the data
decryption pragma expressions change value between key_block pragma expressions of a single
encryption envelope. Multiple key blocks are specified for the purpose of providing alternative decryption
keys for a single decryption envelope.
DECRYPTION INPUT: The key_block is first read in the encoded form, the encoding is reversed, and
then the block is internally decrypted. The resulting text is then parsed to determine the keys required to
decrypt the data_block. If the key_method specifies a CBC encryption algorithm that requires an IV,

<!-- Page 967 -->

IEEE Std then the first cipher-block of the decoded key_block shall be removed for use as the IV. The remainder of
the decoded key_block shall be internally decrypted.
#### 34.5.28 decrypt_license
34.5.28.1 Syntax
decrypt_license = ( library = <string> , entry = <string> ,
feature = <string> [, exit = <string>] [, match = <number> ] )
34.5.28.2 Description
ENCRYPTION INPUT: The decrypt_license pragma expression will typically be found inside a
begin-end pair in the original cleartext. This is necessary so that it is encrypted in the output IP that is
shipped to the end user.
ENCRYPTION OUTPUT: The decrypt_license is output unchanged in the output description except
for encryption and encoding of the pragma exactly as other cleartext in the begin-end pair. Typically, it
will be output in the data_block.
DECRYPTION INPUT: After encountering a decrypt_license pragma expression in an encrypted
model, prior to processing the decrypted text, the application shall load the specified library and call the
entry function, passing it the feature specified string. The return value of the entry function shall be
compared to the match value. If the application is licensed to decrypt the model, the returned value shall
compare equal to the match value and shall compare nonequal otherwise. If the application is not licensed to
decrypt the model, no decryption shall be performed, and the application shall produce an error message that
includes the return value of the entry function. If an exit function is specified, then it shall be called prior
to exiting the decrypting application to allow for releasing the license.
NOTE—This mechanism only provides limited security because the end users of the model have the shared library and
could use readily available debuggers to debug the calling sequence of the licensing mechanism. They could then
produce an equivalent library that returns a 0, but avoids the license check.
#### 34.5.29 runtime_license
34.5.29.1 Syntax
runtime_license = ( library = <string> , entry = <string> ,
feature = <string> [ , exit = <string> ] [ , match = <number> ] )
34.5.29.2 Description
ENCRYPTION INPUT: The runtime_license pragma expression will typically be found inside a
begin-end pair in the original cleartext. This is necessary so that it is encrypted in the output IP shipped to
the end user.
ENCRYPTION OUTPUT: The runtime_license is output unchanged in the output description except
for encryption and encoding of the pragma exactly as other cleartext in the begin-end pair.
DECRYPTION INPUT: After encountering a runtime_license pragma expression in an encrypted
model, prior to executing, the application shall load the specified library and call the entry function, passing
it the feature specified string. The return value of the entry function shall be compared to the match value.
If the application is licensed to execute the model, the returned value shall compare equal to the match value
and shall compare nonequal otherwise. If the application is not licensed to execute the model, execution
shall not begin, and the application shall produce an error message that includes the return value of the entry

<!-- Page 968 -->

IEEE Std function. If an exit is specified, then it shall be called prior to exiting the executing application to allow for
releasing the license.
NOTE 1—Execution could mean any evaluation of the model, including simulation, layout, or synthesis.
NOTE 2—This mechanism only provides limited security because the end users of the model have the shared library and
could use readily available debuggers to debug the calling sequence of the licensing mechanism. They could then
produce an equivalent library that returns a 0, but avoids the license check. IP authors may wish to implement their own
licensing scheme embedded within the behavior of the model, possibly using PLI and/or system tasks.
#### 34.5.30 comment
34.5.30.1 Syntax
comment = <string>
34.5.30.2 Description
ENCRYPTION INPUT: The comment pragma expression can be found anywhere in an input file and
indicates that even if this is found inside a begin-end block, the value shall be output as a comment in
cleartext in the output immediately prior to the data_block.
This is provided so that comments that may end up being included in other files inside a begin-end block
can protect themselves from being encrypted. This is important so that critical information such as copyright
notices can be explicitly excluded from encryption.
Because this constitutes known cleartext that would be found inside the data_block, the pragma itself and
the value should not be included in the encrypted text.
ENCRYPTION OUTPUT: The entire comment including the beginning pragma shall be output in cleartext
immediately prior to the data_block corresponding to the begin-end in which the comment was found.
DECRYPTION INPUT: none
#### 34.5.31 reset
34.5.31.1 Syntax
reset
34.5.31.2 Description
ENCRYPTION INPUT: The reset pragma expression is a synonym for a reset pragma directive that
contains protect in the pragma keyword list. Following the reset, all protect pragma keywords are
restored to their default values.
Because the scope of pragma definitions is lexical and extends from the point of the directive until the end of
the compilation input, if an IP author chooses to put common pragmas such as author and author_info
at the beginning of a list of files, they should include a reset pragma at the end of the list of files so that this
information is not unintentionally visible in other files.
ENCRYPTION OUTPUT: none
DECRYPTION INPUT: none

<!-- Page 969 -->

IEEE Std #### 34.5.32 viewport
34.5.32.1 Syntax
viewport = ( object = <string> , access = <string> )
34.5.32.2 Description
The viewport pragma expression describes objects within the current protected envelope for which access
shall be permitted by the SystemVerilog tool. The specified object name shall be contained within the
current envelope. The access value is an implementation-specific relaxation of protection.

<!-- Page 970 -->

IEEE Std Part Three:
Application Programming Interfaces

<!-- Page 971 -->

IEEE Std 1800-2023