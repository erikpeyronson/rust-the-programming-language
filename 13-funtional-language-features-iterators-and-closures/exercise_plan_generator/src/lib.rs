use std::cmp::Eq;
use std::collections::HashMap;
use std::hash::Hash;

pub struct Cacher<T, U>
where
    T: Fn(U) -> U,
    U: Eq + Hash + Copy,
{
    calculation: T,
    values: HashMap<U, Option<U>>,
}

impl<T, U> Cacher<T, U>
where
    T: Fn(U) -> U,
    U: Eq + Hash + Copy,
{
    pub fn new(calculation: T) -> Cacher<T, U> {
        Cacher {
            calculation,
            values: HashMap::new(),
        }
    }

    pub fn value(&mut self, arg: U) -> U {
        let value = self.values.entry(arg).or_insert(None);
        match value {
            Some(v) => *v,
            None => {
                let v = (self.calculation)(arg);
                *value = Some(v);
                v
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn call_with_different_values() {
        let mut c = Cacher::new(|a| a);
        let v1 = c.value(1);
        let v2 = c.value(2);

        assert_eq!(v1, 1);
        assert_eq!(v2, 2);
    }
}
