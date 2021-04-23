use crate::bencode::*;

#[derive(Debug)]
pub struct Torrent {
    pub announce: String,
    pub info: TorrentInfo,
}

#[derive(Debug)]
pub struct TorrentInfo {
    pub files: Option<TorrentFiles>,
    pub length: Option<usize>,
    pub name: String,
    pub piece_length: usize,
    pub pieces: Vec<u8>,
}

#[derive(Debug)]
pub struct TorrentFiles {
    pub length: usize,
    pub path: Vec<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

//
    // #[test]
    // fn it_works() {
    //     let torrent_bytes =
    //         std::fs::read("ubuntu.torrent").expect("Could not find ubuntu.torrent file.");
    //
    //     let torrent: Torrent = Torrent::from_bytes(torrent_bytes.as_slice());
    //     let info: TorrentInfo = torrent.info;
    //
    //     assert_eq!("https://torrent.ubuntu.com/announce", torrent.announce);
    //     assert!(info.files.is_none());
    //     assert_eq!(2942003200, info.length.expect("this should be populated"));
    //     assert_eq!("ubuntu-20.10-desktop-amd64.iso", info.name);
    //     assert_eq!(262144, info.piece_length);
    //
    //     println!("piece length: {:?}", info.piece_length);
    // }
}
