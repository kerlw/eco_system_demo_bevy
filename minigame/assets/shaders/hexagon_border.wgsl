// assets/shaders/hexagon_border.wgsl
// 导入 Bevy 的 2D 着色器函数
#import bevy_sprite::mesh2d_vertex_output::VertexOutput

struct HexagonBorderMaterial {
    color: vec4<f32>,
    border_color: vec4<f32>,
    border_width: f32,
};

@group(2) @binding(0)
var<uniform> material: HexagonBorderMaterial;

// 尖顶六边形距离函数
fn distance_to_edges(p: vec2<f32>) -> f32 {
    // 尖顶六边形常数
    let SQRT3 = 1.7320508; // √3
    let k = vec2<f32>(0.5, SQRT3 / 2.0); // 1/2, √3/2
    
    // // 坐标变换：将六边形的高度归一化为1（原算法高度为√3 / 2）
    // let p_scaled = vec2<f32>(p.x * SQRT3 / 2.0, p.y);
    
    // 取绝对值并对称处理
    // let p_abs = abs(p_scaled);
    let p_abs = abs(p);
    
    // 计算到六边形边的距离
    let d1 = p_abs.x; // 左右边距离
    let d2 = dot(p_abs, k); // 斜线边距离
    
    return max(d1, d2) - SQRT3 / 2.0;
}

@fragment
fn fragment(
    input: VertexOutput,
) -> @location(0) vec4<f32> {
    // if (input.uv.x < 0.1) {
    //     return vec4<f32>(0.0, 1.0, 0.0, 0.0);
    // } else if (input.uv.x <= 0.9) {
    //     return vec4<f32>(0.0, 0.0, 1.0, 0.0);
    // } else {
    //     return vec4<f32>(1.0, 0.0, 0.0, input.uv.y);
    // }
    
    // 将UV从[0,1]映射到[-1,1]
    let uv = (input.uv - 0.5) * 2.0;
    
    // 计算到六边形边的距离
    let dist = distance_to_edges(uv);

    // 计算边框区域
    let border_start = -material.border_width;
    
    // // 判断颜色
    // if (dist > border_start && dist <= 0.0) {
    //     return material.border_color;
    // } else if (dist <= border_start) {
    //     return material.color;
    // }
    
    // // 透明背景
    // return vec4(0.0, 0.0, 0.0, 0.0);

     // 平滑边缘抗锯齿
    let antialias = fwidth(dist);
    
    // 计算内部填充
    let inside = smoothstep(0.0, -antialias, dist);
    
    // 防止抗锯齿范围超出内部区域
    // let safe_antialias = min(antialias, -border_start);
    let border = smoothstep(border_start - antialias, antialias, dist) * inside;
    
    // 组合颜色
    let fill_color = mix(vec4<f32>(0.0), material.color, inside);
    let final_color = mix(fill_color, material.border_color, border);
    
    return final_color;
}