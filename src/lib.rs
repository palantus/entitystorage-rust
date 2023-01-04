use std::{collections::HashMap, any::Any, any::TypeId};
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
  where T: 'static, T: Clone{
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

  pub fn add_id(&mut self, id: u64){
    self.ids.push(id);
  }

  pub fn add<T>(&mut self, entity: &Entity<T>){
    self.ids.push(entity.id);
  }
}
pub struct EntityDB{
  entities: HashMap<TypeId, HashMap<u64, Entity<Box<dyn Any>>>>,
  next_id: u64
}

impl<'a> EntityDB{
  pub fn lookup_id<T: 'static>(&self, id: u64) -> Option<Entity<T>>
  where T: Clone {
    let entities = self.entities.get(&TypeId::of::<T>());
    match entities {
      Some(entities) => {
        match entities.get(&id){
          Some(e) => Some(Entity{id: e.id, data: e.data.downcast_ref::<T>().unwrap().clone()}),
          None => None
        }
      },
      None => None
    }    
  }

  pub fn search<T: 'static, F>(&self, matcher: F) -> Vec<Entity<&T>>
  where F: Fn(&T) -> bool {
    let entities = self.entities.get(&TypeId::of::<T>());
    match entities {
      Some(entities) => {
        entities.values().into_iter().filter_map(|e| match e.data.downcast_ref::<T>() {
          Some(data) => {
            let entity = Entity{id: e.id, data: data.clone()};
            match matcher(&data){true => Some(entity), false => None}
          },
          None => None
        }).collect()
      },
      None => vec![]
    }    
  }

  pub fn find<T: 'static, F>(&self, matcher: F) -> Option<Entity<&T>>
  where F: Fn(&T) -> bool {
    let entities = self.entities.get(&TypeId::of::<T>());
    match entities {
      Some(entities) => {
        entities.values().into_iter().find_map(|e| {
          let data = e.data.downcast_ref::<T>().unwrap(); // Can unwrap, as the type is guarenteed by the map type of entities
            let entity = Entity{id: e.id, data: data.clone()};
            match matcher(&data){
              true => Some(entity), 
              false => None
            }
        })
      },
      None => None
    }  
  }

  pub fn all<T: 'static>(&self) -> Vec<Entity<T>>
  where T:Clone{
    let entities = self.entities.get(&TypeId::of::<T>());
    match entities {
      Some(entities) => {
        entities.values().into_iter().filter_map(|e| Some(Entity{id: e.id, data: (*e.data.downcast_ref::<T>().unwrap()).clone()})).into_iter().collect::<Vec<_>>()
      },
      None => vec![]
    }  
  }

  pub fn new() -> EntityDB{
    EntityDB { entities: HashMap::new(), next_id: 1 }
  }

  pub fn add<T>(&mut self, data: T) -> Entity<T>
  where T: 'static, T: Clone{
    let id = self.next_id;
    self.next_id += 1;
    let entity = Entity{id, data: Box::new(data) as Box<dyn Any>};


    if !self.entities.contains_key(&TypeId::of::<T>()){
      self.entities.insert(TypeId::of::<T>(), HashMap::new());
    }

    let entities = self.entities.get_mut(&TypeId::of::<T>()).unwrap();
    entities.insert(id, entity);
    self.lookup_id(id).unwrap()
  }

  pub fn save<T>(&'a mut self, e: Entity<T>)
  where T: 'static {
    let entities = self.entities.get_mut(&TypeId::of::<T>());
    if entities.is_some() {
      entities.unwrap().insert(e.id, Entity{id: e.id, data: Box::new(e.data) as Box<dyn Any>});
    }
  }

  pub fn save_id<T>(&mut self, id: u64, data: T)
  where T: 'static {
    let entities = self.entities.get_mut(&TypeId::of::<T>());
    if entities.is_some() {
      entities.unwrap().insert(id, Entity{id: id, data: Box::new(data) as Box<dyn Any>});
    }
  }
}
