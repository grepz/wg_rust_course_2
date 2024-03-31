use std::fmt;

struct Dog {
    id: u64
}

impl Animal for Dog {
    fn class(&self) -> &str { "dog" }
    fn say(&self) -> &str { "woof!" }
}

impl fmt::Debug for Dog {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Dog")
         .field("id", &self.id)
         .finish()
    }
}

struct Cat {
    id: u64
}

impl Animal for Cat {
    fn class(&self) -> &str { "cat" }
    fn say(&self) -> &str { "Hsss!" }
}

impl fmt::Debug for Cat {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Cat")
            .field("id", &self.id)
            .finish()
    }
}

struct Alien {
    id: i64
}

impl Animal for Alien {
    fn class(&self) -> &str { "alien" }
    fn say(&self) -> &str { "::silence::" }
}

impl fmt::Debug for Alien {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("alien")
            .field("id", &self.id)
            .finish()
    }
}

#[derive(Debug)]
enum AnimalsEnum {
    Dog(Dog),
    Cat(Cat),
    Alien(Alien)
}

pub trait Animal {
    fn say(&self) -> &str;
    fn class(&self) -> &str;
}

impl Animal for AnimalsEnum {
    fn class(&self) -> &str {
        match self {
            AnimalsEnum::Dog(dog) => dog.class(),
            AnimalsEnum::Cat(cat) => cat.class(),
            _ => { "Unknown" }
        }
    }

    fn say(&self) -> &str {
        match self {
            AnimalsEnum::Dog(dog) => dog.say(),
            AnimalsEnum::Cat(cat) => cat.say(),
            _ => { "Unknown" }
        }
    }
}

fn main() {
    let dog = AnimalsEnum::Dog(Dog { id: 0 });
    let cat = AnimalsEnum::Cat(Cat { id: 1 });
    let alien = AnimalsEnum::Alien(Alien { id: -1 });

    let animals: Vec<AnimalsEnum> = vec![dog, cat, alien];

    for animal in animals.iter() {
        println!("Animal {} say: {}", animal.class(), animal.say());
    }
}
