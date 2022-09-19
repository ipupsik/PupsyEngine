use ash::vk;
use memoffset::offset_of;

#[repr(C)]
#[derive(Debug, Clone)]
pub struct Vertex {
    pub pos: [f32; 2],
    pub color: [f32; 3],
}

pub trait BindingDescriptions {
    fn get_binding_descriptions() -> Vec<vk::VertexInputBindingDescription>;
}

pub trait AttributeDescriptions {
    fn get_attribute_descriptions() -> Vec<vk::VertexInputAttributeDescription>;
}

impl BindingDescriptions for Vertex {
    fn get_binding_descriptions() -> Vec<vk::VertexInputBindingDescription> {
        let mut result = vec![];

        result.push(
            vk::VertexInputBindingDescription {
            binding: 0,
            stride: std::mem::size_of::<Self>() as u32,
            input_rate: vk::VertexInputRate::VERTEX,
        });

        
        result
    }
}

impl AttributeDescriptions for Vertex {
    fn get_attribute_descriptions() -> Vec<vk::VertexInputAttributeDescription> {
        let mut result = vec![];

        result.push(
            vk::VertexInputAttributeDescription {
            binding: 0,
            location: 0,
            format: vk::Format::R32G32_SFLOAT,
            offset: offset_of!(Self, pos) as u32,
        });
        
        result.push(vk::VertexInputAttributeDescription {
            binding: 0,
            location: 1,
            format: vk::Format::R32G32B32_SFLOAT,
            offset: offset_of!(Self, color) as u32,
        });

        result
    }
}