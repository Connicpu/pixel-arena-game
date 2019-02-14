use super::backend;
use super::hal::{self, memory as m, adapter as a, pool, pso, Backend, Device, Instance, PhysicalDevice, Surface};
use super::B;

use failure::Error;

const MAX_DESC_SETS: usize = 64;

pub struct GraphicsCore {
    pub events_loop: winit::EventsLoop,
    pub window: winit::Window,

    pub instance: backend::Instance,
    pub surface: <B as Backend>::Surface,
    pub adapter: hal::Adapter<B>,
    pub mem_properties: hal::MemoryProperties,
    pub mem_limits: hal::Limits,
    pub device: <B as Backend>::Device,
    pub render_pass: <B as Backend>::RenderPass,

    pub swap_chain: Option<<B as Backend>::Swapchain>,
    pub backbuffer: Option<hal::Backbuffer<B>>,

    pub queue_group: hal::QueueGroup<B, hal::Graphics>,
    pub command_pool: hal::CommandPool<B, hal::Graphics>,
    pub temp_cmd_pool: hal::CommandPool<B, hal::Graphics>,
    pub desc_pool: <B as Backend>::DescriptorPool,
}

pub fn init_graphics() -> Result<GraphicsCore, Error> {
    let (events_loop, window) = init_window()?;

    let instance = backend::Instance::create("pixel arena game", 0x01_00_00_00);
    let mut surface = instance.create_surface(&window);
    let adapter = get_adapter(&instance);
    let mem_properties = adapter.physical_device.memory_properties();
    let mem_limits = adapter.physical_device.limits();

    let (device, queue_group) = adapter.open_with(1, |f| surface.supports_queue_family(f))?;

    let command_pool = unsafe {
        device.create_command_pool_typed(&queue_group, pool::CommandPoolCreateFlags::empty())?
    };
    
    let temp_cmd_pool = unsafe {
        device.create_command_pool_typed(&queue_group, pool::CommandPoolCreateFlags::TRANSIENT)?
    };

    let desc_pool = unsafe {
        device.create_descriptor_pool(
            MAX_DESC_SETS,
            &[
                pso::DescriptorRangeDesc {
                    ty: pso::DescriptorType::CombinedImageSampler,
                    count: MAX_DESC_SETS * 2,
                },
                pso::DescriptorRangeDesc {
                    ty: pso::DescriptorType::UniformBuffer,
                    count: MAX_DESC_SETS * 2,
                },
            ],
        )?
    };

    let (caps, format) = {
        let (caps, formats, _, _) = surface.compatibility(&adapter.physical_device);
        let format = formats.map_or(hal::format::Format::Rgba8Srgb, |formats| {
            formats
                .iter()
                .find(|format| format.base_format().1 == hal::format::ChannelType::Srgb)
                .map(|format| *format)
                .unwrap_or(formats[0])
        });
        (caps, format)
    };

    let (swap_chain, backbuffer) = {
        let dpi = window.get_hidpi_factor();
        let window_size = window.get_inner_size().unwrap();
        let window_size: (u32, u32) = window_size.to_physical(dpi).into();
        let window_size = hal::window::Extent2D {
            width: window_size.0,
            height: window_size.1,
        };

        let swap_config = hal::SwapchainConfig::from_caps(&caps, format, window_size.into());
        unsafe { device.create_swapchain(&mut surface, swap_config, None)? }
    };

    let render_pass = {
        use super::hal::image::Access;
        use super::hal::pass::*;
        use super::hal::pso::PipelineStage;

        let attachment = Attachment {
            format: Some(format),
            samples: 1,
            ops: AttachmentOps::new(AttachmentLoadOp::Clear, AttachmentStoreOp::Store),
            stencil_ops: AttachmentOps::DONT_CARE,
            layouts: hal::image::Layout::Undefined..hal::image::Layout::Present,
        };

        let subpass = SubpassDesc {
            colors: &[(0, hal::image::Layout::ColorAttachmentOptimal)],
            depth_stencil: None,
            inputs: &[],
            resolves: &[],
            preserves: &[],
        };

        let dependency = SubpassDependency {
            passes: SubpassRef::External..SubpassRef::Pass(0),
            stages: PipelineStage::COLOR_ATTACHMENT_OUTPUT..PipelineStage::COLOR_ATTACHMENT_OUTPUT,
            accesses: Access::empty()
                ..(Access::COLOR_ATTACHMENT_READ | Access::COLOR_ATTACHMENT_WRITE),
        };

        unsafe { device.create_render_pass(&[attachment], &[subpass], &[dependency])? }
    };

    Ok(GraphicsCore {
        events_loop,
        window,

        instance,
        surface,
        adapter,
        mem_properties,
        mem_limits,
        device,
        render_pass,

        swap_chain: Some(swap_chain),
        backbuffer: Some(backbuffer),

        queue_group,
        command_pool,
        temp_cmd_pool,
        desc_pool,
    })
}

impl GraphicsCore {
    pub fn memory_type(&self, mask: u64, props: m::Properties) -> a::MemoryTypeId {
        self.mem_properties.memory_types
            .iter()
            .enumerate()
            .position(|(id, mem_type)| {
                mask & (1 << id) != 0 && mem_type.properties.contains(props)
            })
            .unwrap()
            .into()
    }

    pub unsafe fn destroy(self) {
        use hal::Device;

        let device = self.device;

        device.wait_idle().unwrap();

        // Destroy graphics resources
        device.destroy_command_pool(self.command_pool.into_raw());
        device.destroy_command_pool(self.temp_cmd_pool.into_raw());
        device.destroy_descriptor_pool(self.desc_pool);

        device.destroy_render_pass(self.render_pass);

        self.swap_chain.map(|sc| device.destroy_swapchain(sc));
    }
}

fn init_window() -> Result<(winit::EventsLoop, winit::Window), Error> {
    let events_loop = winit::EventsLoop::new();

    let wb = winit::WindowBuilder::new()
        .with_dimensions((1280.0, 720.0).into())
        .with_min_dimensions((640.0, 480.0).into())
        .with_title("Pixel Arena Game (working title)");
    // TODO: Icon

    let window = wb.build(&events_loop)?;

    Ok((events_loop, window))
}

fn get_adapter(instance: &backend::Instance) -> hal::Adapter<B> {
    // TODO: When PR gets merged, uncomment this :3
    /*#[cfg(feature = "dx12")]
    let mut adapters = instance
        .enumerate_adapters_by_gpu_preference(backend::GpuPreference::HighPerformance)
        .unwrap_or_else(|| instance.enumerate_adapters());
    #[cfg(not(feature = "dx12"))]*/
    let mut adapters = instance.enumerate_adapters();

    for adapter in &adapters {
        println!("{:?}", adapter.info);
    }

    adapters.swap_remove(0)
}
