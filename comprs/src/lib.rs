use std::{
    char,
    collections::{BinaryHeap, HashMap},
    fs::File,
    io::{Read, Write},
    mem::size_of,
    path::PathBuf,
    str::{self},
};

use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};

#[derive(Clone, Eq, PartialEq, Debug)]
pub struct Node {
    ch: Option<char>,
    freq: u32,
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
}

impl Node {
    fn new(ch: Option<char>, freq: u32) -> Self {
        Node {
            ch,
            freq,
            left: None,
            right: None,
        }
    }
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other
            .freq
            .cmp(&self.freq)
            .then_with(|| other.ch.cmp(&self.ch))
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl From<&mut BinaryHeap<Node>> for Node {
    /// Construct a huffman tree from a priority queue
    fn from(value: &mut BinaryHeap<Node>) -> Self {
        while value.len() > 1 {
            let a = value.pop().unwrap();
            let b = value.pop().unwrap();

            let mut c = Node::new(None, a.freq + b.freq);

            c.left = Some(Box::new(a));
            c.right = Some(Box::new(b));

            value.push(c);
        }

        value.pop().unwrap()
    }
}

#[derive(Serialize, Deserialize)]
pub struct HuffmanTable(HashMap<char, String>);

impl HuffmanTable {
    fn new() -> Self {
        HuffmanTable(HashMap::new())
    }

    fn insert(&mut self, ch: char, code: String) {
        self.0.insert(ch, code);
    }

    fn get(&self, ch: &char) -> Option<&String> {
        self.0.get(ch)
    }

    fn invert(&self) -> HashMap<&String, &char> {
        self.0.iter().map(|(k, v)| (v, k)).collect()
    }
}

impl From<&String> for HuffmanTable {
    // Constructs a mapping of characters to their huffman encoding
    fn from(value: &String) -> Self {
        fn traverse(table: &mut HuffmanTable, node: &Node, code: String) {
            if let Some(ch) = node.ch {
                table.insert(ch, code);
            } else {
                if let Some(left) = &node.left {
                    traverse(table, left, code.clone() + "0");
                }
                if let Some(right) = &node.right {
                    traverse(table, right, code.clone() + "1");
                }
            }
        }

        let counts = value.chars().fold(HashMap::new(), |mut counts, c| {
            *counts.entry(c).or_insert(0) += 1;
            counts
        });

        let mut heap: BinaryHeap<_> = counts
            .into_iter()
            .map(|(c, count)| Node::new(Some(c), count))
            .collect();

        let root = Node::from(&mut heap);

        let mut table = HuffmanTable::new();
        let code = String::from("");

        traverse(&mut table, &root, code);
        table
    }
}

pub fn encode(source: &PathBuf, destination: &PathBuf) -> Result<()> {
    let contents =
        std::fs::read_to_string(source).context(format!("failed to open file `{:?}`", source))?;
    let table = HuffmanTable::from(&contents);

    let encoded = to_bitstring(&contents, &table)?;
    let (compressed, bits_filled) = compress(&encoded);
    let encoded_tbl = bincode::serialize(&table).context("failed to serialize table")?;

    write_encoded(destination, encoded_tbl, compressed, bits_filled)?;

    Ok(())
}

fn write_encoded(
    destination: &PathBuf,
    encoded_tbl: Vec<u8>,
    encoded: Vec<u8>,
    bits_filled: u8,
) -> Result<(), anyhow::Error> {
    let mut file =
        File::create(destination).context(format!("failed to create file `{:?}`", destination))?;

    file.write_all(&(encoded_tbl.len() as u64).to_le_bytes())?;
    file.write_all(&encoded_tbl).context(format!(
        "could not write contents to file `{:?}`",
        destination
    ))?;
    file.write_all(&encoded)
        .context("could not write encoded contents to file")?;
    file.write_all(&[bits_filled])
        .context("could not write number of bits filled")?;

    Ok(())
}

pub fn to_bitstring(contents: &str, table: &HuffmanTable) -> Result<String> {
    Ok(contents.chars().fold(String::new(), |mut acc, c| {
        acc.push_str(
            table
                .get(&c)
                .context(format!("no encoding for {}", c))
                .unwrap(),
        );
        acc
    }))
}

fn compress(encoded: &str) -> (Vec<u8>, u8) {
    let mut bytes: Vec<u8> = Vec::new();
    let mut current_byte = 0u8;
    let mut bits_filled = 0;

    for bit in encoded.chars() {
        if bit == '1' {
            current_byte |= 1 << (7 - bits_filled);
        }

        bits_filled += 1;

        if bits_filled == 8 {
            bytes.push(current_byte);
            current_byte = 0;
            bits_filled = 0;
        }
    }

    if bits_filled > 0 {
        bytes.push(current_byte);
    } else {
        bits_filled = 8;
    }

    (bytes, bits_filled as u8)
}

pub fn decode(source: &PathBuf, destination: &PathBuf) -> Result<()> {
    let (table, contents, bits_filled) = read_encoded(source)?;
    let table = table.invert();
    let bit_string = bitstring_from_bytes(&contents, bits_filled);
    let contents = bitstring_to_text(&bit_string, &table)?;

    let mut file =
        File::create(destination).context(format!("failed to create file `{:?}`", destination))?;
    file.write_all(contents.as_bytes()).context(format!(
        "could not write contents to file `{:?}`",
        destination
    ))?;

    Ok(())
}

fn read_encoded(source: &PathBuf) -> Result<(HuffmanTable, Vec<u8>, u8), anyhow::Error> {
    let mut file = File::open(source).context(format!("could not open file `{:?}`", source))?;

    let mut buffer = [0u8; size_of::<u64>()];
    file.read_exact(&mut buffer)
        .context("could not read length of encoded huffman tree")?;

    let tree_len = u64::from_le_bytes(buffer) as usize;
    let mut tree = vec![0u8; tree_len];
    file.read_exact(&mut tree)
        .context("could not read huffman tree")?;
    let table: HuffmanTable =
        bincode::deserialize(&tree[..]).context("could not deserialize huffman tree")?;

    let mut contents = Vec::new();
    file.read_to_end(&mut contents)
        .context("could not read encoded contents from file")?;

    let valid_bits = contents.pop().unwrap();

    Ok((table, contents, valid_bits))
}

fn bitstring_from_bytes(bytes: &[u8], valid_bits: u8) -> String {
    let mut bit_string = String::new();

    for (i, &byte) in bytes.iter().enumerate() {
        let bits_to_read = if i == bytes.len() - 1 { valid_bits } else { 8 };
        for j in (8 - bits_to_read..8).rev() {
            let bit = (byte >> j) & 1;
            bit_string.push(if bit == 1 { '1' } else { '0' });
        }
    }

    bit_string
}

pub fn bitstring_to_text(contents: &str, table: &HashMap<&String, &char>) -> Result<String> {
    let mut out = String::new();

    let mut start = 0;
    let mut end = 1;

    while end <= contents.len() {
        if let Some(ch) = table.get(&contents[start..end].to_string()) {
            out.push(**ch);
            start = end;
        }

        end += 1;
    }

    Ok(out)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encoding_table() {
        // Arrange
        let counts = HashMap::from([
            ('C', 32),
            ('D', 42),
            ('E', 120),
            ('K', 7),
            ('L', 42),
            ('M', 24),
            ('U', 37),
            ('Z', 2),
        ]);
        let contents = counts
            .iter()
            .map(|(k, v)| k.to_string().repeat(*v as usize))
            .collect::<String>();

        // Act
        let table = HuffmanTable::from(&contents);

        // Assert
        assert_eq!(table.get(&'C').unwrap(), "1110");
        assert_eq!(table.get(&'D').unwrap(), "101");
        assert_eq!(table.get(&'E').unwrap(), "0");
        assert_eq!(table.get(&'K').unwrap(), "111101");
        assert_eq!(table.get(&'L').unwrap(), "110");
        assert_eq!(table.get(&'M').unwrap(), "11111");
        assert_eq!(table.get(&'U').unwrap(), "100");
        assert_eq!(table.get(&'Z').unwrap(), "111100");
    }

    #[test]
    fn test_byte_from_str() {
        let test_cases = vec![
            (String::from("10010110"), vec![0b10010110], 8),
            (String::from("10010110010"), vec![0b10010110, 0b01000000], 3),
            (
                String::from("1001011010010110"),
                vec![0b10010110, 0b10010110],
                8,
            ),
        ];

        for (input, expected_vec, expected_bits) in test_cases {
            let (result, valid_bits) = compress(&input);
            assert_eq!(result, expected_vec);
            assert_eq!(valid_bits, expected_bits);
        }
    }

    #[test]
    fn test_decompress() {
        let test_cases = vec![
            (vec![0b10010110, 0b01000000], 3, "10010110010"),
            (vec![0b10010110], 8, "10010110"),
        ];

        for (input, valid_bits, expected) in test_cases {
            let result = bitstring_from_bytes(&input, valid_bits);
            assert_eq!(&result, expected);
        }
    }
}
