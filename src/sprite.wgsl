struct VertexInput {
    @location(0) p_position: vec2<f32>,
    @location(1) p_velocity: vec2<f32>,
    @location(2) position: vec2<f32>,
}

struct VertexOutput {
    @builtin(position) position: vec4<f32>,
    @location(4) color: vec4<f32>,
}

@vertex
fn vert_main(
    input: VertexInput,
) -> VertexOutput {
    let angle = -atan2(input.p_velocity.x, input.p_velocity.y);
    let pos = vec2(
        (input.position.x * cos(angle)) - (input.position.y * sin(angle)),
        (input.position.x * sin(angle)) + (input.position.y * cos(angle)),
    )

    var output: VertexOutput;
    output.position = vec4(pos + input.p_position, 0.0, 1.0);
    output.color = vec4(
        1.0 - sin(angle + 1.0) - input.p_velocity.y,
        pos.x * 100.0 - input.p_velocity.y + 0.1,
        input.p_velocity.x + cos(angle + 0.5),
        1.0,
    );

    return output;
}

@fragment
fn frag_main(@location(4) color: vec4<f32>) -> @location(0) vec4<f32> {
    return color;
}
