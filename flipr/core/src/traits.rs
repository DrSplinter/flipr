use space::Place;

pub trait Image {
    type Pixel;

    fn get(&self, p: Place) -> Self::Pixel;
}
