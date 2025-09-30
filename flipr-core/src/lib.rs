//! Core traits and types for functional image processing.
//!
//! This module provides the foundational traits for building image processing pipelines,
//! similar to how `std::iter::Iterator` works for sequences.

/// Represents a pixel value in an image.
pub trait Pixel: Clone + Copy + Send + Sync {
    /// The scalar type used for pixel components.
    type Scalar: Copy + Send + Sync;
}

/// A trait for types that can produce pixels, similar to `Iterator`.
///
/// This trait is the core abstraction for image processing pipelines in flipr.
/// It allows chaining operations in a functional style.
pub trait ImageProcessor: Sized {
    /// The pixel type produced by this processor.
    type Pixel: Pixel;
    
    /// The error type that can occur during processing.
    type Error;
    
    /// Process a single pixel at the given coordinates.
    ///
    /// Returns `None` if the coordinates are out of bounds.
    fn process_pixel(&self, x: usize, y: usize) -> Result<Option<Self::Pixel>, Self::Error>;
    
    /// Get the dimensions of the image being processed.
    fn dimensions(&self) -> (usize, usize);
    
    /// Map each pixel through a function.
    fn map<F, P>(self, f: F) -> Map<Self, F>
    where
        F: Fn(Self::Pixel) -> P,
        P: Pixel,
    {
        Map { processor: self, f }
    }
    
    /// Filter pixels based on a predicate.
    fn filter<F>(self, predicate: F) -> Filter<Self, F>
    where
        F: Fn(&Self::Pixel) -> bool,
    {
        Filter { processor: self, predicate }
    }
    
    /// Chain multiple processors together.
    fn chain<P>(self, other: P) -> Chain<Self, P>
    where
        P: ImageProcessor<Pixel = Self::Pixel, Error = Self::Error>,
    {
        Chain { first: self, second: other }
    }
}

/// An image processor that maps pixels through a function.
pub struct Map<P, F> {
    processor: P,
    f: F,
}

impl<P, F, Pix> ImageProcessor for Map<P, F>
where
    P: ImageProcessor,
    F: Fn(P::Pixel) -> Pix,
    Pix: Pixel,
{
    type Pixel = Pix;
    type Error = P::Error;
    
    fn process_pixel(&self, x: usize, y: usize) -> Result<Option<Self::Pixel>, Self::Error> {
        self.processor.process_pixel(x, y).map(|opt| opt.map(&self.f))
    }
    
    fn dimensions(&self) -> (usize, usize) {
        self.processor.dimensions()
    }
}

/// An image processor that filters pixels based on a predicate.
pub struct Filter<P, F> {
    processor: P,
    predicate: F,
}

impl<P, F> ImageProcessor for Filter<P, F>
where
    P: ImageProcessor,
    F: Fn(&P::Pixel) -> bool,
{
    type Pixel = P::Pixel;
    type Error = P::Error;
    
    fn process_pixel(&self, x: usize, y: usize) -> Result<Option<Self::Pixel>, Self::Error> {
        self.processor.process_pixel(x, y).map(|opt| {
            opt.and_then(|pixel| {
                if (self.predicate)(&pixel) {
                    Some(pixel)
                } else {
                    None
                }
            })
        })
    }
    
    fn dimensions(&self) -> (usize, usize) {
        self.processor.dimensions()
    }
}

/// An image processor that chains two processors together.
pub struct Chain<P1, P2> {
    first: P1,
    second: P2,
}

impl<P1, P2> ImageProcessor for Chain<P1, P2>
where
    P1: ImageProcessor,
    P2: ImageProcessor<Pixel = P1::Pixel, Error = P1::Error>,
{
    type Pixel = P1::Pixel;
    type Error = P1::Error;
    
    fn process_pixel(&self, x: usize, y: usize) -> Result<Option<Self::Pixel>, Self::Error> {
        let (w1, h1) = self.first.dimensions();
        if x < w1 && y < h1 {
            self.first.process_pixel(x, y)
        } else {
            let (_w2, _h2) = self.second.dimensions();
            self.second.process_pixel(x - w1, y)
        }
    }
    
    fn dimensions(&self) -> (usize, usize) {
        let (w1, h1) = self.first.dimensions();
        let (w2, h2) = self.second.dimensions();
        (w1 + w2, h1.max(h2))
    }
}

/// Basic RGB pixel implementation.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Rgb<T> {
    pub r: T,
    pub g: T,
    pub b: T,
}

impl<T: Copy + Send + Sync> Pixel for Rgb<T> {
    type Scalar = T;
}

/// Basic grayscale pixel implementation.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Gray<T> {
    pub value: T,
}

impl<T: Copy + Send + Sync> Pixel for Gray<T> {
    type Scalar = T;
}

#[cfg(test)]
mod tests {
    use super::*;
    
    struct TestProcessor {
        width: usize,
        height: usize,
    }
    
    impl ImageProcessor for TestProcessor {
        type Pixel = Gray<u8>;
        type Error = ();
        
        fn process_pixel(&self, x: usize, y: usize) -> Result<Option<Self::Pixel>, Self::Error> {
            if x < self.width && y < self.height {
                Ok(Some(Gray { value: (x + y) as u8 }))
            } else {
                Ok(None)
            }
        }
        
        fn dimensions(&self) -> (usize, usize) {
            (self.width, self.height)
        }
    }
    
    #[test]
    fn test_basic_processor() {
        let processor = TestProcessor { width: 10, height: 10 };
        let pixel = processor.process_pixel(5, 5).unwrap();
        assert_eq!(pixel, Some(Gray { value: 10 }));
    }
    
    #[test]
    fn test_map() {
        let processor = TestProcessor { width: 10, height: 10 };
        let mapped = processor.map(|p: Gray<u8>| Gray { value: p.value * 2 });
        let pixel = mapped.process_pixel(5, 5).unwrap();
        assert_eq!(pixel, Some(Gray { value: 20 }));
    }
    
    #[test]
    fn test_filter() {
        let processor = TestProcessor { width: 10, height: 10 };
        let filtered = processor.filter(|p: &Gray<u8>| p.value < 10);
        let pixel = filtered.process_pixel(5, 5).unwrap();
        assert_eq!(pixel, None);
        let pixel2 = filtered.process_pixel(2, 2).unwrap();
        assert_eq!(pixel2, Some(Gray { value: 4 }));
    }
}
