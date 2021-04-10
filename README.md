# datgrunch2

[Konzept](https://ytrizja.de/~zseri/datgrunch_Konzept.txt)

## Message types

There are multiple types of messages:

### receiver-specific

* DMs (public-key crypto), usually used to send invites to group chats,
  but can also be used to just publish encrypted private messages,
  but without direct sender verification

* GMs (secret-key crypto), used for group messages

### content-specific

Messages contain multiple blocks of information, are encoded in CBOR

* invites, contain crypto keys for group chats
* message content
* embedded remote hashes (URL + BLAKE3)
* inline attachments (mime type + content)
* maybe signatures of content

## hoster directory hierarchy

```
dg2.dat: contains as CBOR:
- mapping {nickname -> pubkey}

idx.dat: contains as CBOR+zstd:
- {d, g} listing of all files in this directory, sorted by publication date

d: folder, contains DMs
d/*: contains message data
g: folder, contains GMs
g/*: contains message data
```
