#[derive(Debug)]       // debug repr for IterableFloat
#[derive(Clone, Copy)] // make IterableFloat copyable
struct IterableFloat {
    // we put all these values in one struct for simplicity
    start: f64,
    end: f64,
    step: f64,

    init: bool,
    value: f64,
}

impl IterableFloat {
    // default parameterization
    fn default() -> IterableFloat {
        IterableFloat {
            start: 2.7,
            end: 3.1,
            step: 0.1,

            init: true,
            value: 2.7,
        }
    }

    // custom parameterization
    fn from_to(start: f64, end: f64, step: f64) -> IterableFloat {
        IterableFloat {
            start: start,
            end: end,
            step: step,

            init: true,
            value: start,
        }
    }
}

impl Iterator for IterableFloat {
    type Item = IterableFloat;

    fn next(&mut self) -> Option<IterableFloat> {
        if !self.init {
            return None
        }
        if self.value + self.step > self.end {
            return None
        }
        self.value += self.step;
        Some(self.clone())
    }
}


fn main() {
    let ifloat = IterableFloat::default();

    /// manual iteration
    //println!("{:?}", ifloat.next().unwrap());

    /// iterate over default IterableFloat
    for f in ifloat {
        println!("{:?}", f);
    }

    /// iterate over custom IterableFloat
    for f in IterableFloat::from_to(2.7, 3.1, 0.1) {
        println!("{:?}", f);
    }
}
