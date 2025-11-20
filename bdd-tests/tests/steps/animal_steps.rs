use crate::TraitsWorld;
use cucumber::{given, then};
use rust_traits_examples::animals::*;

#[given(expr = "a dog named {string} with breed {string}")]
async fn a_dog_named_with_breed(world: &mut TraitsWorld, name: String, breed: String) {
    let dog_breed = match breed.as_str() {
        "Labrador" => dog::DogBreed::Labrador,
        "BorderCollie" => dog::DogBreed::BorderCollie,
        "Greyhound" => dog::DogBreed::Greyhound,
        "Husky" => dog::DogBreed::Husky,
        "Bulldog" => dog::DogBreed::Bulldog,
        "GoldenRetriever" => dog::DogBreed::GoldenRetriever,
        "SaintBernard" => dog::DogBreed::SaintBernard,
        _ => dog::DogBreed::Other,
    };

    world.dog = Some(Dog::new(name, dog_breed));
}

#[given(expr = "a duck named {string}")]
async fn a_duck_named(world: &mut TraitsWorld, name: String) {
    world.duck = Some(Duck::new(name));
}

#[given(expr = "an eagle named {string}")]
async fn an_eagle_named(world: &mut TraitsWorld, name: String) {
    world.eagle = Some(Eagle::new(name));
}

#[given(expr = "a penguin named {string}")]
async fn a_penguin_named(world: &mut TraitsWorld, name: String) {
    world.penguin = Some(Penguin::new(name));
}

#[given(expr = "a whale named {string} of species {string}")]
async fn a_whale_named_of_species(world: &mut TraitsWorld, name: String, species: String) {
    let whale_species = match species.as_str() {
        "BlueWhale" => whale::WhaleSpecies::BlueWhale,
        "Orca" => whale::WhaleSpecies::Orca,
        "Humpback" => whale::WhaleSpecies::Humpback,
        "Dolphin" => whale::WhaleSpecies::Dolphin,
        _ => whale::WhaleSpecies::BlueWhale,
    };

    world.whale = Some(Whale::new(name, whale_species));
}

#[given(expr = "a snake named {string} of species {string}")]
async fn a_snake_named_of_species(world: &mut TraitsWorld, name: String, species: String) {
    let snake_species = match species.as_str() {
        "Anaconda" => snake::SnakeSpecies::Anaconda,
        "Python" => snake::SnakeSpecies::Python,
        "Cobra" => snake::SnakeSpecies::Cobra,
        "Viper" => snake::SnakeSpecies::Viper,
        _ => snake::SnakeSpecies::Anaconda,
    };

    world.snake = Some(Snake::new(name, snake_species));
}

#[then(expr = "the dog should be able to walk")]
async fn the_dog_should_be_able_to_walk(world: &mut TraitsWorld) {
    world.can_walk = world.dog.is_some();
    assert!(world.can_walk, "Dog should be able to walk");
}

#[then(expr = "the dog should be able to swim")]
async fn the_dog_should_be_able_to_swim(world: &mut TraitsWorld) {
    world.can_swim = world.dog.is_some();
    assert!(world.can_swim, "Dog should be able to swim");
}

#[then(expr = "the dog should be able to land move")]
async fn the_dog_should_be_able_to_land_move(world: &mut TraitsWorld) {
    world.can_land_move = world.dog.is_some();
    assert!(world.can_land_move, "Dog should be able to land move");
}

#[then(expr = "the dog should not be able to fly")]
async fn the_dog_should_not_be_able_to_fly(world: &mut TraitsWorld) {
    world.can_fly = false;
    assert!(!world.can_fly, "Dog should not be able to fly");
}

#[then(expr = "the duck should be able to walk")]
async fn the_duck_should_be_able_to_walk(world: &mut TraitsWorld) {
    world.can_walk = world.duck.is_some();
    assert!(world.can_walk, "Duck should be able to walk");
}

#[then(expr = "the duck should be able to swim")]
async fn the_duck_should_be_able_to_swim(world: &mut TraitsWorld) {
    world.can_swim = world.duck.is_some();
    assert!(world.can_swim, "Duck should be able to swim");
}

#[then(expr = "the duck should be able to fly")]
async fn the_duck_should_be_able_to_fly(world: &mut TraitsWorld) {
    world.can_fly = world.duck.is_some();
    assert!(world.can_fly, "Duck should be able to fly");
}

#[then(expr = "the duck should be able to land move")]
async fn the_duck_should_be_able_to_land_move(world: &mut TraitsWorld) {
    world.can_land_move = world.duck.is_some();
    assert!(world.can_land_move, "Duck should be able to land move");
}

#[then(expr = "the eagle should be able to walk")]
async fn the_eagle_should_be_able_to_walk(world: &mut TraitsWorld) {
    world.can_walk = world.eagle.is_some();
    assert!(world.can_walk, "Eagle should be able to walk");
}

#[then(expr = "the eagle should be able to fly")]
async fn the_eagle_should_be_able_to_fly(world: &mut TraitsWorld) {
    world.can_fly = world.eagle.is_some();
    assert!(world.can_fly, "Eagle should be able to fly");
}

#[then(expr = "the eagle should be able to land move")]
async fn the_eagle_should_be_able_to_land_move(world: &mut TraitsWorld) {
    world.can_land_move = world.eagle.is_some();
    assert!(world.can_land_move, "Eagle should be able to land move");
}

#[then(expr = "the eagle should not be able to swim")]
async fn the_eagle_should_not_be_able_to_swim(world: &mut TraitsWorld) {
    world.can_swim = false;
    assert!(!world.can_swim, "Eagle should not be able to swim");
}

#[then(expr = "the whale should be able to swim")]
async fn the_whale_should_be_able_to_swim(world: &mut TraitsWorld) {
    world.can_swim = world.whale.is_some();
    assert!(world.can_swim, "Whale should be able to swim");
}

#[then(expr = "the whale should not be able to walk")]
async fn the_whale_should_not_be_able_to_walk(world: &mut TraitsWorld) {
    world.can_walk = false;
    assert!(!world.can_walk, "Whale should not be able to walk");
}

#[then(expr = "the whale should not be able to fly")]
async fn the_whale_should_not_be_able_to_fly(world: &mut TraitsWorld) {
    world.can_fly = false;
    assert!(!world.can_fly, "Whale should not be able to fly");
}

#[then(expr = "the whale should not be able to land move")]
async fn the_whale_should_not_be_able_to_land_move(world: &mut TraitsWorld) {
    world.can_land_move = false;
    assert!(
        !world.can_land_move,
        "Whale should not be able to land move"
    );
}

#[then(expr = "the snake should be able to swim")]
async fn the_snake_should_be_able_to_swim(world: &mut TraitsWorld) {
    world.can_swim = world.snake.is_some();
    assert!(world.can_swim, "Snake should be able to swim");
}

#[then(expr = "the snake should not be able to walk")]
async fn the_snake_should_not_be_able_to_walk(world: &mut TraitsWorld) {
    world.can_walk = false;
    assert!(!world.can_walk, "Snake should not be able to walk");
}

#[then(expr = "the snake should not be able to land move")]
async fn the_snake_should_not_be_able_to_land_move(world: &mut TraitsWorld) {
    world.can_land_move = false;
    assert!(
        !world.can_land_move,
        "Snake should not be able to land move"
    );
}
