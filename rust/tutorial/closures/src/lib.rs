use std::collections::HashMap;

pub struct Cacher<T>
where
    T: Fn(u32) -> u32,
{
    calculator: T,
    values: HashMap<u32, u32>,
}

impl<T> Cacher<T>
where
    T: Fn(u32) -> u32,
{
    pub fn new(calculator: T) -> Cacher<T> {
        Cacher {
            calculator,
            values: HashMap::new(),
        }
    }

    pub fn value(&mut self, arg: u32) -> u32 {
        match self.values.get(&arg) {
            Some(&x) => x,
            None => {
                let x = (self.calculator)(arg);
                self.values.insert(arg, x);
                x
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cacher_basic() {
        let mut c = Cacher::new(|a| a);
        let val = c.value(3);

        assert_eq!(3, val);
    }

    #[test]
    fn cacher_different_values() {
        let mut c = Cacher::new(|a| a);
        let val1 = c.value(3);
        let val2 = c.value(4);

        assert_eq!(3, val1);
        assert_eq!(4, val2);
    }
}
