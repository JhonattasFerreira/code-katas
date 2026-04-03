pub struct BitWriter {
    buffer: u8,
    bits_in_buf: u8,
    output: Vec<u8>,
}

impl BitWriter {
    pub fn new() -> Self {
        BitWriter {
            buffer: 0,
            bits_in_buf: 0,
            output: Vec::new(),
        }
    }

    pub fn write_bit(&mut self, bit: bool) {
        if bit {
            self.buffer |= 1 << (7 - self.bits_in_buf);
        }
        self.bits_in_buf += 1;
        if self.bits_in_buf == 8 {
            self.output.push(self.buffer);
            self.buffer = 0;
            self.bits_in_buf = 0;
        }
    }

    pub fn finish(mut self) -> Vec<u8> {
        if self.bits_in_buf > 0 {
            self.output.push(self.buffer);
        }
        self.output
    }
}

pub struct BitReader<'a> {
    data: &'a [u8],
    byte_pos: usize,
    bit_pos: u8,
}

impl<'a> BitReader<'a> {
    pub fn new(data: &'a [u8]) -> Self {
        BitReader {
            data,
            byte_pos: 0,
            bit_pos: 0,
        }
    }

    pub fn read_bit(&mut self) -> Option<bool> {
        if self.byte_pos >= self.data.len() {
            return None;
        }
        let bit = (self.data[self.byte_pos] >> (7 - self.bit_pos)) & 1 == 1;
        self.bit_pos += 1;
        if self.bit_pos == 8 {
            self.byte_pos += 1;
            self.bit_pos = 0;
        }
        Some(bit)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // finish on an empty writer returns an empty vec
    #[test]
    fn test_writer_finish_empty() {
        let w = BitWriter::new();
        assert_eq!(w.finish(), vec![]);
    }

    // 8 bits produce exactly 1 byte
    #[test]
    fn test_writer_8_bits_produce_1_byte() {
        let mut w = BitWriter::new();
        for _ in 0..8 {
            w.write_bit(false);
        }
        assert_eq!(w.finish().len(), 1);
    }

    // bits are written MSB first: [true, false, ...] -> 0b10000000 = 128
    #[test]
    fn test_writer_msb_first() {
        let mut w = BitWriter::new();
        w.write_bit(true);
        for _ in 0..7 {
            w.write_bit(false);
        }
        assert_eq!(w.finish(), vec![0b10000000]);
    }

    // fewer than 8 bits are zero-padded on the right
    #[test]
    fn test_writer_partial_byte_padded_with_zeros() {
        let mut w = BitWriter::new();
        w.write_bit(true);
        w.write_bit(false);
        w.write_bit(true);
        // remaining 5 bits are zero-padded -> 0b10100000 = 160
        assert_eq!(w.finish(), vec![0b10100000]);
    }

    // 16 bits produce exactly 2 bytes
    #[test]
    fn test_writer_16_bits_produce_2_bytes() {
        let mut w = BitWriter::new();
        for _ in 0..16 {
            w.write_bit(true);
        }
        assert_eq!(w.finish().len(), 2);
    }

    // reading from an empty slice returns None immediately
    #[test]
    fn test_reader_empty_returns_none() {
        let mut r = BitReader::new(&[]);
        assert_eq!(r.read_bit(), None);
    }

    // first bit read from 0b10000000 is true (MSB first)
    #[test]
    fn test_reader_reads_msb_first() {
        let mut r = BitReader::new(&[0b10000000]);
        assert_eq!(r.read_bit(), Some(true));
    }

    // reading past the end returns None
    #[test]
    fn test_reader_exhausted_returns_none() {
        let mut r = BitReader::new(&[0b11111111]);
        for _ in 0..8 {
            r.read_bit();
        }
        assert_eq!(r.read_bit(), None);
    }

    // write N bits, read them back — same sequence
    #[test]
    fn test_round_trip() {
        let bits = vec![true, false, true, true, false, false, true, false, true];

        let mut w = BitWriter::new();
        for &b in &bits {
            w.write_bit(b);
        }
        let bytes = w.finish();

        let mut r = BitReader::new(&bytes);
        let mut result = Vec::new();
        for _ in 0..bits.len() {
            result.push(r.read_bit().expect("should have a bit"));
        }

        assert_eq!(result, bits);
    }
}
