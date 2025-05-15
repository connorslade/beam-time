use ahash::{HashMap, HashMapExt, HashSet};
use uuid::Uuid;

use super::Level;

pub struct LevelTree {
    map: HashMap<Uuid, &'static Level>,

    tree: HashMap<Uuid, Vec<Uuid>>,
    root: HashSet<Uuid>,
}

impl LevelTree {
    pub fn new(levels: &'static [Level]) -> Self {
        let mut map = HashMap::new();
        let mut tree = HashMap::<_, Vec<_>>::new();
        let mut root = levels.iter().map(|x| x.id).collect::<HashSet<_>>();

        for level in levels {
            map.insert(level.id, level);
            for parent in &level.parents {
                root.remove(parent);
                tree.entry(*parent).or_default().push(level.id);
            }
        }

        Self { map, tree, root }
    }

    pub fn get(&self, id: Uuid) -> Option<&'static Level> {
        self.map.get(&id).copied()
    }

    pub fn root(&self) -> impl Iterator<Item = Uuid> + '_ {
        self.root.iter().copied()
    }

    pub fn children(&self, parent: Uuid) -> Option<&Vec<Uuid>> {
        self.tree.get(&parent)
    }
}
