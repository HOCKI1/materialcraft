# SBS root graphs

Inventory of the SBS processing graphs in MaterialCraft that do not instantiate another
SBS processing graph. Reusable function calls are pure math and **do not count as dependencies**
for this inventory.

## Scope and method

- Scanned all **549 `.sbs` packages** under [`assets/sbs`](../assets/sbs), including nested package folders.
- Examined **1,025 processing graphs** (`<graph>` elements). The **954 reusable function definitions** (`<function>` elements) are outside this processing-node inventory.
- A root has **zero `<compInstance>` elements** in its graph body. Instances of graphs in the same package and in other packages both disqualify a graph, including disconnected instances.
- Native primitives, embedded Pixel Processor functions, FX-Maps, reusable math function calls, exposed parameters, and resource references do not disqualify a graph.
- Package-level dependency declarations alone do not disqualify a graph: another graph or a math function in that package may need them.
- Included hidden helpers, nested graphs, templates, and work-in-progress graphs. Visibility and category do not affect the criterion.
- Input pins are reported for context: image inputs (type `1` or `2`) and value inputs with `isConnectable=1`. Ordinary exposed parameters are not input pins.
- Paths retain package folders and exact graph identifiers; each link opens the containing SBS package. Paths are relative to `assets/sbs/`.

This is a structural inventory for MaterialCraft's clean-room implementation. It does not
assert that graphs cook successfully or are independent of bitmap resources.

## Results

| Measure | Count |
|---|---:|
| Root processing graphs | 546 |
| Packages containing root graphs | 320 |
| Roots with no input pins | 176 |
| Roots with one or more input pins | 370 |
| Processing graphs excluded because they instantiate another graph | 479 |

Both `white_noise.sbs/white_noise` and `white_noise.sbs/white_noise_fast` qualify.
A graph may accept input pins and still be a root under the dependency criterion.

## Parity tracking

Statuses: **Not checked**, **In progress**, **Parity** (exact pixels or typed values for the linked measured cases), or **Too hard** (reason recorded). Function calls do not count as regular graph dependencies.

The first pass covered all **546 roots**, with **166 Parity** and **380 Too hard**. The current inventory has **376 Parity**, **170 In progress** and **0 Too hard**. The November 17–20 comparisons include raw counts, output formats and remaining differences. [November 17 measurements](parity-fixes.md#2026-11-17-root-comparisons) · [November 18 measurements](parity-fixes.md#2026-11-18-root-comparisons) · [November 19 measurements](parity-fixes.md#2026-11-19-root-comparisons) · [November 20 measurements](parity-fixes.md#2026-11-20-root-comparisons)

[Pattern tiling axes and copy bounds across 215 renders](parity-fixes.md#tiling-axes-and-pattern-copy-bounds) · [Grayscale sampler measurements](parity-fixes.md#grayscale-sampling-of-color-inputs) · [Value sampling across 240 typed comparisons](parity-fixes.md#value-processor-texture-filtering) · [Five saved graphs reopened in the reference](parity-fixes.md#saved-filter-graphs-and-scalar-levels-controls)

Pattern center culling matched all 356,352 pixels across 204 renders. Background-only resizing matched all 150,528 pixels across 84 renders. The mixed-size square controls now match all 188,160 pixels, and the 288 independent depth controls match all 73,728 pixels. [Opacity and accumulator measurements](parity-fixes.md#pattern-opacity-near-zero-and-color-accumulation) [Pattern backgrounds, saved graphs and affected roots](parity-fixes.md#pattern-backgrounds-and-center-culling)

Filtered F32 pixel reads now match 41,664/41,664 pixels across 150 renders. The nearest and directly wired controls preserve subnormal inputs. [Sampler measurements and capture timings](parity-fixes.md#subnormal-values-in-filtered-pixel-reads)

Enlarged RGBA pattern backgrounds now match 71,680/71,680 pixels across 280 independent renders. The earlier source-storage tests still differ at 121 grayscale zero signs. [Background sign measurements](parity-fixes.md#signs-of-zero-in-enlarged-color-backgrounds)

The 32²-to-16² signed threshold controls match 2,048/2,048 pixels. [Floating texture filter measurements](parity-fixes.md#signed-zero-in-floating-point-texture-filtering)

Connected F32 backgrounds now match all 2,048 same-size pixels. Across 132 further renders with changed input sizes, the maximum absolute error is zero. Some RGBA zeros still differ in sign. [Connected background counts and file checks](parity-fixes.md#connected-floating-point-pattern-backgrounds)

Direct links to missing pattern inputs use the field default. All 53,760 opacity, size and iteration-count pixels and all 8,960 seed-inheritance pixels match. [Direct-link cases and saved-file checks](parity-fixes.md#direct-pattern-links-with-an-unbound-variable)

Pattern declarations and sparse image selectors match 199,680/199,680 and 88,320/88,320 pixels in their respective controls. Automatic filtering and uninitialized counters retain differences. [Declarations, image selections and remaining differences](parity-fixes.md#pattern-variable-declarations-and-fixed-image-selectors)

Assignments in quadrant mode expressions now stay within the expression. All 49,920 child-color pixels match. [Mode writes and remaining input-scope differences](parity-fixes.md#assignments-in-quadrant-mode-expressions)

Empty quadrants now skip inactive stamp properties. The variable-writing controls match 49,120/49,920 pixels over 117 renders. [Measured property scopes](parity-fixes.md#active-properties-of-empty-and-drawing-quadrants)

Reading inherited variables directly reduced cook time on six Dirt 1 and BNW Spots 3 cases from 128–133 seconds to 43–44 seconds across two runs. The five completed cases retained every measured output bit. [Timings and root checks](parity-fixes.md#pattern-expression-variables-and-cook-time)

The reference reopened 26 function packages saved by MaterialCraft. All 6,400 pixels and 24 typed values match. [Saved argument checks](parity-fixes.md#connectable-numeric-arguments-in-saved-function-graphs) [Unconnected argument controls](parity-fixes.md#unconnected-function-arguments)

[Runtime square-root measurements](parity-fixes.md#runtime-pixel-square-roots) cover both exponent parities and the signed domain.

[Luminance EXR measurements](parity-fixes.md#luminance-exr-input) cover Y and YA inputs at half and single precision.

[Four-format results by root and output](parity-fixes.md#four-format-inventory-comparison) cover 375 roots. The same table includes the independent 32² measurements for 352 roots.

Reduced Sharpen now matches 16,704/16,704 pixels on the held-out signed field and 158,688/158,688 on six further fields. Four requested 1² cases still differ in output extent. [Filter measurements](parity-fixes.md#sharpen-at-reduced-output-sizes)

A change to Gaussian stamp coverage was checked against 61 roots. Additional rectangular captures reopened both brick-generator Tile helpers. [Root checks](parity-fixes.md#root-checks-after-clipping-gaussian-stamps) [Tile input and shape comparisons](parity-fixes.md#rectangular-fields-in-the-two-brick-generator-tile-helpers)

## Complete inventory

| SBS package / graph path | Input pins | Hidden in library | Parity status |
|---|---:|---|---|
| [3d_gradient.sbs/3d_linear_gradient](../assets/sbs/3d_gradient.sbs) | 1 | No | [Parity · 12 wired and 5 disconnected cases, four formats](parity-fixes.md#runtime-reciprocal-and-3d-gradient-contrast) |
| [3d_perlin_noise.sbs/3D_perlin_noise](../assets/sbs/3d_perlin_noise.sbs) | 1 | No | [Parity · 347,136 exact pixels in 34 shared-lookup renders, bundled defaults differ](parity-fixes.md#3d-noise-with-two-original-permutation-tables) |
| [3d_perlin_noise.sbs/3D_perlin_noise_fractal](../assets/sbs/3d_perlin_noise.sbs) | 1 | No | [Parity: 4 cases](parity-fixes.md#verified-cases) |
| [3d_perlin_noise.sbs/3D_ridged_noise_fractal](../assets/sbs/3d_perlin_noise.sbs) | 1 | No | [Parity · 525,824 exact pixels in 78 shared-lookup renders, bundled defaults differ](parity-fixes.md#3d-noise-with-two-original-permutation-tables) |
| [3d_planar_projection.sbs/3d_planar_projection](../assets/sbs/3d_planar_projection.sbs) | 3 | No | [Parity · 15 projection cases, 168,960 exact pixels](parity-fixes.md#uniform-angles-and-planar-projection) |
| [3d_planar_projection.sbs/3d_planar_projection_color](../assets/sbs/3d_planar_projection.sbs) | 3 | No | [Parity · 15 projection cases, 168,960 exact pixels](parity-fixes.md#uniform-angles-and-planar-projection) |
| [3d_simplex_noise.sbs/3d_simplex_noise](../assets/sbs/3d_simplex_noise.sbs) | 1 | No | [Parity: 4 cases](parity-fixes.md#verified-cases) |
| [3d_texture_jump_flood.sbs/3d_texture_jump_flood_iteration](../assets/sbs/3d_texture_jump_flood.sbs) | 1 | Yes | [Parity: 5 cases](parity-fixes.md#verified-cases) |
| [3d_texture_position.sbs/3d_texture_position](../assets/sbs/3d_texture_position.sbs) | 0 | No | [Parity: native 4K atlas](parity-fixes.md#verified-cases) |
| [3d_texture_render.sbs/3d_texture_surface_render_irradiance](../assets/sbs/3d_texture_render.sbs) | 6 | Yes | [Parity · 47,360 exact pixels in 71 renders](parity-fixes.md#surface-irradiance-and-uniform-power) |
| [3d_texture_render.sbs/3d_texture_surface_render_position](../assets/sbs/3d_texture_render.sbs) | 1 | Yes | [Parity · 92,928 exact pixels in 39 renders](parity-fixes.md#surface-position-and-signed-pixel-products) |
| [3d_texture_render.sbs/3d_texture_volume_render_ibl_pass](../assets/sbs/3d_texture_render.sbs) | 6 | Yes | [Parity · 39 renders, 31,488 exact pixels](parity-fixes.md#scaled-random-draws-and-offsets) |
| [3d_texture_render.sbs/3d_texture_volume_render_light_pass](../assets/sbs/3d_texture_render.sbs) | 4 | Yes | [Parity · 46 renders, all 43,264 RGBA pixels exact](parity-fixes.md#vector-products-in-the-direct-light-pass) |
| [3d_texture_render.sbs/position_debug](../assets/sbs/3d_texture_render.sbs) | 2 | Yes | [Parity · 14 sphere and plane cases, plus a matched 128² preview](parity-fixes.md#tinted-stamp-offsets-and-alpha) |
| [3d_texture_transform.sbs/3d_texture_offset_color](../assets/sbs/3d_texture_transform.sbs) | 1 | No | [Parity: 5 cases](parity-fixes.md#verified-cases) |
| [3d_texture_transform.sbs/3d_texture_offset_grayscale](../assets/sbs/3d_texture_transform.sbs) | 1 | No | [Parity: 4 cases](parity-fixes.md#verified-cases) |
| [3d_viewer.sbs/ibl_specular_contrib_anisotropic/panoramic_lod](../assets/sbs/3d_viewer.sbs) | 1 | Yes | [Parity · 20 cases, 219,392 exact pixels](parity-fixes.md#large-transform-offsets-and-additive-blends) |
| [3d_volume_mask.sbs/3d_volume_mask](../assets/sbs/3d_volume_mask.sbs) | 1 | No | [Parity · 116 renders, all 1,029,632 pixels exact](parity-fixes.md#pixel-quotients-and-volume-mask-interpolation) |
| [3d_worley_noise.sbs/3D_worley_noise](../assets/sbs/3d_worley_noise.sbs) | 1 | No | [Parity · 34 renders, 312,832 exact pixels](parity-fixes.md#per-pixel-random-initialization) |
| [alveolus.sbs/alveolus](../assets/sbs/alveolus.sbs) | 0 | Yes | [In progress · 332,280/334,336 pixels across 41 renders](parity-fixes.md#square-and-rectangular-pattern-generators) · [Single-stamp render-region comparisons](parity-fixes.md#pattern-coordinates-under-a-change-of-render-region) · [Gaussian disc-boundary sweeps](parity-fixes.md#gaussian-coverage-at-the-disc-boundary) · [262,144/262,144 pixels after isolating the generator](parity-fixes.md#alveolus-filters-after-the-first-differing-stage) |
| [angle_to_vector.sbs/angle_to_vector](../assets/sbs/angle_to_vector.sbs) | 1 | No | [Parity: 4 cases](parity-fixes.md#verified-cases) |
| [anisotropic_blur.sbs/anisotropic_blur](../assets/sbs/anisotropic_blur.sbs) | 1 | No | [Parity · 29 renders, 200,192 exact pixels across four formats](parity-fixes.md#integer-and-oblique-directional-blur) |
| [anisotropic_blur.sbs/anisotropic_blur_grayscale](../assets/sbs/anisotropic_blur.sbs) | 1 | No | [Parity · 29 renders, 200,192 exact pixels across four formats](parity-fixes.md#integer-and-oblique-directional-blur) |
| [atlas_spliter.sbs/crop_and_transform_sub](../assets/sbs/atlas_spliter.sbs) | 7 | Yes | [Parity · 37 wired cases, 378,880 exact pixels](parity-fixes.md#runtime-pixel-sine-cosine-and-atlas-rotation) |
| [atlas_spliter.sbs/max_ba](../assets/sbs/atlas_spliter.sbs) | 1 | Yes | [Parity: 3 cases](parity-fixes.md#verified-cases) |
| [auto_crop.sbs/min_max](../assets/sbs/auto_crop.sbs) | 1 | Yes | [Parity: 3 cases](parity-fixes.md#verified-cases) |
| [auto_levels.sbs/auto_levels](../assets/sbs/auto_levels.sbs) | 1 | No | [Parity: 3 cases](parity-fixes.md#verified-cases) |
| [auto_levels.sbs/min_max](../assets/sbs/auto_levels.sbs) | 1 | Yes | [Parity: 4 cases](parity-fixes.md#verified-cases) |
| [bevel_smooth.sbs/blur_gaussian_grayscale](../assets/sbs/bevel_smooth.sbs) | 1 | Yes | [Parity · 26 renders, 311,552 exact grayscale pixels](parity-fixes.md#wide-blur-windows-and-bevel-blur) |
| [bevel_smooth.sbs/debug/bevel_smooth_debug_position](../assets/sbs/bevel_smooth.sbs) | 1 | Yes | [Parity · 38 renders, 354,560 exact pixels across four formats](parity-fixes.md#texture-sampling-in-pattern-functions) |
| [bevel_smooth.sbs/jump_flood_segment_iteration](../assets/sbs/bevel_smooth.sbs) | 1 | Yes | [Parity · 171,264 exact pixels in 18 renders](parity-fixes.md#segment-mapping-and-normal-graph-rechecks) |
| [blackbody.sbs/blackbody](../assets/sbs/blackbody.sbs) | 0 | No | [Parity · 19 renders, 42,496 exact RGBA pixels](parity-fixes.md#literal-arguments-inside-pixel-function-calls) |
| [blend.sbs/add](../assets/sbs/blend.sbs) | 3 | Yes | [Parity: 5 cases](parity-fixes.md#verified-cases) |
| [blend.sbs/add_sub](../assets/sbs/blend.sbs) | 3 | Yes | [Parity · 7 renders, 311,296 exact pixels](parity-fixes.md#source-alpha-rounding-in-highpass) |
| [blend.sbs/color](../assets/sbs/blend.sbs) | 3 | No | [Parity · 18 cases, 171,264 exact pixels](parity-fixes.md#gradient-interpolation-color-blend-and-chrominance) |
| [blend.sbs/color_burn](../assets/sbs/blend.sbs) | 3 | No | [Parity · 18 cases, 171,264 exact RGBA pixels](parity-fixes.md#color-burn-and-color-dodge) |
| [blend.sbs/color_dodge](../assets/sbs/blend.sbs) | 3 | No | [Parity · 18 cases, 171,264 exact RGBA pixels](parity-fixes.md#color-burn-and-color-dodge) |
| [blend.sbs/copy](../assets/sbs/blend.sbs) | 3 | Yes | [Parity: 5 cases](parity-fixes.md#verified-cases) |
| [blend.sbs/difference](../assets/sbs/blend.sbs) | 3 | No | [Parity: 4 cases](parity-fixes.md#verified-cases) |
| [blend.sbs/dissolve_2](../assets/sbs/blend.sbs) | 3 | No | [Parity · 28 renders, 183,040 exact pixels](parity-fixes.md#per-pixel-random-initialization) |
| [blend.sbs/exclusion](../assets/sbs/blend.sbs) | 3 | Yes | [Parity: 4 cases](parity-fixes.md#verified-cases) |
| [blend.sbs/hard_light](../assets/sbs/blend.sbs) | 3 | Yes | [Parity: 4 cases](parity-fixes.md#verified-cases) |
| [blend.sbs/linear_burn](../assets/sbs/blend.sbs) | 3 | No | [Parity: 4 cases](parity-fixes.md#verified-cases) |
| [blend.sbs/luminosity](../assets/sbs/blend.sbs) | 3 | No | [Parity · 15 cases, 165,120 exact pixels](parity-fixes.md#luminosity-axial-gradients-color-lookup-and-luminance-merge) |
| [blend.sbs/max](../assets/sbs/blend.sbs) | 3 | Yes | [Parity: 4 cases](parity-fixes.md#verified-cases) |
| [blend.sbs/min](../assets/sbs/blend.sbs) | 3 | Yes | [Parity: 4 cases](parity-fixes.md#verified-cases) |
| [blend.sbs/multiply](../assets/sbs/blend.sbs) | 3 | Yes | [Parity: 4 cases](parity-fixes.md#verified-cases) |
| [blend.sbs/overlay](../assets/sbs/blend.sbs) | 3 | Yes | [Parity: 4 cases](parity-fixes.md#verified-cases) |
| [blend.sbs/screen](../assets/sbs/blend.sbs) | 3 | Yes | [Parity: 4 cases](parity-fixes.md#verified-cases) |
| [blend.sbs/soft_light](../assets/sbs/blend.sbs) | 3 | Yes | [Parity: 4 cases](parity-fixes.md#verified-cases) |
| [blend.sbs/subtract](../assets/sbs/blend.sbs) | 3 | Yes | [Parity: 4 cases](parity-fixes.md#verified-cases) |
| [blend_switch.sbs/multi_switch](../assets/sbs/blend_switch.sbs) | 20 | No | [Parity: 5 cases](parity-fixes.md#verified-cases) |
| [blend_switch.sbs/multi_switch_grayscale](../assets/sbs/blend_switch.sbs) | 20 | No | [Parity: 5 cases](parity-fixes.md#verified-cases) |
| [blend_switch.sbs/switch](../assets/sbs/blend_switch.sbs) | 2 | No | [Parity: 4 cases](parity-fixes.md#verified-cases) |
| [blend_switch.sbs/switch_grayscale](../assets/sbs/blend_switch.sbs) | 2 | No | [Parity: 6 cases](parity-fixes.md#verified-cases) |
| [blur_hq.sbs/blur_hq](../assets/sbs/blur_hq.sbs) | 1 | No | [Parity · 18 renders, 171,264 exact pixels across four formats](parity-fixes.md#integer-and-oblique-directional-blur) |
| [blur_hq.sbs/blur_hq_grayscale](../assets/sbs/blur_hq.sbs) | 1 | No | [Parity · 18 renders, 171,264 exact pixels across four formats](parity-fixes.md#integer-and-oblique-directional-blur) |
| [bnw_spots_1.sbs/bnw_spots_1](../assets/sbs/bnw_spots_1.sbs) | 0 | Yes | [In progress · 157,008/204,288 pixels across 34 renders](parity-fixes.md#square-and-rectangular-pattern-generators) · [64,000/64,000 downstream pixels](parity-fixes.md#isolating-the-generators-in-spots-bricks-and-cells) |
| [bnw_spots_3.sbs/bnw_spots_3](../assets/sbs/bnw_spots_3.sbs) | 0 | Yes | [In progress · 139,314/185,856 pixels across 27 renders](parity-fixes.md#stored-input-resizing-and-grayscale-levels-controls) · [27,136/27,136 downstream pixels](parity-fixes.md#isolating-the-generators-in-spots-bricks-and-cells) |
| [brick_1.sbs/brick_1](../assets/sbs/brick_1.sbs) | 0 | Yes | [In progress · 202,490/204,288 pixels across 34 renders](parity-fixes.md#square-and-rectangular-pattern-generators) · [64,000/64,000 downstream pixels](parity-fixes.md#isolating-the-generators-in-spots-bricks-and-cells) |
| [brick_2.sbs/brick_2](../assets/sbs/brick_2.sbs) | 0 | Yes | [In progress · 185,560/185,856 pixels across 27 renders](parity-fixes.md#square-and-rectangular-pattern-generators) · [27,136/27,136 downstream pixels](parity-fixes.md#isolating-the-generators-in-spots-bricks-and-cells) |
| [brick_3.sbs/brick_3](../assets/sbs/brick_3.sbs) | 0 | Yes | [In progress · 201,968/204,288 exact pixels in 34 renders](parity-fixes.md#stored-opacity-across-pattern-depths) · [64,000/64,000 downstream pixels](parity-fixes.md#isolating-the-generators-in-spots-bricks-and-cells) |
| [brick_generator.sbs/resources/Tile](../assets/sbs/brick_generator.sbs) | 1 | Yes | [In progress · 149,415/150,528 pixels across 74 renders](parity-fixes.md#the-lower-edge-of-an-axis-aligned-stamp) |
| [cartesian_polar_transformation.sbs/cartesian_to_polar](../assets/sbs/cartesian_polar_transformation.sbs) | 1 | No | [Parity · 171,264 exact pixels in 18 renders](parity-fixes.md#segment-mapping-and-normal-graph-rechecks) |
| [cartesian_polar_transformation.sbs/cartesian_to_polar_grayscale](../assets/sbs/cartesian_polar_transformation.sbs) | 1 | No | [Parity · 171,264 exact pixels in 18 renders](parity-fixes.md#segment-mapping-and-normal-graph-rechecks) |
| [cartesian_polar_transformation.sbs/polar_to_cartesian](../assets/sbs/cartesian_polar_transformation.sbs) | 1 | No | [Parity · 28 renders, all 306,688 pixels exact](parity-fixes.md#reverse-polar-sampling-in-color-and-grayscale) |
| [cartesian_polar_transformation.sbs/polar_to_cartesian_grayscale](../assets/sbs/cartesian_polar_transformation.sbs) | 1 | No | [Parity · 28 renders, all 306,688 pixels exact](parity-fixes.md#reverse-polar-sampling-in-color-and-grayscale) |
| [cells_1.sbs/cells_1](../assets/sbs/cells_1.sbs) | 0 | Yes | [In progress · 144,983/145,664 pixels across 17 renders](parity-fixes.md#square-and-rectangular-pattern-generators) · [27,136/27,136 downstream pixels](parity-fixes.md#isolating-the-generators-in-spots-bricks-and-cells) |
| [channel_mixer.sbs/channel_mixer](../assets/sbs/channel_mixer.sbs) | 1 | No | [Parity · 18 cases, 171,264 exact pixels](parity-fixes.md#large-transform-offsets-and-additive-blends) |
| [checker_1.sbs/checker_1](../assets/sbs/checker_1.sbs) | 0 | Yes | [Parity · eight cases, four formats](parity-fixes.md#transform-edges-and-floating-point-filtering) |
| [checker_2.sbs/checker_2](../assets/sbs/checker_2.sbs) | 0 | Yes | [Parity: 4 cases](parity-fixes.md#verified-cases) |
| [chrominance_extract.sbs/chrominance_extract](../assets/sbs/chrominance_extract.sbs) | 1 | No | [Parity · 18 cases, 171,264 exact pixels](parity-fixes.md#gradient-interpolation-color-blend-and-chrominance) |
| [clamp.sbs/clamp](../assets/sbs/clamp.sbs) | 1 | No | [Parity: 4 cases](parity-fixes.md#verified-cases) |
| [clamp.sbs/clamp_grayscale](../assets/sbs/clamp.sbs) | 1 | No | [Parity: 4 cases](parity-fixes.md#verified-cases) |
| [clone.sbs/clone](../assets/sbs/clone.sbs) | 2 | No | [Parity · eight cases, four formats](parity-fixes.md#clone-mask-and-alpha-rounding) |
| [clone_patch.sbs/transform_from_center](../assets/sbs/clone_patch.sbs) | 1 | Yes | [Parity: 4 cases](parity-fixes.md#verified-cases) |
| [clone_patch.sbs/transform_to_center](../assets/sbs/clone_patch.sbs) | 1 | Yes | [Parity: 4 cases](parity-fixes.md#verified-cases) |
| [clone_patch.sbs/transform_to_center_grayscale](../assets/sbs/clone_patch.sbs) | 1 | Yes | [Parity: 4 cases](parity-fixes.md#verified-cases) |
| [clouds_1.sbs/clouds_1](../assets/sbs/clouds_1.sbs) | 0 | Yes | [In progress · 144,178/145,664 exact pixels in 17 renders](parity-fixes.md#stored-opacity-across-pattern-depths) |
| [clouds_2.sbs/clouds_2](../assets/sbs/clouds_2.sbs) | 0 | Yes | [In progress · 128,263/129,280 exact pixels in 16 renders](parity-fixes.md#stored-opacity-across-pattern-depths) |
| [color_space_conversion.sbs/acescg_to_linear_srgb](../assets/sbs/color_space_conversion.sbs) | 1 | No | [Parity · eight cases in four formats](parity-fixes.md#color-conversion-checker-and-clone) |
| [color_space_conversion.sbs/acescg_to_srgb](../assets/sbs/color_space_conversion.sbs) | 1 | No | [Parity: 4 cases](parity-fixes.md#verified-cases) |
| [color_space_conversion.sbs/linear_srgb_to_acescg](../assets/sbs/color_space_conversion.sbs) | 1 | No | [Parity: 4 cases](parity-fixes.md#verified-cases) |
| [color_space_conversion.sbs/linear_srgb_to_srgb](../assets/sbs/color_space_conversion.sbs) | 1 | No | [Parity: 4 cases](parity-fixes.md#verified-cases) |
| [color_space_conversion.sbs/linear_srgb_to_srgb_grayscale](../assets/sbs/color_space_conversion.sbs) | 1 | No | [Parity: 4 cases](parity-fixes.md#verified-cases) |
| [color_space_conversion.sbs/srgb_to_acescg](../assets/sbs/color_space_conversion.sbs) | 1 | No | [Parity · 171,264 exact pixels across 18 renders](parity-fixes.md#color-conversion-gradient-height-and-histogram-controls) |
| [color_space_conversion.sbs/srgb_to_linear_srgb](../assets/sbs/color_space_conversion.sbs) | 1 | No | [Parity: 4 cases](parity-fixes.md#verified-cases) |
| [color_space_conversion.sbs/srgb_to_linear_srgb_grayscale](../assets/sbs/color_space_conversion.sbs) | 1 | No | [Parity: 4 cases](parity-fixes.md#verified-cases) |
| [color_temperature_adjustment.sbs/color_temperature_adjustment](../assets/sbs/color_temperature_adjustment.sbs) | 1 | No | [Parity · 18 cases, 171,264 exact RGBA pixels](parity-fixes.md#color-temperature-range-reductions-sky-parameters-and-polygon-paths) |
| [color_to_mask.sbs/color_to_mask](../assets/sbs/color_to_mask.sbs) | 1 | Yes | [Parity · 18 cases, 168,192 exact pixels](parity-fixes.md#levels-endpoints-camera-tangents-and-uniform-exponentials) |
| [color_to_mask_v2.sbs/color_to_mask](../assets/sbs/color_to_mask_v2.sbs) | 2 | No | [Parity · 23 renders, all 179,456 grayscale pixels exact](parity-fixes.md#constants-returned-from-pixel-functions) |
| [composite.sbs/composite/background/uniform_color_float3](../assets/sbs/composite.sbs) | 0 | Yes | [Parity: 4 cases](parity-fixes.md#verified-cases) |
| [composite.sbs/composite/blends/blend_content](../assets/sbs/composite.sbs) | 3 | Yes | [Parity · 171,264 exact pixels in 18 renders](parity-fixes.md#segment-mapping-and-normal-graph-rechecks) |
| [composite.sbs/composite/blends/blend_content_grayscale](../assets/sbs/composite.sbs) | 3 | Yes | [Parity: 4 cases](parity-fixes.md#verified-cases) |
| [composite.sbs/composite/blends/blend_content_pre_post_composite](../assets/sbs/composite.sbs) | 2 | Yes | [Parity · 17 renders, all 166,400 RGBA pixels exact](parity-fixes.md#multiply-blending-and-prepost-composition) |
| [composite.sbs/composite/blends/blend_content_pre_post_composite_grayscale](../assets/sbs/composite.sbs) | 2 | Yes | [Parity: 4 cases](parity-fixes.md#verified-cases) |
| [composite.sbs/composite/blends/blend_matte](../assets/sbs/composite.sbs) | 2 | Yes | [Parity · 19 renders, all 168,448 grayscale pixels exact](parity-fixes.md#unbound-property-links-and-composite-matte) |
| [composite.sbs/composite/blends/blend_matte_bg_passthrough](../assets/sbs/composite.sbs) | 2 | Yes | [Parity: 4 cases](parity-fixes.md#verified-cases) |
| [composite.sbs/composite/blends/blend_matte_fg_passthrough](../assets/sbs/composite.sbs) | 2 | Yes | [Parity: 4 cases](parity-fixes.md#verified-cases) |
| [composite.sbs/composite/filters/filter_mask](../assets/sbs/composite.sbs) | 2 | Yes | [Parity: 4 cases](parity-fixes.md#verified-cases) |
| [composite.sbs/composite/filters/filter_split_rgb_a](../assets/sbs/composite.sbs) | 1 | Yes | [Parity: 4 cases](parity-fixes.md#verified-cases) |
| [composite.sbs/composite/filters/filter_transformation](../assets/sbs/composite.sbs) | 1 | Yes | [Parity · 18 cases, 171,264 exact pixels](parity-fixes.md#stored-precision-in-transform-mip-levels) |
| [composite.sbs/composite/filters/filter_transformation_grayscale](../assets/sbs/composite.sbs) | 1 | Yes | [Parity: 4 cases](parity-fixes.md#verified-cases) |
| [constant_values.sbs/constant_boolean](../assets/sbs/constant_values.sbs) | 0 | No | [Parity: 3 typed values](parity-fixes.md#verified-cases) |
| [constant_values.sbs/float/constant_float](../assets/sbs/constant_values.sbs) | 0 | No | [Parity: 3 typed values](parity-fixes.md#verified-cases) |
| [constant_values.sbs/float/constant_float2](../assets/sbs/constant_values.sbs) | 0 | No | [Parity: 3 typed values](parity-fixes.md#verified-cases) |
| [constant_values.sbs/float/constant_float3](../assets/sbs/constant_values.sbs) | 0 | No | [Parity: 3 typed values](parity-fixes.md#verified-cases) |
| [constant_values.sbs/float/constant_float4](../assets/sbs/constant_values.sbs) | 0 | No | [Parity: 3 typed values](parity-fixes.md#verified-cases) |
| [constant_values.sbs/integer/constant_integer](../assets/sbs/constant_values.sbs) | 0 | No | [Parity: 3 typed values](parity-fixes.md#verified-cases) |
| [constant_values.sbs/integer/constant_integer2](../assets/sbs/constant_values.sbs) | 0 | No | [Parity: 3 typed values](parity-fixes.md#verified-cases) |
| [constant_values.sbs/integer/constant_integer3](../assets/sbs/constant_values.sbs) | 0 | No | [Parity: 3 typed values](parity-fixes.md#verified-cases) |
| [constant_values.sbs/integer/constant_integer4](../assets/sbs/constant_values.sbs) | 0 | No | [Parity · 41 exact typed vectors, authored constants checked on reload](parity-fixes.md#stored-integer-value-outputs) |
| [contrast_luminosity.sbs/contrast_luminosity](../assets/sbs/contrast_luminosity.sbs) | 1 | No | [Parity: 4 cases](parity-fixes.md#verified-cases) |
| [contrast_luminosity.sbs/contrast_luminosity_grayscale](../assets/sbs/contrast_luminosity.sbs) | 1 | No | [Parity: 4 cases](parity-fixes.md#verified-cases) |
| [creased.sbs/creased](../assets/sbs/creased.sbs) | 0 | Yes | [In progress · 143,948/145,664 exact pixels in 17 renders](parity-fixes.md#pattern-tint-coverage-and-grayscale-conversion) |
| [crop.sbs/crop](../assets/sbs/crop.sbs) | 1 | No | [Parity · 48 renders, all 377,344 pixels across both outputs exact](parity-fixes.md#half-precision-crop-area-and-input-sizes) |
| [crop.sbs/crop_grayscale](../assets/sbs/crop.sbs) | 1 | No | [Parity · 48 renders, all 377,344 pixels across both outputs exact](parity-fixes.md#half-precision-crop-area-and-input-sizes) |
| [crystal_1.sbs/crystal_1](../assets/sbs/crystal_1.sbs) | 0 | Yes | [In progress · 144,060/145,664 exact pixels in 17 renders](parity-fixes.md#gradient-warp-displacement-and-sampling) |
| [crystal_2.sbs/crystal_2](../assets/sbs/crystal_2.sbs) | 0 | Yes | [In progress · 144,770/145,664 exact pixels in 17 renders](parity-fixes.md#pattern-tint-coverage-and-grayscale-conversion) |
| [curvature.sbs/curvature](../assets/sbs/curvature.sbs) | 1 | No | [Parity · 20 renders, 176,384 exact pixels](parity-fixes.md#sampling-integer-bitmap-inputs-through-wider-bridges) |
| [curvature_smooth.sbs/curvature_smooth](../assets/sbs/curvature_smooth.sbs) | 1 | Yes | [Parity · 20 renders, 176,384 exact pixels](parity-fixes.md#sampling-integer-bitmap-inputs-through-wider-bridges) |
| [curvature_smooth_v2.sbs/compute_curvature_v2](../assets/sbs/curvature_smooth_v2.sbs) | 2 | Yes | [Parity: 6 cases](parity-fixes.md#verified-cases) |
| [diffusion.sbs/impl/pass_RGBA](../assets/sbs/diffusion.sbs) | 4 | Yes | [Parity · 19 renders, 143,872 exact pixels](parity-fixes.md#per-pixel-random-initialization) |
| [diffusion.sbs/impl/pass_UV](../assets/sbs/diffusion.sbs) | 1 | Yes | [Parity · 19 renders, 143,872 exact pixels](parity-fixes.md#per-pixel-random-initialization) |
| [diffusion.sbs/impl/pass_gray](../assets/sbs/diffusion.sbs) | 3 | Yes | [Parity · 19 renders, 143,872 exact pixels](parity-fixes.md#per-pixel-random-initialization) |
| [diffusion.sbs/impl/pass_normal](../assets/sbs/diffusion.sbs) | 2 | Yes | [Parity · 19 renders, 143,872 exact pixels](parity-fixes.md#per-pixel-random-initialization) |
| [directional_distance.sbs/directional_distance](../assets/sbs/directional_distance.sbs) | 3 | No | [Parity · 32 renders, both outputs exact across 342,528 pixels](parity-fixes.md#distance-output-channels-axis-wrapping-and-cutoffs) |
| [directionnal_noise.sbs/directionnal_noise](../assets/sbs/directionnal_noise.sbs) | 0 | Yes | [In progress · 145,135/147,712 pixels across 23 renders](parity-fixes.md#stored-input-resizing-and-grayscale-levels-controls) |
| [dirt_1.sbs/dirt_1](../assets/sbs/dirt_1.sbs) | 0 | Yes | [In progress · 145,661/145,664 exact pixels, overlapping disc additions isolated](parity-fixes.md#overlapping-disc-additions-in-dirt-1) |
| [dirt_3.sbs/dirt_3](../assets/sbs/dirt_3.sbs) | 0 | Yes | [In progress · 145,359/145,664 exact pixels in 17 renders](parity-fixes.md#pattern-tint-coverage-and-grayscale-conversion) |
| [dirt_5.sbs/dirt_5](../assets/sbs/dirt_5.sbs) | 0 | Yes | [In progress · 129,226/129,280 exact pixels in 16 renders](parity-fixes.md#stored-opacity-across-pattern-depths) |
| [dirt_6.sbs/dirt_6](../assets/sbs/dirt_6.sbs) | 0 | Yes | [In progress · 125,252/129,280 exact pixels in 16 renders](parity-fixes.md#signed-sharpen-stencils-and-stored-inputs) |
| [dirt_gradient.sbs/dirt_gradient](../assets/sbs/dirt_gradient.sbs) | 0 | Yes | [In progress · 128,396/129,280 exact pixels in 16 renders](parity-fixes.md#pattern-tint-coverage-and-grayscale-conversion) |
| [exposure.sbs/exposure](../assets/sbs/exposure.sbs) | 1 | No | [Parity: 4 cases](parity-fixes.md#verified-cases) |
| [extend_shape.sbs/extend_shape_color](../assets/sbs/extend_shape.sbs) | 1 | No | [Parity: 4 cases](parity-fixes.md#verified-cases) |
| [extend_shape.sbs/extend_shape_grayscale](../assets/sbs/extend_shape.sbs) | 1 | No | [Parity: 4 cases](parity-fixes.md#verified-cases) |
| [facing_normal.sbs/facing_normal](../assets/sbs/facing_normal.sbs) | 1 | No | [Parity: 4 cases](parity-fixes.md#verified-cases) |
| [fibers_1.sbs/fibers_1](../assets/sbs/fibers_1.sbs) | 0 | Yes | [In progress · 416,773/424,192 pixels across 24 renders](parity-fixes.md#pattern-rotation-phase) |
| [flood_fill.sbs/flood_fill_step](../assets/sbs/flood_fill.sbs) | 1 | Yes | [Parity: 4 cases](parity-fixes.md#verified-cases) |
| [flood_fill.sbs/flood_fill_to_bbox_size](../assets/sbs/flood_fill.sbs) | 1 | Yes | [Parity: 4 cases](parity-fixes.md#verified-cases) |
| [flood_fill.sbs/flood_fill_to_position](../assets/sbs/flood_fill.sbs) | 1 | Yes | [Parity: 4 cases](parity-fixes.md#verified-cases) |
| [flood_fill.sbs/flood_fill_to_random_color](../assets/sbs/flood_fill.sbs) | 1 | Yes | [Parity · 19 renders, 143,872 exact pixels](parity-fixes.md#scaled-random-draws-and-offsets) |
| [flood_fill.sbs/flood_fill_to_random_grayscale](../assets/sbs/flood_fill.sbs) | 1 | Yes | [Parity · 19 renders, 143,872 exact pixels](parity-fixes.md#scaled-random-draws-and-offsets) |
| [flood_fill_2.sbs/flood_fill_impl/flood_fill_step_A](../assets/sbs/flood_fill_2.sbs) | 2 | Yes | [Parity: 4 cases](parity-fixes.md#verified-cases) |
| [flood_fill_2.sbs/flood_fill_impl/flood_fill_step_A_switch](../assets/sbs/flood_fill_2.sbs) | 4 | Yes | [Parity: 4 cases](parity-fixes.md#verified-cases) |
| [flood_fill_2.sbs/flood_fill_impl/flood_fill_step_B](../assets/sbs/flood_fill_2.sbs) | 1 | Yes | [Parity: 4 cases](parity-fixes.md#verified-cases) |
| [flood_fill_2.sbs/flood_fill_to_bbox_size](../assets/sbs/flood_fill_2.sbs) | 1 | No | [Parity: 4 cases](parity-fixes.md#verified-cases) |
| [flood_fill_2.sbs/flood_fill_to_color](../assets/sbs/flood_fill_2.sbs) | 2 | No | [Parity · 19 renders, 143,872 exact RGBA pixels](parity-fixes.md#packed-random-draws-in-flood-fill-color) |
| [flood_fill_2.sbs/flood_fill_to_grayscale](../assets/sbs/flood_fill_2.sbs) | 2 | No | [Parity · 19 renders, 143,872 exact pixels](parity-fixes.md#scaled-random-draws-and-offsets) |
| [flood_fill_2.sbs/flood_fill_to_position](../assets/sbs/flood_fill_2.sbs) | 1 | No | [Parity: 4 cases](parity-fixes.md#verified-cases) |
| [flood_fill_2.sbs/flood_fill_to_random_color](../assets/sbs/flood_fill_2.sbs) | 1 | No | [Parity · 19 renders, 143,872 exact RGBA pixels](parity-fixes.md#packed-random-draws-in-flood-fill-color) |
| [flood_fill_2.sbs/flood_fill_to_random_grayscale](../assets/sbs/flood_fill_2.sbs) | 1 | No | [Parity · 19 renders, 143,872 exact pixels](parity-fixes.md#scaled-random-draws-and-offsets) |
| [floodfill_to_index.sbs/pass](../assets/sbs/floodfill_to_index.sbs) | 1 | Yes | [Parity: 4 cases](parity-fixes.md#verified-cases) |
| [fluid.sbs/fluid](../assets/sbs/fluid.sbs) | 0 | Yes | [In progress · 423,928/424,192 pixels in 24 renders, first difference in the FX-Map](parity-fixes.md#fibers-and-fluid-pattern-stages) |
| [fractal_sum_base.sbs/fractal_sum_base](../assets/sbs/fractal_sum_base.sbs) | 0 | Yes | [Parity · 18 renders, 145,664 exact pixels](parity-fixes.md#stored-grayscale-stops-and-scalar-levels-controls) |
| [fur_1.sbs/fur_1](../assets/sbs/fur_1.sbs) | 0 | Yes | [In progress · 164,031/178,432 exact pixels in 21 renders](parity-fixes.md#stored-opacity-across-pattern-depths) |
| [fur_2.sbs/fur_2](../assets/sbs/fur_2.sbs) | 0 | Yes | [In progress · 134,013/178,432 exact pixels, first difference in FX-Map](parity-fixes.md#fur-pattern-sources-and-downstream-filters) |
| [fur_3.sbs/fur_3](../assets/sbs/fur_3.sbs) | 0 | Yes | [In progress · 145,101/178,432 exact pixels, first difference in FX-Map](parity-fixes.md#fur-pattern-sources-and-downstream-filters) |
| [fur_4.sbs/fur_4](../assets/sbs/fur_4.sbs) | 0 | Yes | [In progress · 148,992/178,432 exact pixels, first difference in FX-Map](parity-fixes.md#fur-pattern-sources-and-downstream-filters) |
| [fxaa.sbs/fxaa_grayscale](../assets/sbs/fxaa.sbs) | 1 | No | [Parity: 4 cases](parity-fixes.md#verified-cases) |
| [gaussian_1.sbs/gaussian_1](../assets/sbs/gaussian_1.sbs) | 0 | Yes | [In progress · 168,522/171,264 pixels across 18 renders](parity-fixes.md#radial-stamp-profiles-and-pattern-generators) |
| [gaussian_2.sbs/gaussian_2](../assets/sbs/gaussian_2.sbs) | 0 | Yes | [In progress · 169,796/171,264 pixels across 18 renders](parity-fixes.md#radial-stamp-profiles-and-pattern-generators) |
| [gaussian_spots_1.sbs/gaussian_spots_1](../assets/sbs/gaussian_spots_1.sbs) | 0 | Yes | [In progress · 1,573/132,096 pixels across 9 renders](parity-fixes.md#radial-stamp-profiles-and-pattern-generators) |
| [gaussian_spots_2.sbs/gaussian_spots_2](../assets/sbs/gaussian_spots_2.sbs) | 0 | Yes | [In progress · 137,856/178,432 exact pixels · first difference in FX-Map](parity-fixes.md#gaussian-pattern-sources-and-repeat-limits) |
| [glow.sbs/clamp](../assets/sbs/glow.sbs) | 1 | No | [Parity: 4 cases](parity-fixes.md#verified-cases) |
| [gradient_axial.sbs/gradient_axial](../assets/sbs/gradient_axial.sbs) | 0 | No | [Parity · 15 cases, 165,120 exact pixels](parity-fixes.md#luminosity-axial-gradients-color-lookup-and-luminance-merge) |
| [gradient_axial.sbs/gradient_axial_reflected](../assets/sbs/gradient_axial.sbs) | 0 | No | [Parity · 15 cases, 165,120 exact pixels](parity-fixes.md#luminosity-axial-gradients-color-lookup-and-luminance-merge) |
| [gradient_circular.sbs/gradient_circular](../assets/sbs/gradient_circular.sbs) | 0 | No | [Parity · 13 cases across sizes, seeds and storage formats](parity-fixes.md#runtime-angles-and-circular-gradient) |
| [gradient_linear_1.sbs/gradient_linear_1](../assets/sbs/gradient_linear_1.sbs) | 0 | No | [In progress · 435,744/435,968 pixels across 47 renders, all 224 differences at rotation 3](parity-fixes.md#pattern-rotation-phase) |
| [gradient_linear_2.sbs/gradient_linear_2](../assets/sbs/gradient_linear_2.sbs) | 0 | No | [Parity · 8 cases, grayscale in four formats](parity-fixes.md#linear-gradient-2-and-multiangle-albedo) |
| [gradient_linear_3.sbs/gradient_linear_3](../assets/sbs/gradient_linear_3.sbs) | 0 | No | [In progress · 265,072/277,760 pixels across 23 renders](parity-fixes.md#pattern-rotation-phase) |
| [gradient_radial.sbs/gradient_radial](../assets/sbs/gradient_radial.sbs) | 0 | No | [Parity · 171,264 exact pixels across 18 renders](parity-fixes.md#color-conversion-gradient-height-and-histogram-controls) |
| [grid_atlas.sbs/grid_atlas_color](../assets/sbs/grid_atlas.sbs) | 16 | No | [Parity: 4 cases](parity-fixes.md#verified-cases) |
| [grid_atlas.sbs/grid_atlas_grayscale](../assets/sbs/grid_atlas.sbs) | 16 | No | [Parity: 4 cases](parity-fixes.md#verified-cases) |
| [hald_clut.sbs/hald_clut](../assets/sbs/hald_clut.sbs) | 2 | No | [Parity · 15 cases, 165,120 exact pixels](parity-fixes.md#luminosity-axial-gradients-color-lookup-and-luminance-merge) |
| [hbao.sbs/hbao_mip](../assets/sbs/hbao.sbs) | 1 | Yes | [Parity · 171,264 exact pixels in 18 renders](parity-fixes.md#centered-transform-resizing-and-untiled-edges) |
| [hbao_2.sbs/hbao_mip](../assets/sbs/hbao_2.sbs) | 1 | Yes | [Parity · 171,264 exact pixels in 18 renders](parity-fixes.md#centered-transform-resizing-and-untiled-edges) |
| [hbao_2.sbs/hbao_subsample](../assets/sbs/hbao_2.sbs) | 2 | Yes | [Parity · 19 cases, 171,520 exact pixels](parity-fixes.md#large-transform-offsets-and-additive-blends) |
| [hdr_exposure_preview.sbs/hdr_exposure_preview](../assets/sbs/hdr_exposure_preview.sbs) | 1 | No | [Parity: 4 cases](parity-fixes.md#verified-cases) |
| [hdr_merge.sbs/average_luminance_merge](../assets/sbs/hdr_merge.sbs) | 1 | Yes | [Parity · 15 cases, 165,120 exact pixels](parity-fixes.md#luminosity-axial-gradients-color-lookup-and-luminance-merge) |
| [hdr_range_viewer.sbs/hdr_range_viewer_grayscale](../assets/sbs/hdr_range_viewer.sbs) | 1 | No | [Parity · 18 cases, 167,424 exact pixels](parity-fixes.md#signed-levels-driven-grayscale-weights-and-grayscale-gradients) |
| [height_blend.sbs/height_blend](../assets/sbs/height_blend.sbs) | 3 | No | [Parity · 342,528 exact pixels across 32 renders](parity-fixes.md#color-conversion-gradient-height-and-histogram-controls) |
| [height_extrude.sbs/impl/acceleration_struct_impl](../assets/sbs/height_extrude.sbs) | 1 | Yes | [Parity: 4 cases](parity-fixes.md#verified-cases) |
| [height_to_normal_world_units.sbs/height_to_normal_world_units](../assets/sbs/height_to_normal_world_units.sbs) | 1 | Yes | [Parity · eight cases in four formats](parity-fixes.md#height-to-normal-in-world-units) |
| [height_to_normal_world_units.sbs/height_to_normal_world_units_2](../assets/sbs/height_to_normal_world_units.sbs) | 1 | No | [Parity · eight cases in four formats](parity-fixes.md#height-to-normal-in-world-units) |
| [highpass.sbs/highpass](../assets/sbs/highpass.sbs) | 1 | No | [Parity · 21 renders, 177,408 exact pixels](parity-fixes.md#source-alpha-rounding-in-highpass) |
| [highpass.sbs/highpass_grayscale](../assets/sbs/highpass.sbs) | 1 | No | [Parity · 30 renders, 216,576 exact pixels](parity-fixes.md#source-alpha-rounding-in-highpass) |
| [histogram_range.sbs/histogram_range](../assets/sbs/histogram_range.sbs) | 1 | No | [Parity: 4 cases](parity-fixes.md#verified-cases) |
| [histogram_scan.sbs/histogram_scan](../assets/sbs/histogram_scan.sbs) | 1 | No | [Parity · 171,264 exact pixels across 18 renders](parity-fixes.md#color-conversion-gradient-height-and-histogram-controls) |
| [histogram_scan_non_uniform.sbs/histogram_scan_non_uniform](../assets/sbs/histogram_scan_non_uniform.sbs) | 3 | No | [Parity: 4 cases](parity-fixes.md#verified-cases) |
| [histogram_select.sbs/histogram_select](../assets/sbs/histogram_select.sbs) | 1 | No | [Parity: 4 cases](parity-fixes.md#verified-cases) |
| [histogram_shift.sbs/histogram_shift](../assets/sbs/histogram_shift.sbs) | 1 | No | [Parity · 171,264 exact pixels in 18 renders](parity-fixes.md#segment-mapping-and-normal-graph-rechecks) |
| [histogram_tools.sbs/histogram_compute](../assets/sbs/histogram_tools.sbs) | 1 | No | [Parity · 4,608 exact pixels across 32 renders](parity-fixes.md#color-conversion-gradient-height-and-histogram-controls) |
| [histogram_tools.sbs/horizontal_max](../assets/sbs/histogram_tools.sbs) | 1 | Yes | [Parity: 4 cases](parity-fixes.md#verified-cases) |
| [id_to_mask.sbs/id_to_mask_color](../assets/sbs/id_to_mask.sbs) | 1 | Yes | [Parity: 4 cases](parity-fixes.md#verified-cases) |
| [id_to_mask.sbs/id_to_mask_grayscale](../assets/sbs/id_to_mask.sbs) | 1 | No | [Parity: 4 cases](parity-fixes.md#verified-cases) |
| [invert.sbs/invert](../assets/sbs/invert.sbs) | 1 | No | [Parity: 4 cases](parity-fixes.md#verified-cases) |
| [invert.sbs/invert_grayscale](../assets/sbs/invert.sbs) | 1 | No | [Parity: 4 cases](parity-fixes.md#verified-cases) |
| [linear_srgb_conversion.sbs/convert_to_linear](../assets/sbs/linear_srgb_conversion.sbs) | 1 | Yes | [Parity: 4 cases](parity-fixes.md#verified-cases) |
| [linear_srgb_conversion.sbs/convert_to_linear_grayscale](../assets/sbs/linear_srgb_conversion.sbs) | 1 | Yes | [Parity: 4 cases](parity-fixes.md#verified-cases) |
| [linear_srgb_conversion.sbs/convert_to_srgb](../assets/sbs/linear_srgb_conversion.sbs) | 1 | Yes | [Parity: 4 cases](parity-fixes.md#verified-cases) |
| [linear_srgb_conversion.sbs/convert_to_srgb_grayscale](../assets/sbs/linear_srgb_conversion.sbs) | 1 | Yes | [Parity: 4 cases](parity-fixes.md#verified-cases) |
| [make_it_tile_patch.sbs/make_it_tile_patch](../assets/sbs/make_it_tile_patch.sbs) | 1 | No | [In progress · 132,090/132,096 exact pixels in 9 renders](parity-fixes.md#gradient-warp-displacement-and-sampling) |
| [make_it_tile_patch_grayscale.sbs/make_it_tile_patch_grayscale](../assets/sbs/make_it_tile_patch_grayscale.sbs) | 1 | No | [In progress · 132,092/132,096 exact pixels in 9 renders](parity-fixes.md#gradient-warp-displacement-and-sampling) |
| [material_switch.sbs/material_switch](../assets/sbs/material_switch.sbs) | 24 | No | [Parity: 4 cases](parity-fixes.md#verified-cases) |
| [material_transforms.sbs/material_transforms](../assets/sbs/material_transforms.sbs) | 5 | Yes | [Parity · 856,320 exact pixels across every output](parity-fixes.md#material-bundles-and-the-eighth-irradiance-pass) |
| [me_water_level.sbs/scratches_2](../assets/sbs/me_water_level.sbs) | 0 | Yes | [In progress · 138,019/178,432 pixels in 21 renders](parity-fixes.md#scratches-messy-fibers-microscope-and-moisture) |
| [mean_of_least_variance.sbs/mlv_color_iteration](../assets/sbs/mean_of_least_variance.sbs) | 1 | Yes | [Parity · 171,264 exact pixels across 18 renders](parity-fixes.md#color-conversion-gradient-height-and-histogram-controls) |
| [mean_of_least_variance.sbs/mlv_grayscale_iteration](../assets/sbs/mean_of_least_variance.sbs) | 1 | Yes | [Parity · 171,264 exact pixels across 18 renders](parity-fixes.md#color-conversion-gradient-height-and-histogram-controls) |
| [median_filter.sbs/median_filter_color](../assets/sbs/median_filter.sbs) | 1 | No | [Parity · 171,264 exact pixels across 18 renders](parity-fixes.md#color-conversion-gradient-height-and-histogram-controls) |
| [median_filter.sbs/median_filter_grayscale](../assets/sbs/median_filter.sbs) | 1 | No | [Parity: 4 cases](parity-fixes.md#verified-cases) |
| [mesh_1.sbs/mesh_1](../assets/sbs/mesh_1.sbs) | 0 | Yes | [In progress · 38,454/132,096 pixels across 9 renders](parity-fixes.md#directional-and-additive-stamp-profiles) |
| [mesh_2.sbs/mesh_2](../assets/sbs/mesh_2.sbs) | 0 | Yes | [In progress · 34,148/132,096 pixels across 9 renders](parity-fixes.md#directional-and-additive-stamp-profiles) |
| [mesh_adaptive_base_color.sbs/base_color_2_inputs](../assets/sbs/mesh_adaptive_base_color.sbs) | 2 | No | [Parity: 4 cases](parity-fixes.md#verified-cases) |
| [mesh_adaptive_base_color.sbs/base_color_4_inputs](../assets/sbs/mesh_adaptive_base_color.sbs) | 2 | No | [Parity: 4 cases](parity-fixes.md#verified-cases) |
| [mesh_adaptive_base_color.sbs/base_color_8_inputs](../assets/sbs/mesh_adaptive_base_color.sbs) | 2 | No | [Parity: 4 cases](parity-fixes.md#verified-cases) |
| [mesh_adaptive_material_blender.sbs/mesh_material_blender_2_inputs](../assets/sbs/mesh_adaptive_material_blender.sbs) | 7 | Yes | [Parity · 513,792 exact pixels across every output](parity-fixes.md#material-bundles-and-the-eighth-irradiance-pass) |
| [mesh_adaptive_material_blender.sbs/mesh_material_blender_4_inputs](../assets/sbs/mesh_adaptive_material_blender.sbs) | 13 | Yes | [Parity · 513,792 exact pixels across every output](parity-fixes.md#material-bundles-and-the-eighth-irradiance-pass) |
| [mesh_adaptive_material_blender.sbs/mesh_material_blender_8_inputs](../assets/sbs/mesh_adaptive_material_blender.sbs) | 25 | Yes | [Parity · 513,792 exact pixels across every output](parity-fixes.md#material-bundles-and-the-eighth-irradiance-pass) |
| [mesh_data_combiner.sbs/mesh_data_combiner](../assets/sbs/mesh_data_combiner.sbs) | 11 | No | [Parity: 4 cases](parity-fixes.md#verified-cases) |
| [messy_fibers_1.sbs/messy_fibers_1](../assets/sbs/messy_fibers_1.sbs) | 0 | Yes | [In progress · 122,750/178,432 pixels in 21 renders](parity-fixes.md#scratches-messy-fibers-microscope-and-moisture) |
| [messy_fibers_2.sbs/messy_fibers_2](../assets/sbs/messy_fibers_2.sbs) | 0 | Yes | [In progress · 96,050/178,432 pixels in 21 renders](parity-fixes.md#scratches-messy-fibers-microscope-and-moisture) |
| [messy_fibers_3.sbs/messy_fibers_3](../assets/sbs/messy_fibers_3.sbs) | 0 | Yes | [In progress · 126,928/132,096 pixels across 9 renders](parity-fixes.md#directional-and-additive-stamp-profiles) |
| [messy_fibers_4.sbs/messy_fibers_4](../assets/sbs/messy_fibers_4.sbs) | 0 | Yes | [In progress · 119,106/162,048 pixels in 20 renders](parity-fixes.md#scratches-messy-fibers-microscope-and-moisture) |
| [microscope_view.sbs/microscope_view](../assets/sbs/microscope_view.sbs) | 0 | Yes | [In progress · 177,304/178,432 pixels in 21 renders](parity-fixes.md#scratches-messy-fibers-microscope-and-moisture) |
| [min_max.sbs/min_max](../assets/sbs/min_max.sbs) | 1 | No | [Parity · both scalar outputs exact in 25 cases](parity-fixes.md#color-temperature-range-reductions-sky-parameters-and-polygon-paths) |
| [moisture_noise.sbs/moisture_noise](../assets/sbs/moisture_noise.sbs) | 0 | Yes | [In progress · 178,192/178,432 pixels in 21 renders](parity-fixes.md#scratches-messy-fibers-microscope-and-moisture) |
| [mosaic.sbs/mosaic](../assets/sbs/mosaic.sbs) | 2 | No | [Parity · 17 renders, 142,592 exact pixels](parity-fixes.md#gradient-warp-displacement-and-sampling) |
| [mosaic.sbs/mosaic_grayscale](../assets/sbs/mosaic.sbs) | 2 | No | [Parity · 17 renders, 142,592 exact pixels](parity-fixes.md#gradient-warp-displacement-and-sampling) |
| [multi_directional_warp.sbs/multi_directional_warp_color](../assets/sbs/multi_directional_warp.sbs) | 2 | No | [Parity · 19 cases, 175,360 exact pixels and intermediate stages confirmed](parity-fixes.md#multi-directional-warp-and-intermediate-storage) |
| [multi_directional_warp.sbs/multi_directional_warp_grayscale](../assets/sbs/multi_directional_warp.sbs) | 2 | No | [Parity · 19 cases, 175,360 exact pixels and intermediate stages confirmed](parity-fixes.md#multi-directional-warp-and-intermediate-storage) |
| [multiangle_to_albedo.sbs/multiangle_to_albedo](../assets/sbs/multiangle_to_albedo.sbs) | 8 | No | [Parity · 14 cases, including eight distinct input images](parity-fixes.md#linear-gradient-2-and-multiangle-albedo) |
| [multiangle_to_normal.sbs/angle_to_normal](../assets/sbs/multiangle_to_normal.sbs) | 1 | Yes | [Parity: 4 cases](parity-fixes.md#verified-cases) |
| [multiangle_to_normal.sbs/angle_weight](../assets/sbs/multiangle_to_normal.sbs) | 1 | Yes | [Parity: 4 cases](parity-fixes.md#verified-cases) |
| [noise_anisotropic_noise.sbs/anisotropic_noise](../assets/sbs/noise_anisotropic_noise.sbs) | 0 | Yes | [Parity · 18 renders, all 171,264 pixels exact](parity-fixes.md#quarter-turn-directional-blur) |
| [noise_anisotropic_noise_v2.sbs/anisotropic_noise_v2](../assets/sbs/noise_anisotropic_noise_v2.sbs) | 0 | No | [Parity · 18 renders, all 171,264 pixels exact](parity-fixes.md#high-pass-anisotropic-noise-and-triangle-grid-controls) |
| [noise_bnw_spots_1.sbs/bnw_spots_1](../assets/sbs/noise_bnw_spots_1.sbs) | 0 | Yes | [In progress · all nine renders cook, 3,216/132,096 exact pixels](parity-fixes.md#empty-size-dependent-pattern-branches) |
| [noise_bnw_spots_1_v2.sbs/bnw_spots_1](../assets/sbs/noise_bnw_spots_1_v2.sbs) | 0 | No | [In progress · nine renders, 41,745/132,096 exact pixels](parity-fixes.md#pattern-backgrounds-and-center-culling) |
| [noise_bnw_spots_3.sbs/bnw_spots_3](../assets/sbs/noise_bnw_spots_3.sbs) | 0 | Yes | [In progress · 104,343/107,776 pixels, 13 execution-limit cases](parity-fixes.md#bnw-spots-3-and-cells-1-pattern-sources) |
| [noise_bnw_spots_3_v2.sbs/bnw_spots_3](../assets/sbs/noise_bnw_spots_3_v2.sbs) | 0 | No | [In progress · 106,559/107,776 pixels, 13 execution-limit cases](parity-fixes.md#bnw-spots-3-and-cells-1-pattern-sources) |
| [noise_cells_1.sbs/cells_1](../assets/sbs/noise_cells_1.sbs) | 0 | Yes | [In progress · 143,321/153,856 pixels in 19 renders](parity-fixes.md#bnw-spots-3-and-cells-1-pattern-sources) |
| [noise_cells_1_v2.sbs/cells_1](../assets/sbs/noise_cells_1_v2.sbs) | 0 | No | [In progress · 20 renders, 158,728/173,312 exact pixels](parity-fixes.md#wire-presence-inside-a-called-function) |
| [noise_cells_4.sbs/cells_4](../assets/sbs/noise_cells_4.sbs) | 1 | Yes | [In progress · 215,500/215,552 pixels in 29 renders](parity-fixes.md#paths-caustics-and-cell-fields-after-pattern-sampling) |
| [noise_cells_4_v2.sbs/cells_4](../assets/sbs/noise_cells_4_v2.sbs) | 1 | No | [In progress · 215,516/215,552 pixels in 29 renders](parity-fixes.md#paths-caustics-and-cell-fields-after-pattern-sampling) |
| [noise_clouds_1.sbs/clouds_1](../assets/sbs/noise_clouds_1.sbs) | 0 | Yes | [In progress · 143,752/149,760 pixels, two execution-limit cases](parity-fixes.md#cloud-and-crystal-pattern-sources-across-scales) |
| [noise_clouds_1_v2.sbs/clouds_1](../assets/sbs/noise_clouds_1_v2.sbs) | 0 | No | [In progress · 135,187/141,568 pixels, seven execution-limit cases](parity-fixes.md#cloud-and-crystal-pattern-sources-across-scales) |
| [noise_clouds_2.sbs/clouds_2](../assets/sbs/noise_clouds_2.sbs) | 0 | Yes | [In progress · 152,152/153,856 pixels in 19 renders](parity-fixes.md#cloud-and-crystal-pattern-sources-across-scales) |
| [noise_clouds_2_v2.sbs/clouds_2](../assets/sbs/noise_clouds_2_v2.sbs) | 0 | No | [In progress · 136,706/141,568 pixels, seven execution-limit cases](parity-fixes.md#cloud-and-crystal-pattern-sources-across-scales) |
| [noise_creased.sbs/creased](../assets/sbs/noise_creased.sbs) | 0 | No | [In progress · 37,716/132,096 pixels across 9 renders](parity-fixes.md#directional-and-additive-stamp-profiles) |
| [noise_crystal_1.sbs/crystal_1](../assets/sbs/noise_crystal_1.sbs) | 0 | No | [In progress · 151,000/153,856 pixels in 19 renders](parity-fixes.md#cloud-and-crystal-pattern-sources-across-scales) |
| [noise_crystal_2.sbs/crystal_2](../assets/sbs/noise_crystal_2.sbs) | 0 | No | [In progress · 152,713/153,856 pixels in 19 renders](parity-fixes.md#cloud-and-crystal-pattern-sources-across-scales) |
| [noise_directional_noise_1.sbs/directional_noise_1](../assets/sbs/noise_directional_noise_1.sbs) | 0 | Yes | [In progress · 136,222/144,640 pixels in 19 comparable renders](parity-fixes.md#directional-noise-and-scratch-pattern-sources) |
| [noise_directional_noise_1_v2.sbs/directional_noise_1](../assets/sbs/noise_directional_noise_1_v2.sbs) | 0 | No | [In progress · 133,251/142,592 pixels in 18 comparable renders](parity-fixes.md#directional-noise-and-scratch-pattern-sources) |
| [noise_directional_noise_2.sbs/directional_noise_2](../assets/sbs/noise_directional_noise_2.sbs) | 0 | Yes | [In progress · 103,497/107,776 pixels in 11 comparable renders](parity-fixes.md#directional-noise-and-scratch-pattern-sources) |
| [noise_directional_noise_2_v2.sbs/directional_noise_2](../assets/sbs/noise_directional_noise_2_v2.sbs) | 0 | No | [In progress · 105,116/107,776 pixels in 11 comparable renders](parity-fixes.md#directional-noise-and-scratch-pattern-sources) |
| [noise_directional_noise_3.sbs/directional_noise_3](../assets/sbs/noise_directional_noise_3.sbs) | 0 | Yes | [In progress · 7/19 whole renders completed, 7,168/7,424 exact pixels](parity-fixes.md#discarding-pattern-stamps-that-miss-every-output-sample) · [Original stage comparisons](parity-fixes.md#directional-noise-and-scratch-pattern-sources) |
| [noise_directional_noise_3_v2.sbs/directional_noise_3](../assets/sbs/noise_directional_noise_3_v2.sbs) | 0 | No | [In progress · 2/19 whole renders completed, 1,280/1,280 exact pixels](parity-fixes.md#discarding-pattern-stamps-that-miss-every-output-sample) · [Original stage comparisons](parity-fixes.md#directional-noise-and-scratch-pattern-sources) |
| [noise_directional_noise_4.sbs/directional_noise_4](../assets/sbs/noise_directional_noise_4.sbs) | 0 | Yes | [In progress · 134,015/144,640 pixels in 19 comparable renders](parity-fixes.md#directional-noise-and-scratch-pattern-sources) |
| [noise_directional_noise_4_v2.sbs/directional_noise_4](../assets/sbs/noise_directional_noise_4_v2.sbs) | 0 | No | [In progress · 134,664/144,640 pixels in 19 comparable renders](parity-fixes.md#directional-noise-and-scratch-pattern-sources) |
| [noise_directional_scratches.sbs/directional_scratches](../assets/sbs/noise_directional_scratches.sbs) | 0 | Yes | [In progress · 119,970/144,640 pixels in 19 comparable renders](parity-fixes.md#directional-noise-and-scratch-pattern-sources) |
| [noise_directional_scratches_v2.sbs/directional_scratches](../assets/sbs/noise_directional_scratches_v2.sbs) | 0 | No | [In progress · 137,786/144,640 pixels in 19 comparable renders](parity-fixes.md#directional-noise-and-scratch-pattern-sources) |
| [noise_dirt_1.sbs/dirt_1](../assets/sbs/noise_dirt_1.sbs) | 0 | Yes | [In progress · 11/19 whole outputs, 104,064/107,776 pixels](parity-fixes.md#dirt-noise-sources-and-downstream-stages) |
| [noise_dirt_1_v2.sbs/dirt_1](../assets/sbs/noise_dirt_1_v2.sbs) | 0 | No | [In progress · 11/19 whole outputs, 107,082/107,776 pixels](parity-fixes.md#dirt-noise-sources-and-downstream-stages) |
| [noise_dirt_2.sbs/dirt_2](../assets/sbs/noise_dirt_2.sbs) | 0 | Yes | [In progress · 11/19 whole outputs, 107,184/107,776 pixels](parity-fixes.md#dirt-noise-sources-and-downstream-stages) |
| [noise_dirt_3.sbs/dirt_3](../assets/sbs/noise_dirt_3.sbs) | 0 | Yes | [In progress · 19/19 whole outputs, 135,236/142,592 pixels](parity-fixes.md#dirt-noise-sources-and-downstream-stages) |
| [noise_dirt_3_v2.sbs/dirt_3](../assets/sbs/noise_dirt_3_v2.sbs) | 0 | No | [In progress · 17/19 whole outputs, 136,738/141,568 pixels](parity-fixes.md#dirt-noise-sources-and-downstream-stages) |
| [noise_dirt_4.sbs/dirt_4](../assets/sbs/noise_dirt_4.sbs) | 0 | Yes | [In progress · 10/19 whole renders completed, 89,052/105,728 exact pixels](parity-fixes.md#discarding-pattern-stamps-that-miss-every-output-sample) · [Original stage comparisons](parity-fixes.md#dirt-noise-sources-and-downstream-stages) |
| [noise_dirt_4_v2.sbs/dirt_4](../assets/sbs/noise_dirt_4_v2.sbs) | 0 | No | [In progress · 5/19 whole renders completed, 99,535/99,584 exact pixels](parity-fixes.md#discarding-pattern-stamps-that-miss-every-output-sample) · [Original stage comparisons](parity-fixes.md#dirt-noise-sources-and-downstream-stages) |
| [noise_dirt_5.sbs/dirt_5](../assets/sbs/noise_dirt_5.sbs) | 0 | Yes | [In progress · 11/19 whole outputs, 107,751/107,776 pixels](parity-fixes.md#dirt-noise-sources-and-downstream-stages) |
| [noise_dirt_5_v2.sbs/dirt_5](../assets/sbs/noise_dirt_5_v2.sbs) | 0 | No | [In progress · 11/19 whole outputs, 107,341/107,776 pixels](parity-fixes.md#dirt-noise-sources-and-downstream-stages) |
| [noise_dirt_gradient.sbs/dirt_gradient](../assets/sbs/noise_dirt_gradient.sbs) | 0 | Yes | [In progress · 19/19 whole outputs, 141,554/142,592 pixels](parity-fixes.md#dirt-noise-sources-and-downstream-stages) |
| [noise_dirt_gradient_v2.sbs/dirt_gradient](../assets/sbs/noise_dirt_gradient_v2.sbs) | 0 | No | [In progress · 6/19 whole outputs, 98,423/101,632 pixels](parity-fixes.md#dirt-noise-sources-and-downstream-stages) |
| [noise_fluid.sbs/fluid](../assets/sbs/noise_fluid.sbs) | 0 | No | [In progress · 396,306/424,192 pixels in 24 renders, first difference in the FX-Map](parity-fixes.md#fibers-and-fluid-pattern-stages) |
| [noise_fractal_sum_base.sbs/fractal_sum_base](../assets/sbs/noise_fractal_sum_base.sbs) | 0 | Yes | [Parity · 18 renders, 145,664 exact pixels](parity-fixes.md#stored-grayscale-stops-and-scalar-levels-controls) |
| [noise_fractal_sum_base.sbs/fractal_sum_base_2](../assets/sbs/noise_fractal_sum_base.sbs) | 0 | Yes | [Parity · 18 renders, 145,664 exact pixels](parity-fixes.md#stored-grayscale-stops-and-scalar-levels-controls) |
| [noise_fractal_sum_base_v2.sbs/noise_blend](../assets/sbs/noise_fractal_sum_base_v2.sbs) | 2 | Yes | [Parity · 17 renders, 142,592 exact pixels](parity-fixes.md#box-blur-and-signed-pixel-sizes) |
| [noise_fractal_sum_base_v2.sbs/noise_disorder](../assets/sbs/noise_fractal_sum_base_v2.sbs) | 1 | Yes | [Parity: 4 cases](parity-fixes.md#verified-cases) |
| [noise_fractal_sum_base_v2.sbs/noise_upscale](../assets/sbs/noise_fractal_sum_base_v2.sbs) | 1 | Yes | [Parity · 17 renders, 142,592 exact pixels](parity-fixes.md#tone-curve-sampling-and-saved-handles) |
| [noise_fur_1.sbs/fur_1](../assets/sbs/noise_fur_1.sbs) | 0 | No | [In progress · 122,549/178,432 exact pixels in 21 renders](parity-fixes.md#stored-opacity-across-pattern-depths) |
| [noise_fur_2.sbs/fur_2](../assets/sbs/noise_fur_2.sbs) | 0 | No | [In progress · 130,666/178,432 exact pixels, first difference in FX-Map](parity-fixes.md#fur-pattern-sources-and-downstream-filters) |
| [noise_fur_3.sbs/fur_3](../assets/sbs/noise_fur_3.sbs) | 0 | No | [In progress · 132,982/178,432 exact pixels, first difference in FX-Map](parity-fixes.md#fur-pattern-sources-and-downstream-filters) |
| [noise_gaussian_noise.sbs/gaussian_noise](../assets/sbs/noise_gaussian_noise.sbs) | 0 | Yes | [In progress · 139,833/178,432 exact pixels · first difference in FX-Map](parity-fixes.md#gaussian-pattern-sources-and-repeat-limits) |
| [noise_gaussian_noise_v2.sbs/gaussian_noise](../assets/sbs/noise_gaussian_noise_v2.sbs) | 0 | No | [In progress · 140,152/178,432 exact pixels · first difference in FX-Map](parity-fixes.md#gaussian-pattern-sources-and-repeat-limits) |
| [noise_gaussian_spots_1.sbs/gaussian_spots_1](../assets/sbs/noise_gaussian_spots_1.sbs) | 0 | Yes | [In progress · 493/132,096 pixels across 9 renders](parity-fixes.md#radial-stamp-profiles-and-pattern-generators) |
| [noise_gaussian_spots_1_v2.sbs/gaussian_spots_1](../assets/sbs/noise_gaussian_spots_1_v2.sbs) | 0 | No | [In progress · 2,984/132,096 pixels across 9 renders](parity-fixes.md#radial-stamp-profiles-and-pattern-generators) |
| [noise_gaussian_spots_2.sbs/gaussian_spots_2](../assets/sbs/noise_gaussian_spots_2.sbs) | 0 | Yes | [In progress · 132,670/174,336 exact pixels · 2 additional cases exceeded the traversal limit](parity-fixes.md#gaussian-pattern-sources-and-repeat-limits) |
| [noise_gaussian_spots_2_v2.sbs/gaussian_spots_2](../assets/sbs/noise_gaussian_spots_2_v2.sbs) | 0 | No | [In progress · 136,601/174,336 exact pixels · 2 additional cases exceeded the traversal limit](parity-fixes.md#gaussian-pattern-sources-and-repeat-limits) |
| [noise_liquid.sbs/liquid](../assets/sbs/noise_liquid.sbs) | 0 | No | [In progress · 19/19 whole outputs, 135,824/142,592 exact pixels](parity-fixes.md#liquid-fiber-and-related-noise-sources) |
| [noise_messy_fibers_1.sbs/messy_fibers_1](../assets/sbs/noise_messy_fibers_1.sbs) | 0 | Yes | [In progress · 19/19 whole outputs, 96,519/142,592 exact pixels](parity-fixes.md#liquid-fiber-and-related-noise-sources) |
| [noise_messy_fibers_1_v2.sbs/messy_fibers_1](../assets/sbs/noise_messy_fibers_1_v2.sbs) | 0 | No | [In progress · 19/19 whole outputs, 131,876/142,592 exact pixels](parity-fixes.md#liquid-fiber-and-related-noise-sources) |
| [noise_messy_fibers_2.sbs/messy_fibers_2](../assets/sbs/noise_messy_fibers_2.sbs) | 0 | Yes | [In progress · 19/19 whole outputs, 84,588/142,592 exact pixels](parity-fixes.md#liquid-fiber-and-related-noise-sources) |
| [noise_messy_fibers_2_v2.sbs/messy_fibers_2](../assets/sbs/noise_messy_fibers_2_v2.sbs) | 0 | No | [In progress · 19/19 whole outputs, 130,496/142,592 exact pixels](parity-fixes.md#liquid-fiber-and-related-noise-sources) |
| [noise_messy_fibers_3.sbs/messy_fibers_3](../assets/sbs/noise_messy_fibers_3.sbs) | 0 | Yes | [In progress · 19/19 whole outputs, 123,834/142,592 exact pixels](parity-fixes.md#liquid-fiber-and-related-noise-sources) |
| [noise_messy_fibers_3_v2.sbs/messy_fibers_3](../assets/sbs/noise_messy_fibers_3_v2.sbs) | 0 | No | [In progress · 19/19 whole outputs, 132,391/142,592 exact pixels](parity-fixes.md#liquid-fiber-and-related-noise-sources) |
| [noise_microscope_view.sbs/microscope_view](../assets/sbs/noise_microscope_view.sbs) | 0 | No | [In progress · 19/19 whole outputs, 140,486/142,592 exact pixels](parity-fixes.md#liquid-fiber-and-related-noise-sources) |
| [noise_moisture_noise.sbs/moisture_noise](../assets/sbs/noise_moisture_noise.sbs) | 0 | Yes | [In progress · 11/19 whole outputs, 107,093/107,776 exact pixels](parity-fixes.md#liquid-fiber-and-related-noise-sources) |
| [noise_moisture_noise_1_v2.sbs/moisture_noise_1](../assets/sbs/noise_moisture_noise_1_v2.sbs) | 0 | No | [In progress · 11/19 whole outputs, 104,176/107,776 exact pixels](parity-fixes.md#liquid-fiber-and-related-noise-sources) |
| [noise_moisture_noise_2.sbs/moisture_noise_2](../assets/sbs/noise_moisture_noise_2.sbs) | 0 | No | [In progress · 11/19 whole outputs, 104,071/107,776 exact pixels](parity-fixes.md#liquid-fiber-and-related-noise-sources) |
| [noise_perlin_noise.sbs/perlin_noise](../assets/sbs/noise_perlin_noise.sbs) | 0 | Yes | [In progress · 19/19 whole outputs, 139,281/142,592 exact pixels](parity-fixes.md#liquid-fiber-and-related-noise-sources) |
| [noise_perlin_noise_v2.sbs/perlin_noise](../assets/sbs/noise_perlin_noise_v2.sbs) | 0 | No | [In progress · 19/19 whole outputs, 132,970/142,592 exact pixels](parity-fixes.md#liquid-fiber-and-related-noise-sources) |
| [noise_plasma.sbs/plasma](../assets/sbs/noise_plasma.sbs) | 0 | No | [In progress · 19/19 whole outputs, 139,719/142,592 exact pixels](parity-fixes.md#liquid-fiber-and-related-noise-sources) |
| [noise_upscale_1.sbs/noise_upscale_1](../assets/sbs/noise_upscale_1.sbs) | 1 | No | [Parity · 21 renders, 697,344 exact pixels](parity-fixes.md#sampling-integer-bitmap-inputs-through-wider-bridges) |
| [noise_upscale_2.sbs/noise_upscale_2](../assets/sbs/noise_upscale_2.sbs) | 1 | No | [Parity · 21 renders, 697,344 exact pixels](parity-fixes.md#sampling-integer-bitmap-inputs-through-wider-bridges) |
| [noise_upscale_3.sbs/noise_upscale_3](../assets/sbs/noise_upscale_3.sbs) | 2 | No | [Parity · 21 renders, 697,344 exact pixels](parity-fixes.md#sampling-integer-bitmap-inputs-through-wider-bridges) |
| [noise_voronoi.sbs/3d_voronoi](../assets/sbs/noise_voronoi.sbs) | 1 | No | [In progress · 37 renders, 152,320/155,392 exact pixels with stock and shared tables](parity-fixes.md#voronoi-distortion-and-shared-noise-inputs) |
| [noise_voronoi.sbs/3d_voronoi_fractal](../assets/sbs/noise_voronoi.sbs) | 1 | No | [In progress · 37 renders, 152,163/155,392 exact pixels with stock and shared tables](parity-fixes.md#voronoi-distortion-and-shared-noise-inputs) |
| [noise_voronoi.sbs/voronoi](../assets/sbs/noise_voronoi.sbs) | 0 | No | [In progress · 37 renders, 152,320/155,392 exact pixels with stock and shared tables](parity-fixes.md#voronoi-distortion-and-shared-noise-inputs) |
| [noise_voronoi.sbs/voronoi_fractal](../assets/sbs/noise_voronoi.sbs) | 0 | No | [In progress · 37 renders, 152,320/155,392 exact pixels with stock and shared tables](parity-fixes.md#voronoi-distortion-and-shared-noise-inputs) |
| [non_square_transform.sbs/non_square_transform](../assets/sbs/non_square_transform.sbs) | 1 | No | [Parity · 204,032 exact pixels in 23 renders](parity-fixes.md#rotated-transforms-and-fractional-texture-coordinates) |
| [non_square_transform.sbs/non_square_transform_greyscale](../assets/sbs/non_square_transform.sbs) | 1 | No | [Parity · 204,032 exact pixels in 23 renders](parity-fixes.md#rotated-transforms-and-fractional-texture-coordinates) |
| [non_uniform_blur.sbs/non_uniform_blur_grayscale_sample](../assets/sbs/non_uniform_blur.sbs) | 2 | Yes | [Parity · 19 cases, 175,360 exact pixels](parity-fixes.md#directional-warp-orientation-and-input-precision) |
| [non_uniform_blur.sbs/non_uniform_blur_sample](../assets/sbs/non_uniform_blur.sbs) | 2 | Yes | [Parity · 19 cases, 175,360 exact pixels](parity-fixes.md#directional-warp-orientation-and-input-precision) |
| [non_uniform_rotation.sbs/non_uniform_rotation_color](../assets/sbs/non_uniform_rotation.sbs) | 3 | No | [Parity · 206,336 exact pixels in 26 renders](parity-fixes.md#segment-mapping-and-normal-graph-rechecks) |
| [non_uniform_rotation.sbs/non_uniform_rotation_grayscale](../assets/sbs/non_uniform_rotation.sbs) | 3 | No | [Parity · 171,264 exact pixels in 18 renders](parity-fixes.md#segment-mapping-and-normal-graph-rechecks) |
| [normal_color.sbs/normal_color](../assets/sbs/normal_color.sbs) | 0 | No | [Parity: 4 cases](parity-fixes.md#verified-cases) |
| [normal_intensity.sbs/normal_intensity](../assets/sbs/normal_intensity.sbs) | 1 | No | [Parity: 4 cases](parity-fixes.md#verified-cases) |
| [normal_invert.sbs/normal_invert](../assets/sbs/normal_invert.sbs) | 1 | No | [Parity: 4 cases](parity-fixes.md#verified-cases) |
| [normal_mapper.sbs/normal_mapper](../assets/sbs/normal_mapper.sbs) | 1 | Yes | [Parity: 4 cases](parity-fixes.md#verified-cases) |
| [normal_normalise.sbs/normal_normalise](../assets/sbs/normal_normalise.sbs) | 1 | No | [Parity · 18 renders, all 171,264 pixels exact](parity-fixes.md#normal-maps-panoramas-and-primitive-shapes) |
| [normal_sobel.sbs/normal_sobel](../assets/sbs/normal_sobel.sbs) | 1 | Yes | [Parity · 27 renders, all 213,504 pixels exact](parity-fixes.md#sobel-normals-and-nadir-projection) |
| [normal_sobel.sbs/normal_sobel_2](../assets/sbs/normal_sobel.sbs) | 1 | No | [Parity · 171,264 exact pixels in 18 renders](parity-fixes.md#segment-mapping-and-normal-graph-rechecks) |
| [normal_to_height_2.sbs/normal_to_height](../assets/sbs/normal_to_height_2.sbs) | 1 | No | [Parity · 18 renders, all 145,664 pixels exact](parity-fixes.md#global-blend-opacity-in-normal-to-height) |
| [normal_to_height_hq.sbs/base_algorithm_height_distance](../assets/sbs/normal_to_height_hq.sbs) | 1 | Yes | [Parity · 13 cases, 163,840 exact pixels](parity-fixes.md#height-distance-texture-mapping-skew-and-weave) |
| [normal_transform.sbs/normal_transform](../assets/sbs/normal_transform.sbs) | 1 | No | [Parity · 18 renders, all 171,264 pixels exact](parity-fixes.md#normal-maps-panoramas-and-primitive-shapes) |
| [normal_uncombine.sbs/get_intensity_iteration](../assets/sbs/normal_uncombine.sbs) | 2 | Yes | [Parity: 4 cases](parity-fixes.md#verified-cases) |
| [normal_vector_rotation.sbs/normal_vector_rotation](../assets/sbs/normal_vector_rotation.sbs) | 2 | No | [Parity: 4 cases](parity-fixes.md#verified-cases) |
| [panorama_gradients.sbs/panorama_gradient_2_points](../assets/sbs/panorama_gradients.sbs) | 0 | No | [Parity · 19 renders, all 143,104 RGBA pixels exact](parity-fixes.md#scalar-products-used-by-sums-inside-a-called-function) |
| [panorama_gradients.sbs/panorama_gradient_linear](../assets/sbs/panorama_gradients.sbs) | 0 | No | [Parity · 19 renders, all 143,104 RGBA pixels exact](parity-fixes.md#scalar-products-used-by-sums-inside-a-called-function) |
| [panorama_nadir_extract.sbs/nadir_extract](../assets/sbs/panorama_nadir_extract.sbs) | 1 | No | [Parity · 18 renders, all 174,336 pixels exact](parity-fixes.md#sobel-normals-and-nadir-projection) |
| [panorama_nadir_patch.sbs/panorama_nadir_patch](../assets/sbs/panorama_nadir_patch.sbs) | 2 | No | [In progress · 17 renders, 172,780/173,312 exact pixels](parity-fixes.md#sobel-normals-and-nadir-projection) |
| [panorama_physical_sun_sky.sbs/Impl/GetOneParam](../assets/sbs/panorama_physical_sun_sky.sbs) | 1 | Yes | [Parity · 25 exact float3 outputs](parity-fixes.md#color-temperature-range-reductions-sky-parameters-and-polygon-paths) |
| [panorama_position.sbs/panorama_3d_position](../assets/sbs/panorama_position.sbs) | 0 | No | [Parity · 18 renders, all 171,264 pixels exact](parity-fixes.md#normal-maps-panoramas-and-primitive-shapes) |
| [panorama_rotation.sbs/panorama_rotation](../assets/sbs/panorama_rotation.sbs) | 1 | No | [Parity · 206,336 exact pixels in 26 renders](parity-fixes.md#segment-mapping-and-normal-graph-rechecks) |
| [panorama_shape.sbs/panorama_line_light](../assets/sbs/panorama_shape.sbs) | 3 | No | [Parity · 30 renders, all 241,152 pixels exact](parity-fixes.md#panoramic-line-light-and-horizon-across-the-seam) |
| [panorama_shape.sbs/panorama_plane_light](../assets/sbs/panorama_shape.sbs) | 3 | No | [In progress · 22 renders, 199,155/199,936 exact pixels](parity-fixes.md#panoramic-line-light-and-horizon-across-the-seam) |
| [panorama_shape.sbs/panorama_shape](../assets/sbs/panorama_shape.sbs) | 1 | No | [Parity · 18 renders, all 171,264 pixels exact](parity-fixes.md#normal-maps-panoramas-and-primitive-shapes) |
| [panorama_shape.sbs/panorama_shape_2](../assets/sbs/panorama_shape.sbs) | 2 | No | [Parity · 18 renders, all 171,264 pixels exact](parity-fixes.md#normal-maps-panoramas-and-primitive-shapes) |
| [panorama_shape.sbs/panorama_sphere_light](../assets/sbs/panorama_shape.sbs) | 2 | No | [Parity · 171,264 exact pixels in 18 renders](parity-fixes.md#segment-mapping-and-normal-graph-rechecks) |
| [panorama_shape.sbs/panorama_spot](../assets/sbs/panorama_shape.sbs) | 1 | Yes | [Parity · 36 renders, all 189,696 RGBA pixels exact](parity-fixes.md#undefined-quotients-in-a-unit-interval-and-panorama-spot-lighting) |
| [panorama_straighten_horizon.sbs/panorama_straighten_horizon](../assets/sbs/panorama_straighten_horizon.sbs) | 1 | No | [Parity · 26 renders, all 212,480 pixels exact](parity-fixes.md#panoramic-line-light-and-horizon-across-the-seam) |
| [paper_path.sbs/paper_path](../assets/sbs/paper_path.sbs) | 0 | Yes | [In progress · 135,674/142,592 exact pixels in 19 whole renders](parity-fixes.md#paper-brick-fiber-mesh-polygon-and-shape-roots) |
| [parquet_1.sbs/parquet_1](../assets/sbs/parquet_1.sbs) | 0 | Yes | [Parity · 25 renders, 289,024 exact pixels](parity-fixes.md#parquet-1-and-outstanding-inventory-controls) |
| [paths_tools.sbs/Impl/compute_paths_lengths](../assets/sbs/paths_tools.sbs) | 1 | Yes | [In progress · 1,029 exact pixels, intermediate lengths reach the execution limit](parity-fixes.md#skipped-loops-and-path-buffers) |
| [paths_tools.sbs/Impl/facets_to_paths_impl_link_template](../assets/sbs/paths_tools.sbs) | 1 | Yes | [Parity · 18 renders, all 57,600 pixels exact](parity-fixes.md#marked-variable-reads-in-path-and-mipmap-graphs) |
| [paths_tools.sbs/Impl/facets_to_paths_impl_stitch_template](../assets/sbs/paths_tools.sbs) | 1 | Yes | [Parity · 18 renders, all 57,600 pixels exact](parity-fixes.md#marked-variable-reads-in-path-and-mipmap-graphs) |
| [paths_tools.sbs/Impl/get_number_of_valid_paths](../assets/sbs/paths_tools.sbs) | 1 | Yes | [Parity: 4 cases](parity-fixes.md#verified-cases) |
| [paths_tools.sbs/Impl/mask_to_facets](../assets/sbs/paths_tools.sbs) | 1 | Yes | [Parity · 17 cases, 52,736 exact pixels](parity-fixes.md#facet-masks-and-stripe-boundaries) |
| [paths_tools.sbs/Impl/sort_by_blue](../assets/sbs/paths_tools.sbs) | 1 | Yes | [Parity · 14 cases, 25,856 exact pixels](parity-fixes.md#skipped-loops-and-path-buffers) |
| [paths_tools.sbs/Utils/RelativeToInputPP](../assets/sbs/paths_tools.sbs) | 1 | Yes | [Parity: 4 cases](parity-fixes.md#verified-cases) |
| [paths_tools.sbs/paths_polygon](../assets/sbs/paths_tools.sbs) | 0 | No | [Parity · 19 polygon buffers, 5,120 exact pixels](parity-fixes.md#color-temperature-range-reductions-sky-parameters-and-polygon-paths) |
| [paths_tools.sbs/paths_trace](../assets/sbs/paths_tools.sbs) | 2 | No | [In progress · 58,213/58,624 pixels in 19 renders](parity-fixes.md#paths-caustics-and-cell-fields-after-pattern-sampling) |
| [paths_tools.sbs/vertex_processor](../assets/sbs/paths_tools.sbs) | 9 | No | [Parity · 18 renders, all 57,600 pixels exact](parity-fixes.md#marked-variable-reads-in-path-and-mipmap-graphs) |
| [paths_tools.sbs/vertex_processor_simple](../assets/sbs/paths_tools.sbs) | 5 | No | [Parity · 18 renders, all 57,600 pixels exact](parity-fixes.md#marked-variable-reads-in-path-and-mipmap-graphs) |
| [pattern_alveolus.sbs/alveolus](../assets/sbs/pattern_alveolus.sbs) | 0 | No | [In progress · 201,876/203,264 pixels across 33 renders](parity-fixes.md#square-and-rectangular-pattern-generators) · [Single-stamp render-region comparisons](parity-fixes.md#pattern-coordinates-under-a-change-of-render-region) · [Gaussian disc-boundary sweeps](parity-fixes.md#gaussian-coverage-at-the-disc-boundary) · [262,144/262,144 pixels after isolating the generator](parity-fixes.md#alveolus-filters-after-the-first-differing-stage) |
| [pattern_arc_pavement.sbs/arc_pavement](../assets/sbs/pattern_arc_pavement.sbs) | 1 | No | [In progress · 206,911/211,200 pixels across 27 renders](parity-fixes.md#pattern-rotation-phase) |
| [pattern_brick_1.sbs/brick_1](../assets/sbs/pattern_brick_1.sbs) | 0 | No | [In progress · 139,565/142,592 exact pixels in 19 whole renders](parity-fixes.md#paper-brick-fiber-mesh-polygon-and-shape-roots) |
| [pattern_brick_2.sbs/brick_2](../assets/sbs/pattern_brick_2.sbs) | 0 | No | [In progress · 139,074/142,592 exact pixels in 19 whole renders](parity-fixes.md#paper-brick-fiber-mesh-polygon-and-shape-roots) |
| [pattern_brick_generator.sbs/resources/Tile](../assets/sbs/pattern_brick_generator.sbs) | 1 | Yes | [In progress · 149,402/150,528 pixels across 74 renders](parity-fixes.md#the-lower-edge-of-an-axis-aligned-stamp) |
| [pattern_checker_1.sbs/checker_1](../assets/sbs/pattern_checker_1.sbs) | 0 | No | [Parity: 4 cases](parity-fixes.md#verified-cases) |
| [pattern_cube_3d.sbs/cube_3d](../assets/sbs/pattern_cube_3d.sbs) | 0 | No | [Parity · 18 renders, all 171,264 pixels exact](parity-fixes.md#normal-maps-panoramas-and-primitive-shapes) |
| [pattern_cube_3d.sbs/cube_3d_gbuffers](../assets/sbs/pattern_cube_3d.sbs) | 0 | No | [Parity · 60 renders, all 685,056 pixels exact](parity-fixes.md#normal-maps-panoramas-and-primitive-shapes) |
| [pattern_fibers_1.sbs/fibers_1](../assets/sbs/pattern_fibers_1.sbs) | 0 | No | [In progress · 416,799/424,192 pixels across 24 renders](parity-fixes.md#pattern-rotation-phase) |
| [pattern_fibers_2.sbs/fibers_2](../assets/sbs/pattern_fibers_2.sbs) | 0 | No | [In progress · 139,312/142,592 exact pixels in 19 whole renders](parity-fixes.md#paper-brick-fiber-mesh-polygon-and-shape-roots) |
| [pattern_gaussian_1.sbs/gaussian_1](../assets/sbs/pattern_gaussian_1.sbs) | 0 | No | [In progress · 168,522/171,264 pixels across 18 renders](parity-fixes.md#radial-stamp-profiles-and-pattern-generators) |
| [pattern_gaussian_2.sbs/gaussian_2](../assets/sbs/pattern_gaussian_2.sbs) | 0 | No | [In progress · 169,796/171,264 pixels across 18 renders](parity-fixes.md#radial-stamp-profiles-and-pattern-generators) |
| [pattern_mesh_1.sbs/mesh_1](../assets/sbs/pattern_mesh_1.sbs) | 0 | No | [In progress · 41,087/142,592 exact pixels in 19 whole renders](parity-fixes.md#paper-brick-fiber-mesh-polygon-and-shape-roots) |
| [pattern_mesh_2.sbs/mesh_2](../assets/sbs/pattern_mesh_2.sbs) | 0 | No | [In progress · 39,574/142,592 exact pixels in 19 whole renders](parity-fixes.md#paper-brick-fiber-mesh-polygon-and-shape-roots) |
| [pattern_polygon_2.sbs/polygon_2](../assets/sbs/pattern_polygon_2.sbs) | 0 | No | [In progress · 141,734/142,592 exact pixels in 19 whole renders](parity-fixes.md#paper-brick-fiber-mesh-polygon-and-shape-roots) |
| [pattern_shape.sbs/hemisphere](../assets/sbs/pattern_shape.sbs) | 0 | No | [Parity · 18 renders, all 171,264 pixels exact](parity-fixes.md#normal-maps-panoramas-and-primitive-shapes) |
| [pattern_shape.sbs/shape_base](../assets/sbs/pattern_shape.sbs) | 0 | Yes | [In progress · 139,876/142,592 exact pixels in 19 whole renders](parity-fixes.md#paper-brick-fiber-mesh-polygon-and-shape-roots) |
| [pattern_splatter_circular.sbs/splatter_circular](../assets/sbs/pattern_splatter_circular.sbs) | 7 | No | [In progress · 130,917/132,096 exact pixels in 9 renders](parity-fixes.md#other-roots-using-uniform-arc-tangents) |
| [pattern_splatter_circular.sbs/splatter_circular_color](../assets/sbs/pattern_splatter_circular.sbs) | 7 | No | [In progress · 130,678/132,096 exact pixels in 9 renders](parity-fixes.md#other-roots-using-uniform-arc-tangents) |
| [pattern_stripes.sbs/stripes](../assets/sbs/pattern_stripes.sbs) | 0 | No | [In progress · 215,036/215,040 exact pixels in 18 renders](parity-fixes.md#other-roots-using-uniform-arc-tangents) |
| [pattern_tile_generator.sbs/tile_generator](../assets/sbs/pattern_tile_generator.sbs) | 7 | No | [In progress · 217,222/226,048 pixels, stamp placement and waveform rounding differ](parity-fixes.md#beveled-square-gradient-and-wave-stamps) |
| [pattern_tile_generator.sbs/tile_generator_color](../assets/sbs/pattern_tile_generator.sbs) | 7 | No | [In progress · 137,997/142,592 whole pixels, first differing stages measured](parity-fixes.md#tile-weave-splatter-and-fiber-stage-comparisons) |
| [pattern_tile_random.sbs/tile_random](../assets/sbs/pattern_tile_random.sbs) | 2 | No | [In progress · 238,215/243,968 exact pixels across 32 renders](parity-fixes.md#tile-random-image-and-soft-patterns) |
| [pattern_tile_random.sbs/tile_random_color](../assets/sbs/pattern_tile_random.sbs) | 2 | No | [In progress · 268,904/276,736 exact pixels in 40 renders](parity-fixes.md#signed-bitmap-blend-precision) |
| [pattern_tile_random_2.sbs/functions/debug_find_cell](../assets/sbs/pattern_tile_random_2.sbs) | 0 | Yes | [Parity: 4 cases](parity-fixes.md#verified-cases) |
| [pattern_tile_random_2.sbs/tile_random_2](../assets/sbs/pattern_tile_random_2.sbs) | 5 | No | [Parity · 285,184 exact pixels in 38 renders](parity-fixes.md#shared-lookups-and-environment-shadow-pass) |
| [pattern_tile_sampler.sbs/arrow](../assets/sbs/pattern_tile_sampler.sbs) | 0 | Yes | [Parity · seven 512×512 runs, four formats](parity-fixes.md#transform-edges-and-floating-point-filtering) |
| [pattern_triangle_grid.sbs/triangle_grid_base](../assets/sbs/pattern_triangle_grid.sbs) | 3 | Yes | [Parity · 18 renders, all 171,264 pixels exact](parity-fixes.md#high-pass-anisotropic-noise-and-triangle-grid-controls) |
| [pattern_weave_1.sbs/weave_1](../assets/sbs/pattern_weave_1.sbs) | 0 | No | [In progress · 139,840/142,592 whole pixels, first differing stages measured](parity-fixes.md#tile-weave-splatter-and-fiber-stage-comparisons) |
| [pattern_weave_2.sbs/weave_2](../assets/sbs/pattern_weave_2.sbs) | 0 | No | [In progress · 140,765/142,592 whole pixels, first differing stages measured](parity-fixes.md#tile-weave-splatter-and-fiber-stage-comparisons) |
| [pavement_1.sbs/pavement_1](../assets/sbs/pavement_1.sbs) | 0 | Yes | [In progress · 141,408/142,592 whole pixels, first differing stages measured](parity-fixes.md#tile-weave-splatter-and-fiber-stage-comparisons) |
| [pbr_albedo_safe_color.sbs/pbr_albedo_safe_color](../assets/sbs/pbr_albedo_safe_color.sbs) | 2 | No | [Parity: 4 cases](parity-fixes.md#verified-cases) |
| [pbr_dielectric_F0.sbs/pbr_dielectric_F0](../assets/sbs/pbr_dielectric_F0.sbs) | 0 | No | [Parity: 4 cases](parity-fixes.md#verified-cases) |
| [pbr_metal_reflectance.sbs/pbr_metal_reflectance](../assets/sbs/pbr_metal_reflectance.sbs) | 0 | No | [Parity: 4 cases](parity-fixes.md#verified-cases) |
| [pbr_render.sbs/PBR_render_mapping_color](../assets/sbs/pbr_render.sbs) | 2 | Yes | [Parity · 13 cases, 163,840 exact pixels](parity-fixes.md#height-distance-texture-mapping-skew-and-weave) |
| [pbr_render.sbs/PBR_render_mapping_grayscale](../assets/sbs/pbr_render.sbs) | 2 | Yes | [Parity · 13 cases, 163,840 exact pixels](parity-fixes.md#height-distance-texture-mapping-skew-and-weave) |
| [pbr_render.sbs/chroma_shift](../assets/sbs/pbr_render.sbs) | 1 | Yes | [Parity · 13 cases, 163,840 exact pixels](parity-fixes.md#height-distance-texture-mapping-skew-and-weave) |
| [pbr_render.sbs/environment_map_rescale](../assets/sbs/pbr_render.sbs) | 1 | Yes | [Parity · 171,264 exact pixels in 18 renders](parity-fixes.md#centered-transform-resizing-and-untiled-edges) |
| [pbr_render.sbs/get_max_coords/get_max_coord](../assets/sbs/pbr_render.sbs) | 1 | Yes | [Parity · 12 typed coordinate comparisons](parity-fixes.md#value-processor-coordinates) |
| [pbr_render.sbs/ibl_specular_contrib_anisotropic/panoramic_lod](../assets/sbs/pbr_render.sbs) | 1 | Yes | [Parity · 18 cases, 186,624 exact pixels](parity-fixes.md#stored-precision-in-transform-mip-levels) |
| [pbr_render.sbs/position_to_normal](../assets/sbs/pbr_render.sbs) | 2 | Yes | [Parity · 30 renders, all 515,328 RGBA pixels exact](parity-fixes.md#position-to-normal-outputs-in-both-renderers) |
| [pbr_render.sbs/raytrace_position](../assets/sbs/pbr_render.sbs) | 4 | Yes | [Parity · 26 renders, all 142,340 RGBA pixels exact](parity-fixes.md#small-graph-parents-and-raytrace-positions) |
| [pbr_render.sbs/raytraced_irradiance_pass_8](../assets/sbs/pbr_render.sbs) | 10 | Yes | [In progress · 124,769/124,928 pixels and 26/26 wired or default pass indices](parity-fixes.md#irradiance-with-a-connected-pass-index) |
| [pbr_render.sbs/raytraced_specular_pass_8](../assets/sbs/pbr_render.sbs) | 9 | Yes | [In progress · 142,061/142,340 pixels, all U8 and U16 exact](parity-fixes.md#eighth-raytraced-specular-passes) |
| [pbr_render2.sbs/PBR_render_mapping_color](../assets/sbs/pbr_render2.sbs) | 2 | No | [Parity · 13 cases, 163,840 exact pixels](parity-fixes.md#height-distance-texture-mapping-skew-and-weave) |
| [pbr_render2.sbs/PBR_render_mapping_grayscale](../assets/sbs/pbr_render2.sbs) | 2 | No | [Parity · 13 cases, 163,840 exact pixels](parity-fixes.md#height-distance-texture-mapping-skew-and-weave) |
| [pbr_render2.sbs/chroma_shift](../assets/sbs/pbr_render2.sbs) | 1 | Yes | [Parity · 13 cases, 163,840 exact pixels](parity-fixes.md#height-distance-texture-mapping-skew-and-weave) |
| [pbr_render2.sbs/environment_map_rescale](../assets/sbs/pbr_render2.sbs) | 1 | Yes | [Parity · 171,264 exact pixels in 18 renders](parity-fixes.md#centered-transform-resizing-and-untiled-edges) |
| [pbr_render2.sbs/get_max_coords/get_max_coord](../assets/sbs/pbr_render2.sbs) | 1 | Yes | [Parity · 12 typed coordinate comparisons](parity-fixes.md#value-processor-coordinates) |
| [pbr_render2.sbs/height_adjust](../assets/sbs/pbr_render2.sbs) | 3 | Yes | [Parity: 3 cases](parity-fixes.md#verified-cases) |
| [pbr_render2.sbs/ibl_specular_contrib_anisotropic/panoramic_lod](../assets/sbs/pbr_render2.sbs) | 1 | Yes | [Parity · 18 cases, 186,624 exact pixels](parity-fixes.md#large-transform-offsets-and-additive-blends) |
| [pbr_render2.sbs/lod](../assets/sbs/pbr_render2.sbs) | 1 | Yes | [Parity · 18 cases, 171,264 exact pixels](parity-fixes.md#stored-precision-in-transform-mip-levels) |
| [pbr_render2.sbs/position_to_normal](../assets/sbs/pbr_render2.sbs) | 2 | Yes | [Parity · 30 renders, all 515,328 RGBA pixels exact](parity-fixes.md#position-to-normal-outputs-in-both-renderers) |
| [pbr_render2.sbs/raytrace_position](../assets/sbs/pbr_render2.sbs) | 4 | Yes | [Parity · 26 renders, all 142,340 RGBA pixels exact](parity-fixes.md#small-graph-parents-and-raytrace-positions) |
| [pbr_render2.sbs/raytraced_irradiance_is_pass_8](../assets/sbs/pbr_render2.sbs) | 9 | Yes | [Parity · 18 renders, 136,704 exact RGBA pixels](parity-fixes.md#material-bundles-and-the-eighth-irradiance-pass) |
| [pbr_render2.sbs/raytraced_specular_pass_8](../assets/sbs/pbr_render2.sbs) | 9 | Yes | [In progress · 142,337/142,340 pixels, three F16 differences](parity-fixes.md#eighth-raytraced-specular-passes) |
| [pbr_render2.sbs/sat/sat_iteration](../assets/sbs/pbr_render2.sbs) | 1 | Yes | [Parity: 4 cases](parity-fixes.md#verified-cases) |
| [pbr_render_ortho.sbs/blue_noise](../assets/sbs/pbr_render_ortho.sbs) | 0 | Yes | [Parity · 285,184 exact pixels in 38 renders](parity-fixes.md#shared-lookups-and-environment-shadow-pass) |
| [pbr_render_ortho.sbs/environment_map_rescale](../assets/sbs/pbr_render_ortho.sbs) | 1 | Yes | [Parity · 171,264 exact pixels in 18 renders](parity-fixes.md#centered-transform-resizing-and-untiled-edges) |
| [pbr_render_ortho.sbs/get_max_coord](../assets/sbs/pbr_render_ortho.sbs) | 1 | Yes | [Parity: 4 cases](parity-fixes.md#verified-cases) |
| [pbr_render_ortho.sbs/ibl_shadow_pass](../assets/sbs/pbr_render_ortho.sbs) | 3 | Yes | [Parity · 151,040 exact pixels in 27 renders](parity-fixes.md#shared-lookups-and-environment-shadow-pass) |
| [pbr_render_ortho.sbs/mipmap_generator](../assets/sbs/pbr_render_ortho.sbs) | 1 | Yes | [Parity · 171,264 exact pixels in 18 renders](parity-fixes.md#centered-transform-resizing-and-untiled-edges) |
| [pbr_render_ortho.sbs/rotate_environment](../assets/sbs/pbr_render_ortho.sbs) | 1 | Yes | [Parity · 18 renders, all 171,264 RGBA pixels exact](parity-fixes.md#environment-rotation-and-wired-irradiance-samples) |
| [pbr_render_ortho.sbs/sphere_displace](../assets/sbs/pbr_render_ortho.sbs) | 1 | Yes | [In progress · 142,163/142,592 whole pixels, first differing stages measured](parity-fixes.md#tile-weave-splatter-and-fiber-stage-comparisons) |
| [pbr_render_ortho.sbs/vector_shadow_pass](../assets/sbs/pbr_render_ortho.sbs) | 4 | Yes | [Parity · 28 cases, 211,456 exact pixels](parity-fixes.md#large-transform-offsets-and-additive-blends) |
| [perlin_noise_zoom.sbs/perlin_noise_zoom](../assets/sbs/perlin_noise_zoom.sbs) | 0 | Yes | [In progress · 131,983/142,592 whole pixels, first differing stages measured](parity-fixes.md#tile-weave-splatter-and-fiber-stage-comparisons) |
| [pow.sbs/pow](../assets/sbs/pow.sbs) | 1 | No | [Parity: 4 cases](parity-fixes.md#verified-cases) |
| [pow.sbs/pow_grayscale](../assets/sbs/pow.sbs) | 1 | No | [Parity: 4 cases](parity-fixes.md#verified-cases) |
| [pre_multiply_straight.sbs/premult_to_straight](../assets/sbs/pre_multiply_straight.sbs) | 1 | No | [Parity · 19 cases, 172,288 exact pixels](parity-fixes.md#large-transform-offsets-and-additive-blends) |
| [quad_transform.sbs/quad_transform](../assets/sbs/quad_transform.sbs) | 1 | No | [Parity · 204,032/204,032 pixels in 23 renders](parity-fixes.md#quad-swirl-trapezoid-and-vector-morph-comparisons) |
| [quad_transform.sbs/quad_transform_grayscale](../assets/sbs/quad_transform.sbs) | 1 | No | [Parity · 204,032/204,032 pixels in 23 renders](parity-fixes.md#quad-swirl-trapezoid-and-vector-morph-comparisons) |
| [quantize.sbs/quantize](../assets/sbs/quantize.sbs) | 1 | No | [Parity: 4 cases](parity-fixes.md#verified-cases) |
| [quantize.sbs/quantize_grayscale](../assets/sbs/quantize.sbs) | 1 | Yes | [Parity: 4 cases](parity-fixes.md#verified-cases) |
| [quantize_2.sbs/quantize_grayscale](../assets/sbs/quantize_2.sbs) | 2 | No | [Parity: 4 cases](parity-fixes.md#verified-cases) |
| [quantize_color.sbs/apply_color_palette_sub](../assets/sbs/quantize_color.sbs) | 2 | Yes | [Parity: 4 cases](parity-fixes.md#verified-cases) |
| [quantize_color.sbs/bitonic_sort/graph/bitonic_sort_iteration](../assets/sbs/quantize_color.sbs) | 1 | Yes | [Parity: 4 cases](parity-fixes.md#verified-cases) |
| [quantize_color.sbs/bitonic_sort/graph/initial_iteration](../assets/sbs/quantize_color.sbs) | 1 | Yes | [Parity: 4 cases](parity-fixes.md#verified-cases) |
| [quantize_color.sbs/bitonic_sort/graph/remap_sorting](../assets/sbs/quantize_color.sbs) | 2 | Yes | [Parity: 4 cases](parity-fixes.md#verified-cases) |
| [quantize_color.sbs/bitonic_sort/graph/remap_sorting_256x1](../assets/sbs/quantize_color.sbs) | 2 | Yes | [Parity: 4 cases](parity-fixes.md#verified-cases) |
| [quantize_color.sbs/color_conversions/convert_Lab_to_RGB](../assets/sbs/quantize_color.sbs) | 1 | Yes | [Parity · 180,736 exact pixels, including after MaterialCraft Save As and external reopen](parity-fixes.md#vector-equality-lab-to-rgb-and-sbs-round-trips) |
| [quantize_color.sbs/color_conversions/convert_RGB_to_Lab](../assets/sbs/quantize_color.sbs) | 1 | Yes | [Parity · 29 renders, all 212,480 RGBA pixels exact](parity-fixes.md#literal-pixel-divisors-and-rgb-to-lab) |
| [quantize_color.sbs/create_color_palette_16colors](../assets/sbs/quantize_color.sbs) | 2 | No | [Parity: 4 cases](parity-fixes.md#verified-cases) |
| [quantize_color.sbs/get_min_max/GetMaxFromAlpha_v2](../assets/sbs/quantize_color.sbs) | 1 | Yes | [Parity · 33 renders, all 36 output pixels exact](parity-fixes.md#alpha-maximum-and-one-texel-reductions) |
| [quantize_color.sbs/get_min_max/GetMaxFromAlpha_v3](../assets/sbs/quantize_color.sbs) | 1 | Yes | [Parity: 4 cases](parity-fixes.md#verified-cases) |
| [quantize_color.sbs/helpers/cdf_pass](../assets/sbs/quantize_color.sbs) | 1 | Yes | [Parity: 4 cases](parity-fixes.md#verified-cases) |
| [quantize_color.sbs/kmeans_iteration](../assets/sbs/quantize_color.sbs) | 2 | Yes | [In progress · 77,776/77,824 whole pixels, first differing stages measured](parity-fixes.md#tile-weave-splatter-and-fiber-stage-comparisons) |
| [quantize_color.sbs/modify_color_palette_sub](../assets/sbs/quantize_color.sbs) | 2 | Yes | [In progress · two of 256 palette entries differ at F32](parity-fixes.md#height-distance-texture-mapping-skew-and-weave) |
| [quantize_color.sbs/view_color_palette_sub](../assets/sbs/quantize_color.sbs) | 2 | Yes | [Parity: 4 cases](parity-fixes.md#verified-cases) |
| [radial_blur.sbs/radial_blur_grayscale](../assets/sbs/radial_blur.sbs) | 1 | No | [In progress · 121,514/142,592 whole pixels, first differing stages measured](parity-fixes.md#tile-weave-splatter-and-fiber-stage-comparisons) · [both isolated branches match in all six cases per shape](parity-fixes.md#tile-weave-splatter-and-fiber-stage-comparisons) |
| [replace_color.sbs/replace_color](../assets/sbs/replace_color.sbs) | 1 | No | [In progress · 136,480/142,592 exact pixels in 19 whole renders](parity-fixes.md#hue-saturation-lightness-and-replace-color) |
| [rgb-a_merge.sbs/rgb-a_merge](../assets/sbs/rgb-a_merge.sbs) | 2 | No | [Parity: 4 cases](parity-fixes.md#verified-cases) |
| [rgb-a_split.sbs/rgb-a_split](../assets/sbs/rgb-a_split.sbs) | 1 | No | [Parity: 4 cases](parity-fixes.md#verified-cases) |
| [rgba_merge.sbs/rgba_merge](../assets/sbs/rgba_merge.sbs) | 4 | No | [Parity: 4 cases](parity-fixes.md#verified-cases) |
| [rgba_split.sbs/rgba_split](../assets/sbs/rgba_split.sbs) | 1 | No | [Parity: 4 cases](parity-fixes.md#verified-cases) |
| [rt_ao.sbs/pp_template/pp_template_64g](../assets/sbs/rt_ao.sbs) | 3 | Yes | [Parity: 4 cases](parity-fixes.md#verified-cases) |
| [rt_ao_v2.sbs/mip_max_step](../assets/sbs/rt_ao_v2.sbs) | 1 | Yes | [Parity: 4 cases](parity-fixes.md#verified-cases) |
| [rt_bent_normal.sbs/pp_template/pp_template_64c](../assets/sbs/rt_bent_normal.sbs) | 3 | Yes | [Parity: 4 cases](parity-fixes.md#verified-cases) |
| [rt_bent_normal_v2.sbs/mip_max_step](../assets/sbs/rt_bent_normal_v2.sbs) | 1 | Yes | [Parity: 4 cases](parity-fixes.md#verified-cases) |
| [rt_caustics.sbs/rt_caustics_base](../assets/sbs/rt_caustics.sbs) | 3 | Yes | [In progress · 152,777/172,288 pixels in 19 renders](parity-fixes.md#paths-caustics-and-cell-fields-after-pattern-sampling) |
| [rt_irradiance.sbs/compute_irradiance](../assets/sbs/rt_irradiance.sbs) | 5 | Yes | [Parity · 32 renders, all 217,088 RGBA pixels exact](parity-fixes.md#environment-rotation-and-wired-irradiance-samples) |
| [rt_irradiance.sbs/compute_ray_direction](../assets/sbs/rt_irradiance.sbs) | 4 | Yes | [Parity · 32 renders, all 217,088 RGBA pixels exact](parity-fixes.md#environment-rotation-and-wired-irradiance-samples) |
| [rt_irradiance.sbs/debug_hit_uv](../assets/sbs/rt_irradiance.sbs) | 3 | Yes | [In progress · 284,156/284,160 pixels in 90 renders; four marker pixels differ](parity-fixes.md#fixed-integer-arguments-and-the-uv-debug-marker) |
| [rt_irradiance.sbs/mip_max](../assets/sbs/rt_irradiance.sbs) | 1 | Yes | [Parity · 167,168 exact pixels in 17 cases](parity-fixes.md#pixel-processor-canvas-and-mip-maxima) |
| [rt_irradiance.sbs/panoramic_lod](../assets/sbs/rt_irradiance.sbs) | 1 | Yes | [Parity · 18 cases, 186,624 exact pixels](parity-fixes.md#large-transform-offsets-and-additive-blends) |
| [rt_irradiance.sbs/template/pp_template](../assets/sbs/rt_irradiance.sbs) | 3 | Yes | [Parity: 4 cases](parity-fixes.md#verified-cases) |
| [rt_shadows.sbs/mip_max](../assets/sbs/rt_shadows.sbs) | 1 | Yes | [Parity · 167,168 exact pixels in 17 cases](parity-fixes.md#pixel-processor-canvas-and-mip-maxima) |
| [rt_shadows.sbs/template/pp_template](../assets/sbs/rt_shadows.sbs) | 4 | Yes | [Parity: 4 cases](parity-fixes.md#verified-cases) |
| [rt_shadows_v2.sbs/mip_max_step](../assets/sbs/rt_shadows_v2.sbs) | 1 | Yes | [Parity: 4 cases](parity-fixes.md#verified-cases) |
| [safe_transform.sbs/safe_transform](../assets/sbs/safe_transform.sbs) | 1 | No | [Parity · 171,264 exact pixels in 18 renders](parity-fixes.md#rotated-transforms-and-fractional-texture-coordinates) |
| [safe_transform.sbs/safe_transform_grayscale](../assets/sbs/safe_transform.sbs) | 1 | No | [Parity · 171,264 exact pixels in 18 renders](parity-fixes.md#rotated-transforms-and-fractional-texture-coordinates) |
| [scratches_1.sbs/scratches_1](../assets/sbs/scratches_1.sbs) | 0 | Yes | [In progress · 133,530/142,592 whole pixels, first differing stages measured](parity-fixes.md#tile-weave-splatter-and-fiber-stage-comparisons) |
| [scratches_2.sbs/scratches_2](../assets/sbs/scratches_2.sbs) | 0 | Yes | [In progress · 118,291/142,592 whole pixels, first differing stages measured](parity-fixes.md#tile-weave-splatter-and-fiber-stage-comparisons) |
| [scratches_4.sbs/scratches_4](../assets/sbs/scratches_4.sbs) | 0 | Yes | [In progress · 106,551/126,208 whole pixels, first differing stages measured](parity-fixes.md#tile-weave-splatter-and-fiber-stage-comparisons) |
| [shape.sbs/shape_base](../assets/sbs/shape.sbs) | 0 | Yes | [In progress · 139,931/142,592 whole pixels, first differing stages measured](parity-fixes.md#tile-weave-splatter-and-fiber-stage-comparisons) · [saved branch 4,626/4,864 authored and 4,864/4,864 disc](parity-fixes.md#tile-weave-splatter-and-fiber-stage-comparisons) |
| [shape_mapper.sbs/shape_mapper](../assets/sbs/shape_mapper.sbs) | 1 | No | [Parity · 310,784/310,784 pixels](parity-fixes.md#wired-spline-helpers-and-shape-mapping) |
| [shape_splatter.sbs/min_max](../assets/sbs/shape_splatter.sbs) | 1 | Yes | [Parity: 4 cases](parity-fixes.md#verified-cases) |
| [shape_splatter.sbs/shape_splatter_blend](../assets/sbs/shape_splatter.sbs) | 12 | No | [Parity · 18 renders, 171,264 exact pixels](parity-fixes.md#per-pixel-random-initialization) |
| [shape_splatter.sbs/shape_splatter_blend_color](../assets/sbs/shape_splatter.sbs) | 12 | No | [Parity · 23 renders, 204,032 exact pixels](parity-fixes.md#per-pixel-random-initialization) |
| [shape_splatter.sbs/shape_splatter_data_extract](../assets/sbs/shape_splatter.sbs) | 2 | No | [Parity · 19 cases on seven outputs, 1,206,016 exact pixels](parity-fixes.md#large-transform-offsets-and-additive-blends) |
| [shape_splatter.sbs/shape_splatter_to_mask](../assets/sbs/shape_splatter.sbs) | 1 | No | [Parity · 4 renders, 114,688 exact pixels](parity-fixes.md#per-pixel-random-initialization) |
| [shape_splatter_v2.sbs/bitonic_sort/graph/bitonic_sort_iteration](../assets/sbs/shape_splatter_v2.sbs) | 1 | Yes | [Parity: 4 cases](parity-fixes.md#verified-cases) |
| [shape_splatter_v2.sbs/bitonic_sort/graph/initial_iteration](../assets/sbs/shape_splatter_v2.sbs) | 1 | Yes | [Parity: 4 cases](parity-fixes.md#verified-cases) |
| [shape_splatter_v2.sbs/bitonic_sort/graph/remap_sorting](../assets/sbs/shape_splatter_v2.sbs) | 2 | Yes | [Parity: 4 cases](parity-fixes.md#verified-cases) |
| [shape_splatter_v2.sbs/grid_to_scanline](../assets/sbs/shape_splatter_v2.sbs) | 1 | Yes | [Parity: 4 cases](parity-fixes.md#verified-cases) |
| [shape_splatter_v2.sbs/mipmap_tools/Mipmap_N_from_O_color](../assets/sbs/shape_splatter_v2.sbs) | 1 | Yes | [Parity · 18 renders, all 287,232 pixels exact](parity-fixes.md#marked-variable-reads-in-path-and-mipmap-graphs) |
| [shape_splatter_v2.sbs/poisson_disc/poisson_disc_iteration](../assets/sbs/shape_splatter_v2.sbs) | 1 | Yes | [Parity: 4 cases](parity-fixes.md#verified-cases) |
| [shape_splatter_v2.sbs/scanline_to_z_order](../assets/sbs/shape_splatter_v2.sbs) | 1 | Yes | [Parity: 4 cases](parity-fixes.md#verified-cases) |
| [shape_splatter_v2.sbs/shape_splatter_compute_normal](../assets/sbs/shape_splatter_v2.sbs) | 10 | Yes | [Parity · 292,864/292,864 pixels in 144 whole renders across four formats](parity-fixes.md#component-rounding-in-the-shape-splatter-normal) |
| [shape_splatter_v2_companion.sbs/shape_splatter_to_mask](../assets/sbs/shape_splatter_v2_companion.sbs) | 1 | No | [Parity: 4 cases](parity-fixes.md#verified-cases) |
| [skew.sbs/skew](../assets/sbs/skew.sbs) | 1 | No | [Parity · 13 cases, 163,840 exact pixels](parity-fixes.md#height-distance-texture-mapping-skew-and-weave) |
| [skew.sbs/skew_grayscale](../assets/sbs/skew.sbs) | 1 | No | [Parity · 13 cases, 163,840 exact pixels](parity-fixes.md#height-distance-texture-mapping-skew-and-weave) |
| [slope_blur.sbs/slope_blur](../assets/sbs/slope_blur.sbs) | 2 | No | [Parity · 18 renders, 145,664 exact pixels](parity-fixes.md#gradient-warp-displacement-and-sampling) |
| [slope_blur.sbs/slope_blur_grayscale](../assets/sbs/slope_blur.sbs) | 2 | Yes | [Parity · 18 renders, 145,664 exact pixels](parity-fixes.md#gradient-warp-displacement-and-sampling) |
| [slope_blur.sbs/slope_blur_grayscale_2](../assets/sbs/slope_blur.sbs) | 2 | No | [Parity · 18 renders, 145,664 exact pixels](parity-fixes.md#gradient-warp-displacement-and-sampling) |
| [smart_auto_tile.sbs/pattern_find_edges_step](../assets/sbs/smart_auto_tile.sbs) | 4 | Yes | [Parity · 16 cases, 3,185,920 exact pixels](parity-fixes.md#tile-edge-masks-and-normalized-spline-distances) |
| [splatter.sbs/splatter](../assets/sbs/splatter.sbs) | 1 | No | [In progress · 142,577/142,592 whole pixels, first differing stages measured](parity-fixes.md#tile-weave-splatter-and-fiber-stage-comparisons) |
| [splatter_circular.sbs/splatter_circular](../assets/sbs/splatter_circular.sbs) | 2 | Yes | [In progress · 139,297/142,592 whole pixels, first differing stages measured](parity-fixes.md#tile-weave-splatter-and-fiber-stage-comparisons) |
| [splatter_circular.sbs/splatter_circular_color](../assets/sbs/splatter_circular.sbs) | 2 | Yes | [In progress · 140,218/142,592 whole pixels, first differing stages measured](parity-fixes.md#tile-weave-splatter-and-fiber-stage-comparisons) |
| [splatter_color.sbs/splatter_color](../assets/sbs/splatter_color.sbs) | 2 | No | [In progress · 142,551/142,592 whole pixels, first differing stages measured](parity-fixes.md#tile-weave-splatter-and-fiber-stage-comparisons) |
| [spline_tools.sbs/-----WIP-----/circular_array_wip](../assets/sbs/spline_tools.sbs) | 2 | Yes | [Parity: 4 cases](parity-fixes.md#verified-cases) |
| [spline_tools.sbs/-----WIP-----/displace_spline_by_spline_wip](../assets/sbs/spline_tools.sbs) | 4 | Yes | [Parity: 4 cases](parity-fixes.md#verified-cases) |
| [spline_tools.sbs/internal_tools/2dtransform_spline_sub](../assets/sbs/spline_tools.sbs) | 3 | Yes | [Parity · 87,570/87,570 pixels and values](parity-fixes.md#wired-spline-helpers-and-shape-mapping) |
| [spline_tools.sbs/internal_tools/append_splines_sub](../assets/sbs/spline_tools.sbs) | 6 | Yes | [Parity: 4 cases](parity-fixes.md#verified-cases) |
| [spline_tools.sbs/internal_tools/compute_bridge_distance](../assets/sbs/spline_tools.sbs) | 3 | Yes | [Parity · 54,528/54,528 pixels](parity-fixes.md#wired-spline-helpers-and-shape-mapping) |
| [spline_tools.sbs/internal_tools/compute_spline_distance_iteration](../assets/sbs/spline_tools.sbs) | 1 | Yes | [Parity: 4 cases](parity-fixes.md#verified-cases) |
| [spline_tools.sbs/internal_tools/compute_spline_length_init](../assets/sbs/spline_tools.sbs) | 2 | Yes | [Parity · 43,776/43,776 pixels](parity-fixes.md#wired-spline-helpers-and-shape-mapping) |
| [spline_tools.sbs/internal_tools/compute_spline_tangents](../assets/sbs/spline_tools.sbs) | 3 | Yes | [Parity · 43,776 exact pixels in 27 renders](parity-fixes.md#clamped-texture-reads-in-spline-graphs) |
| [spline_tools.sbs/internal_tools/get_spline_point_data](../assets/sbs/spline_tools.sbs) | 1 | Yes | [Parity: 4 cases](parity-fixes.md#verified-cases) |
| [spline_tools.sbs/internal_tools/modify_spline_property_sub](../assets/sbs/spline_tools.sbs) | 4 | Yes | [Parity · 28,928 exact pixels in 19 renders](parity-fixes.md#clamped-texture-reads-in-spline-graphs) |
| [spline_tools.sbs/internal_tools/normalize_spline_distance](../assets/sbs/spline_tools.sbs) | 1 | Yes | [Parity · 16 cases, 52,480 exact pixels](parity-fixes.md#tile-edge-masks-and-normalized-spline-distances) |
| [spline_tools.sbs/internal_tools/points_preview](../assets/sbs/spline_tools.sbs) | 3 | Yes | [In progress · 27,904 exact output pixels, text errors and wired cases open](parity-fixes.md#clamped-texture-reads-in-spline-graphs) |
| [spline_tools.sbs/internal_tools/resample_spline_data](../assets/sbs/spline_tools.sbs) | 3 | Yes | [Parity · 29,952 exact pixels in 19 renders](parity-fixes.md#clamped-texture-reads-in-spline-graphs) |
| [spline_tools.sbs/internal_tools/scatter_splines_on_splines_preprocess](../assets/sbs/spline_tools.sbs) | 5 | Yes | [In progress · nonzero counts and images exact, zero count timed out](parity-fixes.md#spline-preprocessing-with-wired-counts) |
| [spline_tools.sbs/internal_tools/select_spline_sub](../assets/sbs/spline_tools.sbs) | 3 | Yes | [Parity · 622,592 exact pixels in 19 renders](parity-fixes.md#clamped-texture-reads-in-spline-graphs) |
| [spline_tools.sbs/internal_tools/spline_warp_sub](../assets/sbs/spline_tools.sbs) | 6 | Yes | [Parity · 28,928 exact pixels in 19 renders](parity-fixes.md#clamped-texture-reads-in-spline-graphs) |
| [spot_emboss.sbs/spot_emboss](../assets/sbs/spot_emboss.sbs) | 2 | No | [Parity · 218,368 exact cook pixels and six exact saved-file renders](parity-fixes.md#image-selection-in-saved-pattern-graphs) |
| [starburst.sbs/starburst](../assets/sbs/starburst.sbs) | 0 | No | [In progress · 141,503/142,592 whole pixels, first differing stages measured](parity-fixes.md#tile-weave-splatter-and-fiber-stage-comparisons) |
| [stripes.sbs/stripes](../assets/sbs/stripes.sbs) | 0 | Yes | [In progress · 215,012/215,040 exact pixels in 18 renders](parity-fixes.md#other-roots-using-uniform-arc-tangents) |
| [summed_area_table.sbs/summed_area_table_iteration](../assets/sbs/summed_area_table.sbs) | 1 | Yes | [Parity: 4 cases](parity-fixes.md#verified-cases) |
| [summed_area_table_2.sbs/summed_area_table_iteration](../assets/sbs/summed_area_table_2.sbs) | 1 | Yes | [Parity: 4 cases](parity-fixes.md#verified-cases) |
| [swirl.sbs/swirl](../assets/sbs/swirl.sbs) | 1 | No | [Parity · 171,264/171,264 pixels in 18 renders](parity-fixes.md#quad-swirl-trapezoid-and-vector-morph-comparisons) |
| [swirl.sbs/swirl_grayscale](../assets/sbs/swirl.sbs) | 1 | No | [Parity · 171,264/171,264 pixels in 18 renders](parity-fixes.md#quad-swirl-trapezoid-and-vector-morph-comparisons) |
| [symmetry.sbs/symmetry](../assets/sbs/symmetry.sbs) | 1 | No | [Parity: 4 cases](parity-fixes.md#verified-cases) |
| [symmetry_slice.sbs/symmetry_slice](../assets/sbs/symmetry_slice.sbs) | 1 | No | [In progress · 156,022/171,264 exact pixels in 18 renders](parity-fixes.md#plane-light-splatter-symmetry-and-waveform-comparisons) |
| [templates.sbs/Templates/Standard_Materials/Extended_Material](../assets/sbs/templates.sbs) | 0 | Yes | [Parity: 3 cases](parity-fixes.md#verified-cases) |
| [templates.sbs/Templates/Standard_Materials/Simple_Material](../assets/sbs/templates.sbs) | 0 | Yes | [Parity: 3 cases](parity-fixes.md#verified-cases) |
| [threshold.sbs/threshold](../assets/sbs/threshold.sbs) | 1 | No | [Parity: 4 cases](parity-fixes.md#verified-cases) |
| [tile_generator.sbs/tile_generator](../assets/sbs/tile_generator.sbs) | 2 | Yes | [In progress · 139,723/142,592 whole pixels, first differing stages measured](parity-fixes.md#tile-weave-splatter-and-fiber-stage-comparisons) |
| [tile_generator_color.sbs/tile_generator_color](../assets/sbs/tile_generator_color.sbs) | 2 | Yes | [In progress · 137,445/142,592 whole pixels, first differing stages measured](parity-fixes.md#tile-weave-splatter-and-fiber-stage-comparisons) |
| [tile_random.sbs/tile_random](../assets/sbs/tile_random.sbs) | 2 | Yes | [In progress · 111,364/142,592 whole pixels, first differing stages measured](parity-fixes.md#tile-weave-splatter-and-fiber-stage-comparisons) · [saved authored and disc branches each match 4,864/4,864](parity-fixes.md#tile-weave-splatter-and-fiber-stage-comparisons) |
| [tile_random.sbs/tile_random_color](../assets/sbs/tile_random.sbs) | 2 | Yes | [In progress · 111,228/142,592 whole pixels, first differing stages measured](parity-fixes.md#tile-weave-splatter-and-fiber-stage-comparisons) · [saved authored and disc branches each match 4,864/4,864](parity-fixes.md#tile-weave-splatter-and-fiber-stage-comparisons) |
| [tile_sampler.sbs/arrow](../assets/sbs/tile_sampler.sbs) | 0 | Yes | [Parity · seven 512×512 runs, four formats](parity-fixes.md#transform-edges-and-floating-point-filtering) |
| [tileable_material_to_mesh_material.sbs/tileable_material_to_mesh_material](../assets/sbs/tileable_material_to_mesh_material.sbs) | 5 | No | [Parity · both outputs, 358,912 exact pixels in 42 renders](parity-fixes.md#diffuse-and-specular-outputs-of-the-mesh-material-graph) |
| [trapezoid_transform.sbs/trapezoid_transform](../assets/sbs/trapezoid_transform.sbs) | 1 | No | [Parity · 848,384 exact pixels in 68 renders across four formats](parity-fixes.md#trapezoid-coordinates-under-fixed-clipping) |
| [trapezoid_transform.sbs/trapezoid_transform_grayscale](../assets/sbs/trapezoid_transform.sbs) | 1 | No | [Parity · 848,384 exact pixels in 68 renders across four formats](parity-fixes.md#trapezoid-coordinates-under-fixed-clipping) |
| [uv_mapper.sbs/uv_mapper_color_sub](../assets/sbs/uv_mapper.sbs) | 2 | Yes | [Parity · 13 cases, 163,840 exact pixels](parity-fixes.md#height-distance-texture-mapping-skew-and-weave) |
| [uv_mapper.sbs/uv_mapper_grayscale_sub](../assets/sbs/uv_mapper.sbs) | 2 | Yes | [Parity · 13 cases, 163,840 exact pixels](parity-fixes.md#height-distance-texture-mapping-skew-and-weave) |
| [value_get_size.sbs/get_size_color](../assets/sbs/value_get_size.sbs) | 1 | No | [Parity: 4 cases](parity-fixes.md#verified-cases) |
| [value_get_size.sbs/get_size_grayscale](../assets/sbs/value_get_size.sbs) | 1 | No | [Parity: 4 cases](parity-fixes.md#verified-cases) |
| [vector_morph.sbs/vector_morph](../assets/sbs/vector_morph.sbs) | 2 | No | [Parity · 171,264/171,264 pixels in 18 renders](parity-fixes.md#quad-swirl-trapezoid-and-vector-morph-comparisons) |
| [vector_morph.sbs/vector_morph_grayscale](../assets/sbs/vector_morph.sbs) | 2 | No | [Parity · 171,264/171,264 pixels in 18 renders](parity-fixes.md#quad-swirl-trapezoid-and-vector-morph-comparisons) |
| [vertical_noise.sbs/vertical_noise](../assets/sbs/vertical_noise.sbs) | 0 | Yes | [In progress · 142,088/142,592 whole pixels, first differing stages measured](parity-fixes.md#tile-weave-splatter-and-fiber-stage-comparisons) |
| [waveform_01_v2.sbs/waveform_01_v2](../assets/sbs/waveform_01_v2.sbs) | 0 | No | [In progress · 142,662/171,264 exact pixels in 18 renders](parity-fixes.md#plane-light-splatter-symmetry-and-waveform-comparisons) |
| [waveform_1.sbs/waveform_1](../assets/sbs/waveform_1.sbs) | 0 | Yes | [In progress · 142,630/171,264 exact pixels in 18 renders](parity-fixes.md#plane-light-splatter-symmetry-and-waveform-comparisons) |
| [weave_1.sbs/weave_1](../assets/sbs/weave_1.sbs) | 0 | Yes | [In progress · 139,856/142,592 whole pixels, first differing stages measured](parity-fixes.md#tile-weave-splatter-and-fiber-stage-comparisons) |
| [weave_2.sbs/weave_2](../assets/sbs/weave_2.sbs) | 0 | Yes | [In progress · 138,111/142,592 whole pixels, first differing stages measured](parity-fixes.md#tile-weave-splatter-and-fiber-stage-comparisons) |
| [weave_3.sbs/weave_3](../assets/sbs/weave_3.sbs) | 0 | Yes | [Parity · 13 cases, 163,840 exact pixels](parity-fixes.md#height-distance-texture-mapping-skew-and-weave) |
| [weave_4.sbs/weave_4](../assets/sbs/weave_4.sbs) | 0 | Yes | [In progress · 170,018/171,264 exact pixels in 18 renders](parity-fixes.md#plane-light-splatter-symmetry-and-waveform-comparisons) |
| [white_noise.sbs/white_noise](../assets/sbs/white_noise.sbs) | 0 | Yes | [Parity · 13 renders, 254,208 exact pixels](parity-fixes.md#stored-grayscale-stops-and-scalar-levels-controls) |
| [white_noise.sbs/white_noise_fast](../assets/sbs/white_noise.sbs) | 0 | Yes | [Parity · 18 renders, 127,488 exact pixels](parity-fixes.md#per-pixel-random-initialization) |
| [white_noise_v2.sbs/white_noise_v2](../assets/sbs/white_noise_v2.sbs) | 0 | No | [Parity · 76 renders, 183,808 exact pixels](parity-fixes.md#conditional-pixel-draws-and-white-noise-v2) |
| [wood_fibers_1.sbs/wood_fibers_1](../assets/sbs/wood_fibers_1.sbs) | 0 | Yes | [In progress · 136,524/142,592 whole pixels, first differing stages measured](parity-fixes.md#tile-weave-splatter-and-fiber-stage-comparisons) |
| [wood_fibers_2.sbs/wood_fibers_2](../assets/sbs/wood_fibers_2.sbs) | 0 | Yes | [In progress · 17/18 whole renders completed, 55,649/60,672 exact pixels](parity-fixes.md#discarding-pattern-stamps-that-miss-every-output-sample) · [216 isolated stamp comparisons](parity-fixes.md#individual-wood-fibers-2-stamps) |
