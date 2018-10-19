use std::any::Any;
use std::any::TypeId;
use std::collections::HashMap;
use std::collections::LinkedList;
use std::marker::PhantomData;
use std::cell::{RefCell, Ref, RefMut};
use std::fmt::Debug;

type EntityID = u32;
type TagID = u32;
type CompID = TypeId;

pub struct IDTracker {
    next_id: EntityID,
    free_ids: Vec<EntityID>,
}

impl IDTracker {
    fn new() -> Self {
        IDTracker {
            next_id: 0,
            free_ids: Vec::new(),
        }
    }

    fn next_id(&mut self) -> EntityID {
        if self.free_ids.is_empty() {
            let id = self.next_id;
            self.next_id += 1;
            id
        } else {
            self.free_ids.pop().unwrap()
        }
    }

    fn free_id(&mut self, id: EntityID) {
        if id == self.next_id - 1 {
            self.next_id -= 1;
        } else {
            self.free_ids.push(id);
        }
    }
}

pub trait Component: Any + Debug {
    fn as_any(&self) -> &std::any::Any;
    fn as_any_mut(&mut self) -> &mut std::any::Any;
}

pub trait ComponentStorage {
    fn put(&mut self, id: EntityID, comp: Box<Component>);
    fn get(&self, id: EntityID) -> Ref<Box<Component>>;
    fn get_mut(&mut self, id: EntityID) -> RefMut<Box<Component>>;
}

impl ComponentStorage for Vec<RefCell<Box<Component>>> {        // Change from RefCell to atomically counted RefCell to make it thread safe
    fn put(&mut self, id: EntityID, comp: Box<Component>) {
        self.insert(id as usize, RefCell::new(comp));
    }

    fn get(&self, id: EntityID) -> Ref<Box<Component>> {
        self[id as usize].borrow()
    }

    fn get_mut(&mut self, id: EntityID) -> RefMut<Box<Component>> {
        self[id as usize].borrow_mut()
    }
}

pub struct EntityBuilder<'a> { // Used for creating and modifying entities
    id: EntityID,
    world: &'a mut World,
}

impl<'a> EntityBuilder<'a> {
    fn new(world: &'a mut World) -> Self {
        EntityBuilder {
            id: world.id_tracker.next_id(),
            world: world,
        }
    }

    fn from_id(world: &'a mut World, id: EntityID) -> Self {
        EntityBuilder {
            id: id,
            world: world,
        }
    }

    fn add_component<T: Component>(&mut self, comp: T) {
        self.world.insert_comp(self.id, comp);
    }

    fn add_tag(&mut self, tag: TagID) {
        self.world.tags.get_mut(&tag)
                       .unwrap()
                       .push_front(self.id);
    }

    fn remove_tag(&mut self, tag: TagID) {
        self.world.tags.get_mut(&tag)
                       .remove
    }
}

pub struct World {
    id_tracker: IDTracker,
    components: HashMap<CompID, Box<ComponentStorage>>, // Possibly replace this with a vec / array which has a size equal to the amount of components
    tags: HashMap<TagID, LinkedList<EntityID>>, // Is linked list the best storage solution here? Could a binary     tree be better?
}

impl World {
    fn new() -> Self {
        World {
            id_tracker: IDTracker::new(),
            components: HashMap::new(),
            tags: HashMap::new(),
        }
    }

    fn insert_comp<T: Component + 'static>(&mut self, id: EntityID, comp: T) {
        self.components.get_mut(&TypeId::of::<T>())
                       .expect("Could not find a matching component storage")
                       .put(id, Box::new(comp));
    }

    fn get_comp<T: Component + 'static>(&self, id: EntityID) -> Ref<T> {
        let comp = self.components.get(&TypeId::of::<T>())
                                  .expect("Could not find a matching component storage")
                                  .get(id);
        Ref::map(comp, |x| {
            let component = &*x;

            let component: &T = match component.as_any().downcast_ref::<T>() {
                Some(c) => c,
                None    => panic!("Could not get component from trait object"),
            };

            component
        })
    }

    fn get_comp_mut<'a, T: Component + 'static>(&'a mut self, id: EntityID) -> RefMut<T> {
        let comp = self.components.get_mut(&TypeId::of::<T>())
                                  .expect("Could not find a matching component storage")
                                  .get_mut(id);

        RefMut::map(comp, |x| {
            let component = &mut *x;

            let component: &mut T = match component.as_any_mut().downcast_mut::<T>() {
                Some(c) => c,
                None    => panic!("Could not get component from trait object"),
            };

            component
        })
    }

    fn new_entity(&mut self) -> EntityBuilder {
        EntityBuilder::new(self)
    }
}

#[derive(Debug)]
struct Pos(f32, f32);

impl Component for Pos {
    fn as_any(&self) -> &std::any::Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut std::any::Any {
        self
    }
}

fn main() {
    let mut world = World::new();

    world.components.insert(TypeId::of::<Pos>(), Box::new(Vec::new()));

    {
        let mut builder = world.new_entity();

        builder.add_component(Pos(3.0, 5.0));
    }

    let mut comp = world.get_comp_mut::<Pos>(0);

    println!("Component: {:?}", comp);

/*
    let id = world.id_tracker.next_id();

    world.insert_comp(id, Pos(3.0, 5.0));

    {
        let mut comp = world.get_comp_mut::<Pos>(id);

        println!("Component: {:?}", comp);

        *comp = Pos(4.0, 3.5);
    }

    let mut comp2 = world.get_comp_mut::<Pos>(id);

    println!("Component: {:?}", comp2);

*/
}
