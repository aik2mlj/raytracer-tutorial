![banner](https://user-images.githubusercontent.com/4198311/89027545-461dd180-d35d-11ea-9972-7bf1b07f942d.png)

# Pseudo Photograph Company of ACM

> [中文版](README_CN.md)

Friends from the Engineering School and ACM Class have finished their courses! Check out these amazing artworks:

- [Engineering School](https://github.com/aik2mlj/raytracer-tutorial/issues/1)
- [ACM Class](https://github.com/aik2mlj/raytracer-tutorial/issues/2)

---

The ACM Pseudo Photography Company, affectionately known as PPCA, was established in 2021. 😉

The main goal of this project is to implement a ray tracing renderer using the Rust programming language. Through this, you'll deepen your understanding of programming language design and compiler principles by learning a new (and beautiful) language, while also getting a fun introduction to the fundamentals of computer graphics.

This year, we've added a peer-review session. Use your hand-written renderer and unleash your artistic talent to create stunning, surreal masterpieces!

Primary reference materials:
- [The Rust Programming Language](https://doc.rust-lang.org/book/title-page.html)
- [Ray Tracing in One Weekend - The Book Series](https://raytracing.github.io)

More references can be found in the Reference section below.

Click "Use this template" at the top right to copy this project to your own GitHub profile. Then, you'll need to do some preparation.

## Task 0: Preparation

* Modify the author information in `raytracer/Cargo.toml`.
* Change the author in `LICENSE` to yourself. You may also switch to a different license.
* [Install Rust using rustup](https://doc.rust-lang.org/book/ch01-01-installation.html). If the download speed is slow, consider using the [SJTUG Mirror](https://mirrors.sjtug.sjtu.edu.cn) for rust-static and crates.io.
* Next, install some tools. Navigate to the project directory and run `rustup component add clippy rustfmt`.
* Then, run `make ci`. If the program runs successfully, your environment is set up.
* Push these changes to GitHub. Both "Lint and Test" and "Build and Upload" should pass in GitHub Actions.
* The program output will appear in the GitHub Actions artifacts. The `output` folder should contain content generated at runtime.
  Changes to the `output` folder should not be synced to GitHub (this folder is listed in `.gitignore` and will be ignored by git).
* Finally, you can remove the tutorial sections from README.md and replace them with your own program description, usage instructions, etc.

## Task INF: Learn about Rust

We recommend spending the first week getting familiar with Rust syntax. Please read The Rust Book (or another tutorial you find suitable).
* Generally, you'll only need Chapters 1-6 and Section 10.2.
* If you run into lifetime issues, read Chapter 4 carefully, especially the examples in 4.2.
* You can also use smart pointers from Chapter 15 to resolve some lifetime-related issues.
* Rust's object-oriented features (traits, analogous to C++ classes) can be found in Section 10.2.
* (Advanced) When working with multi-threaded rendering, you can read Chapters 15 and 16.

## Task 1: One Weekend

- Ray Tracing book 1 — a gentle weekend.

- [x] Code review (60 pts for Engineering School, 30 pts for ACM Class): Monday of Week 2.
  - Details related to Book 1.
  - Rust proficiency (basic, not beyond the required chapters).

## Task 2: Next Week

- Ray Tracing book 2.
- Multi-threaded rendering.

- [x] Code review (Engineering School only, 35 pts for both Engineering School & ACM Class): Friday of Week 2, 3:00 PM.

  - Required portion (20 pts — no points if incomplete): BVH, Rectangles and Lights. The remaining parts are scored based on completion, capped at finishing all of Book 2 + multi-threaded rendering.

  - Pay special attention to the implementation details of Book 2, especially the BVH section — make sure you understand it!
  - Engineering School friends finish the course here! 🎉 Peer review session! 🤯

## Task 3: Rest of Your Life & Extra Work

- Ray Tracing book 3 (20 pts for ACM Class).
- Advanced features (Bonus, 10 pts for ACM Class).
  - Full marks for completing 3 tracks.
  - If you hand-wrote your own obj_loader (without using a library), you only need to complete 2 tracks (including Track 7) for full marks.

- [x] Code review: Friday of Week 4.
  - Details related to Book 3.
  - Details related to advanced features.
  - ACM Class friends finish the course here! 🎉 Peer review session! 🤯 (5 pts for ACM Class)

### Advanced Features
* **Track 1: Reduce Contention** — Prerequisite: multi-threaded rendering. In a multi-threaded environment, cloning/dropping `Arc` can cause performance degradation. The goal is to reduce `Arc` usage: clone `Arc` only when spawning threads; everywhere else, use references instead of `Arc`.
* **Track 2: Static Dispatch** — Calling functions on `Box<dyn trait>` / `Arc<dyn trait>` / `&dyn trait` incurs extra overhead. We can solve this with generics.
  * Define new generic materials, transforms, and objects — e.g., `LambertianStatic<T>` — and use them in your scenes to reduce dynamic dispatch overhead. You can also create a separate module with structs that share names with the original materials.
  * You can find examples of generic usage in `material.rs`.
  * Only use `dyn` in `HitRecord`, `ScatterRecord` (from the remainder of Rest of Your Life), `HittableList`, and `BVHNode`.
  * If interested, explore using `macro_rules!` to reduce boilerplate from writing nearly identical code twice.
* **Track 3: Code Generation** — Prerequisite: BVH.
  * Currently, `BVHNode` is constructed at runtime. This process can actually be done at compile time. Use procedural macros to generate all objects and build a static `BVHNode`, improving rendering performance.
  * `raytracer_codegen` and `raytracer` likely can't share modules, so you may need to copy some implementations (e.g., `Vec3`) into `raytracer_codegen`.
  * You can use `cargo expand` to inspect the code after macro expansion. You can also output the generated code directly during compilation.
  * The `codegen` portion does not need to pass clippy.
  * If interested, explore passing arguments to procedural macros — e.g., `make_spheres_impl! { 100 }` to generate a function that produces 100 spheres.
* **Track 4: PDF Static Dispatch** — Prerequisite: the remainder of Rest of Your Life. Use generics for the objects handled in PDFs, removing `&dyn` from the code path.
* **Track 5: More Code Generation** — In a procedural macro, read a file and generate the scene from a YAML or JSON file (pick one).
  * Examples are provided in the `data` folder.
  * The `bounding_box` field in the example `BVHNode` entries is redundant — you can ignore it.
  * You may use a library to read JSON / YAML.
* **Track 6: Advanced Features** — Add PDF support for Transforms.
* If you have extra time, benchmark the before and after of your optimizations.
  * Before starting Track 3, back up your code (e.g., record the git commit id). When working on Tracks 4, 5, and 6, keep the original scene and program and add new content on top.
  * You can use the `criterion` crate for benchmarking. For example, benchmark the time to cast a single ray by shooting random rays into a pre-built scene.
* **Track 7: Support for .obj** — Support loading and rendering `.obj` files. To complete this, you may need to:
  * Understand the OBJ file format.
  * Implement an obj_loader.
    * You may use a library, e.g., `tobj`.
    * See the [tobj documentation](https://docs.rs/tobj/3.0.1/tobj/) for the OBJ format and library usage, or search on your own.
  * Implement rendering of simple polygons.
  * Supporting OBJ will make your final masterpiece even more spectacular! :)                 Image source: reference materials ↓

  ![Rust logo with friends](http://i.imgur.com/E1ylrZW.png)

## More Information

### Makefile

The `Makefile` contains common commands for running the raytracer. If `make` is not installed, you can also run `cargo ...` directly.

* `make fmt` — Automatically formats all Rust code.
* `make clippy` — Enforces additional code style constraints.
* `make test` — Runs unit tests. Your `Vec3` implementation must pass all tests.
* `make run_release` — Runs the optimized program. Generally, you should use this to run the raytracer; otherwise, rendering will be very slow.
* `make run` — Runs the program in debug mode.
* `make ci` = `fmt + clippy + test + run_release`. It's recommended to run `make ci` before pushing code to the remote repository.

### GitHub Actions

This repository is pre-configured with GitHub Actions. Whenever you push code to the remote repository, GitHub runs the following two checks:

* **Lint and Test** — Runs all unit tests and checks code style.
* **Build and Upload** — Runs the optimized program and uploads the generated files from the `output` directory as build artifacts.

## Reference

* [The Rust Programming Language](https://doc.rust-lang.org/book/title-page.html)
* [rustlings](https://github.com/rust-lang/rustlings) — A collection of small Rust exercises. If you prefer learning Rust through practice, give this a try.
* [Ray Tracing in One Weekend — The Book Series](https://raytracing.github.io)
* (Advanced) Procedural macros:
  * [Procedural Macros](https://doc.rust-lang.org/reference/procedural-macros.html) (focus on function-like procedural macros)
  * [quote crate](https://crates.io/crates/quote)
* (Advanced) JSON / YAML reading:
  * [serde-json](https://docs.serde.rs/serde_json/) — only the untyped part is needed.
  * [yaml-rust](https://docs.rs/yaml-rust/0.4.4/yaml_rust/)
  * Generally, you won't need the serialization/deserialization package below.
  * [serde](https://serde.rs)
