/*
MIT License

Copyright (c) 2021, 2022, 2024, 2025 Vincent Hiribarren

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.
*/

use std::rc::Rc;

use wgpu_igniter::cameras::{PerspectiveConfig, WinitCameraAdapter};
use wgpu_igniter::draw_context::DrawContext;
use wgpu_igniter::primitives::{Object3D, Shareable, Transforms, cube};
use wgpu_igniter::render_loop::{RenderContext, SceneElements, SceneLoopHandler};
use wgpu_igniter::scene_3d::Scene3D;

const DEFAULT_SHADER: &str = include_str!("cube_normals.wgsl");

const ROTATION_DEG_PER_S: f32 = 45.0;

pub struct MainScenario {
    pub cube: Rc<std::cell::RefCell<Object3D>>,
    pub scene_elements: SceneElements,
}

impl MainScenario {
    pub fn new(draw_context: &DrawContext) -> Self {
        let camera = WinitCameraAdapter::new(PerspectiveConfig::default().into());
        let shader_module = draw_context.create_shader_module(DEFAULT_SHADER);
        let mut scene = Scene3D::new(draw_context);
        let cube = cube::create_cube_with_normals(
            draw_context,
            &shader_module,
            &shader_module,
            scene.scene_uniforms(),
            Default::default(),
        )
        .into_shareable();
        scene.add(cube.clone());
        let scene_elements = SceneElements { camera, scene };
        Self {
            cube,
            scene_elements,
        }
    }
}

impl SceneLoopHandler for MainScenario {
    fn scene_elements_mut(&mut self) -> &mut SceneElements {
        &mut self.scene_elements
    }

    fn on_update(&mut self, update_context: &RenderContext) {
        let update_interval = update_context.time_info;
        let delta_rotation = ROTATION_DEG_PER_S * update_interval.processing_delta.as_secs_f32();
        let transform = cgmath::Matrix4::from_angle_z(cgmath::Deg(delta_rotation))
            * cgmath::Matrix4::from_angle_y(cgmath::Deg(delta_rotation));
        self.cube.borrow_mut().apply_transform(transform);
    }
}
