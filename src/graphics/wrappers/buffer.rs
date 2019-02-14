use crate::graphics::hal::{buffer as b, command as c, memory as m, Backend, Device};
use crate::graphics::wrappers::texture::RawTexture;
use crate::graphics::{core::GraphicsCore, B};

use std::ops::Range;

pub struct Buffer {
    memory: <B as Backend>::Memory,
    buffer: <B as Backend>::Buffer,
    size: u64,
}

impl Buffer {
    pub fn buffer(&self) -> &<B as Backend>::Buffer {
        &self.buffer
    }

    pub fn size(&self) -> u64 {
        self.size
    }

    pub unsafe fn new_dynamic<T: Copy>(
        core: &GraphicsCore,
        data: &[T],
        usage: b::Usage,
    ) -> Result<Self, failure::Error> {
        let stride = std::mem::size_of::<T>() as u64;
        let buffer_size = data.len() as u64 * stride;

        let mut buf = Self::new_uninit(core, usage, buffer_size, m::Properties::CPU_VISIBLE)?;
        buf.update(core, 0, data)?;
        Ok(buf)
    }

    pub unsafe fn new_static<T: Copy>(
        core: &mut GraphicsCore,
        data: &[T],
        usage: b::Usage,
    ) -> Result<Self, failure::Error> {
        let stride = std::mem::size_of::<T>() as u64;
        let buffer_size = data.len() as u64 * stride;

        let mut buf = Self::new_uninit(
            core,
            usage | b::Usage::TRANSFER_DST,
            buffer_size,
            m::Properties::DEVICE_LOCAL,
        )?;

        let temp = Self::new_dynamic(core, data, b::Usage::TRANSFER_SRC)?;
        buf.copy_from(core, &temp, 0..buffer_size, 0, None)?;
        temp.destroy(core);

        Ok(buf)
    }

    pub unsafe fn new_texture_stage(
        core: &GraphicsCore,
        data: &RawTexture,
    ) -> Result<Self, failure::Error> {
        let row_pitch = data.device_row_pitch(core);
        let buffer_size = (data.height * row_pitch) as u64;

        let buf = Self::new_uninit(
            core,
            b::Usage::TRANSFER_SRC,
            buffer_size,
            m::Properties::CPU_VISIBLE,
        )?;

        Ok(buf)
    }

    pub fn update<T: Copy>(
        &mut self,
        core: &GraphicsCore,
        offset: u64,
        data: &[T],
    ) -> Result<(), failure::Error> {
        let stride = std::mem::size_of::<T>() as u64;
        let upload_size = data.len() as u64 * stride;

        assert!(offset + upload_size <= self.size);

        unsafe {
            let mut data_target = core
                .device
                .acquire_mapping_writer::<T>(&self.memory, offset..offset + upload_size)?;
            data_target.copy_from_slice(data);
            core.device.release_mapping_writer(data_target)?;
        }

        Ok(())
    }

    pub fn update_texture(
        &mut self,
        core: &GraphicsCore,
        data: &RawTexture,
        layer: usize,
    ) -> Result<(), failure::Error> {
        let row_pitch = data.device_row_pitch(core);
        unsafe {
            let mut target = core
                .device
                .acquire_mapping_writer::<u8>(&self.memory, 0..self.size)?;
            for y in 0..data.height as usize {
                let src_slice = data.row(layer, y);

                let dst = y * row_pitch as usize;
                target[dst..dst + src_slice.len()].copy_from_slice(src_slice);
            }
            core.device.release_mapping_writer(target)?;
        }
        Ok(())
    }

    pub fn copy_from(
        &mut self,
        core: &mut GraphicsCore,
        src_buf: &Buffer,
        src_range: Range<u64>,
        dst_offset: u64,
        fence: Option<&<B as Backend>::Fence>,
    ) -> Result<(), failure::Error> {
        let src = src_range.start;
        let dst = dst_offset;
        let size = src_range.end - src_range.start;

        assert!(src + size <= src_buf.size);
        assert!(dst + size <= self.size);

        let op = c::BufferCopy { src, dst, size };
        unsafe {
            let mut cmd = core.temp_cmd_pool.acquire_command_buffer::<c::OneShot>();
            cmd.begin();
            cmd.copy_buffer(&src_buf.buffer, &self.buffer, &[op]);
            cmd.finish();

            if fence.is_some() {
                core.queue_group.queues[0].submit_nosemaphores(Some(&cmd), fence)
            } else {
                core.queue_group.queues[0].submit_nosemaphores(Some(&cmd), None);
                core.queue_group.queues[0].wait_idle()?;
            }

            core.temp_cmd_pool.free(Some(cmd));
        }

        Ok(())
    }

    pub unsafe fn new_uninit(
        core: &GraphicsCore,
        usage: b::Usage,
        buffer_size: u64,
        mem_prop: m::Properties,
    ) -> Result<Self, failure::Error> {
        let mut buffer = core.device.create_buffer(buffer_size, usage)?;
        let mem_req = core.device.get_buffer_requirements(&buffer);
        let mem_type = core.memory_type(mem_req.type_mask, mem_prop);
        let memory = core.device.allocate_memory(mem_type, mem_req.size)?;
        core.device.bind_buffer_memory(&memory, 0, &mut buffer)?;
        let size = mem_req.size;

        Ok(Buffer {
            memory,
            buffer,
            size,
        })
    }

    pub unsafe fn destroy(self, core: &GraphicsCore) {
        core.device.destroy_buffer(self.buffer);
        core.device.free_memory(self.memory);
    }
}
