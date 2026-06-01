# Annex O: Annex O

## Annex O
(informative)
Encryption/decryption flow
O.1 General
This annex describes a number of scenarios that can be used for IP protection. It also shows how the relevant
pragmas are used to achieve the desired effect of securely protecting, distributing, and decrypting the model.
O.2 Overview
The data to be protected from inappropriate access or from unauthorized modification is placed within a
protect begin-end block. Information in the begin-end block, once encrypted, is also protected.
O.3 Tool vendor secret key encryption system
In the secret key encryption system, the key is tool vendor proprietary and is embedded within the tool itself.
The same key is used for both encryption and decryption. (In the electronic design automation (EDA)
domain, this is the simplest scenario and is roughly equivalent to the historical `protect technique.) It has
the drawback of being completely tool vendor-specific. Using this technique, the IP author can encrypt the
IP, and any IP consumer with appropriate licenses and the same tool vendor can utilize the IP.
O.3.1 Encryption input
The following pragmas are expected when using the tool vendor secret key encryption system. The pragmas
required in the encryption input for use of the secret key encryption system are as follows:
data_keyname=<key name> Where <key name> is a valid name of a tool’s embedded key.
begin-end Surrounding the region(s) to be encrypted.
Additional optional pragmas that may be included are as follows:
author=<string> To embed author name.
author_info=<string> To embed arbitrary author information.
data_keyowner=<owner identity>This shall be the key owner of the provided name.
data_method=<method-specifier>A method appropriate for the given key name. This may be
necessary if something other than the default number of rounds,
initialization vector, or key width is used.
encoding=<encoding-specifier> To specify a different encoding.
digest_block If a message authorization code is desired to validate that the
message has not been modified.
decrypt_license If the IP author desires a decryption license.
runtime_license If the IP author desires a run-time license.

<!-- Page 1347 -->

IEEE Std O.3.2 Encryption output
The encrypting tool should take the input file and copy all cleartext to the corresponding output sections. For
each protect begin-end block, it should generate the following:
begin_protected To start the protected region.
data_keyowner= <owner identity>
data_keyname=<key name>
data_method=<method-specifier>
encoding=<encoding-specifier>
author=<string> If provided in the input.
author_info=<string> If provided in the input.
digest_block Followed on the next line(s) by the encoded encrypted digest.
data_block Followed on the next line(s) by the encoded encrypted data
composed of the following:
decrypt_license
encrypt_license
<text found between begin-end>
end_protected
O.4 IP author secret key encryption system
In this mechanism, the IP is encrypted with the public key (of a public/private key pair) of the IP author, and
the decrypting tool will have the IP author’s private key in its secure key database. The IP authors will have
to provide their private keys to the tools’ database so that the tool will be able to decrypt the design.
O.4.1 Encryption input
The following pragmas are expected when using the IP author secret key encryption system:
data_keyname=< provider’s key name>
begin-end Surrounding the region(s) to be encrypted.
Additional optional pragmas that may be included are as follows:
author=<string> To embed author name.
author_info=<string> To embed arbitrary author information.
data_keyowner=<owner identity>This shall be the key owner of the provided name.
data_method= some_publ_priv_encryption_scheme_name <method-specifier>
A method appropriate for the given key name. This may be
necessary if something other than the default number of rounds,
initialization vector, or key width is used.
encoding=<encoding-specifier> To specify a different encoding.
digest_block If a message authorization code is desired to validate that the
message has not been modified.
decrypt_license If the IP author desires a decryption license.
runtime_license If the IP author desires a run-time license.

<!-- Page 1348 -->

IEEE Std O.4.2 Encryption output
The encrypting tool should take the input file and copy all cleartext to the corresponding output sections. For
each protect begin-end block, it should generate the following:
begin_protected To start the protected region.
data_keyowner=<owner identity>
data_keyname=<provider’s key name>
data_method=some_publ_priv_encryption_scheme_name
encoding=<encoding-specifier>
author=<string> If provided in the input.
author_info=<string> If provided in the input.
digest_block Followed on the next line(s) by the encoded encrypted digest.
data_block Followed on the next line(s) by the encoded encrypted data
composed of the following:
decrypt_license
encrypt_license
<text found between begin-end>
end_protected
O.5 Digital envelopes
In this mechanism, each recipient has a public and private key for an asymmetric encryption algorithm. The
sender encrypts the design using a symmetric key encryption algorithm and then encrypts the symmetric key
using the recipient’s public key. The encrypted symmetric key is recorded in a key_block in the protected
envelope. The recipient is able to recover the symmetric key using the appropriate private key and then
decrypts the design with the symmetric key. This technique permits efficient encryption methods for the
design data, yet secret information is never transmitted without encryption. Digital envelopes can be created
using either tool secret key or IP author secret key protection schemes. The keys for the recipient user or tool
protect the transmission of the symmetric key that encrypts the design data. By using more than one
key_block, a single protected envelope can be decrypted by tools from different vendors and/or different
users.
In the following example, the data_method and data_keyowner/data_keyname are used to encrypt the
data_block. The key to encrypt the data_block can be specified either by a data_keyowner/
data_keyname pair or by a data_decrypt_key pragma expression. In the first case, the encrypting tool
encrypts the data_keyowner and data_keyname pragmas with the key_keymethod/key_keyname and
puts them in the key_block along with data_method. Alternatively, with the data_decrypt_key
pragma, the actual key is provided, which is then encrypted with key_method/key_keyname and stored in
the key_block.
In the first approach, the data_keyowner/data_keyname should also be present with the decrypting tool.
No such dependency exists with the second approach as the key is present in the file itself.
For better security in the first approach, the encrypting tool can actually read the data_keyowner/
data_keyname key and put it in the key_block as data_decrypt_key. This step not only will remove
the dependency mentioned above, but will also protect against the hit-and-trial breaking of the data_block
with the existing keys at the IP user’s end.

<!-- Page 1349 -->

IEEE Std O.5.1 Encryption input
The following pragmas are expected when using the digital envelopes:
key_keyowner=<owner identity>
key_method=some_encryption_scheme_name
key_keyname=<provider’s key name>
data_keyname=<provider’s key name>
begin-end Surrounding the region(s) to be encrypted.
Additional optional pragmas that may be included are as follows:
author=<string> To embed author name.
author_info=<string> To embed arbitrary author information.
data_keyowner=<owner identity>This shall be the key owner of the provided name.
data_method=<method-specifier>A method appropriate for the given key name. This may be
necessary if something other than the default number of rounds,
initialization vector, or key width is used
encoding=<encoding-specifier> To specify a different encoding.
digest_block If a message authorization code is desired to validate that the
message has not been modified.
decrypt_license If the IP author desires a decryption license.
runtime_license If the IP author desires a run-time license.
O.5.2 Encryption output
The encrypting tool should take the input file and copy all cleartext to the corresponding output sections. For
each protect begin-end block, it should generate the following:
begin_protected To start the protected region.
key_keyowner=<owner identity>
key_method=some_encryption_scheme_name
key_keyname=<provider’s key name>
key_block=<encrypted encoded data>
This contains the data_key_owner, data_method, and the symmetric
data_key itself in encrypted form.
encoding=<encoding-specifier>
author=<string> If provided in the input.
author_info=<string> If provided in the input.
digest_block Followed on the next line(s) by the encoded encrypted digest.
data_block Followed on the next line(s) by the encoded encrypted data
composed of the following:
decrypt_license
encrypt_license
<text found between begin-end>
end_protected

<!-- Page 1350 -->

IEEE Std 1800-2023