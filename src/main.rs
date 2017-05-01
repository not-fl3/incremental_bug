use std::marker::PhantomData;

pub struct Entity;

pub trait Deserializers {
}

pub struct DeserializersStorage<T, U : Deserializers> {
    name : String,
    marker : PhantomData<T>,
    next : U
}

impl<T, U : Deserializers> Deserializers for DeserializersStorage<T, U> {

}

impl Deserializers for () {
}

impl<T> DeserializersStorage<T, ()> {
    pub fn new(name : String, _ : PhantomData<T>) ->
        DeserializersStorage<T, ()> {
            DeserializersStorage {
                name   : name,
                marker : PhantomData,
                next   : ()
            }
        }
}

impl<T, U : Deserializers> DeserializersStorage<T, U> {
    pub fn add<T1>(self, name : String) ->
        DeserializersStorage<T1, DeserializersStorage<T, U>> {
            DeserializersStorage {
                name : name,
                marker : PhantomData,
                next : self
            }
        }
}

#[macro_export]
macro_rules! deserializers {
    ($t:ty) => {
        DeserializersStorage::new(stringify!($t).to_string(), ::std::marker::PhantomData::<$t>)
    };
    ($t1:ty, $($t:ty),+) => {
        {
            let deserializers = DeserializersStorage::new(stringify!($t1).to_string(), ::std::marker::PhantomData::<$t1>);
            $(
                let deserializers = deserializers.add::<$t>(stringify!($t).to_string());
            )+
            let deserializers : Box<Deserializers> = Box::new(deserializers);
            deserializers
        }
    };
}


struct Foo;
struct Foo1;
struct Foo2;
struct Foo3;
struct Foo4;
struct Foo5;
struct Foo6;
struct Foo7;
struct Foo8;
struct Foo9;
struct Foo10;
struct Foo11;
struct Foo12;
struct Foo13;

fn main() {
    deserializers!(Foo,
                   Foo1,
                   Foo2,
                   Foo3,
                   Foo4,
                   Foo5,
                   Foo6,
                   Foo7,
                   Foo8,
                   Foo9,
                   Foo10,
                   Foo11,
                   Foo12,
                   Foo13);
}
