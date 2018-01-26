use gfx_gl;

use gfx;
use gfx::traits::FactoryExt;
use gfx::Factory;
use gfx::Primitive;
use gfx::state::Rasterizer;
use gfx::texture::{SamplerInfo,FilterMethod,WrapMode};

use render::Error;
use file_systems::file_system::ReadFile;

use failure::Error;

use super::consts::WorldGlobals;
use super::vertices::P3N3TcVertex;
use super::PipelineError;

pub type ColorFormat = gfx::format::Rgba8;
pub type DepthFormat = gfx::format::DepthStencil;

pub struct SimpleObjectPSO {
    pub pso:gfx::PipelineState<gfx_gl::Resources, ObjectPipeline::Meta>,
    pub sampler:gfx::handle::Sampler<gfx_gl::Resources>,
}


gfx_defines!{
    pipeline SimpleObjectPipeline {
        globals: gfx::ConstantBuffer<WorldGlobals> = "c_world_globals",
        model_matrix: gfx::Global<[[f32; 4]; 4]> = "u_model_matrix",
        texture: gfx::TextureSampler<[f32; 4]> = "t_texture",
        vbuf: gfx::VertexBuffer<ObjectVertex> = (),

        color_target: gfx::BlendTarget<ColorFormat> = ("Target0", gfx::state::MASK_ALL, gfx::preset::blend::ALPHA),
        depth_target: gfx::DepthTarget<DepthFormat> = gfx::preset::depth::LESS_EQUAL_WRITE,
    }
}

impl SimpleObjectPipeline {
    pub fn create(vertex_source:String, fragment_source:String, gfx_factory: &mut gfx_gl::Factory) -> Result<ObjectPSO,Error> {
        let rasterizer = Rasterizer::new_fill();
        let primitive = Primitive::TriangleList;

        let shader=match gfx_factory.link_program(SimpleObjectPipeline.as_str(), fragment_source.as_str()) {
            Ok(shader) => shader,
            Err(e) => bail!(PipelineError::LinkError(e)),
        };

        let pso=match gfx_factory.create_pipeline_from_program( &shader, primitive, rasterizer, ObjectPipeline::new() ) {
            Ok(pso) => pso,
            Err(error) => bail!(PipelineError::CreatePSOError( format!("{}",error) )),
        };

        let sampler_info=SamplerInfo::new(FilterMethod::Bilinear, WrapMode::Tile);
        let sampler = gfx_factory.create_sampler(sampler_info);

        let object_pso=ObjectPSO{
            pso,
            sampler
        };

        ok!(object_pso)
    }
}