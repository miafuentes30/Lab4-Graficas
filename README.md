# Lab4-Graficas: Planetas Procedurales

## Descripción

Este proyecto implementa pipeline de rasterización en CPU y varios
shaders procedurales para renderizar planetas a partir de una única malla (esfera).

### Características principales:
- Gas Giant con 4 bandas atmosféricas y anillo
- Planeta rocoso tipo Marte con cráteres y polvo y luna
- Planeta SciFi tipo planeta de agua
- Planetas extra: Lava (volcánico) y Ice (helado)
- Luna orbital procedimental (del planeta rocoso)
- Anillos procedurales con bandas y transparencia (del planeta gaseoso)
- Rotación y traslación orbital de planetas

## Instalación y Ejecución

### Requisitos previos
- [Rust](https://www.rust-lang.org/tools/install)
- Cargo (incluido con Rust)
- Un sistema con soporte para ventanas (Windows, Linux, macOS)

### Pasos para ejecutar
1. Clonar el repositorio:
```bash
git clone https://github.com/miafuentes30/Lab4-Graficas.git
cd Lab4-Graficas/Lab4
```

2. Compilar el proyecto:
```bash
cargo build
```

3. Ejecutar:
```bash
cargo run
```

### Resolución de problemas
- Si hay errores de compilación, asegúrate de tener Rust actualizado (`rustup update`)
- En Windows, si la ventana no aparece, verifica que estés usando un terminal con permisos suficientes
- Los screenshots se guardan en la carpeta `screenshots/` relativa al directorio de ejecución

## Controles y Uso

### Teclas para cambiar de planeta
- `1`: Mostrar planeta Rocky 
- `2`: Mostrar Gas Giant 
- `3`: Mostrar planeta SciFi 
- `4`: Mostrar planeta Lava 
- `5`: Mostrar planeta Ice 
- `0`: Alternar modo "mostrar todos" 

### Elementos orbitales
- `R`: Activar/desactivar anillos (solo afecta al Gas Giant)
- `M`: Activar/desactivar luna orbital
- Los planetas giran sobre su eje y orbitan automáticamente

### Otras funciones
- `P`: Guardar screenshot en la carpeta `screenshots/`
- `Esc`: Cerrar el programa

### Notas importantes
- El modo "mostrar todos" (`0`) se activa solo al presionar la tecla para evitar toggles accidentales
- Los anillos solo aparecen alrededor del Gas Giant 
- La luna orbita alrededor del planeta actualmente seleccionado
- Cada planeta tiene su propia velocidad de rotación y órbita

## Detalles técnicos

### Parámetros uniformes
Los shaders utilizan parámetros compartidos definidos en `src/renderer/uniforms.rs`:

#### Struct `Uniforms`
- `time: f32`: Tiempo en segundos, usado para:
  - Rotación de planetas sobre su eje
  - Movimiento orbital
  - Animación de texturas y efectos
- `light_dir: Vec3`: Dirección de la luz (normalizada)
- `view`, `proj`, `model: Mat4`: Matrices de transformación
- `camera_pos: Vec3`: Posición de la cámara en espacio mundial
- `planet: PlanetParams`: Parámetros específicos del planeta

#### Struct `PlanetParams`
- `base_color: Color`: Color base del planeta
- `band_freq: f32`: Frecuencia de bandas atmosféricas
- `noise_scale: f32`: Escala del ruido fractal (FBM)
- `rim_power: f32`: Intensidad del efecto rim-light
- `rotation_speed: f32`: Velocidad de rotación
- `has_rings: bool`: Si el planeta tiene anillos
- `has_moon: bool`: Si el planeta tiene luna


## Shaders implementados

### Planetas principales
1. **Rocky** (`src/shaders/rocky_planet.rs`)
   - Apariencia tipo Marte
   - Continentes generados con FBM
   - Cráteres procedurales
   - Polvo y atmósfera tenue
   - Rotación propia y orbital

2. **Gas Giant** (`src/shaders/gas_giant.rs`)
   - 4 bandas atmosféricas distintas
   - Turbulencia y vórtices
   - Spot rotatorio tipo Júpiter
   - Anillos tipo Saturno (opcional)

3. **SciFi** (`src/shaders/scifi_planet.rs`)
   - Degradado de 4 capas fluorescentes
   - Colores neón personalizables 
   - Halos celestes emisivos (aspecto aguoso)
   - Bandas latitudinales animadas

### Planetas adicionales
4. **Lava** (`src/shaders/lava.rs`)
   - Superficie volcánica activa
   - Vetas de lava brillante
   - Puntos calientes emisivos
   - Efecto de flujo de magma

5. **Ice** (`src/shaders/ice.rs`)
   - Superficie helada con grietas
   - Capa de escarcha procedimental
   - Reflejos cristalinos
   - Variación de albedo por latitud

### Shaders de efectos
- **Rings** (`src/shaders/rings_vs.rs`): 
  - Transforma la esfera en disco de anillos
  - Bandas concéntricas con transparencia
  - Variación de densidad y color
  - Solo visible en Gas Giant

- **Moon** (`src/shaders/moon_vs.rs`):
  - Órbita elíptica animada
  - Escala y traslación dinámica
  - Sigue al planeta seleccionado

- **Flat** (`src/shaders/flat.rs`):
  - Shader básico para debug
  - Color sólido sin efectos
