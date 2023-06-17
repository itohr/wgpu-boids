struct Particle {
    position: vec2<f32>,
    velocity: vec2<f32>,
}

struct SimParams {
    delta_time: f32,
    rule1_distance: f32,
    rule2_distance: f32,
    rule3_distance: f32,
    rule1_scale: f32,
    rule2_scale: f32,
    rule3_scale: f32,
}

struct Particles {
    particles: array<Particle>,
}

@binding(0) @group(0) var<uniform> params: SimParams;
@binding(1) @group(0) var<storage, read> particles_a: Particles;
@binding(2) @group(0) var<storage, read_write> particles_b: Particles;

@compute @workgroup_size(64)
fn main(@builtin(global_invocation_id) global_id: vec3<u32>) {
    var index = global_id.x;

    var v_pos = particles_a.particles[index].position;
    var v_vel = particles_a.particles[index].velocity;
    var c_mass = vec2(0.0);
    var c_vel = vec2(0.0);
    var col_vel = vec2(0.0);
    var c_mass_count = 0u;
    var c_vel_count = 0u;
    var pos: vec2<f32>;
    var vel: vec2<f32>;

    for (var i = 0u; i < arrayLength(&particles_a.particles); i++) {
        if i == index {
          continue;
        }

        pos = particles_a.particles[i].position.xy;
        vel = particles_a.particles[i].velocity.xy;
        if distance(pos, v_pos) < params.rule1_distance {
            c_mass += pos;
            c_mass_count++;
        }
        if distance(pos, v_pos) < params.rule2_distance {
            col_vel -= (pos - v_pos);
        }
        if distance(pos, v_pos) < params.rule3_distance {
            c_vel += vel;
            c_vel_count++;
        }
    }

    if c_mass_count > 0u {
        c_mass = (c_mass / vec2(f32(c_mass_count))) - v_pos;
    }
    if c_vel_count > 0u {
        c_vel /= f32(c_vel_count);
    }

    v_vel += (c_mass * params.rule1_scale) + (col_vel * params.rule2_scale) + (c_vel * params.rule3_scale);
    v_vel = normalize(v_vel) * clamp(length(v_vel), 0.0, 0.1);
    v_pos = v_pos + (v_vel * params.delta_time);

    if v_pos.x < -1.0 {
        v_pos.x = 1.0;
    }
    if v_pos.x > 1.0 {
        v_pos.x = -1.0;
    }
    if v_pos.y < -1.0 {
        v_pos.y = 1.0;
    }
    if v_pos.y > 1.0 {
        v_pos.y = -1.0;
    }

    particles_b.particles[index].position = v_pos;
    particles_b.particles[index].velocity = v_vel;
}
