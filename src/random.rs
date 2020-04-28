pub(crate) struct Rng {
    state: u64,
}

impl Rng {
    pub(crate) fn new(seed: u64) -> Rng {
        Rng { state: seed }
    }

    pub(crate) fn next(&mut self) -> u64 {
        // http://xoshiro.di.unimi.it/splitmix64.c
        self.state = self.state.wrapping_add(0x9e3779b97f4a7c15);
        let mut z = self.state;
        z = (z ^ z.wrapping_shr(30)).wrapping_mul(0xbf58476d1ce4e5b9);
        z = (z ^ z.wrapping_shr(27)).wrapping_mul(0x94d049bb133111eb);
        z ^ z.wrapping_shr(31)
    }

    pub(crate) fn random_number_with_upper_bound(&mut self, upper_bound: u64) -> u64 {
        // https://github.com/apple/swift/blob/727e2220412ffa576738007404f46925d1c3f635/stdlib/public/core/Random.swift#L98-L106
        assert!(upper_bound > 0, "upper_bound cannot be zero.");
        let tmp = (u64::max_value() % upper_bound) + 1;
        let range = if tmp == upper_bound { 0u64 } else { tmp };
        let mut random;
        loop {
            random = self.next();
            if random >= range {
                break;
            }
        }
        return random % upper_bound;
    }

    pub(crate) fn random_number_in_range(&mut self, range: std::ops::Range<u64>) -> u64 {
        let delta = range.end - range.start;
        return range.start.wrapping_add(self.random_number_with_upper_bound(delta))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_next() {
        let mut rng1 = Rng::new(0);
        assert_eq!(rng1.next(), 16294208416658607535);
        assert_eq!(rng1.next(),  7960286522194355700);
        assert_eq!(rng1.next(),   487617019471545679);
        assert_eq!(rng1.next(), 17909611376780542444);
        assert_eq!(rng1.next(),  1961750202426094747);
        assert_eq!(rng1.next(),  6038094601263162090);

        let mut rng2 = Rng::new(42);
        assert_eq!(rng2.next(), 13679457532755275413);
        assert_eq!(rng2.next(),  2949826092126892291);
        assert_eq!(rng2.next(),  5139283748462763858);
        assert_eq!(rng2.next(),  6349198060258255764);
        assert_eq!(rng2.next(),   701532786141963250);
        assert_eq!(rng2.next(), 16015981125662989062);

        let mut rng3 = Rng::new(u64::max_value());
        assert_eq!(rng3.next(), 16490336266968443936);
        assert_eq!(rng3.next(), 16834447057089888969);
        assert_eq!(rng3.next(),  4048727598324417001);
        assert_eq!(rng3.next(),  7862637804313477842);
        assert_eq!(rng3.next(), 13015481187462834606);
        assert_eq!(rng3.next(), 15212506146343009075);
    }

    #[test]
    fn test_random_number_with_upper_bound() {
        let mut rng = Rng::new(0);

        assert_eq!(rng.random_number_with_upper_bound(3), 1);
        assert_eq!(rng.random_number_with_upper_bound(3), 0);
        assert_eq!(rng.random_number_with_upper_bound(3), 1);
        assert_eq!(rng.random_number_with_upper_bound(3), 1);
        assert_eq!(rng.random_number_with_upper_bound(3), 1);
        assert_eq!(rng.random_number_with_upper_bound(3), 0);
        assert_eq!(rng.random_number_with_upper_bound(3), 2);
        assert_eq!(rng.random_number_with_upper_bound(3), 2);
        assert_eq!(rng.random_number_with_upper_bound(3), 2);
        assert_eq!(rng.random_number_with_upper_bound(3), 2);

        assert_eq!(rng.random_number_with_upper_bound(10), 1);
        assert_eq!(rng.random_number_with_upper_bound(10), 6);
        assert_eq!(rng.random_number_with_upper_bound(10), 3);
        assert_eq!(rng.random_number_with_upper_bound(10), 1);
        assert_eq!(rng.random_number_with_upper_bound(10), 7);
        assert_eq!(rng.random_number_with_upper_bound(10), 7);
        assert_eq!(rng.random_number_with_upper_bound(10), 5);
        assert_eq!(rng.random_number_with_upper_bound(10), 2);
        assert_eq!(rng.random_number_with_upper_bound(10), 2);
        assert_eq!(rng.random_number_with_upper_bound(10), 4);


        assert_eq!(rng.random_number_with_upper_bound(1), 0);
        assert_eq!(rng.random_number_with_upper_bound(1), 0);
        assert_eq!(rng.random_number_with_upper_bound(1), 0);
        assert_eq!(rng.random_number_with_upper_bound(1), 0);
        assert_eq!(rng.random_number_with_upper_bound(1), 0);
        assert_eq!(rng.random_number_with_upper_bound(1), 0);
        assert_eq!(rng.random_number_with_upper_bound(1), 0);
        assert_eq!(rng.random_number_with_upper_bound(1), 0);


        assert_eq!(rng.random_number_with_upper_bound(2), 0);
        assert_eq!(rng.random_number_with_upper_bound(2), 0);
        assert_eq!(rng.random_number_with_upper_bound(2), 1);
        assert_eq!(rng.random_number_with_upper_bound(2), 1);
        assert_eq!(rng.random_number_with_upper_bound(2), 0);
        assert_eq!(rng.random_number_with_upper_bound(2), 1);
        assert_eq!(rng.random_number_with_upper_bound(2), 1);
        assert_eq!(rng.random_number_with_upper_bound(2), 1);
        assert_eq!(rng.random_number_with_upper_bound(2), 1);
        assert_eq!(rng.random_number_with_upper_bound(2), 1);
        assert_eq!(rng.random_number_with_upper_bound(2), 1);
        assert_eq!(rng.random_number_with_upper_bound(2), 1);
        assert_eq!(rng.random_number_with_upper_bound(2), 0);
        assert_eq!(rng.random_number_with_upper_bound(2), 0);
        assert_eq!(rng.random_number_with_upper_bound(2), 0);
        assert_eq!(rng.random_number_with_upper_bound(2), 1);
        assert_eq!(rng.random_number_with_upper_bound(2), 0);
        assert_eq!(rng.random_number_with_upper_bound(2), 1);
        assert_eq!(rng.random_number_with_upper_bound(2), 1);
        assert_eq!(rng.random_number_with_upper_bound(2), 1);
        assert_eq!(rng.random_number_with_upper_bound(2), 1);
        assert_eq!(rng.random_number_with_upper_bound(2), 0);
        assert_eq!(rng.random_number_with_upper_bound(2), 1);
        assert_eq!(rng.random_number_with_upper_bound(2), 0);
    }

    #[test]
    #[should_panic]
    fn test_random_number_with_zero_upper_bound() {
        let mut rng = Rng::new(123);
        assert_eq!(rng.random_number_with_upper_bound(0), 0);
    }

    #[test]
    fn test_random_number_in_range() {
        let mut rng = Rng::new(11419);

        assert_eq!(rng.random_number_in_range(15..20), 16);
        assert_eq!(rng.random_number_in_range(15..20), 15);
        assert_eq!(rng.random_number_in_range(15..20), 19);
        assert_eq!(rng.random_number_in_range(15..20), 19);
        assert_eq!(rng.random_number_in_range(15..20), 17);
        assert_eq!(rng.random_number_in_range(15..20), 16);
        assert_eq!(rng.random_number_in_range(15..20), 18);
        assert_eq!(rng.random_number_in_range(15..20), 17);
        assert_eq!(rng.random_number_in_range(15..20), 15);
        assert_eq!(rng.random_number_in_range(15..20), 15);

        assert_eq!(rng.random_number_in_range(100000..100002), 100000);
        assert_eq!(rng.random_number_in_range(100000..100002), 100000);
        assert_eq!(rng.random_number_in_range(100000..100002), 100000);
        assert_eq!(rng.random_number_in_range(100000..100002), 100001);
        assert_eq!(rng.random_number_in_range(100000..100002), 100000);
        assert_eq!(rng.random_number_in_range(100000..100002), 100001);
        assert_eq!(rng.random_number_in_range(100000..100002), 100000);
        assert_eq!(rng.random_number_in_range(100000..100002), 100000);
        assert_eq!(rng.random_number_in_range(100000..100002), 100001);
        assert_eq!(rng.random_number_in_range(100000..100002), 100001);
    }
}
