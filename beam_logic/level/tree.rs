use ahash::{HashMap, HashMapExt, HashSet};
use uuid::Uuid;

use super::Level;

pub struct LevelTree {
    map: HashMap<Uuid, &'static Level>,

    reverse: HashMap<Uuid, Vec<Uuid>>,
    root: HashSet<Uuid>,
}

impl LevelTree {
    pub fn new(levels: &'static [Level]) -> Self {
        let mut map = HashMap::new();
        let mut reverse = HashMap::<_, Vec<_>>::new();
        let mut root = levels.iter().map(|x| x.id).collect::<HashSet<_>>();

        for level in levels {
            map.insert(level.id, level);

            for child in &level.children {
                root.remove(child);
                reverse.entry(*child).or_default().push(level.id);
            }
        }

        Self { map, reverse, root }
    }

    pub fn get(&self, id: Uuid) -> Option<&'static Level> {
        self.map.get(&id).copied()
    }

    pub fn root(&self) -> impl Iterator<Item = Uuid> + '_ {
        self.root.iter().copied()
    }

    pub fn parents(&self, child: Uuid) -> Option<&Vec<Uuid>> {
        self.reverse.get(&child)
    }

    pub fn count(&self) -> usize {
        self.map.len()
    }
}
