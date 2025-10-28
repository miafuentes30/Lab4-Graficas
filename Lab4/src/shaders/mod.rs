pub mod common;
pub mod rocky_planet;
pub mod gas_giant;
pub mod scifi_planet;
pub mod lava;
pub mod ice;
pub mod rings_vs;
pub mod moon_vs;
pub mod flat; 

use crate::renderer::pipeline::Shader;

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum ShaderKind {
    Rocky,
    Gas,
    SciFi,
    Rings,
    Moon,
    Flat, 
    Lava,
    Ice,
}

pub fn make_shader(kind: ShaderKind) -> Box<dyn Shader> {
    match kind {
        ShaderKind::Rocky => Box::new(rocky_planet::Rocky::default()),
        ShaderKind::Gas   => Box::new(gas_giant::Gas::default()),
        ShaderKind::SciFi => Box::new(scifi_planet::SciFi::default()),
        ShaderKind::Rings => Box::new(rings_vs::Rings::default()),
        ShaderKind::Moon  => Box::new(moon_vs::Moon::default()),
        ShaderKind::Flat  => Box::new(flat::Flat::default()), 
        ShaderKind::Lava  => Box::new(lava::Lava::default()),
        ShaderKind::Ice   => Box::new(ice::Ice::default()),
    }
}
