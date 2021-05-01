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

impl TorrentFiles {
    fn from(bmap: BMap) -> Option<TorrentFiles> {
        let file_map: BMap = match bmap.get("files") {
            Some(f) => f.to_map(),
            None => return None,
        };

        Some(TorrentFiles {
            length: file_map
                .get("length")
                .expect("No length value found in files dict.")
                .to_usize(),
            path: file_map
                .get("path")
                .expect("No path found in files dict.")
                .to_vec()
                .iter()
                .map(|b| b.to_string())
                .collect::<Vec<String>>(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn torrentfiles_parse() {
        let benc: Benc =
            Benc::parse(&b"d5:filesd6:lengthi10e4:pathl3:foo3:bareee".to_vec()).unwrap();
        let bmap: BMap = benc.to_map();
        println!("{:?}", bmap);
        let tor_files: TorrentFiles = TorrentFiles::from(bmap).unwrap();

        assert_eq!(10, tor_files.length);
        assert_eq!("foo", tor_files.path.get(0).unwrap());
        assert_eq!("bar", tor_files.path.get(1).unwrap());
    }

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
