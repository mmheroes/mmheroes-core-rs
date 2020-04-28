pub(crate) struct Rng {
    state: u64,
}

impl Rng {
    pub(crate) fn new(seed: u64) -> Rng {
        Rng { state: seed }
    }

    pub(crate) fn next(&mut self) -> u64 {
        // http://xoshiro.di.unimi.it/splitmix64.c
        self.state += 0x9e3779b97f4a7c15;
        let mut z = self.state;
        z = (z ^ (z >> 30)) * 0xbf58476d1ce4e5b9;
        z = (z ^ (z >> 27)) * 0x94d049bb133111eb;
        z ^ (z >> 31)
    }
}
