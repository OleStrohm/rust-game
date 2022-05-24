struct VertexOutput {
    [[builtin(position)]] clip_position: vec4<f32>;
    [[location(0)]] world_position: vec4<f32>;
    [[location(1)]] world_normal: vec3<f32>;
    [[location(2)]] uv: vec2<f32>;
};

struct Circle {
    color: vec4<f32>;
    center: vec3<f32>;
    radius: f32;
};
struct Circles {
    circles: array<Circle>;
};
[[group(1), binding(0)]]
var<storage> circles: Circles;

struct Line {
    color: vec4<f32>;
    start: vec2<f32>;
    end: vec2<f32>;
};
struct Lines {
    lines: array<Line>;
};
[[group(1), binding(1)]]
var<storage> lines: Lines;

fn circle(st: vec2<f32>, center: vec2<f32>, radius: f32) -> f32 {
    let dist = st-center;
    return smoothStep(radius*radius-(radius*0.01),
                      radius*radius+(radius*0.01),
                      dot(dist, dist));
}

fn circlearc(st: vec2<f32>, center: vec2<f32>, radius: f32) -> f32 {
    let dist = st-center;
    let len = dot(dist, dist);
    let size = 0.01;
    let buffer = 0.01;

    let outer = smoothStep(
                      (radius + size + buffer)*(radius + size + buffer),
                      (radius + size - buffer)*(radius + size - buffer),
                      len);
    let inner = smoothStep(
                      (radius - size - buffer)*(radius - size - buffer),
                      (radius - size + buffer)*(radius - size + buffer),
                      len);
    return outer * inner;
}

fn line(st: vec2<f32>, p1: vec2<f32>, p2: vec2<f32>) -> f32 {
    let buffer = 0.02;

    let r = normalize(p2 - p1);
    let n = vec2<f32>(-r.y, r.x);
    let d = st - p1;
    let dist = dot(n, st - p1);
    let line = smoothStep(buffer, 0.0, abs(dist));
    let inside_p1 = smoothStep(-buffer, buffer, dot(r, st - p1));
    let inside_p2 = smoothStep(-buffer, buffer, dot(-r, st - p2));
    return line * inside_p1 * inside_p2;
}

fn mix_with_alpha(a: vec4<f32>, b: vec4<f32>) -> vec4<f32> {
    var alpha = a.a + b.a + a.a * b.a;
    var color = (a.rgb * a.a * (1.0 - b.a) + b.rgb * b.a) / (alpha);
    if (a.a == 0.0) {
        color = b.rgb;
    }
    return vec4<f32>(color, alpha);
}

[[stage(fragment)]]
fn fragment(in: VertexOutput) -> [[location(0)]] vec4<f32> {
    var res = vec4<f32>(0.0, 0.0, 0.0, 0.0);

    let num_circles: u32 = arrayLength(&circles.circles);
    for (var i: u32 = 0u; i < num_circles; i = i + 1u) {
        let c = circles.circles[i];
        let alpha = circlearc(in.world_position.xy, c.center.xy, c.radius);
        res = mix_with_alpha(res, vec4<f32>(c.color.rgb, alpha));
    }

    let num_lines: u32 = arrayLength(&lines.lines);
    for (var i: u32 = 0u; i < num_lines; i = i + 1u) {
        let l = lines.lines[i];
        let alpha = line(in.world_position.xy, l.start, l.end);
        res = mix_with_alpha(res, vec4<f32>(l.color.rgb, alpha));
    }

    return res;
}
