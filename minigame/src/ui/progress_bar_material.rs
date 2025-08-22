use bevy::prelude::*;
use bevy::render::mesh::*;
use bevy::render::render_resource::{
    AsBindGroup, RenderPipelineDescriptor, ShaderRef, SpecializedMeshPipelineError,
};
use bevy::sprite::{AlphaMode2d, Material2d, Material2dKey};

// pub const DEFAULT_BACKGROUND_COLOR: Color = Color::srgba(0., 0., 0., 0.75);
// pub const DEFAULT_BORDER_COLOR: Color = Color::srgba(0.02, 0.02, 0.02, 0.95);
// pub const DEFAULT_HIGH_COLOR: Color = Color::srgba(0., 1., 0., 0.95);
// pub const DEFAULT_MODERATE_COLOR: Color = Color::srgba(1., 1., 0., 0.95);
// pub const DEFAULT_LOW_COLOR: Color = Color::srgba(1., 0., 0., 0.95);

#[derive(Asset, AsBindGroup, Debug, Clone, Reflect)]
#[bind_group_data(PBarMaterialKey)]
pub struct ProgressBarMaterial {
    #[uniform(0)]
    pub value_and_dimensions: Vec4,
    // (value, width, height, border_width) vec4 to be 16byte aligned
    #[uniform(1)]
    pub background_color: LinearRgba,
    #[uniform(2)]
    pub high_color: LinearRgba,
    #[uniform(3)]
    pub moderate_color: LinearRgba,
    #[uniform(4)]
    pub low_color: LinearRgba,
    #[uniform(5)]
    pub offset: Vec4,
    #[uniform(6)]
    pub border_color: LinearRgba,
    pub vertical: bool,
}

#[derive(Eq, PartialEq, Hash, Clone)]
pub struct PBarMaterialKey {
    vertical: bool,
    border: bool,
}

impl From<&ProgressBarMaterial> for PBarMaterialKey {
    fn from(material: &ProgressBarMaterial) -> Self {
        Self {
            vertical: material.vertical,
            border: material.value_and_dimensions.w > 0.,
        }
    }
}

impl Material2d for ProgressBarMaterial {
    fn vertex_shader() -> ShaderRef {
        "shaders/progress_bar2d.wgsl".into()
    }

    fn fragment_shader() -> ShaderRef {
        "shaders/progress_bar2d.wgsl".into()
    }

    fn alpha_mode(&self) -> AlphaMode2d {
        AlphaMode2d::Blend
    }

    fn specialize(
        descriptor: &mut RenderPipelineDescriptor,
        layout: &MeshVertexBufferLayoutRef,
        key: Material2dKey<Self>,
    ) -> Result<(), SpecializedMeshPipelineError> {
        let vertex_layout = layout.0.get_layout(&[
            Mesh::ATTRIBUTE_POSITION.at_shader_location(0),
            Mesh::ATTRIBUTE_UV_0.at_shader_location(1),
        ])?;

        let fragment = descriptor.fragment.as_mut().unwrap();
        if key.bind_group_data.vertical {
            fragment.shader_defs.push("IS_VERTICAL".into());
        }

        if key.bind_group_data.border {
            fragment.shader_defs.push("HAS_BORDER".into());
        }

        descriptor.vertex.buffers = vec![vertex_layout];

        Ok(())
    }
}
