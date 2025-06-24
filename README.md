# Software rasterizer

Building a simple software rasterizer in Rust. Initially, the project mostly followed
[Coding Adventure: Software Rasterizer](https://www.youtube.com/watch?v=yyJ-hdISgnw&t=2436s)
by Sebastian Lague with some adjustments to more closely follow modern
graphics APIs.
As the project grew I got interested in implementing more features such as lights,
shadows and more advanced shading.

Performance for pure rasterization is pretty decent but shadows and lights really
slow things down. The design of the rasterizer follows modern graphics APIs
(OpenGL, DirectX, Vulkan) which is probably part of the problem why it is slow.
These APIs target highly parallelized and optimized GPUs which can solve many
of the problems encountered in rasterization much more efficiently than
implementing those same algorithms on the CPU.

## Other sources

- Thomas Akenine-Möller, Eric Haines, and Naty Hoffman (2008) "Real-Time Rendering", 3rd Edition, A K Peters, Ltd
  (see [https://www.realtimerendering.com/](https://www.realtimerendering.com/) for the current edition)
- [Great explanation of orthographic and perspective projection](https://www.youtube.com/watch?v=U0_ONQQ5ZNM)
- [All the math behind the OpenGL projection matrix](https://www.songho.ca/opengl/gl_projectionmatrix.html)
- [Overview of rasterization algorithm](https://www.scratchapixel.com/lessons/3d-basic-rendering/rasterization-practical-implementation/overview-rasterization-algorithm.html)
- Clipping in homogeneous space, [Blinn and Newell (1978) - Clipping using homogeneous coordinates](https://dl.acm.org/doi/10.1145/965139.807398)
- [Perspective-correct interpolation](https://www.comp.nus.edu.sg/%7Elowkl/publications/lowk_persp_interp_techrep.pdf) - a crucial step for interpolating vertex shader outputs across fragments
- [Explanation of shadow mapping](https://learnopengl.com/Advanced-Lighting/Shadows/Shadow-Mapping)

## Goals

- Shadow Maps
- Additional lights: Point lights
- Rudimentary but interesting shader model, e.g. Blinn-Phong
