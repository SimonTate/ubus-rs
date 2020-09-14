use std::io::{Read, Write};
use std::os::unix::net::UnixStream;
use std::{print, println};
use ubus::*;

#[test]
fn test() {
    let (client, mut server) = UnixStream::pair().unwrap();

    std::thread::spawn(move || {
        server.write_all(TEST_HELLO).unwrap();
        let mut command = [0u8; TEST_TX.len()];
        server.read_exact(&mut command).unwrap();
        assert_eq!(&command[..], &TEST_TX[..]);
        for i in TEST_RX {
            server.write_all(i).unwrap();
        }
    });

    let mut connection = Connection::new(client).unwrap();

    connection
        .invoke(0x13333337, "info", &[], |x| {
            for i in x {
                std::dbg!(i);
            }
        })
        .unwrap();
}

const TEST_HELLO: &[u8] = &[
    0x00, 0x00, 0x00, 0x00, 0x2e, 0xb8, 0x63, 0xdb, 0x00, 0x00, 0x00, 0x04,
];

const TEST_TX: &[u8] = &[
    0x00, 0x05, 0x00, 0x01, 0x13, 0x33, 0x33, 0x37, 0x00, 0x00, 0x00, 0x1c, 0x03, 0x00, 0x00, 0x08,
    0x13, 0x33, 0x33, 0x37, 0x04, 0x00, 0x00, 0x09, 0x69, 0x6e, 0x66, 0x6f, 0x00, 0x00, 0x00, 0x00,
    0x07, 0x00, 0x00, 0x04,
];

const TEST_RX: &[&[u8]] = &[
    &[
        0x00, 0x02, 0x00, 0x01, 0x13, 0x33, 0x33, 0x37, 0x00, 0x00, 0x01, 0x34,
    ],
    &[
        0x03, 0x00, 0x00, 0x08, 0x13, 0x33, 0x33, 0x77, 0x07, 0x00, 0x01, 0x28, 0x85, 0x00, 0x00,
        0x14, 0x00, 0x09, 0x6c, 0x6f, 0x63, 0x61, 0x6c, 0x74, 0x69, 0x6d, 0x65, 0x00, 0x5f, 0x5e,
        0x51, 0x6c, 0x85, 0x00, 0x00, 0x14, 0x00, 0x06, 0x75, 0x70, 0x74, 0x69, 0x6d, 0x65, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x03, 0x9b, 0x81, 0x00, 0x00, 0x30, 0x00, 0x04, 0x6c, 0x6f,
        0x61, 0x64, 0x00, 0x00, 0x85, 0x00, 0x00, 0x0c, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x85, 0x00, 0x00, 0x0c, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x85, 0x00,
        0x00, 0x0c, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x82, 0x00, 0x00, 0x98, 0x00,
        0x06, 0x6d, 0x65, 0x6d, 0x6f, 0x72, 0x79, 0x00, 0x00, 0x00, 0x00, 0x84, 0x00, 0x00, 0x14,
        0x00, 0x05, 0x74, 0x6f, 0x74, 0x61, 0x6c, 0x00, 0x00, 0x00, 0x00, 0x00, 0x1e, 0x9a, 0x60,
        0x00, 0x84, 0x00, 0x00, 0x14, 0x00, 0x04, 0x66, 0x72, 0x65, 0x65, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x1c, 0x51, 0x30, 0x00, 0x84, 0x00, 0x00, 0x18, 0x00, 0x06, 0x73, 0x68, 0x61,
        0x72, 0x65, 0x64, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00,
        0x84, 0x00, 0x00, 0x18, 0x00, 0x08, 0x62, 0x75, 0x66, 0x66, 0x65, 0x72, 0x65, 0x64, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x0d, 0x20, 0x00, 0x84, 0x00, 0x00, 0x18, 0x00, 0x09,
        0x61, 0x76, 0x61, 0x69, 0x6c, 0x61, 0x62, 0x6c, 0x65, 0x00, 0x00, 0x00, 0x00, 0x00, 0x1b,
        0x17, 0xa0, 0x00, 0x84, 0x00, 0x00, 0x18, 0x00, 0x06, 0x63, 0x61, 0x63, 0x68, 0x65, 0x64,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x99, 0x70, 0x00, 0x82, 0x00, 0x00,
        0x34, 0x00, 0x04, 0x73, 0x77, 0x61, 0x70, 0x00, 0x00, 0x84, 0x00, 0x00, 0x14, 0x00, 0x05,
        0x74, 0x6f, 0x74, 0x61, 0x6c, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x84,
        0x00, 0x00, 0x14, 0x00, 0x04, 0x66, 0x72, 0x65, 0x65, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00,
    ],
    &[
        0x00, 0x01, 0x00, 0x01, 0x13, 0x33, 0x33, 0x77, 0x00, 0x00, 0x00, 0x14,
    ],
    &[
        0x01, 0x00, 0x00, 0x08, 0x00, 0x00, 0x00, 0x00, 0x03, 0x00, 0x00, 0x08, 0x85, 0x22, 0x59,
        0x61,
    ],
];
