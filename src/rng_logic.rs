use rand::prelude::*;

// fuck this
pub fn parto(pct: &usize) -> usize {
    let mut rng = thread_rng();

    if pct > &100 {
        if rng.gen_ratio((pct - 100).try_into().unwrap(), 100) {
            return 2;
        }

        1
    } else {
        if rng.gen_ratio(*pct as u32, 100) {
            return 1;
        }

        0
    }
}
