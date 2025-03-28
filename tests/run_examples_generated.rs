// File generated by build.rs, do not modify directly

/*
MIT License

Copyright (c) 2025 Vincent Hiribarren

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

// File generated by build.rs, do not modify directly

use assert_cmd::Command;
use escargot;
use std::time::Duration;

const TIMEOUT_DURATION: Duration = Duration::from_secs(10);

macro_rules! generate_test_case {
    ($funcname:ident, $testname:tt) => {
        #[test]
        fn $funcname() {
            let example_under_test = escargot::CargoBuild::new()
                .example($testname)
                .run()
                .unwrap();
            Command::from_std(example_under_test.command())
                .env("HEADLESS", "true")
                .timeout(TIMEOUT_DURATION)
                .assert()
                .success();
        }
    };
}

#[rustfmt::skip]
generate_test_case!(example_canvas_raw_doesnt_panic, "canvas_raw");
#[rustfmt::skip]
generate_test_case!(example_cube_instances_doesnt_panic, "cube_instances");
#[rustfmt::skip]
generate_test_case!(example_cube_normals_doesnt_panic, "cube_normals");
#[rustfmt::skip]
generate_test_case!(example_cube_shader_transition_doesnt_panic, "cube_shader_transition");
#[rustfmt::skip]
generate_test_case!(example_cube_simple_doesnt_panic, "cube_simple");
#[rustfmt::skip]
generate_test_case!(example_cube_two_doesnt_panic, "cube_two");
#[rustfmt::skip]
generate_test_case!(example_egui_integration_doesnt_panic, "egui_integration");
#[rustfmt::skip]
generate_test_case!(example_triangle_direct_doesnt_panic, "triangle_direct");
