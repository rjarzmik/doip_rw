#[cfg(test)]
use crate::Payload;
use crate::{read_message, write_message, DOIP_HEADER_LENGTH};
use std::io::Cursor;

fn assert_decode_smaller<P>(input: &[u8], _expected: &P)
where
    P: Payload + PartialEq + std::fmt::Debug,
{
    if !input.is_empty() {
        let res = P::read(&mut Cursor::new(&input), input.len() - 1);
        assert!(res.is_err());
    }
}

fn assert_decode_bigger<P>(input: &[u8], _expected: &P)
where
    P: Payload + PartialEq + std::fmt::Debug,
{
    let mut bigger = Vec::new();
    bigger.extend_from_slice(input);
    bigger.push(0x12);
    let res = P::read(&mut Cursor::new(&bigger), bigger.len());
    assert!(res.is_err());
}

fn assert_decode_corrupted<P>(input: &[u8], expected: &P, xor_corruptor: u8)
where
    P: Payload + PartialEq + std::fmt::Debug,
{
    assert_ne!(xor_corruptor, 0);
    if input.is_empty() {
        return;
    }
    for i in 0..(input.len() - 1) {
        let mut corrupted = Vec::from(input);
        corrupted[i] ^= xor_corruptor;
        let res = P::read(&mut Cursor::new(&corrupted), corrupted.len());
        if let Ok(p) = res {
            assert_ne!(&p, expected);
        }
    }
}

fn assert_decode_expected<P>(input: &[u8], expected: &P)
where
    P: Payload + PartialEq + std::fmt::Debug,
{
    let res = P::read(&mut Cursor::new(&input), input.len());
    match res {
        Ok(msg) => assert_eq!(&msg, expected),
        Err(_) => panic!("Error in decoding input {:?}", input),
    }
}

fn assert_decode_with_header<P>(input: &[u8], expected: &P)
where
    P: Payload + PartialEq + std::fmt::Debug,
{
    let res: Result<P, crate::DoIpError> = read_message(&mut Cursor::new(&input));
    match res {
        Ok(msg) => assert_eq!(&msg, expected),
        Err(_) => panic!("Error in decoding input {:?}", input),
    }
}

pub fn assert_decode_no_length_change<P>(expected: &P, input: &[u8])
where
    P: Payload + PartialEq + std::fmt::Debug,
{
    let payload = &input[DOIP_HEADER_LENGTH..];

    assert_decode_expected(payload, expected);
    assert_decode_corrupted(payload, expected, 0x01);
    assert_decode_corrupted(payload, expected, 0xaa);
    assert_decode_corrupted(payload, expected, 0xff);
}

pub fn assert_decode<P>(expected: &P, input: &[u8])
where
    P: Payload + PartialEq + std::fmt::Debug,
{
    let payload = &input[DOIP_HEADER_LENGTH..];

    assert_decode_smaller(payload, expected);
    assert_decode_bigger(payload, expected);
    assert_decode_no_length_change(expected, input);
    assert_decode_with_header(input, expected);
}

pub fn assert_encode<P>(input: &P, expected: &[u8])
where
    P: Payload + PartialEq + std::fmt::Debug,
{
    let mut v: Vec<u8> = vec![];
    write_message(input, &mut Cursor::new(&mut v)).unwrap();
    assert_eq!(v, expected);
}
