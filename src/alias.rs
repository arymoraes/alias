use crate::store::Store;

pub struct Alias {
    pub alias: String,
    pub command: String,
}

impl Alias {
    pub fn new(alias: String, command: String) -> Self {
        return Alias { alias, command };
    }

    pub fn create(self, store: &mut Store) -> () {
        store.aliases.insert(self.alias, self.command);
        store.save();
    }

    pub fn remove(self, store: &mut Store) -> () {
        store.aliases.remove(&self.alias);
    }
}
