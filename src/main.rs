fn main() {
    pollster::block_on(wgpu_boids::run());
}
