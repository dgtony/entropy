use std::cmp::max;
use std::collections::HashMap;
use std::hash::Hash;

#[derive(Debug, Default, PartialEq)]
pub struct ByteStat {
    pub byte: u8,
    pub count: usize,
    pub frequency: f64,
}

pub struct ByteFreq {
    bs: HashMap<u8, usize>,
}

impl ByteFreq {
    // initialize empty
    pub fn new() -> Self {
        ByteFreq { bs: HashMap::new() }
    }

    pub fn from_bytes(bytes: &Vec<u8>) -> Self {
        let mut bf = Self::new();
        bytes.iter().for_each(|&b| bf.add(b));
        bf
    }

    pub fn add(&mut self, byte: u8) {
        let counter = self.bs.entry(byte).or_insert(0);
        *counter += 1; // fixme r u sure it's ok?
    }

    pub fn frequencies(&self) -> HashMap<u8, f64> {
        let total = self.total_bytes();
        let mut fs: HashMap<u8, f64> = HashMap::with_capacity(self.bs.len());

        for (&b, &c) in self.bs.iter() {
            fs.insert(b, c as f64 / total as f64);
        }
        fs
    }

    pub fn stats(&self) -> Vec<ByteStat> {
        let mut stats: Vec<ByteStat> = zip_map(&self.bs, &self.frequencies())
            .iter()
            .map(|(&k, &v)| ByteStat {
                byte: k,
                count: v.0.unwrap(),
                frequency: v.1.unwrap(),
            })
            .collect();
        stats.sort_by_key(|s| s.byte);
        stats
    }

    pub fn entropy(&self) -> f64 {
        let fs = self.frequencies();
        fs.iter().fold(0.0, |_, (_, &f)| -1.0 * f * f.log2())
    }

    #[inline]
    fn total_bytes(&self) -> usize {
        self.bs.iter().fold(0, |_, e| *e.1)
    }
}

fn zip_map<K, R, T>(m1: &HashMap<K, R>, m2: &HashMap<K, T>) -> HashMap<K, (Option<R>, Option<T>)>
where
    K: Eq + Hash + Clone,
    R: Clone,
    T: Clone,
{
    let mut zipped = HashMap::with_capacity(max(m1.len(), m2.len()));

    // traverse keys in the first map and concat with possible matches in a second one
    for (key, v1) in m1.iter() {
        zipped.insert(key.clone(), (Some(v1.clone()), m2.get(&key).cloned()));
    }

    // add untouched keys of the second map
    m2.iter()
        .filter(|(k, _)| !m1.contains_key(k))
        .for_each(|(k, v2)| {
            zipped.insert(k.clone(), (None, Some(v2.clone())));
        });

    zipped
}

#[cfg(test)]
mod tests {
    use crate::freq::zip_map;
    use std::collections::HashMap;
    use std::ops::Deref;

    #[test]
    fn zipper() {
        let mut m1: HashMap<&str, i64> = HashMap::new();
        let mut m2: HashMap<&str, f32> = HashMap::new();

        m1.insert("a", 1);
        m1.insert("b", 2);

        m2.insert("b", 0.22);
        m2.insert("c", 0.33);

        let z = zip_map(&m1, &m2);

        assert_eq!(z.get("a").unwrap(), &(Some(1), None));
        assert_eq!(z.get("b").unwrap(), &(Some(2), Some(0.22)));
        assert_eq!(z.get("c").unwrap(), &(None, Some(0.33)));
    }
}
