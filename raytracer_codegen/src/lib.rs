#![allow(clippy::all)]

extern crate proc_macro;
use proc_macro2::TokenStream;
use quote::quote;

/// You can replace this `Vec` with your own version, e.g. the one
/// you wrote in raytracer, if you want.
#[derive(Copy, Clone, Debug)]
struct Vec3(f64, f64, f64);

/// You can replace this `Sphere` with your own version, e.g. the one
/// you wrote in raytracer, if you want.
#[derive(Copy, Clone, Debug)]
struct Sphere {
    pub pos: Vec3,
    pub size: f64,
    pub color: Vec3,
}

/// This function generates code for one Sphere.
fn sphere_code(sphere: &Sphere) -> TokenStream {
    let Vec3(x, y, z) = sphere.pos;
    let Vec3(r, g, b) = sphere.color;
    let size = sphere.size * 0.9;

    // Create a code snippet of `Sphere`.
    // Note that the `Sphere` in `quote` is the one in your ray tracer,
    // not the one you defined in this module.
    quote! {
        Box::new(Sphere {
            center: Vec3::new(#x, #y, #z),
            radius: #size,
            material: DiffuseLight(
                ConstantTexture(
                    Vec3::new(#r, #g, #b)
                )
            )
        })
    }
}

/// This function generates the final `make_spheres` function.
#[proc_macro]
pub fn make_spheres_impl(_item: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let spheres = vec![
        &Sphere {
            pos: Vec3(0.0, 0.0, 0.0),
            size: 1.0,
            color: Vec3(1.0, 1.0, 1.0),
        },
        &Sphere {
            pos: Vec3(0.0, 0.0, 0.0),
            size: 1.0,
            color: Vec3(1.0, 1.0, 1.0),
        },
    ];

    let mut tokens = vec![];

    for sphere in spheres {
        let sc = sphere_code(sphere);
        tokens.push(quote! {
            spheres.push(#sc);
        });
    }

    let x_final: TokenStream = tokens.into_iter().collect();

    let result = proc_macro::TokenStream::from(quote! {
        fn make_spheres() -> Vec<Box<Sphere>> {
            let mut spheres = vec![];
            #x_final
            spheres
        }
    });

    // uncomment this statement if you want to inspect result
    // println!("{}", result);

    result
}
