# datgrunch2

[Konzept](https://ytrizja.de/~zseri/datgrunch_Konzept.txt)

## Security considerations

This package has received no security audits at all and is only a Proof-Of-Concept
experiment. I try to leverage `sodiumoxide` to take care of proper low-level crypto,
but this doesn't mean that my code can't do dumb stuff...

To get around the previous restriction that plausible deniability is hard in the
current design, I propose the following solution: just publish the signing private key.

### Goals

* Group chats should be flexible, e.g. it should be trivial to spawn a
  new group chat out of an existing one.
* Group chat flows should allow for members to be added and removed, so that they
  don't have access to any messages which were posted before they joined, and don't
  have access to any messages which were posted after they were kicked/left
  (except when a malicious member compromises the group key, which does reveal all
   information posted with that group key)
  (this is basically done by generating new groups+keys on the fly when members
   need to be added/removed)

use case: an agent gets invited into a group chat, but shouldn't be able
to access all old information posted into the chat, as that could reveal
private information to a participant who can't be trusted yet, because
they haven't interacted with most group participants yet at all probably.

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
