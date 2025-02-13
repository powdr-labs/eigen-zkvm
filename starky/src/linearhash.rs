#![allow(non_snake_case)]
use crate::errors::Result;
use crate::poseidon_opt::Poseidon;
use crate::ElementDigest;
use plonky::field_gl::Fr as FGL;

#[derive(Default)]
pub struct LinearHash {
    h: Poseidon,
}

impl LinearHash {
    pub fn new() -> Self {
        LinearHash { h: Poseidon::new() }
    }

    pub fn hash_element_matrix(
        &self,
        vals: &Vec<Vec<FGL>>,
        batch_size: usize,
    ) -> Result<ElementDigest> {
        let mut flatvals: Vec<FGL> = vec![];
        for col in vals.iter() {
            for elem in col.iter() {
                flatvals.push(*elem);
            }
        }
        self.hash(&flatvals, batch_size)
    }

    pub fn hash(&self, flatvals: &[FGL], batch_size: usize) -> Result<ElementDigest> {
        let mut bs = batch_size;
        if bs == 0 {
            bs = core::cmp::max(8, (flatvals.len() + 3) / 4);
        }

        let mut st = [FGL::ZERO; 4];
        if flatvals.len() <= 4 {
            for (i, v) in flatvals.iter().enumerate() {
                st[i] = *v;
            }
            return Ok(ElementDigest::from(st));
        }

        let hsz = (flatvals.len() + bs - 1) / bs;
        let mut hashes: Vec<FGL> = vec![FGL::ZERO; hsz * 4];
        // NOTE flatsvals.len <= hashes.len
        hashes
            .chunks_mut(4)
            .zip(flatvals.chunks(bs))
            .for_each(|(outs, inps)| {
                let hv: [FGL; 4] = self._hash(inps).unwrap().into();
                outs[0..hv.len()].copy_from_slice(&hv);
            });

        if hashes.len() <= 4 {
            for (i, v) in hashes.iter().enumerate() {
                st[i] = *v;
            }
            Ok(ElementDigest::from(st))
        } else {
            self._hash(&hashes)
        }
    }

    pub fn _hash(&self, flatvals: &[FGL]) -> Result<ElementDigest> {
        let mut st = [FGL::ZERO; 4];
        if flatvals.len() <= 4 {
            for (i, v) in flatvals.iter().enumerate() {
                st[i] = *v;
            }
            return Ok(ElementDigest::from(st));
        }

        let mut inhashes: Vec<FGL> = vec![];
        for v in flatvals.iter() {
            inhashes.push(*v);
            if inhashes.len() == 8 {
                let t = self.h.hash(&inhashes, &st, 4).unwrap();
                st.copy_from_slice(&t);
                inhashes = vec![];
            }
        }
        if inhashes.len() > 0 {
            while inhashes.len() < 8 {
                inhashes.push(FGL::ZERO);
            }
            let t = self.h.hash(&inhashes, &st, 4).unwrap();
            st.copy_from_slice(&t);
        }
        Ok(ElementDigest::from(st))
    }
}

#[cfg(test)]
mod tests {
    use crate::digest::ElementDigest;
    use crate::linearhash::LinearHash;
    use plonky::field_gl::Fr as FGL;

    #[test]
    fn test_linearhash_gl_hash() {
        let lh = LinearHash::new();
        let raw_inputs = (1u64..28)
            .collect::<Vec<u64>>()
            .chunks(3)
            .collect::<Vec<&[u64]>>()
            .iter()
            .map(|ea| {
                let mut res: Vec<FGL> = vec![];
                for e in ea.iter() {
                    res.push(FGL::from(*e));
                }
                res
            })
            .collect::<Vec<Vec<FGL>>>();

        let res = lh.hash_element_matrix(&raw_inputs, 0).unwrap();
        let expected = ElementDigest::from([
            FGL::from(17618903473682537397u64),
            FGL::from(11844743283521766961u64),
            FGL::from(185773432536380223u64),
            FGL::from(6083210164459944430u64),
        ]);
        assert_eq!(expected, res);
    }

    #[test]
    fn test_linearhash_corner_case() {
        let lh = LinearHash::new();
        let raw_inputs = (1u64..4)
            .collect::<Vec<u64>>()
            .chunks(3)
            .collect::<Vec<&[u64]>>()
            .iter()
            .map(|ea| {
                let mut res: Vec<FGL> = vec![];
                for e in ea.iter() {
                    res.push(FGL::from(*e));
                }
                res
            })
            .collect::<Vec<Vec<FGL>>>();

        let res = lh.hash_element_matrix(&raw_inputs, 0).unwrap();
        let expected = ElementDigest::from([
            FGL::from(1u64),
            FGL::from(2u64),
            FGL::from(3u64),
            FGL::from(0u64),
        ]);
        assert_eq!(expected, res);
    }
}
