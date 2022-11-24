const XOROSHIRO_CONST0: u64 = 0x0F4B17A579F18960;
const XOROSHIRO_CONST: u64 = 0x82A2B175229D6A5B;

#[derive(Copy, Clone)]
pub struct Xoroshiro128Plus {
    pub s0: u64,
    pub s1: u64,
}

impl Default for Xoroshiro128Plus {
    fn default() -> Self {
        Self {
            s0: XOROSHIRO_CONST0,
            s1: XOROSHIRO_CONST,
        }
    }
}

impl ToString for Xoroshiro128Plus {
    fn to_string(&self) -> String {
        format!("{:X}{:X}", self.s1, self.s0)
    }
}

impl Xoroshiro128Plus {
    pub fn new(s0: u64) -> Self {
        Self {
            s0,
            s1: XOROSHIRO_CONST,
        }
    }

    pub fn from_state(state: (u64, u64)) -> Self {
        Self {
            s0: state.0,
            s1: state.1,
        }
    }

    pub fn get_state(&self) -> (u64, u64) {
        (self.s0, self.s1)
    }

    #[allow(clippy::should_implement_trait)]
    pub fn next(&mut self) -> u64 {
        let s0 = self.s0;
        let mut s1 = self.s1;
        let result = s1.wrapping_add(s0);

        s1 = self.s1 ^ s0;
        self.s0 = s1 ^ (s0 >> 0x28 | s0 << 0x18) ^ s1 << 0x10;
        self.s1 = s1 >> 0x1b | s1 << 0x25;

        result
    }

    pub fn next_u32(&mut self) -> u32 {
        self.next() as u32
    }

    pub fn next_masked(&mut self, modulo: u64) -> u64 {
        let mask = Self::get_bit_mask(modulo);
        let mut res;
        while {
            res = self.next() & mask;
            res >= modulo
        } {}
        res
    }

    fn get_bit_mask(mut x: u64) -> u64 {
        x -= 1;
        x |= x >> 1;
        x |= x >> 2;
        x |= x >> 4;
        x |= x >> 8;
        x |= x >> 16;
        x
    }
}
