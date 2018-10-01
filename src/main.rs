use std::any::TypeId;

use std::collections::HashMap;


macro_rules! inputs {
    ( tags = $($tags:ident: $($components:ident),+ ),+ ) => {
        fn get_comps() -> Vec<(TagID, Vec<TypeId>)> {
            vec![$( ($tags, vec![$()] ) ),+] // Finish some version of this and add an output macro too
        }
    }
}

pub type EntityID = u32;
pub type TagID = u32;

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
pub enum Tag {
    HasGravity,
}

pub trait ComponentStorage: std::fmt::Debug {
    fn put(&mut self, id: EntityID, comp: Box<Component>);
}

impl ComponentStorage for HashMap<EntityID, Box<Component>> { // PLEASE DO SOMETHING REALLY CUTE WITH std::Any!!!!!!!!!!!!!!!!!!!!!!!!!!
    fn put(&mut self, id: EntityID, comp: Box<Component>) {
        self.insert(id, comp);
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

pub trait Component: std::fmt::Debug {}

pub type Position = (f32, f32);
pub type Velocity = (f32, f32);

/* ****************World**************** */

#[derive(Debug)]
pub struct World {
    id_count: EntityID,
    components: HashMap<TypeId, Box<ComponentStorage>>,
    tags: HashMap<TagID, Vec<EntityID>>,
}

impl World {
    fn new_world() -> World {
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
impl Component for Pos {}

#[derive(Debug)]
pub struct Vel(f32, f32);
impl Component for Vel {}

macro_rules! t {
    ( $tagName:ident ) => { Tag::$tagName as TagID }
}

fn main() {
    let mut world = World::new_world();

    world.add_storage::<Pos>();
    world.add_storage::<Vel>();

    world.new_entity().add(Pos(4.0, 3.0)).add(Vel(1.0, 3.0)).tag(t!(HasGravity)).done();

    println!("{:?}", world);
}



/*
pub trait System {
    // Read can either take a tag or a component

    fn get()
}
*/
