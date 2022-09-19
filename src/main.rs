use pupsy_engine::imgui;

use winit::event::{Event, VirtualKeyCode, ElementState, KeyboardInput, WindowEvent};
use winit::event_loop::{EventLoop, ControlFlow};

use ash::vk;
use std::ptr;
use pupsy_engine::utility::fps;

use pupsy_engine::utility::constants as global_constants;
use pupsy_engine::rhi::window;
use pupsy_engine::rhi;
use std::time::{SystemTime, UNIX_EPOCH, Duration};

use pupsy_engine::vk::render_device as vk_render;
use pupsy_engine::imgui::constants as imgui_constants;

struct PupsyEngine {
    render_device: vk_render::VkRenderDevice,
    window: window::Window,
    fps_manager: fps::FPSManager,

    ui_engine: imgui::pupsy_ui_engine::PupsyUiEngine,

    is_marked_resized: bool,
}

impl PupsyEngine {
    pub fn new(
        window: window::Window) -> PupsyEngine {
        let render_device = vk_render::VkRenderDevice::new(&window);
        let pupsy_ui_engine = imgui::pupsy_ui_engine::PupsyUiEngine::new(&window);

        PupsyEngine {
             render_device: render_device,
             window: window,
             is_marked_resized: false,
             fps_manager: fps::FPSManager::new(),
             ui_engine: pupsy_ui_engine,
        }
    }

    fn draw_frame(&mut self) {
        let wait_fences = [self.render_device.sync_objects.inflight_fences[self.render_device.current_frame]];

        unsafe {
            self.render_device.device
            .wait_for_fences(&wait_fences, true, std::u64::MAX)
            .expect("Failed to wait for Fence!");
        }

        let (image_index, _is_sub_optimal) = unsafe {
            let acquire_result = self.render_device.swapchain.swapchain_loader
                .acquire_next_image(
                    self.render_device.swapchain.swapchain,
                    std::u64::MAX,
                    self.render_device.sync_objects.image_available_semaphores[self.render_device.current_frame],
                    vk::Fence::null(),
                );

                match acquire_result {
                    Ok(image_index) => {
                        image_index
                    },
                    Err(vk_result) => match vk_result {
                        vk::Result::ERROR_OUT_OF_DATE_KHR => {
                            self.render_device.recreate_swapchain();
                            return;
                        },
                        _ => panic!("Failed to acquire Swap Chain Image"),
                    },
                }
        };

        self.render_device.update_uniform_buffer(image_index as usize, self.fps_manager.delta_time as f32);

        let image_available_semaphore = [self.render_device.sync_objects.image_available_semaphores[self.render_device.current_frame]];
        let render_finished_semaphore = [self.render_device.sync_objects.render_finished_semaphores[self.render_device.current_frame]];

        let wait_stages = [vk::PipelineStageFlags::COLOR_ATTACHMENT_OUTPUT];

        let draw_ui_data = self.ui_engine.render(&self.window, &self.render_device.command_buffers[image_index as usize]);

        // vk_render::VkRenderDevice::create_command_buffers(
        //     &self.render_device.device,
        //     self.render_device.command_pool,
        //     self.render_device.graphics_pipeline,
        //     &self.render_device.swapchain.swapchain_framebuffers,
        //     self.render_device.render_pass,
        //     self.render_device.swapchain.swapchain_extent);

        let submit_infos = [vk::SubmitInfo {
            s_type: vk::StructureType::SUBMIT_INFO,
            p_next: ptr::null(),
            wait_semaphore_count: image_available_semaphore.len() as u32,
            p_wait_semaphores: image_available_semaphore.as_ptr(),
            p_wait_dst_stage_mask: wait_stages.as_ptr(),
            command_buffer_count: 1,
            p_command_buffers: &self.render_device.command_buffers[image_index as usize],
            signal_semaphore_count: render_finished_semaphore.len() as u32,
            p_signal_semaphores: render_finished_semaphore.as_ptr(),
        }];

        unsafe {
            self.render_device.device
                .reset_fences(&wait_fences)
                .expect("Failed to reset Fence!");

            self.render_device.device
                .queue_submit(
                    self.render_device.graphics_queue,
                    &submit_infos,
                    self.render_device.sync_objects.inflight_fences[self.render_device.current_frame],
                )
                .expect("Failed to execute queue submit.");
        }

        let swapchains = [self.render_device.swapchain.swapchain];

        let present_info = vk::PresentInfoKHR {
            s_type: vk::StructureType::PRESENT_INFO_KHR,
            p_next: ptr::null(),
            wait_semaphore_count: 1,
            p_wait_semaphores: render_finished_semaphore.as_ptr(),
            swapchain_count: swapchains.len() as u32,
            p_swapchains: swapchains.as_ptr(),
            p_image_indices: &image_index,
            p_results: ptr::null_mut(),
        };

        let present_result = unsafe {
            self.render_device.swapchain.swapchain_loader
                .queue_present(self.render_device.present_queue, &present_info)
        };

        let time = SystemTime::now().duration_since(UNIX_EPOCH);
        self.fps_manager.update(time.unwrap().as_micros());

        let is_resized = match present_result {
            Ok(_) => !self.is_marked_resized,
            Err(vk_result) => match vk_result {
                vk::Result::ERROR_OUT_OF_DATE_KHR | vk::Result::SUBOPTIMAL_KHR => true,
                _ => panic!("Faile to execure queue present!"),
            },
        };

        if is_resized {
            self.is_marked_resized = true;
            self.render_device.recreate_swapchain();
        }

        self.render_device.current_frame = (self.render_device.current_frame + 1) % global_constants::MAX_FRAMES_IN_FLIGHT;
    }

    pub fn main_loop(mut self, event_loop: EventLoop<()>) {

        event_loop.run(move |event, _, control_flow| {
            match event {
                | Event::NewEvents(_) => {
                    self.ui_engine.imgui.io_mut().update_delta_time(Duration::from_micros(self.fps_manager.delta_time as u64));
                }
                | Event::WindowEvent { event, .. } => {
                    match event {
                        | WindowEvent::CloseRequested => {
                            *control_flow = ControlFlow::Exit
                        },
                        | WindowEvent::KeyboardInput { input, .. } => {
                            match input {
                                | KeyboardInput { virtual_keycode, state, .. } => {
                                    match (virtual_keycode, state) {
                                        | (Some(VirtualKeyCode::Escape), ElementState::Pressed) => {
                                            dbg!();
                                            *control_flow = ControlFlow::Exit
                                        },
                                        | _ => {},
                                    }
                                },
                            }
                        },
                        | _ => {},
                    }
                },
                | Event::MainEventsCleared => {
                    self.window.request_redraw();
                },
                | Event::RedrawRequested(_window_id) => {
                    self.draw_frame();
                },
                | Event::LoopDestroyed => {
                    unsafe {
                        self.render_device.device.device_wait_idle()
                            .expect("Failed to wait device idle!")
                    };
                },
                _ => (),
            }

        })
    }
}

impl Drop for PupsyEngine {
    fn drop(&mut self) {
        self.render_device.drop();
    }
}

fn main() {
    let event_loop = EventLoop::new();
    let window = rhi::window::Window::new(&event_loop);

    let engine = PupsyEngine::new(window);

    engine.main_loop(event_loop);
}
