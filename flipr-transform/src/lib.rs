//! Affine transformations for image processing.
//!
//! This module provides space transformations such as translation, rotation,
//! scaling, and general affine mappings for images.

use flipr_core::ImageProcessor;

/// A 2D affine transformation matrix.
///
/// Represents transformations of the form:
/// ```text
/// [x']   [a  b  tx]   [x]
/// [y'] = [c  d  ty] * [y]
/// [1 ]   [0  0  1 ]   [1]
/// ```
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct AffineTransform {
    pub a: f64,
    pub b: f64,
    pub c: f64,
    pub d: f64,
    pub tx: f64,
    pub ty: f64,
}

impl AffineTransform {
    /// Create an identity transform (no transformation).
    pub fn identity() -> Self {
        Self {
            a: 1.0,
            b: 0.0,
            c: 0.0,
            d: 1.0,
            tx: 0.0,
            ty: 0.0,
        }
    }
    
    /// Create a translation transform.
    pub fn translation(dx: f64, dy: f64) -> Self {
        Self {
            a: 1.0,
            b: 0.0,
            c: 0.0,
            d: 1.0,
            tx: dx,
            ty: dy,
        }
    }
    
    /// Create a scaling transform.
    pub fn scale(sx: f64, sy: f64) -> Self {
        Self {
            a: sx,
            b: 0.0,
            c: 0.0,
            d: sy,
            tx: 0.0,
            ty: 0.0,
        }
    }
    
    /// Create a rotation transform (angle in radians).
    pub fn rotation(angle: f64) -> Self {
        let cos = angle.cos();
        let sin = angle.sin();
        Self {
            a: cos,
            b: -sin,
            c: sin,
            d: cos,
            tx: 0.0,
            ty: 0.0,
        }
    }
    
    /// Apply the transformation to a point.
    pub fn transform_point(&self, x: f64, y: f64) -> (f64, f64) {
        let x_new = self.a * x + self.b * y + self.tx;
        let y_new = self.c * x + self.d * y + self.ty;
        (x_new, y_new)
    }
    
    /// Compute the inverse transformation.
    pub fn inverse(&self) -> Option<Self> {
        let det = self.a * self.d - self.b * self.c;
        if det.abs() < 1e-10 {
            return None;
        }
        
        let inv_det = 1.0 / det;
        Some(Self {
            a: self.d * inv_det,
            b: -self.b * inv_det,
            c: -self.c * inv_det,
            d: self.a * inv_det,
            tx: (self.b * self.ty - self.d * self.tx) * inv_det,
            ty: (self.c * self.tx - self.a * self.ty) * inv_det,
        })
    }
    
    /// Compose two transformations (apply `other` after `self`).
    pub fn then(&self, other: &AffineTransform) -> Self {
        Self {
            a: other.a * self.a + other.b * self.c,
            b: other.a * self.b + other.b * self.d,
            c: other.c * self.a + other.d * self.c,
            d: other.c * self.b + other.d * self.d,
            tx: other.a * self.tx + other.b * self.ty + other.tx,
            ty: other.c * self.tx + other.d * self.ty + other.ty,
        }
    }
}

/// An image processor that applies an affine transformation.
pub struct Transformed<P> {
    processor: P,
    transform: AffineTransform,
}

impl<P> Transformed<P> {
    /// Create a new transformed processor.
    pub fn new(processor: P, transform: AffineTransform) -> Self {
        Self { processor, transform }
    }
}

impl<P> ImageProcessor for Transformed<P>
where
    P: ImageProcessor,
{
    type Pixel = P::Pixel;
    type Error = P::Error;
    
    fn process_pixel(&self, x: usize, y: usize) -> Result<Option<Self::Pixel>, Self::Error> {
        // Apply inverse transformation to find source coordinates
        if let Some(inv) = self.transform.inverse() {
            let (src_x, src_y) = inv.transform_point(x as f64, y as f64);
            
            // Simple nearest-neighbor sampling
            let src_x_i = src_x.round() as isize;
            let src_y_i = src_y.round() as isize;
            
            if src_x_i >= 0 && src_y_i >= 0 {
                self.processor.process_pixel(src_x_i as usize, src_y_i as usize)
            } else {
                Ok(None)
            }
        } else {
            Ok(None)
        }
    }
    
    fn dimensions(&self) -> (usize, usize) {
        // For simplicity, use the same dimensions as the source
        // In a real implementation, we'd compute the bounding box
        self.processor.dimensions()
    }
}

/// Extension trait for adding transformation methods to image processors.
pub trait TransformExt: ImageProcessor + Sized {
    /// Apply an affine transformation to the image.
    fn transform(self, transform: AffineTransform) -> Transformed<Self> {
        Transformed::new(self, transform)
    }
    
    /// Translate the image.
    fn translate(self, dx: f64, dy: f64) -> Transformed<Self> {
        self.transform(AffineTransform::translation(dx, dy))
    }
    
    /// Scale the image.
    fn scale(self, sx: f64, sy: f64) -> Transformed<Self> {
        self.transform(AffineTransform::scale(sx, sy))
    }
    
    /// Rotate the image (angle in radians).
    fn rotate(self, angle: f64) -> Transformed<Self> {
        self.transform(AffineTransform::rotation(angle))
    }
}

impl<P: ImageProcessor> TransformExt for P {}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_identity_transform() {
        let t = AffineTransform::identity();
        let (x, y) = t.transform_point(10.0, 20.0);
        assert_eq!(x, 10.0);
        assert_eq!(y, 20.0);
    }
    
    #[test]
    fn test_translation() {
        let t = AffineTransform::translation(5.0, 10.0);
        let (x, y) = t.transform_point(10.0, 20.0);
        assert_eq!(x, 15.0);
        assert_eq!(y, 30.0);
    }
    
    #[test]
    fn test_scaling() {
        let t = AffineTransform::scale(2.0, 3.0);
        let (x, y) = t.transform_point(10.0, 20.0);
        assert_eq!(x, 20.0);
        assert_eq!(y, 60.0);
    }
    
    #[test]
    fn test_inverse() {
        let t = AffineTransform::translation(5.0, 10.0);
        let inv = t.inverse().unwrap();
        let (x, y) = t.transform_point(10.0, 20.0);
        let (x2, y2) = inv.transform_point(x, y);
        assert!((x2 - 10.0).abs() < 1e-10);
        assert!((y2 - 20.0).abs() < 1e-10);
    }
    
    #[test]
    fn test_composition() {
        let t1 = AffineTransform::translation(5.0, 10.0);
        let t2 = AffineTransform::scale(2.0, 2.0);
        let composed = t1.then(&t2);
        
        let (x1, y1) = t1.transform_point(10.0, 20.0);
        let (x2, y2) = t2.transform_point(x1, y1);
        let (x3, y3) = composed.transform_point(10.0, 20.0);
        
        assert_eq!(x2, x3);
        assert_eq!(y2, y3);
    }
}
