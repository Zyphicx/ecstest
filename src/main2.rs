use std::any::TypeId;

use std::collections::HashMap;

use std::marker::PhantomData;

macro_rules! impl_data {
    ( $($tp: ident; $index:expr),* ) => {
        impl<$($tp: Access),*> Data for ( $($tp),* ) {
            fn fetch(world: &World, ids: Vec<Vec<EntityID>>) -> Self {
                ($($tp::new(world, ids[$index])),*)
            }
        }
    }
}

impl_data!(A; 0);
impl_data!(A; 0, B; 1);
impl_data!(A; 0, B; 1, C; 2);
impl_data!(A; 0, B; 1, C; 2, D; 3);
impl_data!(A; 0, B; 1, C; 2, D; 3, E; 4);
impl_data!(A; 0, B; 1, C; 2, D; 3, E; 4, F; 5);


/*
macro_rules! data {
    ( read: { $($rtag:path : $($rcomp:ident),* ; )* } write: { $($wtag:path : $($wcomp:ident),* ; )* } ) => {
        type Data = ( $($(ReadAccess<'a, $rcomp>),*)* $($(WriteAccess<'a, $wcomp>),*)* );

        fn run_system(world: &World) {
            ids = vec![ $($( world.tags.get($rtag).clone() ),*)*
                $($( world.tags.get($wtag).clone() ),*)*
            ];

            let data = Self::Data::Fetch(world, ids);

            Self::run(data);
        }
    }
}
*/

pub type EntityID = u32;
pub type TagID = u32;

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
pub enum Tag {
    HasGravity,
}

trait Access {
    fn new(world: &World, ids: Vec<EntityID>) -> Self;
}

pub struct ReadAccess<'a, T> {
    phantom: PhantomData<T>,
    current_id: usize,
    ids: Vec<EntityID>,
    storage: &'a ComponentStorage,
}

impl<'a, T> Access for ReadAccess<'a, T> {
    fn new(world: &World, ids: Vec<EntityID>) -> Self {
        ReadAccess {
            phantom: PhantomData,
            current_id: 0,
            ids: ids,
            storage: &**world.components.get(&TypeId::of::<T>()).unwrap(),
        }
    }
}

impl<'a, T: 'a> Iterator for ReadAccess<'a, T> {
    type Item = &'a T;
    fn next(&mut self) -> Option<Self::Item> {
        self.storage.get(self.ids[self.current_id]).unwrap().borrow()
    }
}

pub struct WriteAccess<'a, T> {
    phantom: PhantomData<T>,
    current_id: usize,
    ids: Vec<EntityID>,
    storage: &'a ComponentStorage,
}

impl<'a, T> Access for WriteAccess<'a, T> {
    fn new(world: &World, ids: Vec<EntityID>) -> Self {
        WriteAccess {
            phantom: PhantomData,
            current_id: 0,
            ids: ids,
            storage: &**world.components.get(&TypeId::of::<T>()).unwrap(),
        }
    }
}

trait Data {
    fn fetch(world: &World, Vec<Vec<EntityID>>) -> Self;
}

impl<'a, T: 'a> Iterator for WriteAccess<'a, T> {
    type Item = &'a mut T;

    fn next(&mut self) -> Option<Self::Item> {
        Some(self.storage.get(self.ids[self.current_id]).unwrap().borrow_mut())
    }
}

use Tag::*;

use std::rc::Rc;
use std::cell::RefCell;

type CellBox<T> = Rc<RefCell<Box<T>>>;

pub trait ComponentStorage: std::fmt::Debug {
    fn put(&mut self, id: EntityID, comp: Box<Component>);
    fn get(&self, id: EntityID) -> Option<CellBox<Component>>;
}

impl ComponentStorage for HashMap<EntityID, Box<Component>> {
    fn put(&mut self, id: EntityID, comp: Box<Component>) {
        self.insert(id, comp);
    }

    fn get(&self, id: EntityID) -> Option<CellBox<Component>> {
        *HashMap::get(self, &id)
    }
}

impl ComponentStorage for Vec<Box<Component>> {
    fn put(&mut self, id: EntityID, comp: Box<Component>) {
        self.insert(id as usize, comp);
    }

    fn get(&self, id: EntityID) -> Option<CellBox<Component>> {
        Some(self[id as usize]) // Fix this to actually store empty components
    }
}


pub struct EntityBuilder<'a> {
    id: EntityID,
    world: &'a mut World,
}

impl<'a> EntityBuilder<'a> {
    fn add<T: Component + 'static>(self, comp: T) -> Self { // Adds component
        self.world.components.get_mut(&TypeId::of::<T>())
                             .unwrap()
                             .put(self.id, Box::new(comp));
        self
    }

    fn tag(self, t: TagID) -> Self { // Adds self to specific tag
        if !self.world.tags.contains_key(&t) {
            self.world.tags.insert(t, Vec::new());
        }

        self.world.tags.get_mut(&t)
                       .unwrap()
                       .push(self.id);
        self
    }

    fn done(self) {} // Consumes self
}

pub trait Component: std::any::Any + std::fmt::Debug {
    fn as_any(&self) -> &std::any::Any;
    fn as_any_mut(&mut self) -> &mut std::any::Any;
}

pub type Position = (f32, f32);
pub type Velocity = (f32, f32);

/* ****************World**************** */

#[derive(Debug)] pub struct World {
    id_count: EntityID,
    components: HashMap<TypeId, Box<ComponentStorage>>,
    tags: HashMap<TagID, Vec<EntityID>>,
}

impl World {
    fn new() -> World {
        World { id_count: 0, components: HashMap::new(), tags: HashMap::new() }
    }

    fn next_id(&mut self) -> EntityID {
        let id = self.id_count;
        self.id_count += 1;
        id
    }

    fn new_entity(&mut self) -> EntityBuilder {
        EntityBuilder { id: self.next_id(), world: self }
    }

    fn add_storage<T: 'static>(&mut self) {
        self.components.insert(TypeId::of::<T>(), Box::new(HashMap::new()));
    }
}

#[derive(Debug)]
pub struct Pos(f32, f32);
impl Component for Pos {
    fn as_any(&self) -> &std::any::Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut std::any::Any {
        self
    }
}

#[derive(Debug)]
pub struct Vel(f32, f32);
impl Component for Vel {
    fn as_any(&self) -> &std::any::Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut std::any::Any {
        self
    }
}

macro_rules! t {
    ( $tagName:ident ) => { Tag::$tagName as TagID }
}

fn main() {
    let mut world = World::new();

    world.add_storage::<Pos>();
    world.add_storage::<Vel>();

    world.new_entity().add(Pos(4.0, 3.0)).add(Vel(1.0, 3.0)).tag(t!(HasGravity)).done();

    let mut positions: Vec<&Pos> = Vec::new();

    for entity in world.tags.get(&(Tag::HasGravity as u32)).unwrap().iter() {
        let component_storage = world.components.get(&TypeId::of::<Pos>()).unwrap();
        let component: &Component = &**component_storage.get(*entity).unwrap();

        let component: &Pos = match component.as_any().downcast_ref::<Pos>() {
            Some(comp) => comp,
            None => panic!("Tried to access invalid component (help: check for entities with non-matching tags and components"),
        };

        positions.push(component);
    }

    println!("Positions: {:?}", positions);
}

pub trait System<'a> {
    // Read can either take a tag or a component
    type Data: Data;

    fn run_system(world: &World);

    fn run(read: Self::Data);
}

pub struct MoveSystem {}

impl<'a> System<'a> for MoveSystem {
//    read!( Tag::HasGravity: Vel );
//    write!( Tag::HasGravity: Pos );

/*
    data!(read: {
            HasGravity: Vel;
          }

          write: {
            HasGravity: Pos;
          }
         );

*/
    type Data = ( ReadAccess<'a, Vel>, WriteAccess<'a, Pos> );

    fn run_system(world: &World) {
        let ids = vec![ world.tags.get(HasGravity).clone(),
                        world.tags.get(HasGravity).clone()
                      ];

        let data = Self::Data::Fetch(world, ids);

        Self::run(data);
    }

    fn run(read: Self::Data){
        println!("{:?}", read);
    }
}

//( read: $($rtag:ident: $($rcomp:ident),* );* write: $($wtag:ident: $($wcomp:ident),* );* )
