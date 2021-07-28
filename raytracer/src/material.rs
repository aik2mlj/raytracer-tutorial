#![allow(dead_code)]
#![allow(clippy::boxed_local)]
// You SHOULD remove above line in your code.

// This file shows necessary examples of how to complete Track 4 and 5.

pub trait Texture {}
pub trait Material {}

/// `Lambertian` now takes a generic parameter `T`.
/// This reduces the overhead of using `Box<dyn Texture>`
#[derive(Clone)]
pub struct Lambertian<T: Texture> {
    pub albedo: T,
}

impl<T: Texture> Lambertian<T> {
    pub fn new(albedo: T) -> Self {
        Self { albedo }
    }
}

impl<T: Texture> Material for Lambertian<T> {}

pub trait Hitable {}
pub struct AABB;

/// This BVHNode should be constructed statically.
/// You should use procedural macro to generate code like this:
/// ```
/// let bvh = BVHNode::construct(
///     box BVHNode::construct(
///         box Sphere { .. }
///         box Sphere { .. }
///     ),
///     box BVHNode::construct(
///         box Sphere { .. }
///         box Sphere { .. }
///     )
/// )
/// ```
/// And you can put that `bvh` into your `HittableList`.
pub struct BVHNode<L: Hitable, R: Hitable> {
    left: Box<L>,
    right: Box<R>,
    bounding_box: AABB,
}

impl<L: Hitable, R: Hitable> BVHNode<L, R> {
    pub fn construct(_left: Box<L>, _right: Box<R>) -> Self {
        unimplemented!()
    }
}
