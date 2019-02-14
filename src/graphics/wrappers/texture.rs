use crate::graphics::hal::{buffer as b, format as f, image as i, memory as m, Backend, Device};
use crate::graphics::wrappers::buffer::Buffer;
use crate::graphics::{core::GraphicsCore, B};

pub struct TextureData {
    sampler: <B as Backend>::Sampler,
    image_view: <B as Backend>::ImageView,
    image: <B as Backend>::Image,
    memory: <B as Backend>::Memory,

    transfered_image_fence: <B as Backend>::Fence,
}

impl TextureData {
    pub unsafe fn new(
        core: &mut GraphicsCore,
        data: &RawTexture,
    ) -> Result<TextureData, failure::Error> {
        let staging_buf = Buffer::new_texture_stage(core, data)?;

        let mut image = {
            let layers = data.layers();
            let kind = i::Kind::D2(data.width, data.height, layers as u16, 1);
            core.device.create_image(
                kind,
                1,
                data.format,
                i::Tiling::Optimal,
                i::Usage::TRANSFER_DST | i::Usage::SAMPLED,
                i::ViewCapabilities::KIND_2D_ARRAY,
            )?
        };

        let memory = {
            let mem_req = core.device.get_image_requirements(&image);
            let mem_type = core.memory_type(mem_req.type_mask, m::Properties::DEVICE_LOCAL);

            let memory = core.device.allocate_memory(mem_type, mem_req.size)?;
            core.device.bind_image_memory(&memory, 0, &mut image)?;

            memory
        };

        let image_view = {
            let range = i::SubresourceRange {
                aspects: f::Aspects::COLOR,
                levels: 0..1,
                layers: 0..data.layers(),
            };

            core.device.create_image_view(
                &image,
                i::ViewKind::D2Array,
                data.format,
                f::Swizzle::NO,
                range,
            )?
        };

        let sampler = {
            let filter = if data.is_pixel_art {
                i::Filter::Nearest
            } else {
                i::Filter::Linear
            };

            let mut info = i::SamplerInfo::new(filter, i::WrapMode::Border);
            // If we read off the edge of the texture, just read transparency
            info.border = [0.0, 0.0, 0.0, 0.0].into();

            core.device.create_sampler(info)?
        };

        let mut transfered_image_fence = core.device.create_fence(false)?;

        Ok(TextureData {
            sampler,
            image_view,
            image,
            memory,
            transfered_image_fence,
        })
    }

    pub unsafe fn destroy(self, core: &GraphicsCore) {
        use gfx_hal::Device;

        core.device
            .wait_for_fence(&self.transfered_image_fence, !0)
            .unwrap();
        core.device.destroy_fence(self.transfered_image_fence);

        core.device.destroy_sampler(self.sampler);
        core.device.destroy_image_view(self.image_view);
        core.device.destroy_image(self.image);
        core.device.free_memory(self.memory);
    }
}

pub struct RawTexture<'a> {
    pub data: &'a [u8],
    pub width: u32,
    pub height: u32,
    pub pixel_stride: u32,
    pub format: f::Format,
    pub pitch: u32,
    pub slice_pitch: u32,
    pub is_pixel_art: bool,
}

impl RawTexture<'_> {
    pub fn row_len(&self) -> usize {
        self.width as usize * self.pixel_stride as usize
    }

    pub fn device_row_pitch(&self, core: &GraphicsCore) -> u32 {
        let row_alignment_mask = core.mem_limits.min_buffer_copy_pitch_alignment as u32 - 1;
        (self.width * self.pixel_stride + row_alignment_mask) & !row_alignment_mask
    }

    pub fn row(&self, layer: usize, y: usize) -> &[u8] {
        let start = layer * self.slice_pitch as usize + y * self.pitch as usize;
        let end = start + self.row_len();
        &self.data[start..end]
    }

    pub fn layers(&self) -> u16 {
        match self.slice_pitch as usize {
            0 => 1,
            i => (self.data.len() / i) as u16,
        }
    }
}
