#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Model {
    pub p_position: [f32; 2],
    pub p_velocity: [f32; 2],
}

impl Model {
    pub fn buffer_layout<'a>() -> [wgpu::VertexBufferLayout<'a>; 2] {
        [
            // for instanced particles
            wgpu::VertexBufferLayout {
                array_stride: std::mem::size_of::<Model>() as wgpu::BufferAddress,
                step_mode: wgpu::VertexStepMode::Instance,
                attributes: &wgpu::vertex_attr_array![0 => Float32x2, 1 => Float32x2],
            },
            // for vertex positoins
            wgpu::VertexBufferLayout {
                array_stride: std::mem::size_of::<[f32; 2]>() as wgpu::BufferAddress,
                step_mode: wgpu::VertexStepMode::Vertex,
                attributes: &wgpu::vertex_attr_array![2 => Float32x2],
            },
        ]
    }
}
