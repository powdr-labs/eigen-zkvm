#![allow(dead_code)]
use crate::constant::MG;
use crate::f3g::F3G;
use crate::helper::log2_any;

pub struct FFT {
    pub roots: Vec<Vec<F3G>>,
}

impl FFT {
    pub fn new() -> Self {
        let s = 32;
        let mut self_ = FFT {
            roots: vec![Vec::new(); s],
        };
        self_.set_roots(core::cmp::min(s, 15));
        self_
    }

    fn set_roots(&mut self, s: usize) {
        let mut i = s;
        while !(i > self.roots.len() && self.roots[i].len() > 0) {
            let mut r = F3G::ONE;
            let nroots = 1 << i;
            self.roots[i] = vec![F3G::ZERO; nroots];
            for j in 0..nroots {
                self.roots[i][j] = r;
                r = r * MG.0[i];
            }
            if i == 0 {
                break;
            }
            i -= 1;
        }
    }

    pub fn fft(&mut self, p: &Vec<F3G>) -> Vec<F3G> {
        if p.len() <= 1 {
            return p.clone();
        }
        let bits = log2_any(p.len() - 1) + 1;
        self.set_roots(bits);

        let n = 1 << bits;
        if p.len() != n {
            panic!("Size must be multiple of 2")
        }
        let mut buff = vec![F3G::ZERO; n];
        for i in 0..p.len() {
            let r = (i as u32).reverse_bits() >> (32 - bits);
            buff[r as usize] = p[i];
        }

        for s in 1..=bits {
            let m = 1 << s;
            let mdiv2 = m >> 1;
            let winc = self.roots[s][1];
            for k in (0..n).step_by(m) {
                let mut w = F3G::ONE;
                for j in 0..mdiv2 {
                    let t = w * buff[k + j + mdiv2];
                    let u = buff[k + j];
                    buff[k + j] = u + t;
                    buff[k + j + mdiv2] = u - t;
                    w = w * winc;
                }
            }
        }
        buff
    }

    pub fn ifft(&mut self, p: &Vec<F3G>) -> Vec<F3G> {
        let q = self.fft(p);
        let n = p.len();
        let n2inv = F3G::from(p.len()).inv();
        let mut res = vec![F3G::ZERO; q.len()];
        for i in 0..n {
            res[(n - i) % n] = q[i] * n2inv;
        }
        res
    }
}

#[cfg(test)]
mod tests {
    use crate::f3g::F3G;
    use crate::fft::FFT;
    #[test]
    fn test_single_fft() {
        let mut f = FFT::new();
        let a: Vec<F3G> = vec![1u32, 2u32, 3u32, 5u32]
            .iter()
            .map(|e| F3G::from(*e))
            .collect();
        let aa = f.fft(&a);
        let ac = f.ifft(&aa);
        for i in 0..a.len() {
            assert_eq!(a[i], ac[i]);
        }
    }

    #[test]
    fn test_random_fft() {
        let mut f = FFT::new();
        let mut a: Vec<F3G> = Vec::new();
        let mut rng = ::rand::thread_rng();
        for _i in 0..64 {
            a.push(<F3G as rand::Rand>::rand(&mut rng));
        }
        let aa = f.fft(&a);
        let ac = f.ifft(&aa);
        for i in 0..a.len() {
            assert_eq!(a[i], ac[i]);
        }
    }
}
