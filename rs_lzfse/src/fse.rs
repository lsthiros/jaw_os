
type FseBitCount = i32;

fn fse_mask_lsb64(x: u64, nbits: i32) -> u64 {
    const MTABLE: [u64; 65] = [
        0x0000000000000000, 0x0000000000000001, 0x0000000000000003,
        0x0000000000000007, 0x000000000000000f, 0x000000000000001f,
        0x000000000000003f, 0x000000000000007f, 0x00000000000000ff,
        0x00000000000001ff, 0x00000000000003ff, 0x00000000000007ff,
        0x0000000000000fff, 0x0000000000001fff, 0x0000000000003fff,
        0x0000000000007fff, 0x000000000000ffff, 0x000000000001ffff,
        0x000000000003ffff, 0x000000000007ffff, 0x00000000000fffff,
        0x00000000001fffff, 0x00000000003fffff, 0x00000000007fffff,
        0x0000000000ffffff, 0x0000000001ffffff, 0x0000000003ffffff,
        0x0000000007ffffff, 0x000000000fffffff, 0x000000001fffffff,
        0x000000003fffffff, 0x000000007fffffff, 0x00000000ffffffff,
        0x00000001ffffffff, 0x00000003ffffffff, 0x00000007ffffffff,
        0x0000000fffffffff, 0x0000001fffffffff, 0x0000003fffffffff,
        0x0000007fffffffff, 0x000000ffffffffff, 0x000001ffffffffff,
        0x000003ffffffffff, 0x000007ffffffffff, 0x00000fffffffffff,
        0x00001fffffffffff, 0x00003fffffffffff, 0x00007fffffffffff,
        0x0000ffffffffffff, 0x0001ffffffffffff, 0x0003ffffffffffff,
        0x0007ffffffffffff, 0x000fffffffffffff, 0x001fffffffffffff,
        0x003fffffffffffff, 0x007fffffffffffff, 0x00ffffffffffffff,
        0x01ffffffffffffff, 0x03ffffffffffffff, 0x07ffffffffffffff,
        0x0fffffffffffffff, 0x1fffffffffffffff, 0x3fffffffffffffff,
        0x7fffffffffffffff, 0xffffffffffffffff,
    ];
    x & MTABLE[nbits as usize]
}


fn fse_mask_lsb32(x: u32, nbits: u32) -> u32 {
    const MTABLE: [u32; 33] = [
        0x00000000, 0x00000001, 0x00000003, 0x00000007, 0x0000000f, 0x0000001f,
        0x0000003f, 0x0000007f, 0x000000ff, 0x000001ff, 0x000003ff, 0x000007ff,
        0x00000fff, 0x00001fff, 0x00003fff, 0x00007fff, 0x0000ffff, 0x0001ffff,
        0x0003ffff, 0x0007ffff, 0x000fffff, 0x001fffff, 0x003fffff, 0x007fffff,
        0x00ffffff, 0x01ffffff, 0x03ffffff, 0x07ffffff, 0x0fffffff, 0x1fffffff,
        0x3fffffff, 0x7fffffff, 0xffffffff,
    ];
    x & MTABLE[nbits as usize]
}

fn fse_extract_bits64(x: u64, start: u32, nbits: u32) -> u64 {
    fse_mask_lsb64(x >> start, nbits)
}

fn fse_extract_bits32(x: u32, start: u32, nbits: u32) -> u32 {
    fse_mask_lsb32(x >> start, nbits)
}

#[derive(Debug)]
struct FseOutStream64 {
    accum: u64,            // Output bits
    accum_nbits: FseBitCount, // Number of valid bits in ACCUM, other bits are 0
}

#[derive(Debug)]
struct FseOutStream32 {
    accum: u32,            // Output bits
    accum_nbits: FseBitCount, // Number of valid bits in ACCUM, other bits are 0
}

#[derive(Debug)]
struct FseInStream64 {
    accum: u64,            // Input bits
    accum_nbits: FseBitCount, // Number of valid bits in ACCUM, other bits are 0
}

#[derive(Debug)]
struct FseInStream32 {
    accum: u32,            // Input bits
    accum_nbits: FseBitCount, // Number of valid bits in ACCUM, other bits are 0
}

impl FseOutStream64 {
    fn new() -> Self {
        Self {
            accum: 0,
            accum_nbits: 0,
        }
    }

    fn out_flush(&mut self, buf: &mut [u8]) {
        let nbits = self.accum_nbits & -8; // number of bits written, multiple of 8

        // Write 8 bytes of current accumulator
        buf[..8].copy_from_slice(&self.accum.to_le_bytes());
        buf.rotate_left((nbits >> 3) as usize); // bytes

        // Update state
        self.accum >>= nbits; // remove nbits
        self.accum_nbits -= nbits;
    }

    fn out_finish(&mut self, buf: &mut [u8]) {
        let nbits = (self.accum_nbits + 7) & -8; // number of bits written, multiple of 8

        // Write 8 bytes of current accumulator
        buf[..8].copy_from_slice(&self.accum.to_le_bytes());
        buf.rotate_left((nbits >> 3) as usize); // bytes

        // Update state
        self.accum = 0; // remove nbits
        self.accum_nbits -= nbits;

        assert!(self.accum_nbits >= -7 && self.accum_nbits <= 0);
    }

    fn out_push(&mut self, n: FseBitCount, b: u64) {
        self.accum |= b << self.accum_nbits;
        self.accum_nbits += n;

        assert!(self.accum_nbits >= 0 && self.accum_nbits <= 64);
        assert!(self.accum_nbits == 64 || (self.accum >> self.accum_nbits) == 0);
    }
}

// The previous is the initialization function of the input stream in C. What follows
// is the initialization function in Rust. This function is called "new", and instead
// of a double pointer and a pointer, it takes a mutable reference to a slice of u8s
// and an index (called "buf_start") into that slice.

fn new(buf: &mut [u8], buf_start: usize) -> Self {
    let mut s = Self {
        accum: 0,
        accum_nbits: 0,
    };

    if n != 0 {
        if buf_start < 8 {
            panic!("out of range");
        }
        buf_start -= 8;
        s.accum = u64::from_le_bytes(buf[buf_start..buf_start + 8]);
        s.accum_nbits = n + 64;
    } else {
        if buf_start < 7 {
            panic!("out of range");
        }
        buf_start -= 7;
        s.accum = u64::from_le_bytes(buf[buf_start..buf_start + 7]) & 0xffffffffffffff;
        s.accum_nbits = n + 56;
    }

    if s.accum_nbits < 56 || s.accum_nbits >= 64 || s.accum >> s.accum_nbits != 0 {
        panic!("the incoming input is wrong (encoder should have zeroed the upper bits)");
    }
    s
}
