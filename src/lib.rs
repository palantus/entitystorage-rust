use std::{collections::HashMap, any::Any, any::TypeId, cell::RefCell};
pub use serde::{Serialize, Deserialize};

#[derive(Debug, Clone)]
pub struct Entity<T> {
    pub id: u64,
    pub data: T
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Rels{
  ids: Vec<u64>
}

impl Rels{
  pub fn resolve<T>(&self, db: &EntityDB) -> Vec<Entity<T>>
  where T: 'static, T: Clone, T: Copy{
    let entities = db.all::<T>();
    let entities = entities.iter().filter(|e| self.ids.contains(&e.id));
    entities.map(|e| (*e).clone()).into_iter().collect()
  }

  pub fn resolve_first<T>(&self, db: &EntityDB) -> Option<Entity<T>>
  where T: 'static, T: Clone{
    match self.ids.get(0){
      Some(id) => {
        db.lookup_id(*id)
      },
      None => None
    }
  }

  pub fn new() -> Self{
    Rels{ids: vec![]}
  }

  pub fn new_e<T>(entity: &Entity<T>) -> Self{
    Rels{ids: vec![entity.id]}
  }

  pub fn add_id(&mut self, id: u64){
    self.ids.push(id);
  }

  pub fn add<T>(&mut self, entity: &Entity<T>) -> &Self{
    self.ids.push(entity.id);
    self
  }
}
pub struct EntityDB{
  entities: RefCell<HashMap<TypeId, HashMap<u64, Entity<Box<dyn Any>>>>>,
  next_id: RefCell<u64>
}

impl<'a> EntityDB{
  pub fn lookup_id<T: 'static>(&self, id: u64) -> Option<Entity<T>>
  where T: Clone {
    let entities = self.entities.borrow();
    let entities = entities.get(&TypeId::of::<T>());
    match entities {
      Some(entities) => {
        match entities.get(&id){
          Some(e) => {
            let ret = Some(Entity{id: e.id, data: e.data.downcast_ref::<T>().unwrap().clone()});
            drop(entities);
            ret
          },
          None => None
        }
      },
      None => None
    }    
  }

  pub fn search<T: 'static, F>(&self, matcher: F) -> Vec<Entity<T>>
  where F: Fn(&T) -> bool, T: Clone {
    let entities = self.entities.borrow();
    let entities = entities.get(&TypeId::of::<T>());
    match entities {
      Some(entities) => {
        let mut results = vec![];
        for e in entities.values(){
          let e_typed = &*e.data.downcast_ref::<T>().unwrap();
          if matcher(&e_typed) {
            results.push(Entity{id: e.id, data: e_typed.clone()})
          }
        }
        results
      },
      None => vec![]
    }    
  }

  pub fn find<T: 'static, F>(&self, matcher: F) -> Option<Entity<T>>
  where F: Fn(&T) -> bool, T: Clone  {
    let entities = self.entities.borrow();
    let entities = entities.get(&TypeId::of::<T>());
    match entities {
      Some(entities) => {
        for e in entities.values(){
          let e_typed = &*e.data.downcast_ref::<T>().unwrap();
          if matcher(&e_typed) {
            let entity = Entity{id: e.id, data: e_typed.clone()};
            return Some(entity)
          }
        }
        None
      },
      None => None
    }  
  }

  pub fn all<T: 'static>(&self) -> Vec<Entity<T>>
  where T:Clone{
    let entities = self.entities.borrow();
    let entities = entities.get(&TypeId::of::<T>());
    match entities {
      Some(entities) => {
        entities.values().into_iter().filter_map(|e| Some(Entity{id: e.id, data: (*e.data.downcast_ref::<T>().unwrap()).clone()})).into_iter().collect::<Vec<_>>()
      },
      None => vec![]
    }  
  }

  pub fn new() -> EntityDB{
    EntityDB { entities: RefCell::new(HashMap::new()), next_id: RefCell::new(1) }
  }

  pub fn add<T>(&self, data: T) -> Entity<T>
  where T: 'static, T: Clone{
    let id = *self.next_id.borrow();
    *self.next_id.borrow_mut() += 1;
    let entity = Entity{id, data: Box::new(data) as Box<dyn Any>};


    if !self.entities.borrow().contains_key(&TypeId::of::<T>()){
      self.entities.borrow_mut().insert(TypeId::of::<T>(), HashMap::new());
    }

    {
      let mut entities = self.entities.borrow_mut();
      let entities = entities.get_mut(&TypeId::of::<T>()).unwrap();
      entities.insert(id, entity);
    }
    self.lookup_id(id).unwrap()
  }

  pub fn save<T>(&'a self, e: Entity<T>)
  where T: 'static {
    let mut entities = self.entities.borrow_mut();
    let entities = entities.get_mut(&TypeId::of::<T>());
    if entities.is_some() {
      entities.unwrap().insert(e.id, Entity{id: e.id, data: Box::new(e.data) as Box<dyn Any>});
    }
  }

  pub fn save_id<T>(&self, id: u64, data: T)
  where T: 'static {
    let mut entities = self.entities.borrow_mut();
    let entities = entities.get_mut(&TypeId::of::<T>());
    if entities.is_some() {
      entities.unwrap().insert(id, Entity{id: id, data: Box::new(data) as Box<dyn Any>});
    }
  }
}
