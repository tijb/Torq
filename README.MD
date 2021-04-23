# Torq

A Torrent client implemented from scratch in Rust. (Excluding: Crypto & HTTP)

**Project Goals**

- Implement the pieces of a functioning torrent client from scratch. (Excluding tangential dependencies like SHA hashing
  and HTTP libraries.)
- Cross-platform (support any OS that Rust supports.)
- Minimal / Lightweight. Take inspiration from the unix philosophy of just doing the one thing well.

**Protocols and Research Readings**

- [Bencode File Protocol](https://en.wikipedia.org/wiki/Bencode)
- [BitTorrent Protocol Specification](https://www.bittorrent.org/beps/bep_0003.html)

### Status

- [x] bencode file parsing
- [x] bencode file encoding
- [ ] `Torrent` struct generation from a `Bencode` struct.
- [ ] **BitTorrent Protocol**
  - [ ] communicate with tracker
  - [ ] download pieces into temp file(s)?
  - [ ] assemble completed file from pieces
  - [ ] SHA hash checking to verify file
- [ ] **User Interface**
  - [ ] implement simple CLI interface utilizing flags for options.
- [ ] **Stretch Goals / If-I-Feel-Like-Its**
  - [ ] investigate windows-rs crate to build native Windows UI.
  - [ ] look into adding the ability to host/list a file (tracker).
