use std::any::TypeId;

use std::collections::HashMap;

/*
macro_rules! read {
    ( $($tags:ident: $($components:ident),+ );+ ) => {
        type Read = ( $( $( $components ),+ )+ );

        fn get_read() -> Vec<(TagID, Vec<TypeId>)> {
            vec![$( ($tags as u32, vec![$( TypeId::of::<$components>() ),+] ) ),+] 
        }
    };

    ( $($enums:ident::$tags:ident: $($components:ident),+ );+ ) => {
        type Read = ( $( $( $components ),+ )+ );

        fn get_read() -> Vec<(TagID, Vec<TypeId>)> {
            vec![$( ($enums::$tags as u32, vec![$( TypeId::of::<$components>() ),+] ) ),+] 
        }
    }
}

macro_rules! write {
    ( $($tags:ident: $($components:ident),+ );+ ) => {
        type Write = ( $( $( $components ),+ )+ );

        fn get_write() -> Vec<(TagID, Vec<TypeId>)> {
            vec![$( ($tags as u32, vec![$( TypeId::of::<$components>() ),+] ) ),+] 
        }
    };

    ( $($enums:ident::$tags:ident: $($components:ident),+ );+ ) => {
        type Write = ( $( $( $components ),+ )+ );

        fn get_write() -> Vec<(TagID, Vec<TypeId>)> {
            vec![$( ($enums::$tags as u32, vec![$( TypeId::of::<$components>() ),+] ) ),+] 
        }
    }
}
*/

/*
let component_storage = world.components.get(&TypeId::of::<Pos>()).unwrap();
let component: &Component = &**component_storage.get(*entity).unwrap();

let component: &Pos = match component.as_any().downcast_ref::<Pos>() {
    Some(comp) => comp,
    None => panic!("Tried to access invalid component (help: check for entities with non-matching tags and components"),
*/

macro_rules! read {
    ( $($tags:ident: $($components:ident),+ );+ ) => {
        type Read = ( $( $( &'a $components ),+ )+ );

        fn get_read(world: &World) -> Self::Read {
            ( $( $( world.tags.get( &($tags as u32) ) // Get all entity ID's
                            .unwrap()
                            .iter()
                            .map(|x| { let comp: &Component = &**world.components.get(&TypeId::of::<$components>())
                                                                                 .unwrap()
                                                                                 .get(*x) // Get the corresponding component
                                                                                 .unwrap();
                                       let comp: &$components = match comp.as_any().downcast_ref::<$components>() {
                                            Some(component) => &component,
                                            None => panic!("Tried to access invalid component (help: check for entities with non-matching tags and components"),
                                       };

                                       comp
                                     })
                            .collect()
              ),+)+ )
                           
        }
    };
    ( $($enums:ident::$tags:ident: $($components:ident),+ );+ ) => {
        type Read = ( $( $( &'a $components ),+ )+ );

        fn get_read(world: &World) -> Self::Read {
            $( $( world.tags.get( &($enums::$tags as u32) ) // Get all entity ID's
                            .unwrap()
                            .iter()
                            .map(|x| { let comp: &Component = &**world.components.get(&TypeId::of::<$components>())
                                                                                 .unwrap()
                                                                                 .get(*x) // Get the corresponding component
                                                                                 .unwrap();
                                       let comp: &$components = match comp.as_any().downcast_ref::<$components>() {
                                            Some(component) => &component,
                                            None => panic!("Tried to access invalid component (help: check for entities with non-matching tags and components"),
                                       };

                                       comp
                                     })
                            .collect()
              ),+)+
                           
        }
    }
}

macro_rules! write {
    ( $($tags:ident: $($components:ident),+ );+ ) => {
        type Write = ( $( $( &'a mut $components ),+ )+ );

        fn get_write(world: &mut World) -> Self::Write {
            $( $( world.tags.get( &($tags as u32) ) // Get all entity ID's
                            .unwrap()
                            .iter()
                            .map(|x| { let comp: &mut Component = &**world.components.get_mut(&TypeId::of::<$components>())
                                                                                     .unwrap()
                                                                                     .get_mut(*x) // Get the corresponding component
                                                                                     .unwrap();
                                       let comp: &mut $components = match comp.as_any().downcast_ref::<&mut $components>() {
                                            Some(component) => &mut component,
                                            None => panic!("Tried to access invalid component (help: check for entities with non-matching tags and components"),
                                       };

                                       comp
                                     })
                            .collect()
              ),+)+
                           
        }
    };

    ( $($enums:ident::$tags:ident: $($components:ident),+ );+ ) => {
        type Write = ( $( $( &'a mut $components ),+ )+ );

        fn get_write(world: &mut World) -> Self::Write {
            $( $( world.tags.get( &($enums::$tags as u32) ) // Get all entity ID's
                            .unwrap()
                            .iter()
                            .map(|x| { let comp: &mut Component = &**world.components.get_mut(&TypeId::of::<$components>())
                                                                                     .unwrap()
                                                                                     .get_mut(*x) // Get the corresponding component
                                                                                     .unwrap();
                                       let comp: &mut $components = match comp.as_any().downcast_ref::<&mut $components>() {
                                            Some(component) => &mut component,
                                            None => panic!("Tried to access invalid component (help: check for entities with non-matching tags and components"),
                                       };

                                       comp
                                     })
                            .collect()
              ),+)+
                           
        }
    }
}

pub type EntityID = u32;
pub type TagID = u32;

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
pub enum Tag {
    HasGravity,
}

use Tag::*;

pub trait ComponentStorage: std::fmt::Debug {
    fn put(&mut self, id: EntityID, comp: Box<Component>);
    fn get(&self, id: EntityID) -> Option<&Box<Component>>;
    fn get_mut(&mut self, id: EntityID) -> Option<&mut Box<Component>>;
}

impl ComponentStorage for HashMap<EntityID, Box<Component>> {
    fn put(&mut self, id: EntityID, comp: Box<Component>) {
        self.insert(id, comp);
    }

    fn get(&self, id: EntityID) -> Option<&Box<Component>> {
        HashMap::get(self, &id)
    }

    fn get_mut(&mut self, id: EntityID) -> Option<&mut Box<Component>> {
        HashMap::get_mut(self, &id)
    }
}

impl ComponentStorage for Vec<Box<Component>> {
    fn put(&mut self, id: EntityID, comp: Box<Component>) {
        self.insert(id as usize, comp); 
    }

    fn get(&self, id: EntityID) -> Option<&Box<Component>> {
        Some(&self[id as usize]) // Fix this to actually store empty components
    }

    fn get_mut(&mut self, id: EntityID) -> Option<&mut Box<Component>> {
        Some(&self[id as usize])
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
}

#[derive(Debug)]
pub struct Vel(f32, f32);
impl Component for Vel {
    fn as_any(&self) -> &std::any::Any {
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

    //println!("{:?}", world);

    //println!("{:?}", MoveSystem::get_read());
    //println!("{:?}", MoveSystem::get_write());

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

    //MoveSystem::run(
}

/*
fn get_components(Vec<(TagID, Vec<TypeId>)>) {
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

    println!("{:?}", System::Read);
}
*/

pub trait System<'a> {
    // Read can either take a tag or a component
    type Read;
    type Write;

    //fn get_read() -> Vec<(TagID, Vec<TypeId>)>;
    //fn get_write() -> Vec<(TagID, Vec<TypeId>)>;

    fn get_read(&World) -> Self::Read;
    fn get_write(&mut World) -> Self::Write;

    fn run(read: Self::Read, write: Self::Write);
}

pub struct MoveSystem {}

/*
impl<'a> MoveSystem {
    fn run_system(&self, world: &mut World) {
        let read = &MoveSystem::get_read();

        println!("{:?}", read);
    }
}
*/

impl<'a> System<'a> for MoveSystem {
    read!( Tag::HasGravity: Vel ); 
    write!( Tag::HasGravity: Pos );

    fn run(read: Self::Read, write: Self::Write){
        println!("{:?}", read);
    }
}
