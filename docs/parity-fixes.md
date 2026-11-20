# MaterialCraft parity fixes

Rolling record of fixes made while working through the [SBS root-node inventory](sbs-root-nodes.md).
Parity means exact pixel or typed-value agreement for recorded cases; pending validation is stated explicitly.
The first pass classifies all 546 roots: 166 Parity and 380 Too hard. These measured cases do not establish complete functional parity.

- **2026-11-13 — SBS swizzle defaults (orientation corrected):** Removed the importer's zero-channel overrides for omitted two-, three-, and four-component float/integer swizzle settings. Imports now use the existing identity-channel defaults from the function catalog. The first affected node, `3d_gradient.sbs/3d_linear_gradient`, previously repeated X instead of preserving XYZ, producing a horizontal gradient instead of the reference diagonal gradient. Compared both point-position modes with generated position maps at 256 and 512 pixels. Remaining float differences are recorded below; full node parity is not claimed.
- **2026-11-13 — Filtered function sampling (integer probes exact):** Added cooked-image storage precision to function samplers. Bilinear reads use eight fractional bits for filter weights. Normalized integer textures return 16-bit filtered values (including 8-bit sources); floating-point textures retain floating-point values. Input bridges retain the integer/floating-point distinction, and cache keys include it. Direct reference measurements on a generated position ramp showed a 16-bit half-step rounding to the next integer, explaining the remaining UV-gradient offset after the swizzle fix. All six measured integer-source sampler cases match all 65,536 float pixels exactly. Two of three floating-source cases are exact; one differs by 5.96e-8. Full-gradient rounding differences remain below.

- **2026-11-13 - SBS format inheritance (measured improvement):** Read and save the declared native graph output format instead of dropping it. An omitted format retains inheritance; comparison cases explicitly align the parent format to the reference preview (eight bits). Pixel Processors with omitted format settings now inherit the parent format instead of an image input format. This was exposed by `3d_perlin_noise.sbs/3D_perlin_noise`: the reference processor produces eight-bit output even though its position and lookup inputs are sixteen-bit. The Perlin comparison improved from 63,461/65,536 to 65,533/65,536 matching 8-bit pixels at 256 pixels; three additional 128-pixel cases match exactly. All four native format codes are serialized explicitly on save.

- **2026-11-13 - Floored floating-point modulo (randomized jump flood exact):** Replaced signed remainder with `a - b * floor(a / b)`. Direct reference probes confirm all sign combinations, including `-1 mod 4 = 3`, `1 mod -4 = -3`, and `-5.5 mod 4 = 2.5`. This corrects negative coordinate wrapping used by 3D texture jump flood. Both randomized 128x128 jump-flood cases (iterations 0 and 1, with disabled seed points) now match all 32,768 RGBA pixels exactly; the three ramp cases also match. All 131,072 RGBA pixels across five cases are exact.

- **2026-11-13 - Bound input storage and resizing:** Bound image inputs now store their declared output precision instead of passing unconverted floats downstream. Resizing clamps resource edges and proceeds through factor-two stored levels. A generated HDR 16x8 ramp resized to 32x32 now matches every RGBA8 input-bridge pixel; previously its center sample was 1.0 instead of 0.9843137. A randomized 64x32-to-32x32 downsample also matches every pixel. Three harder randomized multilevel cases retain single-level differences (4/1,024, 51/4,096, and 6/256 pixels), so general resampling parity is not claimed.

- **2026-11-13 - Half-float output storage:** Sixteen-bit floating-point outputs now convert to half precision, truncate toward zero, and saturate at +/-65,504. Previously they retained full 32-bit precision. All 2,304 pixels across nine direct reference probes match exactly, covering positive/negative values, rounding boundaries, a subnormal, maximum finite values, and positive/negative overflow. Uses the existing workspace half dependency (MIT OR Apache-2.0).

- **2026-11-13 - Volume-light function expansion:** Raised the bounded expanded-function allowance from 65,536 to 131,072 nodes after measuring the legitimate volume-light processor at 72,973 nodes and four call levels. The individual 4,096-node graph limit, 16-level call limit, cycle checks, and execution budget remain enforced. Error messages now report the actual limits. Three previously rejected light-render cases cook successfully; 760/768 RGBA16F pixels match, with the remaining differences recorded below.

- **2026-11-13 - Omitted function sampler settings:** Corrected Sample Color/Gray defaults to image zero with nearest filtering. Explicit bilinear settings are retained. Both original omitted-setting probes match all 32,768 float pixels. This removes the shifted edges of the 3D volume mask: default outputs at 128x128 and 256x256 and the changed-scale 128x128 case now match every sixteen-bit pixel.

- **2026-11-13 - Finite results after undefined pixel intermediates:** Pixel Processor arithmetic and vector operations now retain IEEE intermediate values so a later min/max can clamp division by zero to a finite result. Non-finite final pixels still return an actionable error. Parameter and pattern functions are checked at their public evaluation boundary. Four original clamp probes (positive infinity, negative infinity, NaN, and finite division) match all 1,024 float pixels. Cylinder-mask cases previously failed at the exact sampled center; they now cook successfully, with the residual conversion differences recorded below.

- **2026-11-13 - Independent transform wrapping:** Transformation 2D now uses horizontal and vertical tiling flags independently, with matching per-axis image sampling. Previously a horizontal-only panorama lost horizontal wrapping because the two flags collapsed to one boolean. All 131,072 RGBA8 pixels across eight original transform probes match exactly, covering both one-axis modes, positive/negative offsets, matte edges, and varying alpha. The panorama helper improves, but its remaining minification/filtering differences are recorded below.

- **2026-11-13 - Blend alpha, copy, and HDR:** Corrected the Switch stages used by Copy and other graphs to interpolate RGBA without weighting foreground alpha twice. Copy, Add, and other color modes now combine alpha coverage when alpha blending is enabled; disabling alpha blending preserves background alpha while RGB still uses foreground coverage. Integer clipping happens after opacity; floating-point formats retain HDR output. All 19 independent blend graphs are being measured with opaque/varying-alpha inputs, masks, opacity changes, and alpha-channel blending disabled. Direct Add/Copy HDR cases match all 512 RGBA32F pixels. Fourteen blend graphs now match every pixel across four RGBA8 cases each (1,605,632 pixels); Add and Copy also pass their HDR cases. The blend properties label and all four alpha-interpretation menu entries were checked visually in the running native app.

- **2026-11-13 - Function validation at result boundaries:** Original clamp probes confirm that node-parameter and pattern functions also accept undefined intermediates when later operations produce a finite result. Deferred those intermediate checks and retained explicit final-result validation. The original four node-parameter and four pattern probes match all 2,048 output pixels; unclamped pixel and node-parameter probes still return clear non-finite-output errors. Ordered NaN comparisons now return false, as measured for all four relational operators. All 5,120 float pixels across the twenty NaN/infinity/finite/zero comparison cases match exactly.

- **2026-11-13 - Grayscale default precision:** Default grayscale rendering now uses one third per RGB channel instead of 0.333, which previously darkened equal channels. This makes Add Sub exact in all four RGBA8 cases and reduces Luminosity from 8,407 mismatching pixels to two. Original float probes establish the fused red/green/blue/alpha dot-product order; all 65,536 float pixels across default, custom, sum-three, and zero-weight cases match exactly. Explicit channel weights retain their authored values.

- **2026-11-13 - Declared image types:** Imported single-output filters with an omitted color-mode parameter now retain the color/grayscale type declared in their SBS output. The bevel debug helper previously dropped RGBA coordinate data into a grayscale pattern buffer. It now cooks in the correct four-channel format. Explicit settings and functions remain authoritative.

- **2026-11-13 - Pattern alpha and empty backgrounds:** Color pattern maps now start transparent. Scalar luminosity/opacity affects alpha as well as RGB, without multiplying RGB twice. Eight original empty/scalar/RGBA/HDR pattern probes match all 2,048 RGBA32F pixels across additive and both maximum blend modes. The existing White Noise result still matches every one of its 65,536 sixteen-bit pixels.

- **2026-11-13 - Color resources on grayscale input pins:** Bound color images now use equal RGB weights at grayscale input bridges. The general display-luminance utility retains its documented behavior. All nine eight-bit grayscale switch cases match 294,912 pixels; two additional float switch cases match all 32,768 pixels exactly, including varying source alpha. Previously only about one percent of the eight-bit pixels matched.

- **2026-11-13 - Transform matrix layout:** Transformation 2D now reads consecutive columns in the stored matrix and uses the same convention when selecting nearest mip levels. Previously rotations ran in the opposite direction and shears used the wrong axis. All 131,072 RGBA8 pixels across eight original shear/quarter-turn/diagonal/offset probes match exactly. Both unscaled rotated-checker cases also become exact. Scaled filtering remains open.

- **2026-11-13 - Legacy blend alpha interpretation:** Imported and implemented the older source-alpha/ignore-alpha/straight/premultiplied setting instead of dropping it. Direct runtime enum inspection confirms the four modes. Original Copy/Add/Max probes cover opacity changes, alpha-channel blending disabled, and 128x128/256x256 outputs. Ignore-alpha controls RGB weighting independently of output coverage; straight compositing includes background coverage, while premultiplied Copy consumes premultiplied foreground RGB. All four Clamp cases now match every one of 114,688 RGBA8 pixels. All 48 direct Copy/Add/Max cases cook; seven of the twelve mode/interpretation combinations match all 114,688 pixels, while the other five retain 11 one-level differences in total. Straight HDR output remains unclipped; its float differences reach 3.577e-7.

- **2026-11-13 - Integer options read as Boolean:** A legacy color-temperature function reads an integer option through Get Boolean. Original probes confirm that zero is false and positive/negative integers, including the minimum 32-bit value, are true. Added that conversion for user integer/enum variables while retaining strict system-variable and other getter types. All 1,280 float pixels across the five direct cases match exactly. Color temperature now cooks in all four cases; only two one-level pixels remain different.

- **2026-11-13 - Internal images down to one texel:** Removed the evaluator's 16-pixel floor for internal node sizes. The graph editor retains its own graph-size setting range. Internal reductions and lookup images can now use their declared 1..8,192 dimensions. Direct 1x1, 2x4, and 8x2 RGBA32F probes match all 25 pixels. The curvature helper's four small cases now use the correct 2x2/4x4 outputs and match all 28 pixels; two larger nonconstant cases also match all 16,640 RGBA32F pixels. All 655,360 pixels across 22 earlier Perlin-fractal, simplex, jump-flood, angle-vector, checker, and live White Noise cases still match exactly.

- **2026-11-13 - Integer Pixel Processor storage:** Original eight- and sixteen-bit probes confirm NaN converts to black, positive infinity to white, and negative infinity to black. Deferred Pixel Processor finite checks to integer storage for those formats, retaining final floating-point output validation. Integer storage now saturates infinities instead of treating every non-finite value as zero. All 2,560 pixels across ten direct integer probes match exactly. The zero-softness color mask and radial gradient now cook in all four cases each, with only one-level residual differences. Unclamped floating-point Pixel Processors still return a non-finite-output error.

- **2026-11-13 - Swizzle padding:** Vector swizzles now read components outside the source dimension as zero, including scalar inputs. Arithmetic scalar broadcasting remains available to arithmetic operations. All 1,792 RGBA32F pixels across seven original float/integer probes match exactly. Histogram computation now cooks in all four cases; its remaining numerical differences are recorded below.

- **2026-11-13 - Grayscale resources on color input pins:** Color input bridges now promote grayscale resources to RGB with opaque alpha. Both original 128x128 and 256x256 probes match all 81,920 RGBA8 pixels exactly.

- **2026-11-13 - Unassigned user variables:** Numeric user-variable getters now return typed zero, and Boolean getters return false, before the first assignment. Previously they raised an undefined-variable error. Original probes add a nonzero value after each getter so a failed black render cannot look like a match. All 2,304 float/RGBA32F pixels across Boolean, integer1..4, and float1..4 cases match exactly. System-variable availability, existing-variable type checks, and string errors remain strict. All four affected Voronoi graphs now cook across four 16x16/32x32 cases each; numerical differences remain. The 128x128 fractal case also cooks and matches 15,457/16,384 pixels. The previously failing matte and shadow helper now cook in all four cases each, with remaining output differences.

- **2026-11-13 - RGBA results in grayscale Pixel Processors:** Grayscale processors now convert a four-component result with equal RGB weights instead of taking red alone. Alpha does not contribute. Original HDR and negative-value probes establish the fused green/red/blue accumulation order; all 9,216 LUM32F pixels across 36 cases match exactly. The three affected grayscale rendering templates now match all 344,064 LUM8 pixels across twelve cases; the color template retains all 114,688 exact RGBA8 pixels.

- **2026-11-13 - Float-to-integer conversion:** Finite scalar and vector conversion now rounds down, correcting negative coordinates that previously truncated toward zero. All 6,144 float/RGBA32F pixels and 18 typed integer outputs match the original reference probes. Non-finite and out-of-range integer conversions retain actionable errors; parity for those inputs is not claimed. The 22 previously verified cases still match all 655,360 pixels after this change.

- **2026-11-13 - Unconnected output images:** Unconnected output bridges now produce transparent RGBA instead of grayscale. The importer separately retains each output's declared image type so saving an unconnected output preserves the graph interface. Both material templates match all 1,179,648 RGBA8 pixels across six recorded cases. All 2,560 direct grayscale/color declaration probe pixels also match, and both declared image types survive save/reload.

## 2026-11-20 root comparisons

### Hue, saturation, lightness and Replace Color

The HSL primitive adds twice each slider's displacement from .5 to saturation and lightness, then clamps. For an HDR input it divides RGB by the largest of 1 and the three input channels before adjusting color and restores that gain afterwards. With default settings pure red produces (.999877930, .0001220703125, .00109827518) in the reference. Equal-channel .125 gray with saturation .75 produces (.1875,.0625,.0625). Changing lightness to .75 instead gives (.625,.625,.625).

| Measured outputs | Exact pixels before | Maximum U8 / U16 / F16 / F32 error before | Exact pixels after | Maximum U8 / U16 / F16 / F32 error after |
|---|---:|---|---:|---|
| Palette and two coordinate fields, 90 renders | 13,380/42,240 | 126 / 32256 / 0.492195129 / 1.02734375 | 35,732/42,240 | 0 / 1 / 0.000122070312 / 1.54972076e-06 |
| Independent colors and nearly equal channels, 72 renders | 8,189/79,872 | 255 / 65535 / 1.31054688 / 3.25 | 52,896/79,872 | 0 / 1 / 0.001953125 / 0.000122368336 |
| Replace Color, 19 whole renders | 99,049/142,592 | 64 / 15809 / 0.241210938 / 0.244705796 | 136,480/142,592 | 1 / 1 / 0.000244140625 / 1.07288361e-06 |

Alpha matched the source in both applications for all 75,520 direct F32 pixels. The color error that remains is largest on nearly gray colors with raised saturation. In a separate 26-hue sweep with saturation 1 and lightness .5, 5,387/13,312 F32 pixels match, maximum error 0.000122845173. Four original HSL graphs saved by MaterialCraft reopened in the reference. Their 24 before/after readbacks across all four formats matched 15,104/15,104 pixels, maximum error zero.

Build a single HSL node with a linear F32 color input. For the first six cases at each setting use 16² F32, 16×32 F32, 32×16 F32, then repeat the 16×32 source in U8, U16 and F16. Seed case k at setting j with 104887+17j+k, both zero-based. In the 16² palette vary alpha by row y/16 and cycle RGB across black, white, .5 gray, .125 gray, red, green, blue, yellow, magenta, cyan, (.25,.5,.75), (.75,.5,.25), (.625,.375,.125), (.875,.375,.5), (.1875,.375,.25), and (.75,.625,.125). For channel c at coordinates x,y use v=((11x+17y+13xy+7c+5xc+3yc) mod 67)/64. The unit field uses min(1,v). The HDR field uses 2.5v−.75 and replaces the first 16 RGB pixels in row zero with the palette.

Start with omitted controls, then set (hue,saturation,lightness) to (.5,.5,.5), (.625,.5,.5), (.375,.5,.5), (.5,.25,.5), (.5,.75,.5), (.5,0,.5), (.5,1,.5), (.5,.5,.25), (.5,.5,.75), (.5,.5,0), (.5,.5,1), (.3125,.6875,.15625), (.875,.1875,.8125) and (−.125,1.125,−.25).

For the independent six-case batch use 64×32 F32, 32² F32, 32×16 F32, then the 32² field in U16, F16 and U8. Use seed 105289+17j+k. In the first image choose low=floor(y/6)/16, high=1−low and middle=low+(x/64)(high−low). Permute the channels using lexicographic permutation index (y+floor(x/32)) mod 6. For the second image draw PCG64 seed 105022 uniformly over [−.8125,2.3125), convert to F32, and replace even rows with F32 uniform draws in [0,1). For the third image choose a base cyclically by (x+3y) from [−.5,−0,0,smallest positive F32,2⁻¹⁸,2⁻¹²,.125,.49999997,.5,.50000006,.875,1,2,4]. Add and subtract a difference from [0,2⁻²⁴,2⁻¹⁸,2⁻¹²,.0625,.25] indexed by (floor(x/3)+2y) mod 6. Permute (base,base+d,base−d) by (x+y) mod 6, and use (.7,.3,.1) in the first pixel. Draw the three alpha fields over [−.375,1.375) with the corresponding PCG64 streams seeded 105021, 105022 and 105023. Set the first pixel's alpha to .6 in each image.

The 12 settings for the independent fields are (.5,.5,.5), (0,.5,.5), (1,.5,.5), (.75,.5,.5), (.5,1,.5), (.25,1,.5), (.625,1,.5), (.5,.5,.625), (.5,.625,.5), (.1875,.3125,.75), (.8125,.625,.4375) and (.5000031,.499817,.499987).

The separate hue sweep uses 32×16 linear F32, seed 106871+j and hue [0, 0.03125, 0.08333333, 0.1092233, 0.125, 0.15625, 0.18739991, 0.25, 0.3125, 0.4375, 0.46875, 0.49999997, 0.5, 0.50000006, 0.500371, 0.5772527, 0.5833333, 0.625, 0.6666667, 0.6875, 0.75, 0.81115973, 0.8333333, 0.9101257, 0.96875, 1]. Set alpha to (x+.5)/32. For x<8 take gray from [0,.0625,.1875,.25,.3125,.49999997,.5,.5625,.6875,.75,.9375,1,1.5,2,3.125,4] indexed by row y. For 8≤x<20 use ((31x+79y+3xy) mod 253)/252. For x≥20 take the same positions in a row-major 32×16 PCG64 field drawn uniformly over [−.125,1.5) with seed 106913. Set all RGB channels to that F32 gray.

For Replace Color repeat the four archived cases, the [five precision and seed cases](#four-format-inventory-comparison), and the ten [linear input and size cases](#shared-lookups-and-environment-shadow-pass) with i=442 and a single RGBA input. Use source/target color settings (.5,.5,.5)/(.5,.5,.5) for A, (.125,.625,.8125)/(.8125,.25,.375) for B, and (.875,.1875,.3125)/(.25,.75,.125) for C. The root still differs in all four output formats. Its standalone regressions for hue endpoints, additive adjustments and HDR gain failed on the previous implementation and passed with the change. All 252 release node tests, formatting, strict workspace Clippy, dependency layers and WebAssembly checks passed.

### Shared lookups and environment shadow pass

Three whole roots matched all 721,408 pixels across 103 renders. The tile and noise roots used two original lookup images in both applications. Their earlier comparisons used different lookup data. The parity measurements below apply to the two common inputs.

| Root | Renders | Exact pixels / compared | Pixels in U8 / U16 / F16 / F32 | Maximum error in U8 / U16 / F16 / F32 |
|---|---:|---:|---|---|
| tile_random_2 | 38 | 285,184/285,184 | 10,240 / 240,640 / 14,336 / 19,968 | 0 / 0 / 0 / 0 |
| blue_noise | 38 | 285,184/285,184 | 10,240 / 11,264 / 14,336 / 249,344 | 0 / 0 / 0 / 0 |
| ibl_shadow_pass | 27 | 151,040/151,040 | 6,144 / 6,656 / 9,728 / 128,512 | 0 / 0 / 0 / 0 |

Generate two 256² grayscale U16 lookups with zero-based coordinates. The first is `19*(809*x+4079*y+127*x*y+3121) mod 65536`. The second is 53249 where `(5*x+13*y) mod 17 < 4` and 1023 elsewhere. Bind the same image to the active Bitmap in each application. For each lookup repeat the archived 128² and 256² defaults, 128² seed 42, 128² changed settings, and the [five precision and seed cases](#four-format-inventory-comparison).

For the ten additional cases use zero-based inventory index i=378, 410 or 413. Add [0,1,2,2,2,2,6,7,8,9] to seed 104351+17i. Use [16²,32²,32²,32²,32²,32²,32×16,16×32,64×32,32×64], storage [F32,F32,U8,U16,F16,F32,F32,U16,F32,F16] and settings [defaults,A,B,B,B,B,C,C,B,A].

| Published controls | A | B | C |
|---|---|---|---|
| Tile counts X/Y, random size and random position multipliers | 2/2, 0, 0 | 3/5, .5625, .3125 | 5/3, .875, .6875 |
| Corner radius, random size map multiplier, mask start, shape rotation | 0, 0, 0, 0 | .15625, .375, .1875, .15625 | .4375, .6875, .4375, −.21875 |
| Dithering | 0 | .3125 | .8125 |
| Shadow sequence, total samples, horizontal rotation | 1, 2, 0 | 2, 5, .15625 | 5, 9, −.21875 |

For bound inputs number each component n=4p+c with zero-based input index p and channel c. Use random_size_map, random_slant_map, round_corners_radius_map, bevel_distance_map, mask_map in the tile graph and blue_noise, environment, normal in the shadow graph. The first two cases use U8, taking the floor of the following levels divided by 257. The flat level is 8199+2413n. The impulse field is 59003−391n where `(x+2n) mod 11 = (y+3n) mod 11` and 302+67n elsewhere. For the two C settings use U16 with level 32769+n where `(7x+13y+3n) mod 11 = 0`, and 32767−n elsewhere. The other cases use F32 with level `7919*(11x+31y+7xy+93+47n+(n+1)x) mod 65536` divided by 65535. Import the fields as linear data. Swapping lookup images changes 4,766 tile pixels numerically across these ten cases. The noise root also changes with the input, including 1,024/1,024 pixels in the 32² F32 B case.

Eight additional shadow renders use curved normals and a bright diagonal band in an HDR environment. At pixel centers u=(x+.5)/width and v=(y+.5)/height, set z=2v−1 and angle=2π(u+m/16). Encode the unit normal (sqrt(1−z²) cos(angle), sqrt(1−z²) sin(angle), z) in [0,1]. The environment RGB is (u+1.5b, v+.75b, .125+.75uv), with b=1 where `abs((x+4y+3m) mod width−width/2) < max(width/9,1)` and zero elsewhere. Both alphas are one. The noise input's channel c is `(37x+59y+13xy+47c+11m) mod 251` divided by 250.

Use m=[0,1,2,3,3,3,4,5], seed 106399+17m, sizes [16²,32×16,16×32,32²,32²,32²,64×32,32×64] and storage [F32,F32,F16,U8,U16,F32,F32,F16]. For m=0..5 use sequences [1,2,4,3,5,7], sample counts [2,3,5,6,9,11] and rotations [0,.125,−.1875,.375,.5625,−.125]. All 8,448 shadow pixels match. The 16² F32 output contains values as high as 35.6196213.

### Paper, brick, fiber, mesh, polygon and shape roots

All eight roots produced 19 whole outputs apiece. Across 152 renders, 915,896/1,140,736 pixels matched. Every root differs in at least one recorded case. Integer error counts storage levels. Floating-point error is the largest absolute difference in a stored channel.

| Root | Exact whole pixels / compared | Maximum U8 / U16 / F16 / F32 error |
|---|---:|---|
| paper_path | 135,674/142,592 | 0 / 1 / 0.0009765625 / 5.69224358e-06 |
| brick_1 | 139,565/142,592 | 0 / 1 / 0 / 6.94394112e-06 |
| brick_2 | 139,074/142,592 | 0 / 1 / 3.81469727e-06 / 1 |
| fibers_2 | 139,312/142,592 | 0 / 2 / 0.000244140625 / 3.05175781e-05 |
| mesh_1 | 41,087/142,592 | 0 / 5317 / 0.0375976562 / 0.0811322033 |
| mesh_2 | 39,574/142,592 | 0 / 9102 / 0.0595703125 / 0.138887614 |
| polygon_2 | 141,734/142,592 | 0 / 4 / 3.05175781e-05 / 6.10351562e-05 |
| shape_base | 139,876/142,592 | 0 / 1 / 0.00048828125 / 1.37090683e-06 |

Use the four archived cases, the [four-format sweep and independent seed](#four-format-inventory-comparison), and ten additional renders. Use the zero-based inventory index i and seed 103547+17i plus [0,1,2,2,2,2,6,7,8,9]. The sizes are [16²,32²,32²,32²,32²,32²,32×16,16×32,64×32,32×64]. Use [F32,F32,U8,U16,F16,F32,F32,U16,F32,F16] and settings [defaults,A,B,B,B,B,C,C,B,A]. Apply the controls exposed by the selected root.

| Controls | A | B | C |
|---|---|---|---|
| Paper width, height, disorder | 100, 100, 0 | 73.5, 119.25, .375 | 137.25, 65.5, .8125 |
| Integer scale, edge smoothness, interstice width | 1, 0, 0 | 3, .15625, .21875 | 5, .546875, .0625 |
| Nonsquare expansion, tiling, rotate 45° | on, 1, off | off, 3, on | on, 2, off |
| Boolean rotation, polygon sides, polygon scale | off, 3, 1 | on, 5, .75 | off, 9, 1.1875 |
| Polygon rotation, curve, gradient, invert gradient | 0, 0, 1, off | .125, .3125, .625, on | −.21875, −.1875, .1875, off |
| Autoscale, pattern, pattern-specific control | on, 2, 0 | off, 5, .1875 | on, 9, .5625 |
| Shape scale, XY size, angle, nonsquare | 1, (1,1), 0, on | .8125, (.625,.8125), .21875, off | 1.1875, (1.1875,.4375), −.3125, on |

For each connected pattern stage retain the shortest path to an authored stamp, breaking ties by connection identifier, and clear image inputs. For the shape root use its single authored quadrant and choose square or disc through its published pattern control. Use the nine cases excluding 64×32. Each variant covers ten stages in 90 renders.

| Root | Authored shape numbers | Authored exact / compared | Maximum authored U8 / U16 / F16 / F32 error | Square exact / compared | Disc exact / compared |
|---|---|---:|---|---:|---:|
| paper_path | 15 | 8,446/8,448 | 0 / 0 / 0 / 1.07288361e-06 | 8,448/8,448 | 8,448/8,448 |
| brick_1 | 9 | 8,173/8,448 | 0 / 1 / 0 / 1.7285347e-06 | 8,448/8,448 | 8,448/8,448 |
| brick_2 | 9 | 8,330/8,448 | 0 / 1 / 0 / 3.08454037e-06 | 8,448/8,448 | 8,448/8,448 |
| fibers_2 | 6 | 985/1,728 | 0 / 1 / 0 / 5.96046448e-07 | 1,728/1,728 | 1,728/1,728 |
| mesh_1 | 13 | 2,000/3,264 | 0 / 1 / 3.05175781e-05 / 1.07288361e-06 | 3,264/3,264 | 3,261/3,264 |
| mesh_2 | 15 | 2,380/3,456 | 0 / 1 / 0.00048828125 / 2.83122063e-06 | 3,456/3,456 | 3,456/3,456 |
| polygon_2 | 3, 10, 16 | 21,510/25,344 | 0 / 1 / 0.00048828125 / 4.17232513e-07 | 25,344/25,344 | 25,344/25,344 |
| shape_base | published pattern + 1 | 7,951/8,448 | 0 / 1 / 0 / 1.49011612e-07 | 8,448/8,448 | 8,448/8,448 |

Authored shapes match 59,775/67,584 pixels, squares 67,584/67,584, and discs 67,581/67,584. The reference produced nonzero pixels in 89, 90 and 89 of the 90 renders respectively. The three unequal disc pixels are all in the first mesh root. Its disc maximum U8 / U16 / F16 / F32 error is 0 / 65535 / 0 / 1.00211096.

Four roots also have downstream stages. Replace the source patterns with the original fields, sizes and input storage used in the [dirt source substitutions](#dirt-noise-sources-and-downstream-stages). Each row covers ten outputs. The two mesh roots produce identical residual totals with these inputs. Their 32×64 F16 field cases account for their maximum floating error.

| Root | Exact pixels / compared | Maximum U8 / U16 / F16 / F32 error |
|---|---:|---|
| fibers_2 | 10,496/10,496 | 0 / 0 / 0 / 0 |
| mesh_1 | 8,984/10,496 | 0 / 1 / 0.25 / 1.52587891e-05 |
| mesh_2 | 8,984/10,496 | 0 / 1 / 0.25 / 1.52587891e-05 |
| polygon_2 | 10,496/10,496 | 0 / 0 / 0 / 0 |

### Bounded traversal of deep patterns

Four roots reached the traversal guard even at 4×4. An instrumented release cook raised the visit ceiling from 1,048,576 to 2,097,152 and the raster copy ceiling from 1,048,576 to 8,388,608 for this experiment. The 256,000,000-pixel-work guard remained active. Use defaults, F32, and seeds 106622, 106635, 106765 and 106778 in table order. Each process cooked one graph on one worker. Peak resident memory includes graph loading. The visit and stamp counts are for the most expensive pattern stage. Times are single observations.

| Package | Whole pixels exact | Maximum F32 error | Visits / emitted stamps | Cook seconds | Peak MiB | First differing stage |
|---|---:|---:|---:|---:|---:|---|
| noise_directional_noise_3 | 13/16 | 2.50339508e-05 | 1,398,103 / 1,392,640 | 14.65 | 291.0 | Depth-ten pattern, 13/16 U16 pixels, one level |
| noise_directional_noise_3_v2 | 16/16 | 0 | 1,430,879 / 1,392,640 | 42.12 | 291.6 | All observed stages exact |
| noise_dirt_4 | 0/16 | 0.103746146 | 1,398,103 / 1,398,080 | 13.63 | 289.4 | Sharpen, after an exact 2×2 U16 source |
| noise_dirt_4_v2 | 0/16 | 1.63018703e-05 | 1,463,647 / 1,398,080 | 42.36 | 290.2 | Sharpen, after an exact 2×2 U8 source |

Both dirt graphs already match all four source texels before sharpening. In the first graph a reference F32 edge is −.103746146 and the local edge is zero. The second has only small positive differences. In the first directional graph the two smaller patterns and their blend match exactly. Three values differ by one stored level in the deepest pattern, and those differences reach the final Levels node.

For a traversal-only control, connect all four slots of each empty quadrant to the next quadrant, down to depth nine or ten. Use absolute 4×4 F32 output, zero roughness, unit global opacity, additive axis-aligned square leaves, gray level 2⁻²⁰, and seed 107011 + 17 × depth + 4. Use leaf size (1,1) for one cell or (2^depth,2^depth) for an image-wide square. The input consists entirely of these original parameters.

| Depth and footprint | Visit ceiling | Exact pixels | Maximum error | Cook seconds | Peak MiB |
|---|---:|---:|---:|---:|---:|
| 9, cell | 1,048,576 | 16/16 | 0 | 0.94 | 190.6 |
| 9, wide | 1,048,576 | 16/16 | 0 | 0.95 | 190.4 |
| 10, cell | 1,048,576 | Execution budget | — | 2.55 | 239.3 |
| 10, cell | 2,097,152 | 16/16 | 0 | 3.98 | 225.2 |
| 10, wide | 1,048,576 | Execution budget | — | 2.48 | 224.8 |
| 10, wide | 2,097,152 | 16/16 | 0 | 3.80 | 225.4 |

Depth ten produces 1,398,101 visits and 1,048,576 stamps. Raising only the visit ceiling renders both footprints. The cell case produces exactly one increment per pixel. In the wide case each pixel receives 409,600 to 802,816 increments. Both controls match every stored bit. At depth nine the wide case receives 102,400 to 200,704 increments. Dropping the last subdivision would therefore change the output.

For a separate partition test, isolate the deepest pattern of each directional graph and the sole pattern of each dirt graph. At the first quadrant whose four outputs point to the same child, keep just one connected slot. Repeat with each of the four slots, retaining its original slot number. Render 4², 8², and 8×4 F32, using seeds 103307 + 13 × the zero-based inventory index + 0, 1 and 2 respectively. Use the first two distinct default/changed settings from the linked family measurements. All 48 partitioned renders complete with the standard guard. They match 160/1,120 pixels, with maximum F32 error .0000105798244. These are separate quadrant images. Full-root parity remains open for all four graphs.

### Clamped texture reads in spline graphs

The sampling mode stored with an image slot controls both filtering and edge addressing. Modes 0 and 1 repeat the image, with nearest and bilinear filtering respectively. Modes 2 and 3 clamp to its edges with the same two filters. Previously the two clamped modes both used bilinear repeat. At a spline boundary this mixed the opposite end of the tangent field into the first sample.

| Whole root | Renders | Previously exact | Exact / compared | Pixels in U8 / U16 / F16 / F32 | Maximum channel error |
|---|---:|---:|---:|---|---:|
| compute_spline_tangents | 27 | 43,296 | 43,776/43,776 | 7,168 / 7,168 / 7,168 / 22,272 | 0 |
| modify_spline_property_sub | 19 | 24,317 | 28,928/28,928 | 5,120 / 5,632 / 7,168 / 11,008 | 0 |
| resample_spline_data | 19 | 29,816 | 29,952/29,952 | 5,120 / 5,632 / 7,168 / 12,032 | 0 |
| select_spline_sub | 19 | 583,680 | 622,592/622,592 | 65,536 / 98,304 / 98,304 / 360,448 | 0 |
| spline_warp_sub | 19 | 28,448 | 28,928/28,928 | 5,120 / 5,632 / 7,168 / 11,008 | 0 |

All five roots match 754,176/754,176 pixels. The tangent root includes the 18 independently wired line, circle and signed-curve outputs in the [spline helper controls](#wired-spline-helpers-and-shape-mapping). Three other roots use the same sampler modes in their graph definitions. The grayscale grid atlas and the sorting iteration in each of the quantization and shape-splatter packages still match all 344,064 pixels in their 12 archived renders.

For the four other spline roots, add ten cases to the nine archived and changed-precision controls per root. Use sizes, formats and A/B/C settings from the [shared lookup matrix](#shared-lookups-and-environment-shadow-pass). The base seed is 109121 + 17 × the zero-based inventory index. Add 0, 1, 2, 2, 2, 2, 6, 7, 8 and 9 in case order.

For field A/B/C let m be 0/1/2. Defaults use m=0. At each of the first two rows j=0,1, use angle 2π((x+.5)/width+m/16+j/8), center (.34375+.28125j,.59375−.1875j) and radii (.1875,.1875), (.25,.09375) or (.15625,.28125). Fill the coordinate map with (cx+rx cos angle, cy+ry sin angle, 1, −1) and the data map with (−sin angle, cos angle, 0, 1). Other rows are zero. At normalized pixel centers u,v, the UV input is (u,v,1−u,1). The vector map is (.5+.25 cos(2π(u+v)), .5+.375 sin(2πu), .5, 1). Use u for the attenuation curve. Other scalar fields have H=((11x+7y+5xy+13m) mod 251)/250. Their color equivalents use (H,1−H,.125+.75H,1). Supply the fields as linear F32.

| Root and controls in order | A | B | C |
|---|---|---|---|
| Modify property, sampling mode / property / opacity / blending mode | 0 / 0 / 1 / 0 | 1 / 1 / .6875 / 1 | 1 / 2 / .3125 / 2 |
| Select, spline index / output / selection mode / range start and end / normalized start and end | 0 / 0 / 0 / 0…1 / 0…1 | 1 / 1 / 1 / 0…1 / .1875….8125 | 0 / 1 / 0 / 1…2 / .3125….6875 |
| Warp, intensity / center / sampling mode / vector map / attenuation curve / start and end attenuation / intensity map tiling | 0 / .5 / 0 / off / off / 0 and 0 / 1 | .1875 / .375 / 1 / on / on / .15625 and .21875 / 2 | .34375 / .6875 / 0 / on / on / .25 and .09375 / 3 |

Resample uses its published defaults. In the original pixel probe, fill an 8×4 image with Q=(79x+53y+29xy+17c+11xc+3) mod 251 in channel c. Store Q for U8, 251Q for U16, or (Q−91.75)/71.375 for F16 and F32. In a Pixel Processor read color or luminosity at UV×gain+offset. Broadcast the luminosity to four channels. Test all four sampling modes in each source format.

| Output and extent | UV gain | UV offset |
|---|---|---|
| 16² F32 | (1,1) | (0,0) |
| 32×16 F32 | (2,2) | (−.5,−.5) |
| 16×32 F32 | (−1.75,2.5) | (1.34375,−.6875) |
| 32×16 U8, U16, F16 | (2,2) | (−.5,−.5) |
| 8×4 F32, five separate reads | (0,0) | (0,0), (1,1), (−.125,.25), (.3359375,.40625), (1.3125,−.375) |
| 32² F32 | (.8125,−.6875) | (.1875,.875) |

Start at seed 109847 + 17 × the input storage index (U8, U16, F16, F32), adding one per row or repeated read. The 384 pixel-processor renders improved from 76,755 to 128,000/128,000 exact pixels. The maximum became zero in every output format. Repeat modes 2 and 3 in the [uniform texture controls](#texture-sampling-in-pattern-functions). Use the same 13 positions and add controls running mode 0 or 1 at coordinates clamped to [1/64,63/64]. Across 416 pattern renders, 262,144 pixels previously matched and all 425,984 now match. The 104 typed Float4 values improved from 64 to 104 exact, maximum error zero. Uniform bilinear reads clamp each tap index and retain the original fractional weights. At (1.0311,−.65315) on the F32 source, the unclamped fractions produce green bits `bed93e38`, one bit different from clamping the coordinate before interpolation.

Six original sampler packages saved by MaterialCraft reopened in the reference. Across six reads per package, all 15,040 pixels and all 12 Float4 values matched the respective original packages, maximum error zero. The two pixel graphs used modes 2 and 3 on F32 color and U16 luminosity. Each uniform scope used nearest and filtered clamp on the U8 color source.

Two further roots with these modes remain open. The shape-splatter normal processor matches 666/142,592 pixels over 19 renders. Maximum finite U8 / U16 / F16 / F32 errors are 255 / 65,462 / .998570919 / .998893440. Some outputs contain nonfinite channels. The point preview returned 27,904/27,904 equal final pixels in 15 comparable cases, but the text stage reports an external-font error. In the three separate count/offset wrappers the counts are 4, 8 and 12 and the offsets are 0, 1 and 2. The error prevents a whole-root comparison in those wrappers.

### 3D noise with two original permutation tables

The basic and ridged 3D noise roots matched all 872,960 output pixels across 112 renders with identical procedural lookups. In every render the Bitmap in each application also matched all 16,384 U16 lookup texels. Replacing one lookup with the other changed the result in all 17 basic cases and all 39 ridged cases. It changed 172,483 and 261,463 pixel values respectively.

| Whole root | Renders | Exact / compared | Pixels in U8 / U16 / F16 / F32 | Maximum error |
|---|---:|---:|---|---:|
| 3D_perlin_noise | 34 | 347,136/347,136 | 231,424 / 34,816 / 38,912 / 41,984 | 0 |
| 3D_ridged_noise_fractal | 78 | 525,824/525,824 | 239,616 / 43,008 / 47,104 / 196,096 | 0 |

For each table initialize Python 3.12 `random.Random` once, using seed 104729 for the first and 1299709 for the second. In each of 16 rows, shuffle a fresh list of integers 0 through 1023 using that generator. Store each shuffled value k at level `round(k × 65535 / 1023)` in a linear grayscale 1024×16 U16 image. Bind the image to the active Bitmap in both applications. Repeat the [seven basic cases](#basic-and-ridged-3d-perlin) and the [29 ridged cases](#natural-exponential-and-ridged-noise) with each table.

Add ten position fields per root and table. At integer coordinates x,y let u=(x+.5)/width, v=(y+.5)/height and R=((31x+47y+13xy+17m) mod 251)/250. Store the linear RGBA32F vector (u+.3125v−.25, 1.375v−.1875u−.125, 2R−.875+.0625m, 1). Use seed 134269+17m. Setting A uses defaults. B/C use scale 6/11 for basic noise. For ridged noise use octave levels 1–4 and roughness .6875 for B, and levels 2–6 and roughness .3125 for C.

| Extent | Storage | m | Setting |
|---|---|---:|---|
| 16² | F32 | 0 | A |
| 32×16 | F32 | 1 | B |
| 16×32 | F32 | 2 | C |
| 32² | U8, U16, F16, F32 | 1 | B |
| 8×32 | F32 | 3 | C |
| 64×32 | F32 | 2 | B |
| 32×64 | F16 | 1 | C |

After the sampler and Sharpen changes, 24 repeat renders reproduced all 21,504 pixels bit for bit. The inventory status covers the common procedural inputs. The applications' bundled tables give different default output. On the 128² seed-zero ramp, basic noise matched 173/16,384 U8 pixels (maximum 203 levels) and ridged noise matched 176/16,384 (maximum 139 levels). [Bundled table measurements](#natural-exponential-and-ridged-noise)

### Tile, weave, splatter and fiber stage comparisons

Across 25 roots, 3,294,234/3,483,648 whole output pixels matched in 474 renders. All 25 had a measured difference. Another 18 renders of wood_fibers_2 reached the traversal limit before producing a whole image. The numbers below use zero-based inventory indices to distinguish graphs with identical names. Error is the largest stored-channel difference, in levels for integers and absolute units for floats. A dash indicates an unavailable comparison.

| Index and graph | Renders | Exact whole pixels / compared | Maximum U8 / U16 / F16 / F32 error | First differing stage IDs in the ten new renders |
|---|---:|---:|---|---|
| 374 · tile_generator_color | 19 | 137,997/142,592 | 0 / 49 / 0.0009765625 / 0.00121033192 | 1238191865 pattern |
| 381 · weave_1 | 19 | 139,840/142,592 | 0 / 1 / 0 / 1.52587891e-05 | 1064675946 pattern |
| 382 · weave_2 | 19 | 140,765/142,592 | 1 / 59 / 0.0009765625 / 0.00169374049 | 1298573641 pattern, 1298573810 pattern |
| 383 · pavement_1 | 19 | 141,408/142,592 | 0 / 2 / 0 / 2.06083059e-05 | 1047650292 pattern |
| 416 · sphere_displace | 19 | 142,163/142,592 | 0 / 329 / 0.000244140625 / 0.000122070312 | 1335843323 pixel processor |
| 418 · perlin_noise_zoom | 19 | 131,983/142,592 | 0 / 1 / 0.000244140625 / 8.34465027e-07 | 1051010199 pattern |
| 438 · kmeans_iteration | 19 | 77,776/77,824 | 0 / 0 / 0 / 8.94069672e-08 | 1502542656 pixel processor |
| 441 · radial_blur_grayscale | 19 | 121,514/142,592 | 1 / 14 / 0.00048828125 / 0.000137329102 | 1310833960 pattern, 1312489342 pattern |
| 463 · scratches_1 | 19 | 133,530/142,592 | 0 / 3 / 0.00048828125 / 4.06503677e-05 | 1028634246 pattern |
| 464 · scratches_2 | 19 | 118,291/142,592 | 1 / 2 / 0.00048828125 / 4.4465065e-05 | 1028227090 pattern |
| 465 · scratches_4 | 18 | 106,551/126,208 | 1 / 5 / 0.00048828125 / 4.57763672e-05 | 1047058056 pattern |
| 466 · shape_base | 19 | 139,931/142,592 | 0 / 1 / 0.00048828125 / 1.37090683e-06 | 1041346098 pattern |
| 488 · splatter | 19 | 142,577/142,592 | 0 / 91 / 0.00122070312 / 0.00137332082 | 1028307162 pattern |
| 489 · splatter_circular | 19 | 139,297/142,592 | 231 / 59438 / 1 / 1.00000006 | 1048865333 pattern |
| 490 · splatter_circular_color | 19 | 140,218/142,592 | 247 / 65535 / 0.96862793 / 0.996078431 | 1048865333 pattern |
| 491 · splatter_color | 19 | 142,551/142,592 | 1 / 83 / 0.00134277344 / 0.00141908228 | 1028307162 pattern |
| 509 · starburst | 19 | 141,503/142,592 | 0 / 1 / 1.52587891e-05 / 1.52592547e-05 | 1065612277 pattern |
| 520 · tile_generator | 19 | 139,723/142,592 | 0 / 42 / 0.00048828125 / 0.000639975071 | 1238191865 pattern |
| 521 · tile_generator_color | 19 | 137,445/142,592 | 1 / 103 / 0.00146484375 / 0.00199997425 | 1067616990 pattern |
| 522 · tile_random | 19 | 111,364/142,592 | 223 / 57311 / 1 / 1.00000006 | 1126353318 pattern |
| 523 · tile_random_color | 19 | 111,228/142,592 | 250 / 65535 / 1 / 2.00000003 | 1126353318 pattern |
| 534 · vertical_noise | 19 | 142,088/142,592 | 0 / 2 / 0.00048828125 / 1.75237656e-05 | 1094664716 pattern, 1041349787 output |
| 537 · weave_1 | 19 | 139,856/142,592 | 0 / 1 / 6.10351562e-05 / 1.52587891e-05 | 1064675946 pattern |
| 538 · weave_2 | 19 | 138,111/142,592 | 0 / 3 / 0.00048828125 / 4.57763672e-05 | 1035222092 pattern |
| 544 · wood_fibers_1 | 19 | 136,524/142,592 | 1 / 10 / 0.00048828125 / 7.51614571e-05 | 1035311965 pattern |
| 545 · wood_fibers_2 | 18 | 0 comparable outputs | 0 / 0 / 0 / 0 | pattern traversal |

Repeat the archived default, seed 42, changed controls when available, and the [four-format and independent-seed comparison](#four-format-inventory-comparison). For the ten additional renders use the extents, formats, settings and seed offsets in the [shared lookup matrix](#shared-lookups-and-environment-shadow-pass). The base seed is 104351+17i through index 441 and 107839+17i thereafter. The table gives changes in A / B / C order. Other parameters retain their published defaults.

| Inventory indices | Controls and their A / B / C values |
|---|---|
| 374 | x_amount 2 / 3 / 5 · y_amount 2 / 5 / 3 · pattern 2 / 1 / 3 · pattern_input_number 1 / 6 / 3 · image_input_filtering 2 / 1 / 0 · pattern_size (1,1) / (0.6875,0.8125) / (1.25,0.5625) · position_random (0,0) / (0.125,0.1875) / (0.3125,0.0625) · pattern_rotation_random 0 / 0.375 / 0.6875 · rotation 0 / 0.15625 / -0.21875 · global_opacity 1 / 0.8125 / 0.375 |
| 381 | Tiling 1 / 2 / 3 · rotate_45_degrees off / on / off · non_square_expansion on / off / on |
| 382 | scale 1 / 3 / 7 · Disorder 0 / 0.3125 / 0.8125 · rotate_45_degrees off / on / off · non_square_expansion on / off / on |
| 383 | Roudness 0 / 0.3125 / 0.8125 |
| 416 | size 1 / 0.8125 / 0.5625 · displacement_intensity 0 / 0.1875 / 0.625 · uv_scale (1,1) / (0.75,1.25) / (1.5,0.625) · uv_offset (0,0) / (0.1875,-0.28125) / (-0.3125,0.4375) · tilt_uvs off / on / off · samples 2 / 5 / 9 |
| 418 | Distance 2 / 5 / 11 · Disorder 0 / 19.375 / 61.25 |
| 438 | k 2 / 3 / 5 · ignore_alpha off / on / off · color_space 1 / 0 / 1 |
| 441 | samples 2 / 5 / 11 · angle 0 / 0.09375 / -0.21875 · center_position (0.5,0.5) / (0.375,0.6875) / (0.1875,0.8125) |
| 463 | Pattern_size_width 100 / 73.5 / 137.25 · Pattern_size_height 100 / 119.25 / 65.5 · Disorder 0 / 0.375 / 0.8125 · Rotation_Var 0 / 31.25 / 63.75 · Rotation 0 / 22.5 / -16.875 |
| 464 | Pattern_size_width 100 / 73.5 / 137.25 · Pattern_size_height 100 / 119.25 / 65.5 · Disorder 0 / 0.375 / 0.8125 |
| 465, 545 | Published defaults |
| 466 | pattern 1 / 5 / 8 · pattern_specific 0 / 0.1875 / 0.5625 · scale 1 / 0.8125 / 1.1875 · size_xy (1,1) / (0.625,0.8125) / (1.1875,0.4375) · angle 0 / 0.21875 / -0.3125 |
| 488, 491 | Pattern_size_width 100 / 73.5 / 137.25 · Pattern_size_height 100 / 119.25 / 65.5 · Disorder 0 / 0.375 / 0.8125 · Rotation_Var 0 / 31.25 / 63.75 · Rotation 0 / 22.5 / -16.875 · Grid_Number 2 / 3 / 5 · Pan_X 0 / 0.125 / -0.1875 · Pan_Y 0 / -0.0625 / 0.21875 · Gain 1 / 0.625 / 0.8125 · image_input_filtering 2 / 1 / 0 · Disorder_Random off / on / on |
| 489 | Number 2 / 5 / 7 · Radius 0.25 / 0.375 / 0.1875 · Radius_Random 0 / 0.21875 / 0.4375 · Pattern 2 / 1 / 3 · PatternSize (0.25,0.25) / (0.1875,0.3125) / (0.375,0.125) · PatternSize_Random 0 / 0.15625 / 0.4375 · Rotation 0 / 22.5 / -16.875 · image_input_filtering 2 / 1 / 0 · CenterOrientation off / on / off · RandomMask 0 / 0.1875 / 0.375 |
| 490 | number 2 / 5 / 7 · radius 0.25 / 0.375 / 0.1875 · radius_random 0 / 0.21875 / 0.4375 · pattern 2 / 1 / 3 · pattern_size (0.25,0.25) / (0.1875,0.3125) / (0.375,0.125) · pattern_size_Random 0 / 0.15625 / 0.4375 · rotation 0 / 0.21875 / -0.3125 · image_input_filtering 2 / 1 / 0 · center_orientation off / on / off · random_mask 0 / 0.1875 / 0.375 |
| 509 | Branches 3 / 7 / 11 · BlurAmount 0 / 1 / 2 · Randomness 0 / 0.375 / 0.8125 · SizeMin 0.125 / 0.21875 / 0.0625 · SizeMax 0.875 / 0.6875 / 1 · SizeVariation 0 / 0.3125 / 0.625 · Tiling 1 / 2 / 3 |
| 520, 521 | Nb_X 2 / 3 / 5 · Nb_Y 2 / 5 / 3 · Scale 1 / 0.8125 / 0.5625 · ScaleVariation 0 / 0.21875 / 0.4375 · Offset 0 / 0.375 / -0.25 · Pattern_Type 2 / 1 / 3 · image_input_filtering 2 / 1 / 0 · Free_Rotation 0 / 0.09375 / 0.21875 · Position_Random (0,0) / (0.125,0.0625) / (0.3125,0.25) · GlobalOpacity 1 / 0.6875 / 0.4375 |
| 522, 523 | x_amount 2 / 3 / 5 · y_amount 2 / 5 / 3 · pattern 2 / 1 / 3 · image_input_filtering 2 / 1 / 0 · pattern_rotation_random 0 / 0.21875 / 0.4375 · split_threshold 0.5 / 0.3125 / 0.8125 · shape_scale 1 / 0.6875 / 1.125 · position_random 0 / 0.1875 / 0.4375 · mask_random 0 / 0.125 / 0.3125 |
| 534, 544 | Disorder 0 / 0.375 / 0.8125 |
| 537 | Tiling 1 / 3 / 2 · Rotate_45_Degrees off / on / off |
| 538 | Tiling 1 / 3 / 2 · Rotate_45_Degrees off / on / off · Disorder 0 / 0.375 / 0.8125 |

For each bound pin use the flat, impulse, near-threshold and hashed fields in the [shared input recipe](#shared-lookups-and-environment-shadow-pass). Set n=4p+c for indices through 441, or n=4(p+1)+c for later indices, with p the zero-based pin position in the following table and c the channel. Single-channel inputs use c=0.

| Index | Bound pins in field order |
|---|---|
| 374 | pattern_input_1, pattern_input_2, pattern_input_3, pattern_input_4, pattern_input_5, pattern_input_6, background |
| 416 | input |
| 438 | input, seeds |
| 441 | input |
| 488 | Source |
| 489 | Source, background |
| 490 | Source, background |
| 491 | Source, Source_alpha |
| 520 | Source, background |
| 521 | Source, background |
| 522 | pattern_input, background_input |
| 523 | background_input_1, pattern_input |

To isolate the pattern stages, connect the identified stage directly to the output, disconnect its input images, and retain the shortest branch from the pattern root to a node with a nonempty or dynamic shape. For ties, visit the output slots in identifier order. Render once with its authored shape and again with a disc (pattern 3). Use the ten-case matrix indices 0, 2, 3, 4, 5 and 7. This retains the branch parameters, seeds and output-format inheritance. Stage dimensions can differ from the whole output. The counts cover every stored channel of each output pixel.

| Index and stage ID | Retained branch IDs, root to stamp | Authored exact / compared | Authored maximum U8 / U16 / F16 / F32 | Disc exact / compared | Disc maximum U8 / U16 / F16 / F32 |
|---|---|---:|---|---:|---|
| 374 · 1238191865 | 1299185944 → 1299185946 → 1032256671 → 1032256560 | 4,777/4,864 | 0 / 0 / 0 / 2.95042992e-06 | 4,864/4,864 | 0 / 0 / 0 / 0 |
| 381 · 1064675946 | 4087961384 → 4087966788 → 4087966766 → 4087966772 | 1,390/1,408 | 0 / 1 / 0 / 9.53674316e-07 | 1,408/1,408 | 0 / 0 / 0 / 0 |
| 382 · 1298573641 | 1298572907 → 1298572909 → 1298572911 → 1298574249 | 4,864/4,864 | 0 / 0 / 0 / 0 | 4,864/4,864 | 0 / 0 / 0 / 0 |
| 382 · 1298573810 | 1035222495 → 1035222491 | 1,183/1,216 | 0 / 0 / 0 / 5.36441803e-07 | 1,216/1,216 | 0 / 0 / 0 / 0 |
| 383 · 1047650292 | 1027784760 → 1027785067 → 1027785022 | 152/152 | 0 / 0 / 0 / 0 | 152/152 | 0 / 0 / 0 / 0 |
| 418 · 1051010199 | 1051010310 → 1051010203 | 3,649/4,864 | 0 / 1 / 0 / 8.34465027e-07 | 4,864/4,864 | 0 / 0 / 0 / 0 |
| 463 · 1028634246 | 4087623626 → 4087677897 → 1028225479 → 1028225487 → 1028634317 → 4087677903 | 4,863/4,864 | 0 / 0 / 0 / 1.14087015e-08 | 4,864/4,864 | 0 / 0 / 0 / 0 |
| 464 · 1028227090 | 4087623626 → 4087677895 → 1025020977 → 1025023617 → 1028227170 → 1028227669 → 1028227391 | 4,864/4,864 | 0 / 0 / 0 / 0 | 4,864/4,864 | 0 / 0 / 0 / 0 |
| 465 · 1047058056 | 4087623626 → 4087677895 → 4087677897 → 1047057855 → 1047057852 → 1027434391 → 4087677903 | 4,861/4,864 | 0 / 0 / 3.05175781e-05 / 2.4959445e-06 | 4,864/4,864 | 0 / 0 / 0 / 0 |
| 488 · 1028307162 | 1033746299 → 1033745600 | 4,864/4,864 | 0 / 0 / 0 / 0 | 4,864/4,864 | 0 / 0 / 0 / 0 |
| 489 · 1048865333 | 1048865351 → 1181069015 | 4,843/4,864 | 0 / 65535 / 0 / 0 | 4,603/4,864 | 255 / 65535 / 1 / 1 |
| 490 · 1048865333 | 1048865351 → 1181069015 | 4,834/4,864 | 0 / 53713 / 0 / 0 | 4,710/4,864 | 209 / 53713 / 0.819335938 / 0.819607973 |
| 491 · 1028307162 | 1033746299 → 1033745600 | 4,864/4,864 | 0 / 0 / 0 / 0 | 4,864/4,864 | 0 / 0 / 0 / 0 |
| 509 · 1065612277 | 1048629866 → 1048629846 | 1,290/1,408 | 0 / 1 / 9.53674316e-07 / 2.68220901e-06 | 1,408/1,408 | 0 / 0 / 0 / 0 |
| 520 · 1238191865 | 1032256671 → 1032256560 | 4,777/4,864 | 0 / 0 / 0 / 2.95042992e-06 | 4,864/4,864 | 0 / 0 / 0 / 0 |
| 521 · 1067616990 | 1032256671 → 1032256560 | 4,777/4,864 | 0 / 0 / 0 / 2.95042992e-06 | 4,864/4,864 | 0 / 0 / 0 / 0 |
| 534 · 1094664716 | 1028056080 → 1028056090 → 1028376477 → 1028376473 → 1028055564 | 2,430/2,432 | 0 / 0 / 0 / 5.96046448e-08 | 2,432/2,432 | 0 / 0 / 0 / 0 |
| 537 · 1064675946 | 4087961384 → 4087966788 → 4087966766 → 4087966772 | 1,390/1,408 | 0 / 1 / 0 / 9.53674316e-07 | 1,408/1,408 | 0 / 0 / 0 / 0 |
| 538 · 1035222092 | 1028306906 → 1028306857 → 4089929701 → 1035222497 → 1035222495 → 1035222491 | 1,405/1,408 | 0 / 0 / 2.38418579e-07 / 4.07162588e-08 | 1,408/1,408 | 0 / 0 / 0 / 0 |
| 544 · 1035311965 | 1029866950 → 1030039276 | 3,938/4,864 | 0 / 1 / 0 / 8.94069672e-07 | 4,864/4,864 | 0 / 0 / 0 / 0 |

The 120 authored single-branch renders matched 70,015/72,664 pixels. The 120 disc controls matched 72,249/72,664. A disc still differing leaves placement, coverage or evaluation upstream of the original shape open. At indices 441, 466, 522 and 523 the native runtime rejected the isolated packages. Whole-root and connected-stage counts are available for all four. Sphere displacement and K-means first differ in their pixel processors. Wood fibers 2 still needs a bounded whole-graph cook.

### Signed Sharpen stencils and stored inputs

Sharpen preserves negative lobes in floating-point output and uses the input's working storage when resizing. Its four-neighbor stencil starts with the weighted left texel. It adds the center, right, top and bottom terms in that order, fusing the product and addition at each step. With horizontal weight H and vertical weight V, the center weight is 1+2(H+V). H and V scale intensity by the shorter output side divided by the respective axis length. The two earlier sign and resizing regressions and two further accumulation checks failed on the preceding implementations. All 256 release node tests, formatting, strict workspace Clippy, dependency layers and WebAssembly passed.

| Original probe | Renders | Exact / compared | Maximum error |
|---|---:|---:|---|
| Impulse, signed ramp and constant, U16 and F32 sources | 16 | 5,632/5,632 | 0 in all four output formats |
| Grayscale and RGBA with output size inherited from the input | 108 | 10,512/10,512 | 0 in all four output formats |
| Output size set by parent | 108 | 38,549/38,832 comparable | 0 / 0 / 0 / 1.87000686 in U8 / U16 / F16 / F32 |
| Independent signed field with five nonzero strengths and changed sizes | 80 | 16,417/16,704 | 4.39762998 F32 |
| Dirt 4 and its second version, shared 2² sources | 8 | 136/136 | 0 F32 |

For the impulse matrix, place a single white texel at (8,8) in a black 16² image. Render at intensity one in all four formats and at 32² F32. At 16² the floating-point output is 5 at the center, −1 at its four axial neighbors and zero at the diagonals. In U8 and U16 the center is white and the neighbors black. Add the 16² signed field (((x+3y) mod 17)−5)/8 at intensities .375 and 0, and a flat .28125 at intensity 1. Use F32 inputs or clip and round to U16, and seeds 138559+11j in the order F32, F16, U8, U16, enlarged F32, two ramps and flat.

For the input-size and parent-size matrices, use source extents 2², 8×4 and 16². Test grayscale and four-channel inputs in U16 and F32. In channel c use level Q=(7919x+21473y+1103xy+2927c+139m+58997) mod 65536 for m=0 or 2. For m=1 use 59841−223c where (x+y+c) mod 3=0, and 3431+125c elsewhere. Write Q for U16 or Q/19001−.8125 for F32. Use the following cases with seed 134381+19j+i, where i is 1 for U16 and 3 for F32. The parent-size matrix has four additional size discrepancies at a requested 1². The reference returned 8×4. The pixel totals exclude those four outputs.

| Requested extent relative to source | Strength | Field m | Stored output |
|---|---:|---:|---|
| Same | 1 | 0 | F32 |
| Double both axes | 1, then 0 | 0 | F32 |
| Double both axes | .375 | 1 | F32, U8, U16, F16 |
| Four times width, twice height | 1.25 | 2 | F32 |
| Half both axes | 1 | 2 | F32 |

For the independent field use Q=(269x+1723y+257xy+9587c+7919((x+3y+5c) mod 13)+7537) mod 65536 at 8×4 or 16². Write Q for U16 or (Q−31007)/15019+.03125(x mod 3) for F32. Use F32 output. At the source extent use strengths 0, 1, .375, .8125, 1.34375 and 2.71875, then double both axes at 0 and .8125, and halve both axes at 0 and 1.34375. Use seed 142867+19j. The 287 differing pixels were in the reduced nonzero-strength cases. At 16² on an independent x/15 F32 ramp, strength one gives bits 3d888887, 3e4ccccf, 3eeeeef1 and 3f5ddddc at columns 1, 3, 7 and 13. Three finite ramp controls match 768/768 pixels. A separate constant 2×10³⁸ input produces positive infinity in all 256 reference pixels. MaterialCraft preserves that finite HDR plateau.

The two 4×4 reference outputs in the [deep traversal experiment](#bounded-traversal-of-deep-patterns) reproduce exactly when Sharpen alone receives the measured sources. Use U16 levels (30680,17968 / 0,24181) for dirt_4 and U8 levels (89,103 / 90,43) for its second version, in row order. Test 4² at strengths 1 and 0, 2² at 1, and 8×4 at .375. Use seeds 103307+13i at inventory indices i=266 and 267. All 136 F32 pixels match, including both whole-reference 4² outputs. Their full pattern traversal remains open. Dirt 6 improved from 117,826 to 125,252/129,280 exact pixels across 16 whole renders. Its U8 / U16 / F16 / F32 maxima are 0 / 8 / .000244140625 / .0000927746296.

A MaterialCraft-saved U16 input graph and an independently saved RGBA32F graph reopened in the reference. The first six cases of each reproduced all 1,620 stored pixels byte for byte.

## 2026-11-19 root comparisons

### Texture sampling in pattern functions

The Bevel Smooth debug position root matches **354,560/354,560 RGBA pixels in 38 renders**, maximum error zero. This includes 12,288 pixels in each of U8, U16 and F16 and 317,696 in F32. Previously the 22 recorded cases matched 285,715/285,952 and the 16 offset controls matched 68,394/68,608. At 64² on the [position field](#segment-mapping-and-normal-graph-rechecks), the incoming image was exact but the last FX-Map drew a 17-pixel diagonal at x=22…38, y=x+9. Its texture read determined the location and angle. The upstream processor, both FX-Maps and the input and output bridges now match 368,640/368,640 pixels across 18 instrumented renders.

A pattern function wraps both axes when sampling an image, including when output tiling is disabled. A nearest read on an exact negative texel boundary takes the preceding texel. For example, (−.125,−.25) in a 32² image selects (27,23). The [value-function filter](#value-processor-texture-filtering) also applies to pattern reads. Its bilinear fractions retain F32 precision. For a gray read from a color source, filter each channel and then evaluate ((G+B)+R) × float32(1/3).

| Original sampler controls | Renders | Previously exact pixels | Exact / compared | Maximum error |
|---|---:|---:|---:|---:|
| Nearest and bilinear, color and gray, plus a disconnected sampler | 90 | 190,720 | 422,400/422,400 | 0 |
| Four input formats, signed coordinates and both output tiling settings | 208 | 36,864 | 212,992/212,992 | 0 |
| Separate Value Processors, U8 and F32 sources | 52 | — | 52/52 Float4 values | 0 |

For the first row, fill a square stamp from the sampled color, or broadcast the sampled luminosity with alpha one. Use source (x/N, y/N, ((47x+31y+7xy+13) mod 257)/256, 1). For N=16,32,64,128 read (.5,.5) at 16 and (.3125,.6875) at the other sizes. At 64² place a sample (.25,.5,.75,.999 or .001) texels past (20,44) on both axes. Also sample (−.125,.4375), (1.125,1.625), (0,0), (1,1), (.199,.78125) and (.49999,.50001). Finish with (.1234567,.7654321) at 32², (.4891,.9125) at 128² and (.103,.253) at 16². Seeds start at 3877 and increase by 17. Repeat on an empty second image socket with the nearest color sampler.

For the second row use a 32² field with RGB level L=((47x+31y+7xy+13+c(29+3x)) mod 251) in channel c and alpha level 255. For U16 multiply all four levels by 251. For F16/F32 take (L−99.3125)/31.375 and alpha one. Test color and gray in both sampling modes with transparent black behind the stamp. In order, use (−.125,−.25), (−4.001,−7.999)/32, (0,0), (1,−1), (2.25,−1.625), (.3125,.6875), (.5,6.5)/32, (.0013121,.998713), (.71897,.39151), (.625,.427), (−.12,.016), (1.0311,−.65315) and (.920211,.090037). Start at seed 3989 plus the storage index in U8/U16/F16/F32 order and add 11 per coordinate. Enable tiling when the storage index plus the bilinear and grayscale flags is odd. At (.71897,.39151) the U8 bilinear red has F32 bits `3e90e1aa` and its luminosity has bits `3ef8d016`. The separate value graphs use the same coordinates and return the color directly.

For the 16 additional Bevel renders set position offset (1,1) and supply the constant RGBA32F field (p.x+d.x, p.y+d.y, .375, a). Default to p=(.3125,.6875), a=1, debug mode on, 64² F32. Start with p=(.5,.5) at 32² and d=(0,0), then the default p with d=(0,0). Use d=(.1875,−.1875) in all four formats, then (.1875,.1875), (.1875,0) and (0,.1875). Repeat (.1875,−.1875) with a=0, then at 128² and 32² with a=1. For the mode-off cases use p=(.25,.75), d=(.0625,.0625) and then d=(−.125,.375), a=.5 at 32². Finish with p=(.25,.75), d=(−.2,.3) and the default p with (.1875,−.1875), mode on. Seed j is 3659+17j, starting at j=0.

Standalone signed rectangles and discs still render at both signs of either size component. The 60 color/gray rectangle and gray disc renders match 310,272/310,272 pixels. At 64² a rectangle centered at (.4766544,.6173407), size (.37573713,−.01), rotated −.37499994 turns covers 17 pixels in both applications. A separate directional gradation on the same five geometries and four sign combinations matches 103,316/103,424 pixels. Its maxima are 0 U8, 1 U16, .00006103515625 F16 and .00000011920929 F32.

### Diffuse and specular outputs of the mesh material graph

Both material outputs match across **42 renders and 46 output images**, totaling **358,912/358,912 RGBA pixels**. The four archived renders each supply both outputs. The two four-format sweeps, two independent 32² cases and 28 additional renders each select one output. The same inputs before the [Add Sub rounding fix](#source-alpha-rounding-in-highpass) gave 340,909/358,912 exact pixels. Maximum component errors were zero U8 levels, one U16 level, zero F16 and 1.19209290e−7 F32.

| Output storage | Exact pixels / compared | Maximum error |
|---|---:|---:|
| RGBA8 | 245,760/245,760 | 0 |
| RGBA16 | 16,384/16,384 | 0 |
| RGBA16F | 16,384/16,384 | 0 |
| RGBA32F | 80,384/80,384 | 0 |

Each new render reads the selected output, all 15 filters and all five input bridges. All 1,576,200 pixels across those 588 images match. To reproduce the inputs set H=((31x+53y+7xy+23v+67c+3cy+19) mod 509)/508 for channel c. In order, assign v=0–4 to diffuse, specular, world-space normal, occlusion and curvature. The last two use channel zero. For the normal set X=.625(H₀−.5), Y=.75(H₁−.5), Z=sqrt(1−X²−Y²), then encode (.5+.5X,.5+.5Y,.5+.5Z,1). Supply original F32 images on all five ports.

The four controls below are pre-lighting, occlusion, edge sharpness and specular occlusion in that order. For each output start at 16² F32 with defaults. Render 64² in U8, U16, F16 and F32 with (.3125,.6875,.4375,.5625). At 32² F32 set just one control to .625 in each of four renders. At 128² F32 use (.8125,.1875,.75,.375) and opaque diffuse and specular. Next use (.1875,.8125,.25,.875) for a 32×16 source at 32², then a 32² source with diffuse and specular RGB equal to 2.5H−.625. Finish with two 64² F32 renders using the original four controls, first with a flat normal (.5,.5,1,1), then with the original normal. Each series starts at seed 6761 and advances by 17.

The same change also reaches other unresolved roots. On the independent 32² F32 input, BW Spots 3 matches 629/1,024 pixels (maximum 3.91006470e−5), Normal to Height 270/1,024 (.305256728), Polygon 2 1,012/1,024 (.0000152587891), Slope Blur 0/1,024 (.768627450), and each grayscale Slope Blur 12/1,024 (.329411790). The other BW Spots 3 graph exceeds the pattern execution budget. Symmetry Slice matches its independent 1,024 pixels but only 11,445/16,384 on the changed four-format input. [Independent inputs and controls](#four-format-inventory-comparison)

### Global blend opacity in Normal to Height

Normal to Height matches **145,664/145,664 stored pixels across 18 renders**. U8 contributes 5,120 pixels in two renders, U16 121,856 in seven, F16 5,120 in two and F32 13,568 in seven. Each format has zero maximum error. The 11 renders instrumented before and after the fix previously matched 3,201/18,688 final pixels. At each of 92 reachable stages whose inputs and output fit within 128², the previous result was 2,181,843/2,796,800. The corrected result is 2,796,800/2,796,800, maximum zero across all 1,012 intermediate images.

The two earliest differences were 16² U16 Add Sub blends fed by transforms. Both inputs matched at both stages in all 11 renders. At a computed global opacity of 6.000000476837158, foreground 35,122 and background 32,767 give 37,476. A second foreground of 32,767 gives 32,766. Applying the authored opacity directly gave 61,021 and 32,761. The blend caps global opacity to [0,1] before applying the mask. With the same two inputs and F32 mask values 73/255 and 126/255 the expected levels are 34,115 and 32,767. The measured bound applies to the nine native blend modes, Copy through Divide. It is applied during rendering. Ordinary saves retain the authored opacity expression.

Use the four inventory renders at 128² and 256², seeds 0 and 42. The changed controls are normal format 1, relief balance (.2,.6,.8) and global opacity .6. The four 64² formats and the independent 32² field supply five further renders. For the nine additional inputs, start with tangent components R=((17x+23y+3xy) mod 31−15)/32 and G=((11x−7y+5xy) mod 29−14)/32. Store (.5+.5R,.5+.5G,.5+.5sqrt(1−R²−G²),1). Round U8 sources to the nearest byte with even ties. A flat field uses R=G=0. The impulse field is flat except for R=.625 at (16,16) and G=−.375 at (10,8). The following rows give every added run. [Four-format inventory comparison](#four-format-inventory-comparison)

| Size and seed | Input | Normal format, relief balance, global opacity | Output |
|---|---|---|---|
| 16² / 61253 | Flat U8 | Defaults | F32 |
| 32² / 61300 | Impulse F32 | 0, (.875,.125,.5), .5 | F32 |
| 32² / 61347 | U8 field | 1, (.1875,.6875,.8125), .5625 | U8, U16, F16, F32 |
| 64×32 / 61535 | F32 field | 0, (.25,.875,.375), .8125 | F32 |
| 32×64 / 61582 | U8 field | 0, (.25,.875,.375), .8125 | U16 |
| 64² / 61629 | F32 field | 1, (.1875,.6875,.8125), .5625 | F32 |

For an independent constant-opacity check use a single native blend with the original foreground, background and mask fields described under [source-alpha rounding](#source-alpha-rounding-in-highpass). Use unsigned 16² and signed 32² F32 inputs with source alpha enabled. For each native mode m from 0 to 8 set the primitive opacity to −.5, 0, .375, 1, 1.25 and the next F32 above 6. Use seed 71711+19m, plus one for the signed input. All 69,120 RGBA32F pixels in 108 renders match, up from 42,258. Comparing the native results for −.5 against 0 and both high settings against 1 gives 34,560/34,560 exact pixels.

The separate computed-opacity controls use grayscale or color inputs with and without a mask. Set opacity to −.5, 0, .375, 1, 1+2⁻²⁰, 1.25 and the next F32 above 6. For Add Sub test unsigned 16² F32, unsigned 32² U8, U16 and F16, and signed 32² F32. For color also test the three other alpha interpretations on signed 16² F32 inputs. For unmasked grayscale and masked color repeat the unsigned 16² F32 settings for the eight other modes. In all 42 groups the out-of-range native renders agree byte-for-byte with their respective endpoint. MaterialCraft matches 158,392/161,280 pixels across these 294 renders, up from 88,416. All 157,696 pixels in the 280 renders outside straight interpretation match. The signed straight-alpha controls remain open at 696/3,584 exact RGBA32F pixels, maximum component error 772.524188.

For these controls use L=(53x+17y+7xy+35v+61c+3cy+3) mod 251, with v=0,1,2 for foreground, background and mask, respectively, and c for the channel. Use L/255 for unsigned inputs and (L−83.125)/53.5 for signed images or (L−83.125)/101 for a signed mask. Grayscale image bridges store U16. For their source texels use (round(131071H)+97x+3y²+257v) mod 65536, where H is the unsigned or signed input value above. The first two row-zero foreground levels are 35,122 and 32,767, and both background levels are 32,767. Color and mask bridges store F32. Use seed 64109+37b+k, where b indexes the groups in the order given and k=0,1,2,3 for unmasked gray, masked gray, unmasked color and masked color.

The release also matches all 2,577,920 pixels across 164 renders of 18 previously measured blend roots. For file fidelity, save the current-header Normal to Height graph with the ordinary save command. Reopen it in the reference app using the nine inputs above, the changed 64² F32 input and the independent 32² input. The saved and original graphs match all 18,688 pixels with zero maximum error.

### Source-alpha rounding in Highpass

| Root | Renders | Exact pixels / compared | Maximum error |
|---|---:|---:|---:|
| Highpass, RGBA | 21 | 177,408/177,408 | 0 |
| Highpass, grayscale | 30 | 216,576/216,576 | 0 |
| Add Sub graph | 7 | 311,296/311,296 | 0 |

The RGBA high-pass uses the source as background and a blurred, inverted image as foreground. The input bridge, blur and levels matched before the fix. The first discrepancy was the final Add Sub blend at half opacity. Across the same 21 renders, 165,429/177,408 pixels matched before the change. Maximum errors were one U16 level and 1.19209290e−7 F32. The corrected RGBA counts are 122,880 U8, 8,192 U16, 8,192 F16 and 38,144 F32, all exact. In each of the 12 new renders of each high-pass graph, all five observed nodes match. The grayscale total also includes the [earlier nine field variations](#high-pass-anisotropic-noise-and-triangle-grid-controls).

For those 12 renders, channel c of the original F32 source is H=((31x+53y+11xy+67c+5xc+17) mod 509)/508. Use channel zero for grayscale. In order, use 16² F32 with the default radius, 32² F32 with radius zero, and 64² with radius 2.75 in U8, U16, F16 and F32. Repeat the 64² F32 case with alpha one. At 32² use radius .125 and alpha zero. At 128² use radius 6.5 and alpha one. Follow with a 32×16 source at 32² and radius 1.375, then 32² at radius .6875. In the last two cases RGB is 2.375H−.625. Finish by repeating the original 64² F32 radius 2.75 render. Seeds start at 5021 and advance by 19.

For source alpha, form a=1−(1−foreground alpha) in F32 and w=a×opacity. For each color channel, foreground contribution is ((background+2×foreground)−1)×w. Use a fused multiply-add to compute 1−a×opacity, then another to multiply that weight by background and add the foreground contribution. Coverage is the fused sum background alpha+(1−background alpha)×(foreground alpha×opacity). With background bytes (24,5,8,8), foreground (231,250,247,8) and opacity .5, the F32 result has bits 3ddad4cf, 3d0e0b89, 3d3cb8b5 and 3d3ebcbb.

The standalone Add Sub filter matches **59,904/59,904 pixels across 36 renders** using source, ignored or premultiplied alpha. These include enabled and disabled alpha blending, an absent or connected opacity mask, grayscale and color, and signed floating-point inputs. The three straight-alpha probes at 32² F32 remain open. Connected-mask and unmasked color match 64/1,024 and 62/1,024 pixels, maximum 2.38418579e−7 for both. Masked grayscale matches 604/1,024, maximum 1.19209290e−7. The Add Sub graph row reports its own seven renders.

To reproduce the filter controls use one graph per input layout, with foreground, background and optionally mask. In binding order use slot v=0,1,2 for foreground, background and mask. For each channel c use L=(53x+17y+7xy+35v+61c+3cy+3) mod 251. Use L/255 normally. For signed cases use (L−83.125)/53.5 for the images and (L−83.125)/101 for the mask. Masks and grayscale images take channel zero. Set image bridges to F32 for the color graphs and U8 for the grayscale graph. For each graph use 16² F32 at opacity 1, then 32² at opacity .5 in all four formats. At 64² F32 with signed inputs use .6875 with alpha blending disabled, then .3125 with it enabled. At 32² F32 and opacity .6137 run ignored, straight, premultiplied and source alpha. Finish with signed inputs at 16² F32 and opacity zero, followed by 64² F32 with unsigned inputs and opacity 1. Each series starts at seed 5113 and advances by 17.

### Stored input precision in box and directional blur

Dirt 1 matches **145,626/145,664 pixels across 17 renders**. Previously 144,472 matched, with maximum U16 error 304 levels. U16 now matches 123,869/123,904 with maximum error one level. All 5,120 U8 and all 5,120 F16 pixels match. F32 matches 11,517/11,520 with maximum 1.52587891e−5. The 64² case that previously had 255/256 exact pixels at the 16² U8 blur now matches 256/256 at that stage and 4,096/4,096 at the output. The repeated 64² case first differs at the U16 FX-Map, with 4,094/4,096 and maximum one level, while the U8 blur still matches 256/256. [Generator inputs, controls and seeds](#square-and-rectangular-pattern-generators)

A zero blur copies its resized input at the destination precision. With a nonzero intensity, the resize stores each level at the wider input or destination precision, including the final reduction. If the filter has a nonzero radius, it reads the resized texture at that working precision. A matching-size input retains its existing format. This applies to both filters even when the intensity is too small to cover another texel.

| Direct comparisons | Renders | Exact pixels / compared | Max U8 / U16 levels | Max F16 / F32 error |
|---|---:|---:|---:|---:|
| Box blur, mixed input sizes and storage | 133 | 238,336/238,336 | 0 / 0 | 0 / 0 |
| Box blur, zero through 4.001 | 48 | 49,152/49,152 | 0 / 0 | 0 / 0 |
| Box blur, signed subpixel settings | 18 | 1,152/1,152 | 0 / — | 0 / 0 |
| Both filters, signed and wider kernels | 36 | 2,302/2,304 | 0 / — | 0 / 2.98023224e-08 |
| Directional blur, mixed sizes and angles | 91 | 162,606/163,072 | 0 / 0 | 0 / 1.1920929e-07 |

For the mixed-size rows, generate the [original four-channel field](#stored-input-resizing-and-grayscale-levels-controls) with component c = ((131x+367y+29xy+41c+17xc+seed) mod 2053)/2052. For gray take the first component. Encode normalized inputs at their specified depth. For floating inputs use 1.5 times the component minus .25 in linear F32, then store at the indicated input depth. Before the blur, feed a neutral Levels at the input dimensions. At each index j below, use base seed 11677+11(250+j). Render 16² F32, 32² U8/U16/F16/F32, 64² F32 and 64² U16. Add 0, 1, 1, 1, 1, 5 and 6 to the base seed. Multiply the output width and height by two to the respective powers shown to obtain the input dimensions. The angle column gives the additional directional renders in turns.

| j | Input | Size powers (X,Y) | Tiling | Intensity | Directional angles |
|---:|---|---|---|---:|---|
| 0 | U16 gray | (2, 2) | XY | 0 | 0 |
| 1 | U16 gray | (2, 2) | XY | 3.2 | 0.25 |
| 2 | U16 gray | (2, 1) | XY | 0 | — |
| 3 | U16 gray | (1, 0) | XY | 0 | 0.125 |
| 4 | U16 gray | (-1, 0) | XY | 0 | — |
| 5 | U16 gray | (-2, -1) | X | 0 | -0.0625 |
| 6 | U16 gray | (-1, 1) | Y | 0 | — |
| 7 | U8 RGBA | (-2, -1) | XY | 0 | — |
| 8 | U8 RGBA | (-1, -1) | off | 0 | 0.1875 |
| 9 | F16 RGBA | (-2, -1) | XY | 0 | 0.125 |
| 10 | F16 RGBA | (1, 1) | XY | 0 | — |
| 11 | F32 RGBA | (2, 1) | XY | 0 | — |
| 12 | F32 RGBA | (-1, 0) | off | 0 | 0 |
| 13 | U16 gray | (-1, 0) | XY | 8.5 | 0, 0.25 |
| 14 | U16 gray | (1, 1) | XY | 9 | 0.125 |
| 15 | U16 gray | (2, 2) | off | 12 | 0.25 |
| 16 | F32 RGBA | (-1, -1) | X | 12 | — |
| 17 | F16 RGBA | (-1, 0) | Y | 6 | 0 |
| 18 | U8 RGBA | (-1, -1) | XY | 10 | 0.125 |

For the threshold series, take the 128² U16 field from j=0 at the 32² setting. Keep seed 15701 and store all four formats at intensities 0, 2⁻¹⁸, 2⁻¹⁰, .03125, .125, .5, 1, 1.5, 2, 3.2, 4 and 4.001. With U8 output, using destination precision throughout matches all 1,024 pixels at zero and 889/1,024 at 2⁻¹⁸. Using the wider precision reverses those counts.

For the two signed series, generate a 32² U16 image with level ((17x+113y+59xy+19)×7919) mod 65536. Reduce to 8² U8, F16 and F32. At seed 15709 test box intensities 0, 2⁻¹⁰, 3.2, 16, −2⁻¹⁰ and −3.2. At U8 the row-zero pixels at columns 2 and 5 are 145 and 139 at zero and 146 and 140 at ±3.2. At seed 15709, repeat both filter types at 0, 2⁻¹⁰, 3.2, −3.2, 64 and −64, with an eighth turn for directional blur. The +64 directional F32 result matches 62/64 pixels, maximum 2.98023224e−8. The independent directional angles retain the F32 differences shown above.

### Pattern rotation phase

An original horizontal gradient distinguishes .5 from 1.5 turns and .75 from 1.75 turns. The latter rotations leave a small cross-axis component. At 1.5 turns, a stamp four pixels wide and 256 pixels high gives a fitted component of 1.87244e−7. At −3.75 turns the corresponding component is −7.49176e−7. MaterialCraft’s pixel sine and cosine give 1.87261e−7 and −7.49045e−7 from the stored F32 angles (bits 34491200 and b5491200). The pattern renderer now uses that same phase calculation. Edges with an exactly zero axis component keep their image-space boundary rule.

| Original controls | Renders | Previously exact | Now exact / compared | Maximum absolute error by format |
|---|---:|---:|---:|---|
| Unit gradient, extended turns and neighboring F32 angles | 79 | 225,623 | 241,479/438,272 | 2.38418579e-07 F32 / 1 U16 |
| Thin gradient, extended turns | 57 | 3,560,754 | 3,595,790/3,735,552 | 8.34465027e-07 F32 |
| Unit gradient, cardinal rotations | 96 | 1,309,160 | 1,325,612/1,867,776 | 8.94069672e-08 F32 / 1 U16 |
| Unit gradient, oblique rotations | 16 | 208,057 | 211,579/311,296 | 2.38418579e-07 F32 / 1 U16 |
| Shape and render-region edge controls | 104 | 618,848 | 618,848/647,168 | 1.78813934e-07 F32 |
| Oblique square footprints | 78 | 4,542,719 | 4,542,720/4,542,720 | 0 F32 |

For the first row draw a centered unit horizontal gradient. On a 64² canvas, visit −4 through 4 turns in quarter turns, then 1.5 plus each of [−2⁻²⁰,−2⁻²³,2⁻²³,2⁻²⁰], .5 plus [−2⁻²⁰,−2⁻²⁴,2⁻²⁴,2⁻²⁰], and finally [.1875,−.1875,.625,.8125]. Use seed 42313+53j with zero-based angle index j. Add 256×32 and 32×256 at −2.5, −.5, .5, 1.5, 1.75, 2, 2.5 and 3.5 turns. Use F32 throughout and also U16 at −2.5, .5, 1.5, 1.75, 2.5 and 3.5. The extended-angle U16 subset goes from 122,078 to 122,042 exact pixels out of 122,880, with maximum error one level. For all 18 paired U16 and F32 settings, applying U16 storage to each app’s own F32 samples reproduces all 122,880 of its U16 levels. The 838 differing U16 samples originate before output storage.

For the thin-gradient row use a 256² F32 canvas, height 1 and widths 1/8, 1/32 and 1/64. Center the first two widths and offset the last by .25 pixel on both axes. Use turns [−3.75,−3,−2.5,−1.75,−1.5,−.5,0,.5,1.5,1.75,2,2.25,2.5,2.75,3,3.5,4,1.5−2⁻²³,1.5+2⁻²³] and seed 53851+43j+k for angle j and width k. The maximum pixel error drops from 2.39610672e−5 to 8.34465027e−7. The square that previously missed pixel (307,492) uses size (.755,.391), offset (−.1113,.1437), .13125 turns, seed 1108 and a 512² F32 output. Its right and lower neighbors remain outside. [Shape and render-region controls](#quarter-turn-stamps-and-render-bounds)

| Root | Renders | Previously exact | Now exact / compared | Maximum error in U8, U16, F16 and F32 |
|---|---:|---:|---:|---|
| Gradient Linear 1 | 47 | 435,120 | 435,744/435,968 | 1, 1, .000244140625, .0000152587891 |
| Gradient Linear 3 | 23 | 265,072 | 265,072/277,760 | 0, 2, 0, .0000305175781 |
| Arc Pavement | 27 | 206,904 | 206,911/211,200 | 0, 0, 0, .00000993907452 |
| Fibers | 24 | 416,631 | 416,773/424,192 | 0, 1, .0000305175781, .0000153183937 |
| Pattern Fibers | 24 | 416,669 | 416,799/424,192 | 0, 1, .0000305175781, .0000153183937 |

Arc Pavement previously had a maximum F32 error of .00000987946987. Gradient Linear 1 still has 16 unequal pixels in each of U8, U16 and F16 and 176 in F32. On the 24 profiles per root, Fluid and Noise Fluid give the same counts and maxima as the earlier [pattern-stage comparison](#fibers-and-fluid-pattern-stages). All 3,211,776 pixels from 159 equal-resolution renders of 19 previously matched pattern roots are exact. For the separate file check, save the original test graph, Gradient Linear 1, Fibers and Fluid through the normal save command with input header `1.1.0.202502`. Reopen each save in the reference app and compare to the original in that same app. Across 14 extended-angle controls and nine settings per root, all 134,144 pixels in 41 renders match, with maximum error zero.

### Pattern opacity near zero and color accumulation

| Measured output | Renders | Exact pixels / compared | Maximum component error |
|---|---:|---:|---:|
| Independent depth, roughness and opacity | 288 | 73,728/73,728 | 0 |
| Smallest stored gains, signed values and very large tints | 165 | 42,240/42,240 | 0 |
| Point-sampled original F32 gray and RGBA bitmaps | 32 | 8,192/8,192 | 0 |
| Positive and negative subnormal clear colors | 40 | 10,240/10,240 | 0 |
| Mixed-channel clear colors and transparent or signed squares | 56 | 14,336/14,336 | 0 |
| Three color blend modes on generated F32 backgrounds, four output formats | 63 | 76,032/76,032 | 0 |
| Earlier mixed-size backgrounds with a square or an opposite-type input | 105 | 188,160/188,160 | 0 |

The first row improves from 73,442 exact pixels and closes both 143-pixel discrepancies in the [depth controls](#stored-opacity-across-pattern-depths). A solid RGBA square with RGB 2⁶⁰, alpha .25, roughness zero and gain 2⁻¹²⁶ has all four components zero. With RGB 1, alpha −1, gain F32 bits `00800001` and a 2⁻¹²⁶ background, each result has bits `80000000`. The renderer flushes subnormal operands and intermediate products to signed zero before later stages can amplify them. For an authored gray clear color the mean uses F32 RGB before flushing. For example the mixed swatch (smallest normal, −next-to-largest subnormal, −next-to-largest subnormal) gives negative zero. A zero additive contribution retains the destination sign.

For the 165 depth renders use a 16² F32 output, seed 91813 + 7j, one square at depth 0, 7 of 8 or 10 of 10. At depth zero use signed gains with F32 magnitude bits 1, 00100000, 00370011, 007fffff, 00800000, 00800001 and 01000000. Use tints 1, 2⁶⁰, .25 and 2⁻¹⁶ over zero or 2⁻¹²⁶. At the other depths cross roughness 2⁻¹⁸, .0001, .00032 and 2⁻¹² with gains .37, 1, 2¹⁸ and 2⁻¹². For the point-sampled bitmap use a 16² field with white pixels or an eleven-entry repeated sequence [1, t, n, .25, 2⁶⁰, 0, −.3125, −n, −t, 2n, 1.125], where t has bits 00371245 and n is the smallest positive normal. Select element (11y + 3x + 5c) mod 11 for channel c. The clear-color series also draws on ±t, draws on mixed normal and subnormal color components and compares an empty map with a half-width square.

The 63 blend renders previously matched 66,507/76,032 pixels. F32 matched 38,861/48,384 with maximum .5. U16 matched 9,214/9,216 with maximum one level. F16 and U8 each matched 9,216/9,216. A built-in color stamp now uses the same measured accumulator precision as the bitmap stamp. With alpha .71875 and gain .625 over destination alpha `393fa080`, the stored result is `3ee60d32`. A single F32 multiply-add gives `3ee60d31`. On the 105 previous mixed-size renders this closes the two one-ULP alpha differences. [Background resize controls](#pattern-backgrounds-and-center-culling)

For each blend mode use one unit square and (R,G,B,A,gain) values (.5625,.3125,.8125,.71875,.625), (.35,−.6971,1.78125,.6337,.7723) and (−.372,.8123,1.326,1.1873,−1.359). Generate F32 RGBA backgrounds using PCG64 with seeds 100049 through 100052, drawing channels uniformly from [−1.25,1.625). Use 16², 32×64, 64×32 and 32² fields in order. Replace the first eight rows of field zero with F32 bits `393fa080 bb89bb22 3ee60000 393fa080`. Multiply field one by powers of two with integer exponents from −22 through 21, drawn from the same generator. Set R to zero in rows 0–7, A to one in rows 8–15 and G to negative zero in rows 16–23 of field two. Store field three in all four formats.

The remaining F32 gradient measurements separate coverage from the tint arithmetic. Across 204 gray and RGBA renders the actual output matches 48,128/52,224 pixels. A gain of 2⁶⁰ amplifies a coverage difference to 68,719,476,736. Using identical measured unit-tint coverage gives 52,224/52,224 exact pixels. Compare shapes 2 and 17 at 16², on zero or ±2⁻¹²⁶, with seeds 96259 + j. Additional connected F32 backgrounds match 854/2,048 pixels in eight renders, maximum 1.17549421e−38. This difference is visible with an empty pattern. The gray input bridge retains the negative sign for subnormal values and negative zero, while the empty gray output gives positive zero. The color output preserves their sign.

Six current-header saves reopened in the reference app and matched 34,816/34,816 pixels across 58 renders, maximum error zero. The 19 previously verified pattern roots retained 3,211,776/3,211,776 exact pixels in 159 renders, maximum error zero.

### Subnormal values in filtered pixel reads

The F32 bilinear Pixel Processor sampler flushes subnormal input components to signed zero before filtering. It applies the same rule to the filtered result. Nearest sampling and directly wired images preserve their input bit patterns. The 150 original renders below match **41,664/41,664 RGBA32F pixels**, maximum error zero. The first 60 previously matched 14,489/15,360, maximum 1.10202595e−38.

| Sample mode and field | Renders | Exact pixels / compared |
|---|---:|---:|
| U16, F16 and F32 fields sampled at output pixel centers | 60 | 15,360/15,360 |
| Mixed signed F32 fields at six input sizes, bilinear at output centers | 18 | 4,608/4,608 |
| Same fields, nearest at output centers or at (.390625,.34375) | 36 | 9,216/9,216 |
| Same fields, bilinear at (.390625,.34375) | 18 | 4,608/4,608 |
| Direct wire at the six original input sizes | 18 | 7,872/7,872 |

Use the [three-storage sampler fields](#signed-zero-in-floating-point-texture-filtering) for the first row. For the other rows use a 16² output and scale the source width and height by (1,1), (2,2), (½,½), (2,1), (1,2) or (½,2). At relative size j generate three fields with seeds 100151+19j+i. Use F32 bit patterns 00000000, 80000000, 00371245, 80371245, 007fffff, 807fffff, 00800000, 80800000, 00800001, 80800001, 009fffff, 80bfffff, 01000000, 81000000, 017fffff, 817fffff, 02000001 and 82000001. The first field samples this list uniformly with a seeded PCG64 generator in row, column and channel order. For the second field take index (5x+9y+3c) mod 18, with index 12+(c mod 2) where (floor(x/2)+3floor(y/2)+c) mod 7 is zero. For the last field take index (x+7y+c) mod 18, with −.25 where (x+y+c) mod 3 is zero. At output centers the native nearest reads preserve all 3,547 subnormal components. The filtered reads return signed zeros there when the result falls below the smallest positive normal.

Eight small capture graphs also tested the cost of loading the node library. Use the first row at size/storage pairs (0,U16), (1,F16), (2,F32), (3,U16), (4,F16), (3,F32), (0,F32) and (4,U16), including all four field variants for each pair. Running eight independent sessions in separate processes took 26.12 and 24.70 seconds. Running the same sessions in one process with a shared read-only library took 3.34 and 3.29 seconds. Test the two orders on one worker. Each run produced 96 identical images, 30,976/30,976 captured pixels, identical node values and errors, and zero maximum difference.

### Signs of zero in enlarged color backgrounds

MaterialCraft now preserves a negative zero in an enlarged connected RGBA pattern background when all three filtered RGB channels have their sign bit set. A positive sign in any RGB channel changes zero components to positive zero, including an alpha of negative zero. The rule applies at the final enlargement before output storage. A 2× enlargement of the constant (−0,−.375,.5,−0) gives (+0,−.375,.5,+0). With blue set to −.5 both negative zeros survive. Same-size controls preserve the original RGBA signs.

| Original control | Renders | Previous exact | Exact pixels / compared | Maximum numerical error |
|---|---:|---:|---:|---:|
| Vary RGB and alpha signs in independent spatial fields | 80 | 14,027 | 20,480/20,480 | 0 |
| Flat fields with mixed zero signs and ordinary values | 80 | 12,800 | 20,480/20,480 | 0 |
| Both sides of the first nonzero half value, four output formats | 120 | 30,131 | 30,720/30,720 | 0 |
| Earlier signed F32 inputs at six relative sizes | 72 | 17,764 | 18,432/18,432 | 0 |
| Earlier subnormal and normal threshold fields | 60 | 14,575 | 15,360/15,360 | 0 |
| Earlier U8, U16 and F16 sources | 120 | 30,559 | 30,599/30,720 | 0 |

For the first row render an empty color pattern at 16² F32. Use F32 and F16 RGBA sources at 16², 32², 8², 8×16, 16×8, 8×32, 4² and 32×4. Number those sizes j from zero, and the five fields i from zero. For the first three fields use sign mask (floor(x/2)+3floor(y/2)+5i) mod 16. For the last two use (3x+5y+3xy+i) mod 16. Bit c sets the sign of channel c, starting with red. Use zero magnitude in field 0. In field 1 use .375+.125c where (floor(x/2)+floor(y/2)+c) mod 3 is zero. In field 2 use .1875+.0625c where (5x+3y+c) mod 3 is zero. Elsewhere these two fields use zero. In fields 3 and 4 choose magnitude at index (x+5y+3c+i) mod 4 from (0,2⁻¹²⁶,2⁻¹²⁵,2⁻¹²⁷) for F32 and (0,2⁻²⁴,2⁻²⁰,.75) for F16. Use seed 102389+13j+i.

For the second row use both source formats, sizes 8², 16², 32², 4² and 8×32, and seed 102991+16j+i. Use the two constant examples above followed by (−0,−0,−0,+0), (+0,−0,−0,−0), (.25,−0,−0,−0), (−.25,−0,−0,.75), (−0,.25,−0,−.75) and (−0,−0,.25,−.75).

For the half threshold row use source sizes 16², 32², 8², 4² and 8×32. Draw three fields with sign mask (5x+3y+7xy+i) mod 16 and magnitudes (0,2⁻²⁴,2⁻²⁵,2⁻²³,2⁻²⁶,2⁻²¹,2⁻²⁴,.3125). For field 0 index the magnitudes by (floor(x/2)+5floor(y/2)+2c) mod 8. For fields 1 and 2 use (3x+y+3c+i) mod 8. Use seed 103109+7j+i and render both source formats to F32, F16, U16 and U8.

The 19 previously verified roots match all 3,211,776 pixels in 159 renders after the zero-sign correction. All 15,360 RGBA pixels in the earlier mixed-storage controls now match. The remaining 121 unequal grayscale pixels differ only in their zero sign. The reference input bridge already clears negative zero in the measured half-gray source. [Earlier texture filtering measurements](#signed-zero-in-floating-point-texture-filtering) · [Original background inputs](#connected-floating-point-pattern-backgrounds)

### Signed zero in floating-point texture filtering

A bilinear footprint containing only zeros retains a negative sign in a channel when every contributing texel has a negative sign. Texels with zero weight have no effect. On 20 original color Pixel Processor renders, all **20,480/20,480 F16- and F32-source component signs** match this rule. The full 60-render matrix improved from 10,741 to 14,489/15,360 bit-exact RGBA32F pixels. The remaining maximum absolute error is 1.10202595e−38.

At a 16² output sample the original field at each pixel center using bilinear filtering. Use source sizes 32², 8², 32×16, 16² and 8×32. Test U16, F16 and F32 inputs at each size. First fill all channels with negative zero, then use negative zero except where (7x+3y+c) mod 11 is divisible by five. These two fields on floating sources supply the sign comparisons above. For a mixed field use index (7x+3y+c) mod 11 into [0,−0,0,−0,−0,q,−q,−2q,4q,.125,−.125], with q=2⁻¹²⁶ for F32 and 2⁻²⁴ for F16. For the last field replace each negative zero of the second field with −2×2⁻¹²⁶ and each positive zero with negative zero. Encode the U16 controls as level zero for zero values and 32333 otherwise. Use seeds 100397+11s+j for size index s and field index j.

The [connected pattern background controls](#connected-floating-point-pattern-backgrounds) now match 2,048/2,048 on the eight 32²-to-16² threshold renders, 17,764/18,432 on the 72 relative-size renders and 14,575/15,360 on the 60 threshold-field renders. The 120 lower-precision source cases remain at 30,559/30,720. Maximum numeric error is zero in all four groups. The 19 previously verified roots still match 3,211,776/3,211,776 pixels across 159 renders.

### Connected floating-point pattern backgrounds

| Original inputs, 16² output | Renders | Previously exact | Exact pixels / compared | Maximum absolute error |
|---|---:|---:|---:|---:|
| Same-size F32 inputs with an empty map or one square | 8 | 854 | 2,048/2,048 | 0 |
| Signed F32 fields at six relative input sizes, four output formats | 72 | 12,921 | 17,715/18,432 | 0 |
| Five F32 threshold fields at six relative input sizes | 60 | 1,924 | 11,851/15,360 | 0 |
| U8, U16 or F16 source at four relative input sizes, four output formats | 120 | 30,559 | 30,559/30,720 | 0 |

Keep the output pattern empty and feed it a floating-point image of the same channel type. In alternating columns, set one sample to F32 bits `007fffff` and its neighbor to twice the smallest positive normal. Reducing the two columns to one gives `00800000`. Averaging the unprepared input gives `00c00000`. Alternating the smallest normal and zero gives positive zero after reduction. A same-size gray input maps both signed subnormals and negative zero to positive zero. A same-size RGBA input retains the sign of each zero. MaterialCraft now prepares connected F32 samples before filtering and at each filter result. [Authored clear colors and stamp blending](#pattern-opacity-near-zero-and-color-accumulation)

At 32² to 16², test those first two column pairs, then switch the second to negative values and finally fill the source with the negative largest subnormal. For RGBA alternate channel signs. At seeds 100391 through 100394, gray matches all 1,024 pixels. Color matches 768/1,024, with a zero-sign difference on the uniform field.

For the eight same-size cases repeat F32 bits `00000000 80000000 00371245 80371245 007fffff 807fffff 00800000 80800000 00800001 80800001 3e8914f7 bf102ff3` at index (7x+5y+3c) mod 12 for channel c. Use seed 100109. Compare an empty map and a centered half-size square with gain zero, one or the smallest positive normal. For the 72 renders use input width and height relative to the output by factors (1,1), (2,2), (½,½), (2,1), (1,2), (½,2). Mix both signs of zero, subnormal and normal values around the smallest F32 normal, then replace every third sample of the last field with −.25. Store each field as F32 and the last in all four output formats. All 9,216 grayscale pixels match bit for bit. The only unequal RGBA pixels differ in their sign of zero.

For the 60 threshold renders use 32², 8², 64², 4², 4×32 and 32×4 inputs. With n the smallest F32 normal and t the largest positive subnormal, alternate t and 2n by column. Use −t everywhere in the second field. Select −2n or −t according to (x+3y+2c) mod 5 in the third, and 2n or −2n on the same mask in the fourth. In the fifth field select −n or t by (floor(x/2)+floor(y/2)+c) mod 3. Use seed 100237+7j+i for size j and field i. All 7,680 gray pixels match bit for bit. RGBA matches 4,171/7,680. Before the fix the maximum error across these fields was 1.80915927e−38.

For the 120 source-storage controls use 16², 32×16, 8² and 32×8 inputs. Read half words `0000 8000 0001 8001 03ff 83ff 0400 8400 3555 bffe 02ff 01ff 8000` at index (3x+7y+c) mod 13. Add floor(x/2)+5floor(y/3) to the index on a second field. For integer sources use (53i+17) mod 256 or (11677i+77) mod 65536 with index i. At 16² the first half-gray file contains 39 negative zeros. Its reference input bridge has already converted all 39 to positive zero. The corresponding half-color file retains all 156 negative zeros at the bridge. The remaining differences in these controls are signed zeros. Across six files saved by MaterialCraft, the reference app reproduced all 6,144/6,144 source-file pixels in 24 renders.

The previous 105 mixed-size stamp renders still match 188,160/188,160 pixels. The 159 renders of 19 previously verified roots match 3,211,776/3,211,776, maximum error zero.

### Trapezoid coordinates under fixed clipping

| Root | [Original inputs](#quad-swirl-trapezoid-and-vector-morph-comparisons) | Boundary fields | Four tiling settings on the boundary fields | Maximum component error |
|---|---:|---:|---:|---:|
| trapezoid_transform | 171,264/171,264 | 135,424/135,424 | 541,696/541,696 | 0 |
| trapezoid_transform_grayscale | 171,264/171,264 | 135,424/135,424 | 541,696/541,696 | 0 |

Before the fix, both roots matched 171,262/171,264 pixels on the original 18 renders. On the 10 extra boundary renders the color and grayscale roots matched 135,365/135,424, 135,411/135,424 respectively. The final 40 renders per root repeat those fields with horizontal and vertical tiling disabled, horizontal only, vertical only and both axes. For boundary inputs store (x,y,x mod 2,y mod 2) as F32, using x alone in grayscale. In the binary case use (x mod 2,y mod 2,floor(x/2) mod 2,floor(y/2) mod 2). For signed inputs multiply by .1875 and subtract 2.125. Use square fields at 16, 128, 128, 256 and 64 pixels, then a 64×32 field and a repeated 128² field, with seeds 89,131,233,331,421,509,601. The 64² signed field covers all four output formats. Top/bottom stretch is .3/−.2 for the coordinate, binary and repeated cases, .3125/.75 for signed, and .3125/−.2 for the thin field. The 16² case retains defaults. For the four tiling settings use seed 94423 + 17 × mask + j for case j and masks 0 to 3.

Let q = (x − .5) / lerp(1 − top, 1 − bottom, y). The sample uses (q + .5,y). Clipping checks abs(q) > .5 only when horizontal tiling is disabled. At (19,0) of the 128² coordinate field with top=.3 and bottom=−.2, the numerator and denominator have F32 bits `beb20000` and `3f33b333`. With clipping fixed off the stored coordinate is `3b9aff02`. A live comparison of abs(q) stores q separately and produces `3b9aff00` at the same pixel. The compiler counts numerical uses of the intermediate after a Boolean condition has been fixed by a literal or the tiling mask. A user parameter that happens to contain false still supplies a runtime condition.

Expose the position, division factors and quotient independently using the 16² default, the 128² coordinate, 256² large-grid, 64² signed and 64² thin fields, plus the 128² large and 32² signed inputs from the original matrix. Each of the three projections matches 107,776/107,776 F32 pixels. Write the local position to a four-channel F32 texture and read its xy components in a second processor. The reference and local samplers agree in both roots for separate point and linear sampling, the original sampling selector and the two corresponding inline samplers. The ten seven-case groups match 1,077,760/1,077,760 pixels. In the unmodified graph the comparison was the extra reader of q.

The 105 branch-isolation renders use the live absolute-value check, the original tiling gate, literal gates, exposed gates, a varying per-pixel gate, separately returned absolute values and quotients, and multiplication controls. They match 1,616,640/1,616,640 pixels. The additional 56 renders swap gate operands, use disjunction, test a fixed gate on a multiplication, and retain the quotient as a live output. They match 862,208/862,208 pixels. All maxima are zero.

The three other inventory roots containing the same direct combination of pure pixel arithmetic and Boolean operations retained their recorded results. Curvature v2, Shape Splatter to Mask and Summed Area Table Iteration match 246,044/246,044 pixels in 14 renders. Both transformed roots also reproduce all 270,848/270,848 source pixels when their current-header MaterialCraft saves are opened in the reference app. The two saves retaining the older `0.1.3` header are refused at package load. Saved-file evidence covers header `1.1.0.202502`. [Format-header comparison](#integer-defaults-in-saved-graph-interfaces)

### Scratches, Messy Fibers, Microscope and Moisture

MaterialCraft at `7283a5b` produced the following whole-root counts. Each F32 or F16 error is an absolute component difference. U8 and U16 errors use stored levels. The Scratches output is RGBA. The other five outputs are grayscale.

| Package root | Renders | Exact pixels / compared | U8 exact / maximum | U16 exact / maximum | F16 exact / maximum | F32 exact / maximum |
|---|---:|---:|---:|---:|---:|---:|
| me_water_level.sbs/scratches_2 | 21 | 138,019/178,432 | 119,761/119,808 · 1 | 12,842/15,360 · 2 | 5,070/5,120 · 0.00048828125 | 346/38,144 · 2.24113464e-05 |
| messy_fibers_1.sbs/messy_fibers_1 | 21 | 122,750/178,432 | 5,117/5,120 · 1 | 94,328/130,048 · 4 | 5,072/5,120 · 0.00048828125 | 18,233/38,144 · 6.13927841e-05 |
| messy_fibers_2.sbs/messy_fibers_2 | 21 | 96,050/178,432 | 5,117/5,120 · 1 | 85,685/130,048 · 5 | 5,000/5,120 · 0.00048828125 | 248/38,144 · 6.47902489e-05 |
| messy_fibers_4.sbs/messy_fibers_4 | 20 | 119,106/162,048 | 5,114/5,120 · 1 | 82,167/113,664 · 200 | 5,043/5,120 · 0.00048828125 | 26,782/38,144 · 0.00244146585 |
| microscope_view.sbs/microscope_view | 21 | 177,304/178,432 | 5,120/5,120 · 0 | 129,145/130,048 · 66 | 5,112/5,120 · 0.000244140625 | 37,927/38,144 · 0.00115966797 |
| moisture_noise.sbs/moisture_noise | 21 | 178,192/178,432 | 5,120/5,120 · 0 | 129,875/130,048 · 6 | 5,117/5,120 · 0.00048828125 | 38,080/38,144 · 7.80820847e-05 |

Each row includes the previous defaults, seed changes, available parameter changes and stored-format comparisons. The twelve additional stage captures used seed 95383 + 7i + k with zero-based inventory indices i = 206, 220, 221, 223, 224 and 226 respectively. Use 16²/default/k=0, 32²/A/k=1, 32²/B/k=2 in U8, U16, F16 and F32, 64×32/C/k=6 in F32, 32×64/C/k=7 in U16, 64²/B/k=8 and 128×64/B/k=9 in F32, 64×128/B/k=9 in U16 and 128²/default/k=11 in F32. Other single-format cases use F32.

- For Scratches and Moisture use (width, height, disorder) = (100,100,0), (36,74,12.75), (81,23,57.5).
- For Messy Fibers 1 use (rotation variation, links, disorder, emboss intensity, emboss angle) = (0,1,0,0,0), (132,13,10.75,1.375,37), (283,29,44.25,−.875,163). For Messy Fibers 2 use (lines, disorder) = (1,0), (7,17.125), (27,53.75). Messy Fibers 4 has no exposed controls.
- For Microscope use (warp intensity, disorder) = (0,0), (2.5,15.75), (11.125,46.25).

The earliest unequal stages in every comparable differing stage capture were pattern generators. Substitute a distinct original U16 image for each live generator in the four graphs with downstream filters. At zero-based case j=0 give source h the constant 8191(h+1). At j=1 use 52111−83h on (x+h) mod width = y mod width and 713+39h elsewhere. For j=6,7,8 use 32776+h where (3x+13y+7h) mod 23 is zero and 32767+h elsewhere. For the other settings use [7919(11x+31y+7xy+93+47h+(h+1)x)] mod 65536. Preserve each source’s original dimensions, including the half-width, quarter-height images in Messy Fibers 4.

| Matching-input graph | Output exact / compared | All observed stages, including output and input bridges | Maximum error |
|---|---:|---:|---:|
| messy_fibers_1.sbs/messy_fibers_1 | 46,336/46,336 | 139,008/139,008 | 0 |
| messy_fibers_4.sbs/messy_fibers_4 | 46,336/46,336 | 382,272/382,272 | 0 |
| microscope_view.sbs/microscope_view | 46,336/46,336 | 185,344/185,344 | 0 |
| moisture_noise.sbs/moisture_noise | 46,336/46,336 | 231,680/231,680 | 0 |

The four matching-input roots give 185,344/185,344 exact output pixels across 48 renders. They cover Emboss, two directional warps, transformation, blending, ordinary warp and Levels. The moisture package also contains a disconnected source and filter. The stage counts cover the nodes reachable from its output.

For each live source in the original graph, route it directly to the output. Keep the shortest connected path to a drawing quadrant, using ascending output slot to break ties. Keep the authored shape, then substitute a square or disc. The last control retains all drawing branches with a square at every authored draw site. Use the 16² default, all four 32²/B formats, 64×32/C F32, 64²/B F32 and 64×128/B U16. The maxima columns use U8 / U16 / F16 / F32 in that order.

| Root and pattern source UID | Isolated square | Isolated disc | Isolated authored profile | Authored maxima | Full square layout | Layout maxima |
|---|---:|---:|---:|---|---:|---|
| scratches_2 · 1028227090 · shape 6 | 18,688/18,688 | 18,688/18,688 | 18,687/18,688 | 0 / 0 / 0 / 6.55651093e-07 | 18,688/18,688 | 0 / 0 / 0 / 0 |
| messy_fibers_1 · 1028375992 · shape 11 | 18,688/18,688 | 18,688/18,688 | 17,447/18,688 | 0 / 2 / 0.000244140625 / 2.40802765e-05 | 18,688/18,688 | 0 / 0 / 0 / 0 |
| messy_fibers_2 · 1028631746 · shape 11 | 18,688/18,688 | 18,688/18,688 | 18,522/18,688 | 1 / 1 / 0.000122070312 / 5.27501106e-06 | 18,688/18,688 | 0 / 0 / 0 / 0 |
| messy_fibers_4 · 1047483128 · shape 11 | 18,688/18,688 | 18,688/18,688 | 17,878/18,688 | 0 / 1 / 3.05175781e-05 / 3.55839729e-05 | 18,688/18,688 | 0 / 0 / 0 / 0 |
| messy_fibers_4 · 1047551360 · shape 6 | 2,336/2,336 | 2,336/2,336 | 1,845/2,336 | 0 / 0 / 0 / 1.63912773e-07 | 2,336/2,336 | 0 / 0 / 0 / 0 |
| messy_fibers_4 · 1047551723 · shape 6 | 2,336/2,336 | 2,336/2,336 | 1,743/2,336 | 0 / 1 / 1.90734863e-06 / 2.23517418e-07 | 2,336/2,336 | 0 / 0 / 0 / 0 |
| messy_fibers_4 · 1047551778 · shape 11 | 18,688/18,688 | 18,688/18,688 | 17,792/18,688 | 0 / 2 / 3.81469727e-06 / 2.4408102e-05 | 18,688/18,688 | 0 / 0 / 0 / 0 |
| microscope_view · 1028199215 · shape 6 | 18,688/18,688 | 18,688/18,688 | 18,642/18,688 | 0 / 0 / 0 / 3.7252903e-08 | 13,369/18,688 | 0 / 0 / 0 / 3.33786011e-06 |
| moisture_noise · 1152046381 · shape 4 | 18,688/18,688 | 18,688/18,688 | 18,669/18,688 | 0 / 0 / 0 / 5.96046448e-08 | 18,625/18,688 | 0 / 0 / 0 / 5.96046448e-08 |
| moisture_noise · 1152046384 · shape 4 | 18,688/18,688 | 18,688/18,688 | 18,664/18,688 | 0 / 0 / 0 / 2.09547579e-08 | 18,613/18,688 | 0 / 0 / 0 / 1.49011612e-08 |

All isolated squares and discs here have maximum error zero. In Scratches, Messy Fibers 1, 2 and 4 the complete square controls also agree. The authored soft and wave profiles still differ. Microscope and Moisture also have differences with all original branches drawing squares. All six whole roots remain in progress.

### BNW Spots 3 and Cells 1 pattern sources

| Root package | Whole outputs compared / attempted | Exact pixels / compared | Maximum U8 / U16 / F16 / F32 error |
|---|---:|---:|---|
| noise_bnw_spots_3 | 6/19 | 104,343/107,776 | — / 2 / .00048828125 / .0000259578228 |
| noise_bnw_spots_3_v2 | 6/19 | 106,559/107,776 | 1 / — / .00048828125 / .00000336766243 |
| noise_cells_1 | 19/19 | 143,321/153,856 | 0 / 1 / .00048828125 / .000000894069672 |

Each row includes the original 128² and 256² defaults, 128² seed 42 and changed controls, the four-format and independent inputs, and ten additional stage captures. In each spots root 13 settings hit the pattern execution limit. Counts omit outputs without a comparable image. In the v2 spots root the 128² U8 seed-42 case is exact at 16,384/16,384. Cells 1 matches all 5,120 U8 pixels and 4,397/13,568 F32 pixels. [Original four-format controls](#four-format-inventory-comparison)

For the ten stage captures use inventory index i = 238, 239 or 240 and seeds 100333+11i plus [0,1,2,2,2,2,6,7,8,9]. Start with 16² F32 at defaults. At 32² F32 set (scale, disorder, non-square expansion) to (1,0,on). Use (3,.3125,off) at 32² in U8, U16, F16 and F32. Use (5,.8125,on) at 64×32 F32 and 32×64 U16, then (3,.3125,off) at 64² F32 and (1,0,on) at 128×64 F16. For v2 use (speed, anisotropy, anisotropy angle, tile offset) of (1,0,0,[0,0]), (.6875,.25,.3125,[.125,−.1875]) and (1.4375,.625,−.21875,[−.3125,.4375]) at scales 1, 3 and 5 respectively.

In the first spots root the three U16 generators match 63/64, 988/1,024 and 255/256 pixels at the 32² scale-one setting. Replace those generators with original grayscale fields matching each stage's resolution and format. The remaining two blends and Levels then give 21,760/21,760 exact output pixels in ten renders. For field j use level 8199(j+1) at defaults, a diagonal of 59003−49j over 302+69j at scale one, and ((11x+31y+7xy+93+47j+(j+1)x)×7919) mod 65536 for the format sweep. For the two rectangular scale-five cases use levels 32769+j and 32767+j selected by (7x+13y+3j) mod 11. Use the format-sweep field at 64² and 128×64.

The v2 spots graph has three pattern stages in series. At the same 32² setting its first two images match 64/64 and 256/256 U8 pixels. The terminal F32 pattern stage matches 81/1,024. In separate tests substitute an original input for the first stage, then for the second stage. The comparable whole outputs match 8,200/9,472 and 8,208/9,472 respectively. Seven of ten settings per substitution still hit the execution limit. The original Cells 1 graph consists of a single pattern stage.

For each of these seven generators isolate a shortest connected path from the pattern root to a drawing quadrant. Retain the original Iterate and Switch expressions on the path, keep one outgoing connection per node, and give the stage an empty background input. Compare its authored paraboloid, then substitute a solid square and a disc. Use the ten-case series above with the 64² case omitted. On those isolated paths the authored profiles match 23,180/23,184, 23,097/23,184 and 14,864/17,664 pixels respectively. The square and disc each match 64,032/64,032 pixels across all seven sources, maximum zero. The authored profiles retain F32 maxima of 4.17232513e−7, 6.25848770e−7 and 8.94069672e−7. These three roots remain in progress.

### Cloud and crystal pattern sources across scales

| Root package | Whole outputs compared / attempted | Exact pixels / compared | Maximum U8 / U16 / F16 / F32 error |
|---|---:|---:|---|
| noise_clouds_1 | 17/19 | 143,752/149,760 | 1 / 2 / .00048828125 / .0000305175781 |
| noise_clouds_1_v2 | 12/19 | 135,187/141,568 | 1 / 1 / .00048828125 / .00000429153442 |
| noise_clouds_2 | 19/19 | 152,152/153,856 | 1 / 3 / .00048828125 / .0000230669975 |
| noise_clouds_2_v2 | 12/19 | 136,706/141,568 | 1 / 1 / .00048828125 / .00000119209290 |
| noise_crystal_1 | 19/19 | 151,000/153,856 | 0 / 2 / .0001220703125 / .0000207051635 |
| noise_crystal_2 | 19/19 | 152,713/153,856 | 0 / 46 / .00048828125 / .0000152597204 |

The two original cloud graphs and both crystal graphs produce bit-exact downstream outputs with original procedural fields substituted for all source generators, **87,040/87,040 pixels in 40 renders**, maximum error zero. The v2 packages draw into the preceding stage. In noise_clouds_1_v2 two independent captures used the same field in place of the first generator and retained the final generator. Those captures matched 8,174/9,472 and 8,163/9,472 whole pixels. Both reached the execution limit at seven settings. The maximum errors were .00048828125 F16 and .00000234693289 F32. See also the [gradient warp measurements for Crystal 2](#gradient-warp-displacement-and-sampling).

For noise_clouds_2_v2, replace its first or second pattern stage with the original field at that stage’s resolution and precision. The two comparable sets match 8,348/9,472 and 8,352/9,472 pixels, with seven execution-limit settings in each set. F32 maxima are .000000834465027 in both sets. F16 maxima are .0001220703125 and .000244140625.

Use the archive, four-format and independent cases, then the ten [spots scale, resolution and field controls](#bnw-spots-3-and-cells-1-pattern-sources) with inventory indices 244, 245, 246, 247, 249 and 250 in table order. The reference produced all 114 whole outputs. The two missing local outputs for noise_clouds_1 are the scale-five rectangular cases. For each v2 cloud graph the scale-three and scale-five settings reached the execution limit.

Isolate the shortest connected path from the pattern root to a drawing quadrant in each of the 13 source stages. Keep the original Iterate and Switch controls, disconnect image inputs, and omit the 64² render from the ten new cases. Each solid square and each disc matched every compared pixel. The authored profile retains the following differences across the same paths.

| Root package | Authored shape number | Square or disc exact pixels per shape | Authored profile exact / compared | Maximum U8 / U16 / F16 / F32 profile error |
|---|---:|---:|---:|---|
| noise_clouds_1 | 8 | 22,080 | 22,074/22,080 | 0 / 0 / 0 / .000000894069672 |
| noise_clouds_1_v2 | 8 | 22,080 | 22,071/22,080 | 0 / 0 / .00000762939453 / .000000834465027 |
| noise_clouds_2 | 8 | 23,184 | 23,166/23,184 | 0 / 0 / .00000762939453 / .000000298023224 |
| noise_clouds_2_v2 | 8 | 23,184 | 23,153/23,184 | 0 / 0 / 0 / .000000759959221 |
| noise_crystal_1 | 4 | 35,328 | 30,215/35,328 | 0 / 1 / .00048828125 / .00000101327896 |
| noise_crystal_2 | 12 | 17,664 | 17,512/17,664 | 0 / 1 / .000244140625 / .00000238418579 |

### Pattern variable declarations and fixed image selectors

A draw expression that declares a variable can update its previous value within the same cook. The declaration is visible to that node's later draw properties and descendants. A sibling that only reads the name sees the published graph input, or the field default for an unbound direct link. If their parent declares that name, both siblings can read and write the shared pattern value. The declaration exists even when its Set is on an untaken conditional branch. In a quadrant's five mode expressions a shadowed graph input is unavailable. Its direct link uses the field default and an arithmetic read contributes zero.

| Original control | Renders | Previous exact | Exact pixels / compared | Previous maximum F32 / U16 | Maximum F32 / U16 |
|---|---:|---:|---:|---|---|
| Pattern type or tint in a sibling, Iterate child or quadrant child | 120 | 48,560 | 51,200/51,200 | 0.65625 / 32,767 | 0 / 0 |
| Sibling order with and without an ancestor declaration | 144 | 59,040 | 61,440/61,440 | 0.75 / 49,151 | 0 / 0 |
| Five read expressions with an earlier sibling assignment | 120 | 48,800 | 51,200/51,200 | 0.5 / 32,767 | 0 / 0 |
| Conditional ancestor assignment with both branches exercised | 84 | 35,840 | 35,840/35,840 | 0 / 0 | 0 / 0 |
| Five quadrant modes reading traversal and image variables | 150 | 59,204 | 62,440/64,000 | 1.75195312 / 32,768 | 0.84176302 / 27,569 |

Use an Iterate with one repeat and a square on each branch. In the reader use size (.75,.625), offset (.015625,−.03125), seed offset 43 and RGBA background (.125,.3125,.6875,.5). Use 16² F32 at seed 100673, then 32×16 F32 and U16 at 100691. The small writer square has size (.125,.125) and offset (−.375,−.375). For the type controls publish 2 and assign 3. For tint publish (.25,.375,.5,1) and assign (.75,.625,.5,1). Move the assignment from the preceding to the following sibling, the parent Iterate count or the parent quadrant tint. Compare bare getters and getter plus zero with each literal alternative. An integer getter plus 2 in a shadowed child mode selects square 2.

For the ancestor test, remove or retain the published tint, interchange the children, and either omit the parent assignment or initialize it to (.5625,.1875,.75,1). The first child can update the parent's value before the second reads it. Repeat with an unrelated assignment, a random red addition in [0,.125], a red addition of .125 times the horizontal stamp position, a copy to another name, and a copy to the same name. Only the same-name copy reopens the preceding sibling's pattern value when the parent has no declaration. For the conditional control let a published Boolean choose between the tint assignment followed by count 1, and a bare count 1. Both choices expose the declaration to the children.

A further 120 same-node controls matched 51,200/51,200 RGBA pixels with zero maximum error. Assign size (.75,.4375) in the seed, frame offset or pattern variation and read it in size. Assign tint in the seed or branch offset and read it in opacity. Run with the published value present and absent and compare each getter to the published, assigned and default literals. The seed and frame offset precede size. Variation follows size, and branch offset follows opacity.

For the mode controls use the [five original quadrant settings and input field](#assignments-in-quadrant-mode-expressions). At quadrant depth one test depth > .5, depth scale > .25 and > .75, horizontal position > .125, and image width > 24. Use the iteration number > .5 on three repeats. Choose disc or square for the shape and 1 or 0 for the other four modes. Use 16² F32 at seed 100673+j, then 32×16 F32 and U16 at 100691+j, for zero-based mode index j. All five read the traversal depth, depth scale, position and iteration number as zero, and image width follows the output size. Each of shape, blending, premultiplication and seed inheritance matches 12,800/12,800 pixels. Filtering matches 11,240/12,800, with maximum .841763020 F32 and 27569 U16. The separate direct-link seed-inheritance controls now match 8,960/8,960 pixels, maximum zero.

| Original image selector | Renders | Previous exact | Exact pixels / compared | Previous maximum F32 / U16 / F16 | Maximum F32 / U16 / F16 |
|---|---:|---:|---:|---|---|
| Generic and packed selectors with four, two or one wired slots | 108 | 28,160 | 46,080/46,080 | 0.776470602 / 50,886 / 0.776428223 | 0 / 0 / 0 |
| Offsets to missing and published integer selectors | 99 | 28,160 | 42,240/42,240 | 0.898039222 / 58,853 / 0.897949219 | 0 / 0 / 0 |

Use four original 16² gray U8 stamps at levels 31, 79, 151 and 229. Wire slots {0,1,2,3}, {1,3} or {2}. Render at 16² F32, 32×16 U16 and 16×32 F16. Add offsets 0, 1 and 2 to seed 102667, 102682 or 102687 for the three wiring cases. For pattern 1, 257 or 259, omit the index, set it to zero, bind it directly to a missing integer, or add zero to that getter. All four choices preserve the fixed slot. For pattern 259 add 0, 1, 2 or 4 to a missing integer, then add a published integer of 2. Compare against direct literal indices and a direct link to the published 2. An expression containing only constants and missing user getters selects the fixed numbered slot. A live selector advances across unwired slots.

The reference's renders also varied when a property incremented a variable without first initializing it. With a 0.25 increment per square, three visits in a fresh file produced 1.5, and a subsequent render of the same graph could give 6.0. Run at 16²/103779 and 32×16/103781 singly and in both orders, using one visit, three visits, two sibling nodes, two quadrant children or an explicit initial value .125. Across 60 renders MaterialCraft matches 18,304/23,040 F32 pixels, maximum 9. All 12 explicitly initialized renders match 4,608/4,608 pixels and both sizes give 1.875 for three visits.

Seven new original packages saved through MaterialCraft at format version 1.1.0.202502 reopened in the reference app. Across 20 source-to-save renders covering ancestor and sibling reads, a conditional declaration, mode arithmetic, two sparse image selectors and the initialized counter, all 8,448 pixels match with maximum zero. The 19 previously verified roots match all 3,211,776 pixels in 159 renders after the changes.

### Direct pattern links with an unbound variable

An unbound user variable connected directly to a pattern property leaves that property at its default. Within an arithmetic expression the same getter contributes its typed zero. A bound zero supplies an actual zero. MaterialCraft now distinguishes these cases when evaluating a pattern graph.

| Original control | Renders | Exact pixels / compared | Maximum F32 / U16 error |
|---|---:|---:|---|
| Grayscale and RGBA square, scalar and Float4 opacity | 84 | 35,840/35,840 | 0 / 0 |
| Square size from a Float2 link | 21 | 8,960/8,960 | 0 / 0 |
| Iterate count from an integer link | 21 | 8,960/8,960 | 0 / 0 |
| Seed inheritance from a Boolean link | 21 | 8,310/8,960 | .694821596 / 40631 |

For the first row, render a centered square of size (.75,.75). In each output kind and for each opacity type, compare seven settings. Omit opacity, read an unbound user variable directly, bind that variable to zero or a second value in an outer Iterate, add the second value to an unbound read, or author either value directly. Use .375 for the scalar and (.25,.5,.75,1) for the vector. Author the scalar controls as constant-return expressions and the vector controls as literal colors. In grayscale, the vector expression supplies its alpha and the literal vector supplies its mean RGB. Use 16² F32, 32×16 F32 and 32×16 U16 at seeds 100801 through 100803. Before the change 32,960/35,840 pixels matched, with maxima of 1 F32 and 65535 U16 levels.

For the next three rows start with an outer Iterate's initial child. Use a square of size (.8125,.625) and RGBA (.125,.25,.375,1). Compare omission, an unbound read, explicit constant-return expressions for zero and a second value, matching assignments in the outer Iterate, and a derived expression. Use (.375,.625) for the second size. To measure count, insert a second Iterate and draw (.125,.25,.375,.25) on its repeated branch with a changed count of 2. For seed inheritance, use a seed offset of 43, a random stamp color, true as the second value and logical negation for the derived expression. For property index j in the displayed order use the same sizes and formats at seed 101621+11j+i, with i from 0 through 2. Together these rows improved from 23,650 to 26,230/26,880. The remaining 650 pixels occur when the outer Iterate assigns false to the seed-inheritance variable. The reference uses the field default there. [Quadrant mode reads and writes](#assignments-in-quadrant-mode-expressions)

Six additional original files at format version 1.1.0.202502 were saved by MaterialCraft, reopened in the reference app and compared against their source-file renders. The four pattern files exercise writes from empty and drawing squares, an image-index expression and an exposed bilinear filter. The two pixel-read files exercise an 8² source and a 32×16 source feeding a 16² output. All 19 stored-image comparisons matched, **6,912/6,912 pixels**, maximum error zero. [Pattern properties](#active-properties-of-empty-and-drawing-quadrants) · [Filtered pixel reads](#subnormal-values-in-filtered-pixel-reads)

Five further saved-file checks cover an unbound scalar opacity, an outer-bound Float4 opacity, missing size, derived count and a false seed-inheritance binding. All 6,400 pixels in 15 source-to-save renders match, maximum zero.

The 19 previously verified roots also match all 3,211,776 pixels in 159 renders after the direct-link change, maximum error zero.

### Assignments in quadrant mode expressions

Local assignments inside the pattern type, blend mode, image filter, premultiplication and seed-inheritance expressions affect that expression's result. Later properties and children retain their incoming variables. Across the [117 child-color controls](#active-properties-of-empty-and-drawing-quadrants), the count improved from 49,120 to **49,920/49,920 exact RGBA pixels**, maximum error zero. The image-index expression still publishes its assignment on a bound image stamp.

Use an image, solid or empty parent and an initialized child color as in the linked measurements. Separately, render the parent without the child to verify these five settings. Use seed 43 inside the stamp, global seed 100673+j at 16² F32, and 100691+j at 32×16 F32 and U16 for property index j in the order above. Set its size to (.75,.625), offset to (.015625,−.03125), color to (.5,.625,.875,.75) and background to (.125,.3125,.6875,.5). For the image cases, generate a 16² texture with channels ((13x+7y) mod 17)/16, ((5x+11y+xy) mod 19)/18, ((3x+7y) mod 11)/10 and .125+.75((x+5y) mod 7)/6. Substitute a random color for the seed-inheritance cases.

The native literal, computed-constant and exposed-input settings agree for all 33,280 pixels in the 78 nonliteral comparisons. The corresponding whole-stamp comparison between applications gives 47,760/49,920 exact pixels in 117 renders. Errors remain in bitmap filtering, up to 21504 U16 levels and .516796827 F32. In another 156 renders an outer Iterate assigns an integer or Boolean immediately before the quadrant. Compare a literal, a read of that outer variable, a read with a published graph default, and a Set followed by a Get inside the same expression. The cross-application result is 55,812/66,560, maximum 41882 U16 levels and .701812745 F32. Outer-variable reads in the reference select the property default in all five tests. Writes subsequently read within the same expression select the authored value.

### Active properties of empty and drawing quadrants

In an empty quadrant, branch offset and random seed can change a variable that a child reads. Frame offset, opacity, rotation, size and variation publish their assignments on drawing quadrants. Image index publishes its assignment on a bound image stamp. MaterialCraft now skips the inactive draw and image properties. Across 117 independently authored renders the raw count improved from **48,160 to 49,120/49,920 RGBA pixels**. The F32 result is 29,472/29,952 with maximum .5, and U16 is 19,648/19,968 with maximum 32767 levels.

Use an Iterate with zero repeats. In its count expression set a Float4 variable to (.125,.25,.375,1) before returning zero. Connect the initial child to an empty quadrant, a solid square or an image stamp bound to an original white 16² image. Make that quadrant transparent, except for the stamp being tested. In each property under test first assign (.625,.75,.875,1) to the variable, then return the authored property value. Let a solid child in quadrant zero read the variable as its color. In order, test branch offset, frame offset, opacity, rotation, size, variation, blend mode, image filter, image index, premultiplication, seed, seed inheritance and pattern type. Use zero offsets and rotation, unit size, zero variation, additive blend, image filter 1, index 0, premultiplication enabled, seed 0 and inheritance enabled. At property index j render 16² F32 with seed 100477+j, then 32×16 F32 and U16 with seed 100519+j.

The child retained its initial color for property writes in blend mode, image filter, premultiplication, seed inheritance and pattern type on all three kinds of parent. Some local children still see those assignments. Independent single-stamp controls confirm that literal, computed-constant and exposed-variable settings choose the same reference draw in 33,280/33,280 pixels across 78 comparisons against 39 literal renders. The 19 previously verified pattern roots match 3,211,776/3,211,776 pixels in 159 renders.

### Liquid, fiber and related noise sources

The fourteen roots produced 242/266 comparable whole outputs and 1,703,324/1,891,840 exact pixels. Each microscope-view root reached the pattern graph execution limit in eight of its 19 cases. A failed cook contributes zero compared pixels. Integer differences are storage levels.

| Root package | Whole outputs compared / attempted | Exact pixels / compared | Maximum U8 / U16 / F16 / F32 error |
|---|---:|---:|---|
| noise_liquid | 19/19 | 135,824/142,592 | 1 / 268 / 0.0009765625 / 0.451499209 |
| noise_messy_fibers_1 | 19/19 | 96,519/142,592 | 1 / 4 / 0.00048828125 / 4.52399254e-05 |
| noise_messy_fibers_1_v2 | 19/19 | 131,876/142,592 | 1 / 2 / 0.00048828125 / 2.95042992e-05 |
| noise_messy_fibers_2 | 19/19 | 84,588/142,592 | 1 / 7 / 0.00048828125 / 7.802248e-05 |
| noise_messy_fibers_2_v2 | 19/19 | 130,496/142,592 | 1 / 4 / 0.00048828125 / 6.29425049e-05 |
| noise_messy_fibers_3 | 19/19 | 123,834/142,592 | 0 / 2 / 0.00048828125 / 2.47955322e-05 |
| noise_messy_fibers_3_v2 | 19/19 | 132,391/142,592 | 1 / 3 / 0.00048828125 / 3.12328339e-05 |
| noise_microscope_view | 19/19 | 140,486/142,592 | 1 / 48 / 0.000244140625 / 0.000274658203 |
| noise_moisture_noise | 11/19 | 107,093/107,776 | 0 / 1 / 0.00048828125 / 1.52587891e-05 |
| noise_moisture_noise_1_v2 | 11/19 | 104,176/107,776 | 0 / 1 / 0 / 1.43051147e-06 |
| noise_moisture_noise_2 | 11/19 | 104,071/107,776 | 1 / 1 / 0.00048828125 / 1.49011612e-06 |
| noise_perlin_noise | 19/19 | 139,281/142,592 | 0 / 4 / 0.00048828125 / 3.05771828e-05 |
| noise_perlin_noise_v2 | 19/19 | 132,970/142,592 | 1 / 39650 / 0.00048828125 / 0.435275048 |
| noise_plasma | 19/19 | 139,719/142,592 | 0 / 2 / 0.000122070312 / 1.52885914e-05 |

Use the four archived cases, the [four-format sweep and independent seed](#four-format-inventory-comparison), and ten fresh renders. In inventory order, number these roots i=287 through 300, starting with noise_liquid. Use 102011+17i as the base seed. Add [0,1,2,2,2,2,6,7,8,9] for the ten renders. Use [16²,32²,32²,32²,32²,32²,32×16,16×32,64×32,32×64], storage [F32,F32,U8,U16,F16,F32,F32,U16,F32,F16], and settings [defaults,A,B,B,B,B,C,C,B,A]. Apply each setting to the published controls present in that package.

| Controls | A | B | C |
|---|---|---|---|
| Scale when the package default is 1 | 1 | 1 | 2 |
| Scale with any other default | 1 | 5 | 13 |
| Disorder, nonsquare expansion, disorder speed | 0, on, 1 | .3125, off, .6875 | .8125, on, 1.4375 |
| Disorder anisotropy, its angle, tile offset | 0, 0, (0,0) | .25, .3125, (.125,−.1875) | .625, −.21875, (−.3125,.4375) |
| Warp intensity, angle, angle randomness | 0, 0, 0 | .3125, .21875, .375 | .8125, −.3125, .6875 |
| Line and link counts, color and luminance randomness | 2, 2, 0, 0 | 5, 5, .3125, .3125 | 9, 9, .625, .625 |
| Pattern size, angle, angle randomness, global opacity | (1,1), 0, 0, 1 | (.75,1.25), .21875, .375, .8125 | (1.375,.625), −.3125, .6875, 1.1875 |

In each of 17 pattern stages, keep the shortest connected path to a quadrant with an authored shape and clear its image inputs. Break equal-length ties by connection identifier. Use the nine cases that exclude 64×32. Render the authored shape, then a square and a disc with the same parameters and connections. The three groups match 122,941/143,616, 143,301/143,616 and 143,298/143,616 pixels respectively. Of 153 runs per group, 146 authored runs, 148 square runs and 148 disc runs produced at least one nonzero reference pixel. Only the Perlin noise root differs on its hard shapes, with maximum U8 / U16 / F16 / F32 error 0 / 32768 / 0 / 3.96000028 for the square and 0 / 32768 / 0 / 3.52000022 for the disc.

| Root package | Authored shape numbers | Authored exact / compared | Maximum authored U8 / U16 / F16 / F32 error | Square exact / compared | Disc exact / compared |
|---|---|---:|---|---:|---:|
| noise_liquid | 6 | 5,773/8,448 | 0 / 1 / 0.000244140625 / 9.83476639e-07 | 8,448/8,448 | 8,448/8,448 |
| noise_messy_fibers_1 | 11 | 8,099/8,448 | 0 / 1 / 0.000122070312 / 1.84476376e-05 | 8,448/8,448 | 8,448/8,448 |
| noise_messy_fibers_1_v2 | 11 | 8,045/8,448 | 0 / 1 / 3.05175781e-05 / 1.26063824e-05 | 8,448/8,448 | 8,448/8,448 |
| noise_messy_fibers_2 | 11 | 8,357/8,448 | 0 / 1 / 6.10351562e-05 / 6.99050725e-06 | 8,448/8,448 | 8,448/8,448 |
| noise_messy_fibers_2_v2 | 11 | 8,321/8,448 | 0 / 1 / 3.05175781e-05 / 4.35113907e-06 | 8,448/8,448 | 8,448/8,448 |
| noise_messy_fibers_3 | 14 | 8,437/8,448 | 0 / 1 / 0 / 2.5331974e-06 | 8,448/8,448 | 8,448/8,448 |
| noise_messy_fibers_3_v2 | 14 | 8,434/8,448 | 0 / 1 / 0 / 3.41795385e-06 | 8,448/8,448 | 8,448/8,448 |
| noise_microscope_view | 6 | 5,773/8,448 | 0 / 1 / 0.000122070312 / 1.07288361e-06 | 8,448/8,448 | 8,448/8,448 |
| noise_moisture_noise | 4 | 16,669/16,896 | 0 / 1 / 2.38418579e-07 / 3.57627869e-07 | 16,896/16,896 | 16,896/16,896 |
| noise_moisture_noise_1_v2 | 4 | 8,336/8,448 | 0 / 1 / 0 / 3.87430191e-07 | 8,448/8,448 | 8,448/8,448 |
| noise_moisture_noise_2 | 4 | 8,034/8,448 | 0 / 1 / 0 / 2.38418579e-07 | 8,448/8,448 | 8,448/8,448 |
| noise_perlin_noise | 5 | 11,401/16,896 | 0 / 1 / 0.00048828125 / 1.66893005e-06 | 16,896/16,896 | 16,896/16,896 |
| noise_perlin_noise_v2 | 5 | 5,692/8,448 | 0 / 32768 / 0 / 0.653483272 | 8,133/8,448 | 8,130/8,448 |
| noise_plasma | 6 | 11,570/16,896 | 0 / 1 / 0.000244140625 / 9.53674316e-07 | 16,896/16,896 | 16,896/16,896 |

Where there are downstream stages, replace the source patterns with the same original fields in both applications. Use each source's output size and storage and the ten field settings from the [dirt noise source comparison](#dirt-noise-sources-and-downstream-stages). All five substitutions produced ten comparable images each.

| Root package | Exact pixels / compared | Maximum U8 / U16 / F16 / F32 error |
|---|---:|---|
| noise_liquid | 8,448/10,496 | 0 / 0 / 0 / 0.836438522 |
| noise_microscope_view | 10,496/10,496 | 0 / 0 / 0 / 0 |
| noise_moisture_noise | 10,496/10,496 | 0 / 0 / 0 / 0 |
| noise_perlin_noise | 10,496/10,496 | 0 / 0 / 0 / 0 |
| noise_plasma | 10,496/10,496 | 0 / 0 / 0 / 0 |

### Dirt noise sources and downstream stages

The eleven dirt-noise roots produced **116/209 comparable whole outputs** and **1,045,373/1,067,264 exact pixels**. MaterialCraft reached its pattern execution limit in 93 cases, including every whole-graph case for both Dirt 4 packages. The counts below exclude replacement images emitted after a cook error. Integer errors are storage levels.

| Root package | Whole outputs compared / attempted | Exact pixels / compared | Maximum U8 / U16 / F16 / F32 error |
|---|---:|---:|---|
| noise_dirt_1 | 11/19 | 104,064/107,776 | 0 / 538 / 0.00048828125 / 0.000222146511 |
| noise_dirt_1_v2 | 11/19 | 107,082/107,776 | 1 / 14 / 0.00048828125 / 0.000208899379 |
| noise_dirt_2 | 11/19 | 107,184/107,776 | 0 / 21 / 0.00048828125 / 0.000252112746 |
| noise_dirt_3 | 19/19 | 135,236/142,592 | 1 / 3 / 0.000244140625 / 3.39820981e-05 |
| noise_dirt_3_v2 | 17/19 | 136,738/141,568 | 1 / 2 / 0.000244140625 / 2.52574682e-05 |
| noise_dirt_4 | 0/19 | No comparable image | — / — / — / — |
| noise_dirt_4_v2 | 0/19 | No comparable image | — / — / — / — |
| noise_dirt_5 | 11/19 | 107,751/107,776 | 0 / 4 / 0 / 5.08278608e-05 |
| noise_dirt_5_v2 | 11/19 | 107,341/107,776 | 0 / 0 / 0 / 5.96046448e-08 |
| noise_dirt_gradient | 19/19 | 141,554/142,592 | 0 / 1 / 0.000244140625 / 1.52587891e-05 |
| noise_dirt_gradient_v2 | 6/19 | 98,423/101,632 | 10 / — / 0.16015625 / 0.027451098 |

Use the four archived size, seed and parameter cases and the [four-format sweep and independent seed](#four-format-inventory-comparison). Add ten renders at sizes 16², 32², 32², 32², 32², 32², 32×16, 16×32, 64×32 and 32×64, in F32, F32, U8, U16, F16, F32, F32, U16, F32 and F16 respectively. Use defaults, A, B, B, B, B, C, C, B and A. Seed each graph at 100987+17i plus [0,1,2,2,2,2,6,7,8,9], with i from 261 through 271 in table order. Apply each setting where the root exposes it.

| Controls | A | B | C |
|---|---|---|---|
| Scale, disorder, nonsquare expansion | 1, 0, on | 1, .3125, off | 2, .8125, on |
| Disorder speed, anisotropy, anisotropy angle, tile offset | 1, 0, 0, (0,0) | .6875, .25, .3125, (.125,−.1875) | 1.4375, .625, −.21875, (−.3125,.4375) |

For each of 20 pattern stages, retain the shortest connected path to an authored drawing quadrant, breaking ties by connection identifier. Clear the stage inputs. Render its authored shape, then a square and a disc, with all other properties preserved. Use the ten-case sequence above except the 64×32 case. The square and disc each match **115,032/115,032** with zero maximum error in 180 renders. The authored paths match 111,112/115,032. Of those 180 renders per shape, 97 authored renders, 102 square renders and 101 disc renders contained at least one nonzero pixel.

| Root package | Reached shape numbers | Authored path exact / compared | Maximum U8 / U16 / F16 / F32 error | Exact pixels per hard shape |
|---|---|---:|---|---:|
| noise_dirt_1 | 1,4 | 16,416/17,664 | 0 / 0 / 0 / 3.57627869e-07 | 17,664 |
| noise_dirt_1_v2 | 1,4 | 16,416/17,664 | 0 / 0 / 0 / 3.57627869e-07 | 17,664 |
| noise_dirt_2 | 1,4 | 16,416/17,664 | 0 / 0 / 0 / 3.57627869e-07 | 17,664 |
| noise_dirt_3 | 4,16 | 10,560/10,560 | 0 / 0 / 0 / 0 | 10,560 |
| noise_dirt_3_v2 | 4,16 | 10,560/10,560 | 0 / 0 / 0 / 0 | 10,560 |
| noise_dirt_4 | 8 | 2,071/2,112 | 0 / 0 / 0 / 7.15255737e-07 | 2,112 |
| noise_dirt_4_v2 | 8 | 2,088/2,112 | 0 / 1 / 0 / 7.74860382e-07 | 2,112 |
| noise_dirt_5 | 3 | 8,448/8,448 | 0 / 0 / 0 / 0 | 8,448 |
| noise_dirt_5_v2 | 3 | 8,448/8,448 | 0 / 0 / 0 / 0 | 8,448 |
| noise_dirt_gradient | 6 | 9,059/9,108 | 0 / 0 / 0 / 5.21540642e-08 | 9,108 |
| noise_dirt_gradient_v2 | 6 | 10,630/10,692 | 0 / 0 / 0 / 3.7252903e-08 | 10,692 |

Supply identical original fields to the downstream graph by replacing its source pattern stages with image inputs. The second substitution retains the final pattern and supplies its immediate source patterns. Use the same ten settings and the native source stage's width, height and storage in each case. For input index a starting at zero, use constant 8199(a+1) in the default case. For A at 32² place 59003−49a on (x+2a) mod width = y mod width and 302+69a elsewhere. For C use 32769+a where (7x+13y+3a) mod 11 is zero and 32767+a elsewhere. For the other cases use ((11x+31y+7xy+93+47a+(a+1)x)×7919) mod 65536. Use the integer field directly for U16, its quotient by 257 rounded down for U8, and its value divided by 65535 for floating storage.

| Root package | Replaced stages | Outputs compared / attempted | Exact pixels / compared | Maximum U8 / U16 / F16 / F32 error |
|---|---|---:|---:|---|
| noise_dirt_1 | Source stages | 8/10 | 8,735/9,472 | 1 / 48 / 0.000915527344 / 0.000764921308 |
| noise_dirt_1 | Inputs of final pattern | 8/10 | 8,740/9,472 | 1 / 48 / 0.000915527344 / 0.000764921308 |
| noise_dirt_1_v2 | Source stages | 8/10 | 8,207/9,472 | 1 / 47 / 0.00317382812 / 0.000719666481 |
| noise_dirt_1_v2 | Inputs of final pattern | 8/10 | 8,220/9,472 | 1 / 47 / 0.00317382812 / 0.000719666481 |
| noise_dirt_2 | Source stages | 8/10 | 9,420/9,472 | 0 / 38 / 0.00105667114 / 0.000882223248 |
| noise_dirt_2 | Inputs of final pattern | 8/10 | 9,420/9,472 | 0 / 38 / 0.00105667114 / 0.000882223248 |
| noise_dirt_3 | Source stages | 10/10 | 9,438/10,496 | 0 / 3 / 0.00048828125 / 2.8192997e-05 |
| noise_dirt_3 | Inputs of final pattern | 10/10 | 9,444/10,496 | 0 / 3 / 0.00048828125 / 2.8192997e-05 |
| noise_dirt_3_v2 | Source stages | 8/10 | 8,722/9,472 | 0 / 0 / 0.00048828125 / 2.68220901e-07 |
| noise_dirt_3_v2 | Inputs of final pattern | 8/10 | 8,730/9,472 | 0 / 0 / 0.00048828125 / 2.68220901e-07 |
| noise_dirt_4 | Source stages | 10/10 | 3,097/10,496 | 1 / 3 / 0.349609375 / 0.349553674 |
| noise_dirt_4_v2 | Source stages | 10/10 | 1,902/10,496 | 4 / 3 / 0.352294922 / 0.352193475 |
| noise_dirt_5 | Source stages | 10/10 | 10,496/10,496 | 0 / 0 / 0 / 0 |
| noise_dirt_gradient | Source stages | 10/10 | 10,496/10,496 | 0 / 0 / 0 / 0 |
| noise_dirt_gradient_v2 | Source stages | 3/10 | 1,617/3,328 | — / — / 0.122558594 / 0.0274509192 |
| noise_dirt_gradient_v2 | Inputs of final pattern | 3/10 | 2,167/3,328 | — / — / 0 / 9.53674316e-07 |

### Directional noise and scratch pattern sources

The ten directional generators produced **135 comparable whole outputs across 190 cases**. The reference produced all 190. The other 55 local evaluations reached the pattern execution limit. Integer errors below are stored levels. A missing error value indicates an unavailable image.

| Root package | Whole outputs compared / attempted | Exact pixels / compared | Maximum U8 / U16 / F16 / F32 error |
|---|---:|---:|---|
| noise_directional_noise_1 | 19/19 | 136,222/144,640 | 1 / 2 / 0.00048828125 / 4.19616699e-05 |
| noise_directional_noise_1_v2 | 18/19 | 133,251/142,592 | 1 / 1 / 0.00048828125 / 4.15444374e-05 |
| noise_directional_noise_2 | 11/19 | 103,497/107,776 | 0 / 4 / 0.00048828125 / 1.06692314e-05 |
| noise_directional_noise_2_v2 | 11/19 | 105,116/107,776 | 1 / 1 / 0.000244140625 / 1.57654285e-05 |
| noise_directional_noise_3 | 0/19 | No comparable image | — / — / — / — |
| noise_directional_noise_3_v2 | 0/19 | No comparable image | — / — / — / — |
| noise_directional_noise_4 | 19/19 | 134,015/144,640 | 0 / 1 / 0.00048828125 / 6.67572021e-06 |
| noise_directional_noise_4_v2 | 19/19 | 134,664/144,640 | 1 / 1 / 0.00048828125 / 4.76837158e-06 |
| noise_directional_scratches | 19/19 | 119,970/144,640 | 1 / 1 / 0.00048828125 / 2.37524509e-05 |
| noise_directional_scratches_v2 | 19/19 | 137,786/144,640 | 1 / 1 / 0.000244140625 / 2.8796494e-05 |

Use the four archived defaults, seed and changed cases, the [four-format sweep and independent seed](#four-format-inventory-comparison), and ten stage-instrumented renders. For the latter, seed j is 100537+13i plus [0,1,2,2,2,2,6,7,8,9], with i = 251 through 260 in table order. Start at 16² F32 with defaults. Use A at 32² F32, B at 32² in U8/U16/F16/F32, C at 64×32 F32, B at 32×64 U16, D at 32² F32 and A at 64×32 F16.

| Exposed controls | A | B | C | D |
|---|---|---|---|---|
| Scale, disorder, expand nonsquare, angle, angle random | 1, 0, on, 0, 0 | 1, .3125, off, .15625, .21875 | 2, .6875, on, −.21875, .40625 | 3, .125, off, .375, .03125 |
| v2 speed, anisotropy, anisotropy angle, tile offset | 1, 0, 0, (0,0) | .6875, .25, .3125, (.125,−.1875) | 1.4375, .625, −.21875, (−.3125,.4375) | .875, .125, .15625, (−.0625,.125) |
| Scratch amount, size, size random | 1, (.5,20), (0,0) | .5625, (.6875,3.25), (.0625,.21875) | .8125, (1.25,7.5), (.15625,.3125) | .375, (.375,1.75), (.03125,.09375) |

For each of the 19 source stages, isolate the shortest connected path to a drawing quadrant, retain its original controls and clear the stage background. Repeat for a square and disc, skipping the 32² D case. All **146,520/146,520 square pixels** and **146,520/146,520 disc pixels** match across 171 renders per shape. The authored wave and directional profiles retain differences on these same paths.

| Root package | Authored shape number | Exact square or disc pixels per shape | Authored profile exact / compared | Maximum U8 / U16 / F16 / F32 profile error |
|---|---:|---:|---:|---|
| noise_directional_noise_1 | 11 | 34,560 | 30,657/34,560 | 0 / 1 / 3.05175781e-05 / 2.33855098e-06 |
| noise_directional_noise_1_v2 | 11 | 15,120 | 14,847/15,120 | 0 / 1 / 5.96046448e-08 / 4.270114e-07 |
| noise_directional_noise_2 | 11 | 11,520 | 7,926/11,520 | 0 / 1 / 6.10351562e-05 / 1.34110451e-06 |
| noise_directional_noise_2_v2 | 11 | 14,400 | 13,476/14,400 | 0 / 1 / 1.52587891e-05 / 9.83476639e-07 |
| noise_directional_noise_3 | 11 | 12,420 | 12,336/12,420 | 0 / 0 / 1.90734863e-06 / 1.1920929e-07 |
| noise_directional_noise_3_v2 | 11 | 12,420 | 12,345/12,420 | 0 / 0 / 0 / 2.51457095e-07 |
| noise_directional_noise_4 | 6 | 11,520 | 11,485/11,520 | 0 / 0 / 0 / 8.34465027e-07 |
| noise_directional_noise_4_v2 | 6 | 11,520 | 11,462/11,520 | 0 / 0 / 0 / 4.76837158e-07 |
| noise_directional_scratches | 16 | 11,520 | 11,515/11,520 | 0 / 0 / 0 / 1.51991844e-06 |
| noise_directional_scratches_v2 | 16 | 11,520 | 11,479/11,520 | 0 / 0 / 0 / 3.03983688e-06 |

At 32² and scale one, noise_directional_noise_3 completes its 4² and 8² sources before its 32² source reaches the traversal limit. Replace all three source generators with the [original grayscale fields](#bnw-spots-3-and-cells-1-pattern-sources) at each source's measured size and precision. The remaining two blends and Levels match 12,544/12,544 pixels over the ten new cases, maximum zero. The same substitution in noise_directional_noise_1 also matches 12,544/12,544. In noise_directional_noise_1_v2, retaining the two later generators or just the terminal generator gives 7,067/10,496 and 7,001/10,496 pixels respectively. Each substitution reaches the execution limit at 64×32, scale two. The respective F32 maxima are .0000418424606 and .0000414848328. Both have maxima of zero U8, one U16 level and .00048828125 F16.

For the final-pattern input substitution in noise_directional_noise_2_v2, 6,832/9,472 pixels match, with maximum U8 / U16 / F16 / F32 errors 0 / 1 / 0.00048828125 / 1.56760216e-05. In noise_directional_noise_3_v2 both substitutions reached the execution limit in all ten runs.

### Pattern expression variables and cook time

Pattern expressions now read inherited graph variables from the active context. Each expression stores its local writes and passes explicit assignments to the next expression. A general evaluation still returns the complete variable environment. In two release runs in opposite order, raw outputs matched between builds on all five successful controls, **17,024/17,024 captured pixels per run**, maximum error zero. A sixth control reached the same execution limit and produced the same intermediate data. The independent reference comparisons on 19 previously verified roots matched 3,211,776/3,211,776 pixels in 159 renders.

| Six-case release batch, one worker | Previous cook time | Revised cook time | Previous process CPU time | Revised process CPU time |
|---|---:|---:|---:|---:|
| Previous build followed by revised build | 133.28 s | 43.59 s | 145.86 s | 62.12 s |
| Revised build followed by previous build | 127.98 s | 42.96 s | 147.44 s | 62.53 s |

Use the [Dirt 1 disc stage](#overlapping-disc-additions-in-dirt-1) at 64²/seed 42/disorder .4, 32²/100189/.15625 and 64²/43/.4, in F32. Then run the two [BNW Spots 3](#bnw-spots-3-and-cells-1-pattern-sources) scale-one controls at 32² F32, and the first spots root at scale three and 32² U8. Use a fresh session for each case and the same processor affinity for both release binaries. Cook time is the sum reported by the six graph evaluations. Process CPU time also includes loading the node library, configuring the cases and capturing the images.

### Gaussian pattern sources and repeat limits

At commit `793e0b7`, each root received 21 reference renders. The first table gives exact pixels / compared pixels / maximum error in each stored format. Integer errors are stored levels. The two Spots packages with a scale control exceeded the local traversal limit in two cases each. Their pixel denominators include the 19 cases that emitted comparable images. The reference app completed all four of the omitted renders.

| Package root | Whole pixels exact / compared | U8 | U16 | F16 | F32 |
|---|---:|---|---|---|---|
| gaussian_spots_2 | 137,856/178,432 | 5,119/5,120/1 | 126,779/130,048/1 | 5,057/5,120/0.000244140625 | 901/38,144/6.40749931e-06 |
| noise_gaussian_noise | 139,833/178,432 | 5,120/5,120/0 | 127,801/130,048/43886 | 5,116/5,120/0.000122070312 | 1,796/38,144/0.668966413 |
| noise_gaussian_noise_v2 | 140,152/178,432 | 119,802/119,808/1 | 13,718/15,360/51197 | 5,113/5,120/0.000244140625 | 1,519/38,144/1.0108676 |
| noise_gaussian_spots_2 | 132,670/174,336 | 5,118/5,120/1 | 121,908/128,000/3 | 4,907/5,120/0.000244140625 | 737/36,096/3.65674496e-05 |
| noise_gaussian_spots_2_v2 | 136,601/174,336 | 119,790/119,808/1 | 11,396/13,312/3 | 4,951/5,120/0.000244140625 | 464/36,096/3.70144844e-05 |

Include the previous defaults, seed, parameter and four-format cases, plus the independent 32² input. For the twelve stage-traced renders use seed 85919 + 7i + k with i = 163, 281, 282, 285 or 286 in table order. Use 16²/default/k=0, 32²/A/k=1, 32²/B/k=2 in U8, U16, F16 and F32, 64×32/C/k=6 in F32, 32×64/C/k=7 in U16, 64²/B/k=8 and 128×64/B/k=9 in F32, 64×128/B/k=9 in U16 and 128²/default/k=11 in F32. Other single-format controls use F32. Every unequal case with comparable stage images first differs in the pattern generator.

For gaussian_spots_2 set Disorder to 0, .3125 and .8125 for A/B/C. For the Noise packages use (scale,disorder,non-square expansion) tuples (1,0,on), (3,.3125,off) and (7,.8125,on). In the two Gaussian Noise packages use 19 for the last scale. For the v2 packages also set (disorder speed, anisotropy, angle, tile offset) to (1,0,0,(0,0)), (.625,.375,.1875,(.1875,−.3125)) and (1.375,.625,−.3125,(−.4375,.0625)).

Isolate the shortest reachable path to a drawing quadrant. Select sibling connections in ascending identifier order. Start at the first quadrant on that path and retain only the connections leading to the drawing quadrant. Draw the authored soft profile, a unit square or a disc while preserving the other authored properties. Also draw the square with every original branch connected. Compare the 16² default, 32²/A, all four 32²/B formats, 64²/B and 128×64/B. Each cell gives exact pixels / 17,664, followed by maximum error across formats. These 160 renders use the updated [stamp tint](#pattern-tint-coverage-and-grayscale-conversion).

| Package root | Isolated square | Isolated disc | Isolated soft profile | Complete square layout |
|---|---:|---:|---:|---:|
| gaussian_spots_2 | 17,664 / 0 | 17,664 / 0 | 17,657 / 1.37090683e-06 | 17,664 / 0 |
| noise_gaussian_noise | 17,664 / 0 | 17,664 / 0 | 3,565 / 6.10351562e-05 | 17,664 / 0 |
| noise_gaussian_noise_v2 | 17,664 / 0 | 17,664 / 0 | 3,547 / 1 | 17,664 / 0 |
| noise_gaussian_spots_2 | 17,664 / 0 | 17,664 / 0 | 17,664 / 0 | 17,663 / 0.254905939 |
| noise_gaussian_spots_2_v2 | 17,664 / 0 | 17,664 / 0 | 17,663 / 5.40167093e-08 | 14,329 / 0.161732554 |

All 176,640 isolated square and disc pixels agree. The complete square layout and the original soft stamp must be considered separately. The isolated soft stamp in noise_gaussian_spots_2 is exact across these eight renders, while the complete square layout has a differing pixel. The large rectangular cases and the full soft generators remain open.

A separate 24-render scale sweep used 2 through 7, C for the other controls, and seed 93281 + 7i. For noise_gaussian_spots_2 the 16² F32 target completed scales 2–6 and the 64×32 U16 target completed scales 2–4. For the v2 root the corresponding completed scales were 2–4 and 2–3. The remaining eleven local cases reached the traversal budget. The reference app produced all 24 outputs. Across the 13 comparable outputs 8,115/12,288 pixels matched. The maximum was 4 stored U16 levels and 5.22732735e-05 F32.

### Pattern tint, coverage and grayscale conversion

| Original stamp control | Renders | Previous exact | Current exact / pixels | Maximum U8 / U16 / F16 / F32 error |
|---|---:|---:|---:|---|
| One solid square, changed source channels and global opacity | 92 | 8,448 | 23,552/23,552 | — / — / — / 0 |
| 64 independently tinted squares | 78 | 16,612 | 26,112/26,112 | 0 / 0 / 0 / 0 |
| White and hashed bitmap inputs | 56 | 10,248 | 14,336/14,336 | 0 / 0 / 0 / 0 |
| Four nonconstant shapes with RGBA tint | 88 | 36,432 | 71,184/90,112 | 0 / 0 / 9.53674316e-07 / 0.000280380249 |
| Four shapes with a scalar expression | 88 | 72,398 | 75,796/90,112 | 0 / 0 / 2.38418579e-07 / 5.76451421e-05 |

Use additive blending, one full-size square on transparent black, roughness zero and a 16² F32 output. For the swatch (.9,.11,.57,.732), the grayscale square gives bits `3f06d3a0`. Returning the same value from a four-channel expression gives `3f3b645a`. Direct swatches use ((R + G) + B) / 3 in F32. Four-channel expressions supply A and scalar expressions supply their single value. A white point-sampled bitmap gives the same respective values. The alternate 16² input uses U8 intensity (41x + 29y + 13xy + 3) mod 256. Render both inputs with gains 1 and .6137. The solid-square controls also include red, green, blue, transparent black, signed and over-one channels, with gains 1, .6137, −1.1875 and 327.125.

For the color grid, fully subdivide three times and draw one white square in each leaf. Use the original scalar and four-channel expressions as well as literal swatches. Draw on RGBA (.125,.25,.375,.5) under all three blend modes. The 64 colors use PCG64 seed 90287 with four F32 draws per leaf, drawn uniformly from [−1.25,1.625). Include primary colors, equal RGB, zero and unit alpha, signed cancelling channels and values up to 32 times the original amplitude. Use (roughness,gain) pairs (0,1), (.95,.6137), (0,−1.1875), (1.03125,327.125), (.75,2.500080108642578), (0,.37) and (1,1). Alternate 32×16 and 16², with seeds 90739 + 11j. Cases j=1 and j=3 also cover U8, U16 and F16.

In color mode, first round A × depth opacity. Multiply each RGB channel by that result before applying shape coverage. Shapes 17 and 18 add their variation as part of the final multiply-add. Using the reference app’s unit-tint coverage as identical input reproduces 53,248/53,248 F32 pixels across shapes 5, 10, 17 and 18, with maximum error zero. MaterialCraft’s own nonconstant coverage still differs on the cases in the table. At (16,16) of a 32² shape 5 with the example swatch and gain .6137, RGBA gives `3ecd6456 3d48d3e4 3e8214e2 3ee43699`. A scalar expression returning .732 gives `3ee43699` in all four channels. Use seeds 91117 + 17j and variations 0, .125 and −.25.

| Full root on the [previous inputs](#stored-opacity-across-pattern-depths) | Renders | Previous exact | Current exact / pixels | Maximum U8 / U16 / F16 / F32 error |
|---|---:|---:|---:|---|
| brick_3 | 34 | 201,968 | 201,968/204,288 | 0 / 3 / 0 / 1.33514404e-05 |
| clouds_1 | 17 | 144,178 | 144,178/145,664 | 1 / 11 / 0.00048828125 / 0.000111341476 |
| clouds_2 | 16 | 128,263 | 128,263/129,280 | 1 / 3 / 0.00048828125 / 2.29477882e-05 |
| creased | 17 | 143,936 | 143,948/145,664 | 0 / 29 / 0.00048828125 / 1.52587891e-05 |
| crystal_2 | 17 | 144,774 | 144,770/145,664 | 0 / 2 / 0 / 3.05175781e-05 |
| dirt_1 | 17 | 145,661 | 145,661/145,664 | 0 / 1 / 0 / 1.51991844e-05 |
| dirt_3 | 17 | 145,351 | 145,359/145,664 | 0 / 1 / 1.52587891e-05 / 1.52736902e-05 |
| dirt_5 | 16 | 129,226 | 129,226/129,280 | 0 / 4 / 6.10351562e-05 / 5.0842762e-05 |
| dirt_6 | 16 | 117,822 | 117,826/129,280 | 0 / 8 / 0.133789062 / 0.140006408 |
| dirt_gradient | 16 | 128,449 | 128,396/129,280 | 0 / 2 / 0.000244140625 / 1.98483467e-05 |

Dirt 1 also has 12 new comparisons per setting. Repeat seed S eight times interleaved with S+1, S+2 and S+19 in the sequence 0,0,1,0,2,0,0,19,0,0,1,0 relative to S. At the default 128² U16 output with S=0, three pixels vary by one stored level within the eight reference runs. Every MaterialCraft value appears in at least one reference run, and two entire reference frames match it. At 64² with the recorded changed controls and S=42, one U16 pixel varies by one level and the local frame matches one entire repeat. The corresponding F32 reference output is byte-identical across all eight runs. It differs from MaterialCraft at the same single pixel by .0000151991844. Dirt 1 remains in progress.

Opening six MaterialCraft saves in the reference app reproduces 37,632/37,632 source-file pixels in 69 renders, maximum error zero. The files exercise a grayscale swatch, constant expression, color grid, gradient with a four-channel getter, radial shape with a scalar getter and a bitmap with an expression. Rerunning 19 previously verified pattern roots gives 3,211,776/3,211,776 exact pixels across 159 renders.

### Overlapping disc additions in Dirt 1

The whole root still matches 145,661/145,664 pixels across 17 renders. Its first differing stage draws grayscale discs at depths seven and eight. Connect that stage directly to the output, retain all four root quadrants or one at a time, and enable both drawing depths or either depth alone. To disable a depth, set its pattern type to zero and leave the descendants, offsets and seed expressions connected. Across the 15 variants, 120 renders match **629,340/629,760** pixels.

| Output format | Exact pixels / compared | Maximum absolute error |
|---|---:|---:|
| U8 | 61,440/61,440 | 0 |
| U16 | 61,439/61,440 | 1 level |
| F16 | 61,440/61,440 | 0 |
| F32 | 445,021/445,440 | 1.19209290e−7 |

For each variant render 64² with disorder .4 and seed 42 in F32, U16, F16 and U8. Then render 32² with disorder .15625 and seed 100189, 128² at defaults with seed zero, 64² with disorder .4 and seed 43, and repeat the first F32 case. At the pattern output with all quadrants and both depths enabled, the two seed-42 reference F32 frames differ at 29/4,096 pixels, maximum 5.96046448e−8. [Repeated final-root outputs](#pattern-tint-coverage-and-grayscale-conversion)

For an independent check take the centers, sizes and luminosities from MaterialCraft's traversal of each quadrant. Each quadrant emits 20,480 axis-aligned discs across both depths. At each pixel center, wrap the disc center to its tile, test the unit-disc radius and accumulate the covered luminosities with their depth opacities in traversal order using F32. This reconstruction matches all 384,000 local F32 pixels for the four distinct settings across the 15 variants. Against the reference it matches 383,644/384,000. At every one of the 356 unequal pixels some ordering of those same individual additions reproduces the observed reference bits exactly. The differing pixels each have three to eight contributions. Reversing the whole traversal matches 383,365 pixels, grouping by ascending depth matches 383,584, and accumulating in F64 before rounding matches 383,470. These results isolate the sensitivity to addition order. The remaining whole-root difference and the repeat variability are recorded separately.

### Stored opacity across pattern depths

| Direct pattern control | Renders | Exact pixels / compared | Maximum U8 / U16 / F16 / F32 error |
|---|---:|---:|---|
| One white square at a selected depth | 1080 | 359,424/359,424 | 0 / 0 / 0 / 0 |
| Independent roughness values and large exponents | 288 | 73,442/73,728 | — / — / — / 4.34931234e-39 |
| Power-rounding and overflow controls | 85 | 21,760/21,760 | — / — / — / 0 |
| Signed and extended global opacity | 192 | 49,152/49,152 | — / — / — / 0 |

On the main single-square matrix the previous cook matched 286,511/359,424 pixels. The updated cook matches 359,424/359,424, maximum error zero. In noise_fur_1 the isolated square at depth five now gives F32 bits `3ebaf23c` in both apps. On the complete pattern layout the flat square improves from 3,072/26,880 and the disc from 3,078/26,880 to 26,880/26,880 each. The isolated square and disc also match all 26,880 pixels each. The original soft profile at the root still differs. [Fur inputs and layout isolation](#fur-pattern-sources-and-downstream-filters)

Draw one white square with maximum blending at depth `d`. Give it size `(2^d,2^d)`, disable tiling and leave every other quadrant empty. Connect the only child of quadrant `j` at slot `j mod 4`. The following lists specify maximum depth and drawing depths. Use 0/[0], 1/[0,1], 2/[0,1,2], 3/[0..3], 4/[0..4], 5/[0..5], 6/[0,3,6], 8/[0,1,4,7,8], 10/[0,5,9,10] and 12/[0,6,12]. The deepest visited quadrant counts even when it draws nothing. For the main matrix use gain 1 with roughness [0,.125,.1875,.375,.5,.5625,.75,.8125,.9,.9375,.95,.99,.9999998807907104,1,1.03125,1.125,1.5,2,4]. Then use [.375,.95,1,1.5] with each gain in [.6137,2.500080108642578]. Number those settings `j` from zero. Use 32×16 when `j mod 3 = 0` and 16² otherwise, with seed `81799 + 13 × maximum depth + d + j`. Read F32 for every case and also U8, U16 and F16 at roughness .95 with the larger gain.

For positive roughness `r` and `n` visited depths, the first depth uses the geometric normalization `(1 − r) / (1 − pow(r,n))`. Store the endpoint power in F32 before subtraction. At unit roughness the first weight is `1/n`. Multiply the first weight by the global opacity, then multiply each stored parent opacity by the roughness to obtain the child. Zero roughness gives every depth the full global opacity. A chain with a square at depth 2, roughness .99 and gain 1 returns `3ea8f445`. With roughness 1.5 and gain .6137 it returns `3e94d6a2`. The 37 rounding-boundary cases distinguish the native single-precision power from a power evaluated in double precision and rounded once. For one witness use roughness 1.2230699062347412, depth 5 of 5 and gain 1. The observed output is `3e852977`. The 48 overflow controls use roughness `2^e` with `e` in [6,10,15,22,31,45] at (maximum depth, drawing depth) [(5,5),(1,0),(3,1),(10,10),(2,2),(8,4),(4,3),(12,12)]. A depth-five square with roughness 4,194,304 returns zero. MaterialCraft previously limited the roughness passed to the cook to 34.

In the 192 additive square controls use (0,0), (1,0), (3,1) and (5,5), the six roughness values [0,.95,1,1.5,34,64] and global opacity [−1,0,.6137,10,170,171,327.125,65536]. Render at 16² F32 with seed `86917 + j`. The unit square at depth zero returns the input opacity, including negative values and values above 170. With a four-component quadrant tint and a colored background the additional 84 renders match 29,448/35,328 pixels. Their maximum F32 error is 8064.03027. Those controls expose a separate tint discrepancy.

For the independent roughness controls, use (2,0), (3,1), (4,4), (5,5), (6,3), (8,7), (10,10) and (12,12). Use F32 16² with seed `87211 + 23j` and gains [1,.37,1.73,2.500080108642578] cycling by case. The roughness list starts with [.0001,2^-18,.03125,.32,.73,.86,.9823,.99873], the adjacent F32 values below and above one, then [1.000012,1.0043,1.33,2.71,4.125,7,13]. Append 13 values drawn uniformly from [.01,2.25] with one NumPy PCG64 stream seeded 80627 across the eight depth pairs, followed by the six powers of two above.

At this revision the independent controls had 286 differing pixels. For depth 7 of 8, roughness 2^-18 and gain .37, the local covered pixels have F32 bits `002f5c1d`. For depth 10 of 10, roughness .0001 and gain 1, they have bits `000116bb`. Each capture differs in 143 pixels and the reference output stores zero. The later [opacity and accumulator controls](#pattern-opacity-near-zero-and-color-accumulation) reproduce both cases exactly. The power-rounding measurements use the Windows reference session.

| Original root | Renders | Exact pixels before | Exact pixels after / compared | Maximum U8 / U16 / F16 / F32 error after |
|---|---:|---:|---:|---|
| brick_3 | 34 | 200,505 | 201,968/204,288 | 0 / 3 / 0 / 1.33514404e-05 |
| clouds_1 | 17 | 144,184 | 144,178/145,664 | 1 / 11 / 0.00048828125 / 0.000111341476 |
| clouds_2 | 16 | 128,259 | 128,263/129,280 | 1 / 3 / 0.00048828125 / 2.29477882e-05 |
| creased | 17 | 143,936 | 143,936/145,664 | 0 / 29 / 0.00048828125 / 1.52587891e-05 |
| crystal_2 | 17 | 144,774 | 144,774/145,664 | 0 / 2 / 0 / 3.05175781e-05 |
| dirt_1 | 17 | 145,626 | 145,661/145,664 | 0 / 1 / 0 / 1.51991844e-05 |
| dirt_3 | 17 | 145,297 | 145,351/145,664 | 0 / 1 / 1.52587891e-05 / 1.52736902e-05 |
| dirt_5 | 16 | 129,218 | 129,226/129,280 | 0 / 4 / 6.10351562e-05 / 5.0842762e-05 |
| dirt_6 | 16 | 117,822 | 117,822/129,280 | 0 / 8 / 0.133789062 / 0.140006408 |
| dirt_gradient | 16 | 128,383 | 128,449/129,280 | 0 / 2 / 0.000244140625 / 1.98483467e-05 |
| fur_1 | 21 | 164,031 | 164,031/178,432 | 1 / 2 / 0.00048828125 / 3.05771828e-05 |
| noise_fur_1 | 21 | 122,553 | 122,549/178,432 | 1 / 3 / 0.00048828125 / 8.31484795e-05 |

These are full root outputs on the [original generator inputs](#square-and-rectangular-pattern-generators) and the linked Fur settings. Clouds 1 and noise_fur_1 have slightly fewer exact pixels on those inputs after the depth correction. Dirt 1 retains three differing pixels across 17 renders. All 19 previously verified roots in the direct FX-Map guard set still match 3,211,776/3,211,776 pixels in 159 renders, maximum error zero. The independently authored depth-five chain, fur_1 and noise_fur_1 were also saved via MaterialCraft and opened again in the reference app. Against the source-file renders, the 26 reopened cases match 61,184/61,184 pixels, maximum error 0. The root files used the current-format header for this handoff. Open each file at its save destination so package-relative dependencies resolve. A separately moved copy of noise_fur_1 lost its relative link to `functions.sbs` and matched only 24/28,928 pixels. Reopening the identical saved bytes at the original destination matches 28,928/28,928.

### Fur pattern sources and downstream filters

Each root has 21 raw comparisons, totaling 178,432 pixels per root. Each format cell gives exact pixels / maximum error. The denominators are 5,120 U8, 130,048 U16, 5,120 F16 and 38,144 F32. Integer errors are stored levels. Every differing stage-traced case first diverges in the FX-Map.

| Package root | Exact pixels | U8 | U16 | F16 | F32 |
|---|---:|---:|---:|---:|---:|
| fur_1 | 164,031 | 5,119 / 1 | 118,988 / 2 | 5,091 / 0.00048828125 | 34,833 / 3.05771828e-05 |
| fur_2 | 134,013 | 5,119 / 1 | 123,497 / 1 | 5,080 / 0.000244140625 | 317 / 1.06096268e-05 |
| fur_3 | 145,101 | 5,117 / 1 | 106,705 / 4 | 5,067 / 0.00048828125 | 28,212 / 6.10351562e-05 |
| fur_4 | 148,992 | 5,120 / 0 | 107,228 / 4 | 5,066 / 0.00048828125 | 31,578 / 6.10351562e-05 |
| noise_fur_1 | 122,553 | 5,117 / 1 | 112,234 / 3 | 5,027 / 0.00048828125 | 175 / 8.31186771e-05 |
| noise_fur_2 | 130,666 | 5,119 / 1 | 120,328 / 5 | 5,018 / 0.000244140625 | 201 / 7.84248114e-05 |
| noise_fur_3 | 132,982 | 5,119 / 1 | 122,585 / 3 | 5,048 / 0.000244140625 | 230 / 5.67138195e-05 |

The 12 stage-traced renders add to the four archived renders, four output-precision renders and one independent seed. In case order the new renders use 16², 32² five times, 64×32, 32×64, 64², 128×64, 64×128 and 128². Their output formats are F32, F32, U8, U16, F16, F32, F32, U16, F32, F32, U16 and F32. Number inventory rows from zero. Use seed `73939 + 7 × row` plus offsets 0, 1, 2, 2, 2, 2, 6, 7, 8, 9, 9 and 11. The first and last cases keep defaults. The other cases use the following tuples in the order A, B, B, B, B, C, C, B, B, B.

| Root | Parameter order | A | B | C |
|---|---|---|---|---|
| fur_1 | disorder | 0 | 0.375 | 7.5 |
| fur_2 | disorder, wave scale, wave rotation | 0, 0, 0 | 0.3125, 0.5625, 0.125 | 0.71875, 0.25, 0.8125 |
| fur_3 | density, disorder | 0.1875, 0.125 | 0.4375, 0.625 | 0.75, 0.25 |
| fur_4 | disorder | 0 | 0.3125 | 0.71875 |
| noise_fur_1 | scale, disorder, expand non-square | 1, 0, on | 2, 0.375, off | 3, 0.6875, on |
| noise_fur_2 | scale, wave scale, wave rotation, expand non-square | 1, 0, 0, on | 2, 0.5625, 0.125, off | 3, 0.25, 0.8125, on |
| noise_fur_3 | scale, disorder, wave amount, expand non-square | 1, 0, 0, on | 2, 0.3125, 2.75, off | 3, 0.6875, 5.125, on |

For fur_1, fur_3 and fur_4, wire a U16 input to the consumers of the pattern generator. Across the 12 settings, supply 32767 everywhere for case 0, 52111 on the diagonal over 713 for case 1, and 32768 where `(x + 13y) mod 23 = 0` over 32767 for cases 6–8. Otherwise use `((11x + 31y + 7xy + 93) × 7919) mod 65536`. Three additional captures per root pass MaterialCraft's own generated U16 pattern into both renderers at cases 0, 5 and 8. Across all 45 equal-input renders the roots match 155,136/155,136 pixels. All 210 stage images match 723,968/723,968 pixels. The reference app also reproduces the original local root at all 16,128 pixels fed from the local pattern, maximum error zero.

To isolate the generator, route it directly to the output and render cases 0, 2, 3, 4, 5, 6, 8 and 11 with each layout below. Retain the authored pattern, replace it with a flat square or replace it with a disc. For the single-path layout, follow the shortest path to an explicit drawing quadrant, traversing sibling connection identifiers in ascending order. Start at the first quadrant on that path and retain only its path to the selected drawing quadrant. Each cell gives exact raw pixels out of 26,880.

| Package root | Full, authored | Full, square | Full, disc | Single path, authored | Single path, square | Single path, disc |
|---|---:|---:|---:|---:|---:|---:|
| fur_1 | 3,542 | 26,880 | 26,880 | 26,691 | 26,880 | 26,880 |
| fur_2 | 3,219 | 26,879 | 26,878 | 26,819 | 26,880 | 26,880 |
| fur_3 | 5,217 | 26,880 | 26,880 | 26,855 | 26,880 | 26,880 |
| fur_4 | 5,403 | 26,880 | 26,880 | 26,819 | 26,880 | 26,880 |
| noise_fur_1 | 3,022 | 3,072 | 3,078 | 26,832 | 26,823 | 26,832 |
| noise_fur_2 | 3,078 | 26,874 | 26,878 | 26,833 | 26,880 | 26,880 |
| noise_fur_3 | 3,119 | 26,879 | 26,875 | 26,776 | 26,880 | 26,880 |

For noise_fur_1 the single square has the same coverage in both renderers. All 57 covered pixels across the five F32 renders have local bits `3ebaf23a` and reference bits `3ebaf23c`, maximum error 5.96046448e−8. The path has six quadrants, with the white square at depth five, roughness 0.949999988079071 and global opacity 2.500080108642578. Each of the other six single-square probes matches all eight renders. Some complete layouts still differ in coverage, and the authored soft pattern differs on each isolated path. All seven roots remain open.

### Fibers and Fluid pattern stages

All four roots first differ in their FX-Map. Each row covers 24 full-root renders and compares 5,120 pixels each in U8 and F16, 285,696 in U16 and 128,256 in F32. Cells show exact pixels and the largest absolute error in that storage format.

| Package and root | All formats, exact / compared | U8 | U16 | F16 | F32 |
|---|---:|---:|---:|---:|---:|
| `fibers_1.sbs/fibers_1` | 416,631/424,192 | 5,120 · 0 | 280,553 · 1 | 5,116 · 3.05175781e-05 | 125,842 · 1.53183937e-05 |
| `fluid.sbs/fluid` | 423,928/424,192 | 5,120 · 0 | 285,544 · 201 | 5,120 · 0 | 128,144 · 0.00306707621 |
| `noise_fluid.sbs/fluid` | 396,306/424,192 | 5,118 · 1 | 263,238 · 79 | 5,103 · 0.0009765625 | 122,847 · 0.00108343363 |
| `pattern_fibers_1.sbs/fibers_1` | 416,669/424,192 | 5,120 · 0 | 280,422 · 1 | 5,116 · 3.05175781e-05 | 126,011 · 1.53183937e-05 |

Start with the four archive renders and the five [precision and independent runs](#four-format-inventory-comparison). Use the following settings for the additional 15 runs. The indices are 137 and 361 for Fibers and Pattern Fibers, and 153 and 272 for Fluid and Noise Fluid, respectively.

| Controls in graph order | A | B | C |
|---|---|---|---|
| Fibers, tiling | 1 | 2 | 3 |
| Pattern Fibers, tiling and non-square expansion | 1, on | 2, off | 3, on |
| Fluid, disorder, warp intensity, flow, pattern length, pattern width | 0, 0, 0, .5, .5 | .3125, 17, .1875, .625, .25 | .71875, 41, −.125, .8125, .375 |
| Noise Fluid, scale, disorder, warp intensity, pattern size, non-square expansion | 1, 0, 0, (.5,1), on | 2, .3125, .1875, (.625,.25), off | 3, .71875, .4375, (.8125,.375), on |

At 16² use defaults in F32. At 32² use A in F32, then B in U8, U16, F16 and F32. Use C at 64×32 in F32 and 32×64 in U16. At 64² use B in F32. The seeds are 31721+7i plus offsets 0, 1, 2, 2, 2, 2, 6, 7 and 8, where i is the root index. The last six profiles are 128² F32 at defaults/41713, 256² F32 at defaults/0, 256² U16 at defaults/41713, 256×128 F32 and 128×256 U16 at B/13217, and 256² U16 at C/6217.

In Fluid at 256² and seed 0, three of the 16,384 U8 pattern samples differ by a single level. At (76,1) the local and reference levels are 139 and 140. At (17,104) they are 226 and 225, and at (43,110) they are 210 and 209. At C and seed 6217 only (5,45) differs, at 212 and 213. Reconnect the downstream filters to a 128² grayscale input with the same stored format. Using the locally rendered field reproduces the 112 differing default output pixels in U16 and F32 and the 40 differing pixels in the C U16 output. Adjusting the three default samples or the single C sample in the scratch control makes all 65,536 output pixels exact in each of the three comparisons with the original root.

The two versions of each input field were measured at 256² U16 and F32. Three independent 128² byte inputs add four output formats each. At texel (x,y) the first field is (11x+31y+7xy+93) mod 256. The second is 237 on the main diagonal and 13 elsewhere. The third is 128 when (x+13y) mod 23 is zero and 127 elsewhere. Use seeds 16319, 16390 and 16461, with defaults for the first two and C for the third. The 20 renders match all **1,310,720 output pixels**, and all 6,225,920 pixels in the 200 observed stage images, maximum zero.

For each graph, connect its pattern primitive directly to the graph output. Use defaults at 128²/0, 256²/0 and 256²/41713, then C at 256²/6217. Also repeat 256² defaults at seed 0 in a second visit. Each visit covers U8, U16, F16 and F32. The primitive's own size rules apply within the parent. Check the resolved storage in both apps. The isolated native U8, U16 and F32 buffers also match those of the full graph with the same pattern storage in all 60 paired captures.

| Isolated pattern | Exact / compared in 20 renders | U8 max | U16 max | F16 max | F32 max |
|---|---:|---:|---:|---:|---:|
| fibers_1.sbs/fibers_1 | 347,323/458,752 | 1 | 1 | 0.00048828125 | 1.10268593e-06 |
| fluid.sbs/fluid | 217,580/278,528 | 1 | 1 | 0.00048828125 | 4.17232513e-06 |
| noise_fluid.sbs/fluid | 209,826/278,528 | 1 | 2 | 0.00048828125 | 2.87890434e-05 |
| pattern_fibers_1.sbs/fibers_1 | 347,323/458,752 | 1 | 1 | 0.00048828125 | 1.10268593e-06 |

The isolated Fibers primitives use Gaussian stamps. Both Fluid primitives use the asymmetric quartic profile at a quarter turn and variation .779999971. Their remaining differences are present in the pattern output before the downstream transform, blur, Levels and warp filters. The [single-stamp measurements](#horizontal-stamp-slope-and-height) constrain the shared coordinate calculation.

### Stored grayscale stops and scalar Levels controls

Three fractal sum roots match **436,992/436,992 output pixels across 54 renders**, maximum error zero. Each root contributes 119,808 U16, 5,120 U8, 5,120 F16 and 15,616 F32 pixels. Use the nine inventory renders and the nine seed, size and parameter profiles recorded with the [box blur measurements](#box-blur-and-signed-pixel-sizes). The comparisons include defaults, changed roughness and opacity, restricted octave ranges and both rectangular orientations. All reachable stage images in the nine instrumented renders per root also match.

| Package and root | Exact before / compared | Exact after / compared | Instrumented images | Exact stage pixels |
|---|---:|---:|---:|---:|
| `fractal_sum_base.sbs/fractal_sum_base` | 142,777/145,664 | 145,664/145,664 | 1,188 | 1,007,841,792 |
| `noise_fractal_sum_base.sbs/fractal_sum_base` | 143,104/145,664 | 145,664/145,664 | 1,008 | 1,310,976 |
| `noise_fractal_sum_base.sbs/fractal_sum_base_2` | 143,120/145,664 | 145,664/145,664 | 1,008 | 1,310,976 |

On these roots, a linear grayscale Gradient Map stores the arithmetic mean of its RGB stop channels as a normalized U16 word before interpolation. Clamp the mean to [0,1] and round to the nearest stored level. The first stop at a shared stored position owns that position. Interpolate in the existing 256-entry table's index space and sample the stored table with eight-bit weights. An original four-stop ramp at positions 0, .5, .504218996 and 1 with grayscale values 0, 1, 0 and 1 gives 35,142 at input 128/255. This sample distinguished two calculations that both matched 203,520 pixels in the first fifteen affected root stages.

For a three-stop linear ramp at 0, .5 and 1, give stop j an RGB level of ((71j+23j²+51) mod 283)/282. For input columns 68, 77, 86, 143, 214 and 234 divided by 255, the U16 outputs are 23,502, 25,044, 26,586, 29,657, 11,152 and 5,939. Use 64 evenly spaced stops with the same levels at j=0…63. Columns 2, 62, 102, 144, 181 and 211 give 22,648, 10,313, 21,474, 34,965, 12,604 and 63,000.

A grayscale Levels control fed by a scalar function uses the scalar directly. At a white input, a scalar output-high with F32 bits 0x3ec0c0c2 yields exactly those bits. An explicit four-component control with that value in RGB and alpha one yields 0x3ec0c0c3. The mean of three equal components rounds once more. A direct scalar input link has the same behavior as a scalar literal. An explicitly repeated four-channel swizzle has the same grayscale behavior as the vector control. The property inspector retains its four displayed components. Changing the function type with a warm cache reproduces both results.

The standalone Levels comparison matched **20,480/20,480 pixels in 80 renders**. Use scalar bit patterns 0x3ec0c0c2 and 0x3bc0c0c2. For each, set output-high to a literal RGBA property, a scalar function constant, an RGBA function constant, an explicit four-lane scalar repeat or a direct link to a scalar graph input. Set the alpha of the two RGBA constants to one. Use 16² gray and color F32 inputs, all four output formats and seed 24389 or 24390 respectively. Set H=((83x+131y+521xy+31) mod 389)/388 and replace rows zero and one with 1 and .5. For RGBA shift each row of H right by 0, 3, 5 and 7 columns with wraparound. In four reduced fractal graphs with a white 8² input, all 1,536/1,536 F32 pixels across 24 weight settings match. The prior scalar implementation matched 1,216. The full storage ramps and explicit-swizzle controls also retain 528,384/528,384 exact pixels in 24 renders.

Ordinary saves of the three current-header root graphs reopened in the reference app. They reproduced **40,704/40,704 pixels in 27 renders**. Twenty standalone Levels files and thirteen standalone gradient files also reproduced all **978,944 pixels in 158 reopened renders**.

| Standalone Gradient Map series | Renders | Exact pixels / compared | Maximum U8 / U16 levels | Maximum F16 / F32 error |
|---|---:|---:|---:|---:|
| Gray and mixed-color stops | 72 | 851,059/884,736 | 1 / 1 | 0 / 2.22027302e-06 |
| Shared positions, signed stops and endpoints | 42 | 514,547/516,096 | 175 / 1 | 0 / 1.1920929e-07 |
| Steep and shallow segments | 72 | 880,464/884,736 | 1 / 1 | 0.00048828125 / 3.15904617e-06 |
| Byte tables with exhaustive inputs | 24 | 835,309/835,584 | 1 / — | — / — |

For the first series use 2, 3, 7, 64, 19 and 11 stops, each in both gray and mixed RGB. Channel c at stop j is ((71j+23j²+31c+11jc+51) mod 283)/282. For gray use channel zero for all three RGB components. The positions are j/(N−1), except the 11-stop series uses (j+.175×(j mod 3))/(N−.65). For N=19 use smooth interpolation with the stored midpoint field ((j mod 5)−2)×.15625. Use linear interpolation for the others. Render the F32 input x/255 at 256×16 U16 and (x+256y)/65535 at 256² U16. Use the [signed modular grayscale field](#gradient-warp-displacement-and-sampling) at 32² in all four output formats with seed 23473+17k, where k starts at zero and gray precedes mixed RGB. For the linear cases alone, 703,484/706,560 U16 pixels match, maximum one level.

For the byte-table controls, repeat the two 256-wide ramps for all twelve stop sets at U8 output. For the seven limit cases use these (position, gray level) pairs. Apply the same six renders and a common 32² field with seed 23473.

| Case | Stops |
|---:|---|
| 0 | (0,−.25), (.351,1.25), (.697,−.25), (1,.75) |
| 1 | (.125,.3125), (.5,.875), (.5,.1875), (.875,.625) |
| 2 and 3 | As in case 1, with the middle positions a−.0000001 and a+.0000001 for a=.5 or 128/255 |
| 4 | (.3,.15125) |
| 5 | (.25,.67), (.75,.125) |
| 6 | (0,1), (1,0) |

For the twelve steep and shallow cases use four positions (0,x,x+d,1) with values (0,1,0,1). Use (x,d)=(.5,.004218996), (.5,1/255), (.5,2/255), (.25,.004218996), (1/3,.004218996), then (.5,.004218996) twice with values (.1,.9,.2,.9) and (.2,.85,.3,.85). Finish with (.4675,1.1/255), (.375,1.7/255), (.75,.004218996), (.5,.0001) and (.5,.1). Apply the six renders from the limit cases. Across the 36 U16 outputs, 846,214/847,872 pixels match with maximum error one level. Smooth tables, HDR stops at U8, and some independent linear table entries retain the differences in the table above.

White Noise retained all 229,376 U16 pixels in five earlier renders. Eight more original-seed renders matched 24,832/24,832 pixels. Use 16² F32 at seed zero, 32² in all four formats at seed 11231, 64×32 U16 at 7159, 32×64 F32 at 9677 and 128² F32 at 17233. The root has no exposed controls. The release replay of 30 previously verified roots with reachable gradient or scalar-control paths retained 5,057,536/5,057,536 exact pixels across 256 renders, maximum zero.

### Tone curve sampling and saved handles

The `noise_upscale` root matched **142,592/142,592 output pixels across 17 renders**, maximum error zero. The eight instrumented renders also matched all 62,976 pixels across 48 images. Before the change the same root matched 2,687 output pixels, with maximum errors of 15 U8 levels, 3,791 U16 levels, .228057861 F16 and .228064887 F32. The curve is the first differing filter on the independent input.

Use the [nine inventory renders](#four-format-inventory-comparison), then the [eight additional input profiles for the noise blend](#box-blur-and-signed-pixel-sizes) with a single grayscale input, graph defaults and seed 14604 in place of 14578. The 17 renders account for 119,808 U8, 5,120 U16, 5,120 F16 and 12,544 F32 output pixels. On the changed 32² floating case the U8 codes 32 and 128 entering the curve yield .250977516 and −.228034332. The latter measures the narrow descending segment between adjacent anchors.

A broken outgoing or incoming handle retains its authored position in the file. Its effective control point is the anchor. For two broken handles the sampled segment follows its secant. The table has 1,024 float entries. For index i, evaluate x=i×(1/1023), with the reciprocal rounded to F32. The anchor covers table centers within half a step. Each segment still supplies at least one interior sample, including when several anchors occupy a single texel. Sample the stored table with eight-bit linear weights before converting to the output format. For example, use anchors (0,.75), (.41031,.25), (.41036,1.125), (.4104,−.0625), (1,.875) with both handles broken. Indices 420, 421 and 422 give .25, 21.6786346 and −62.7770119.

Ordinary saves preserved the active and dormant handles and lock flags. All **78,080/78,080 pixels in 56 reopened renders** matched the original files in the reference. These cover eight original standalone packages and a current-header copy of the complete noise upscale graph. Before the fix, five of those standalone files changed appearance on save. Only 110/42,240 pixels matched across their 30 renders.

| Standalone curve measurements | Renders | Exact pixels / compared | Max U8 / U16 levels | Max F16 / F32 error |
|---|---:|---:|---:|---:|
| Closely spaced anchors, broken handles | 24 | 393,216/393,216 | 0 / 0 | 0 / 0 |
| Ten curves on scalar ramps through an RGBA bridge | 60 | 74,253/84,480 | 1 / 3 | 0.00048828125 / 5.05000353e-05 |
| Independent incoming and outgoing handle flags | 36 | 30,877/50,688 | 1 / 28 | 0.00048828125 / 0.000418391079 |
| Seven curves at table boundaries and off-grid inputs | 21 | 171,363/344,064 | 0 / 0 | 0 / 0.000418391079 |
| Color fields with varying alpha | 84 | 34,099/101,376 | 1 / 3 | 0.25 / 0.25 |

For the first series, use these eight ordered sets of (x,y) anchors. Give each a left/right handle at x−.03125 and x+.046875, both at the same y and both broken.

| Set | Anchors |
|---:|---|
| 0 | (0,.125), (.4122,.875), (.4139,.0625), (1,.6875) |
| 1 | (0,−.125), (.5,.875), (.5012,.0625), (1,.8125) |
| 2 | (.0733,.375), (.3261,.875), (.3262,−.0625), (.8123,.625) |
| 3 | The five anchors in the example above |
| 4 | (0,.1875), (.25,.5625), (.75231,.40625), (1,.875) |
| 5–7 | Repeat set 4 with intermediate x positions (.2501,.7501), (.2504,.7506), (.2508,.7508) respectively |

Use three 128² F32 input images with seeds 21143+13j+h for set j and image h. For row-major pixel index p, set i=p mod 1024 and use offset a from row group floor(p/1024). The offsets are 0, 1/2, 1/8, 1/4, 3/8, 5/8, 3/4, 7/8, 1/256, 31/256, 63/256, 127/256, 129/256, 191/256, 223/256 and 255/256. The first two inputs are (i+a)/1024 and (i+a−.5)/1023. The third uses PCG64 seed 20219 and uniform samples on [−.0625,1.0625) rounded to F32. Replace its first twelve values with −.1, 0, 1, 1.1, 2⁻²⁰, 1−2⁻²⁰, .25, .25+2⁻²⁰, .5, .5+2⁻²⁰, .75 and .75+2⁻²⁰.

For the handle comparison, use endpoints (0,.0625) and (1,.8125), outgoing control (.1875,−.25) and incoming control (.578125,.9375). Test both flags true, both false, outgoing only, incoming only and both omitted in grayscale. Repeat both true in RGBA. Use the F32 ramp p/(W×H−1) at 16² F32, 32² in all four formats and 64² F32. The two independent controls, the omitted flags and their saved copies distinguish the file behavior from the remaining rendering differences. With active handles the continuous cubic and the reference's sampled curve can still differ. The RGBA fields also expose independent differences on alpha, including signed input. For a direct alpha control, an input of .996078432 gives .996078491 at F32 with the default alpha mapping.

### Box blur and signed pixel sizes

The `noise_blend` root matched **142,592/142,592 pixels across 17 renders**, maximum error zero. With the same inputs it previously matched 2,550 pixels. The eight additional renders include every reachable filter and input bridge. All 73,472 pixels in those 56 intermediate and final images matched. The root totals are 119,808 U8, 5,120 U16, 5,120 F16 and 12,544 F32 pixels.

Use the [four-format inventory comparison](#four-format-inventory-comparison) for the first nine renders. For the other eight, generate both inputs using the [modular field and storage formats used for Mosaic](#gradient-warp-displacement-and-sampling). Use channel zero, with seed 14578 for `input` and 31 more for `previous`. Add 0, 1, 2, 2, 2, 2, 6 and 7. For the 16² U16 input use graph defaults. At 32² with F32 inputs set roughness, global opacity, minimum level, maximum level, level and contrast to (.375,.5,0,3,0,.625). Next set (.6,.8,1,3,2,.8) at 32² with U16 inputs and all four output formats. Finish with (.25,.875,1,4,3,.375) using 128×64 F32 inputs at 64² and 32×64 U8 inputs at 32². All other outputs are F32.

Box Blur multiplies the radius intensity×min(W,H)/256 by the resolved pixel size on each axis. An axis whose radius is at most half a pixel copies its samples. The horizontal pass still stores its result at output precision before an active vertical pass. On a 32² unit impulse at intensity one with pixel size (1,64), the center is 1/16, with endpoints of 1/32 eight pixels above and below. With (64,1) the same values run horizontally. With (64,64) the center is 1/256. At a relative pixel size, multiply the two signed components by the corresponding input or parent components before using the filter. For example, (−2,3) followed by (−64,8) resolves to (128,24). A signed or zero component also survives an ordinary save.

| Comparison | Renders | Exact pixels / compared | Maximum error |
|---|---:|---:|---:|
| Independent X and Y blur radii | 135 | 203,520/203,520 | 0 |
| Signed sizes on connected Blur and Emboss nodes | 93 | 112,384/112,384 | 0 |
| Current-header saves of the four roots, reopened in the reference | 35 | 51,200/51,200 | 0 |
| Six standalone saves with authored or inherited sizes, reopened in the reference | 52 | 69,120/69,120 | 0 |

For the 135 blur controls, use an F32 center impulse at 16², 32², 32×64 and 64×32, and the signed modular field at 32² in U8/U16/F16/F32 and 64² F32. The seeds in render order are 19373, 19374, four copies of 19375, 19379, 19380 and 19381. Cross these with (pixel width, pixel height, intensity) of (1,1,1), (64,64,1), (1,64,1), (64,1,1), (0,16,4), (16,0,4), (.5,1.5,16), (1.5,.5,16), (−2,3,16), (3,−2,16), (2,4,8), (4,2,8), (16,32,1.25), (1,1,64) and (1,1,32). Tile both axes except for X only, Y only and neither on settings nine through eleven.

Repeat the same nine images at intensity one for five connected size controls. Use input/filter sizes (−2,3)/(−64,8), (−2,3)/(64,−8) and (0,4)/(8,−4), relative to the input. Repeat the last two with an absolute filter size. For the 48 relief comparisons set intensity one and angle .125 turns. Use input/filter (−2,3)/(1,1) and (0,4)/(1,1) relative, (1,1)/(−2,3) and (1,1)/(0,4) absolute, then (−2,3)/(−1,2) and (−2,3)/(1,−2) relative. Use the [relief fields](#emboss-lighting-and-physical-pixel-size): X ramp at 16² and 32², Y ramp, impulse, step, small field, hash with quarter gray and hash with three-quarter gray at 32², seeds 97, 126, 184, 213, 242, 271, 300 and 329.

Three fractal sum roots improved on the same 18 renders each. Their [subsequent gradient and scalar-control measurements](#stored-grayscale-stops-and-scalar-levels-controls) reached exact output on all 54. The first differences in the new measurements include five U16 gradient maps with matching inputs. The second and third roots also have isolated one-level errors in Levels. All four current-header root saves reproduced their original outputs when both files were rendered by the reference.

| Remaining root | Exact pixels before | Exact pixels after / compared | Maximum U8 / U16 levels | Maximum F16 / F32 error |
|---|---:|---:|---:|---:|
| `fractal_sum_base.sbs/fractal_sum_base` | 253 | 142,777/145,664 | 0 / 2 | 0.000244140625 / 3.05175781e-05 |
| `noise_fractal_sum_base.sbs/fractal_sum_base` | 238 | 143,104/145,664 | 0 / 2 | 0 / 1.52587891e-05 |
| `noise_fractal_sum_base.sbs/fractal_sum_base_2` | 311 | 143,120/145,664 | 0 / 2 | 0 / 1.52587891e-05 |

In addition to the nine inventory renders, use seed 19661+7i for root indices 154, 273 and 274. Add 0, 1, 2, 2, 2, 2, 6, 7 and 8. Use 16² F32 defaults, then 32² F32 with roughness .3125, global opacity .875 and levels 1–3. At 32² use (.6875,.625,2,6) for those controls in all four output formats. Use (.5,.75,1,8) at 64×32 and 32×64 F32, then repeat (.3125,.875,1,3) at 64² F32. These three roots have no image inputs.

### Gradient warp displacement and sampling

Both Mosaic roots and all three Slope Blur roots match the reference in **88 renders and 722,176 output pixels**, maximum error zero. The new intermediate comparisons also match all **3,461,888 pixels across 2,369 images**, including the input bridges. Each Mosaic contains 16 gradient warps and each Slope Blur contains 32. Before the fix the color Mosaic matched 1,024/142,592 pixels and the gray Mosaic matched 2,695/142,592 on the same inputs.

| Root | Renders | Exact pixels / compared | Maximum error |
|---|---:|---:|---:|
| `mosaic.sbs/mosaic` | 17 | 142,592/142,592 | 0 |
| `mosaic.sbs/mosaic_grayscale` | 17 | 142,592/142,592 | 0 |
| `slope_blur.sbs/slope_blur` | 18 | 145,664/145,664 | 0 |
| `slope_blur.sbs/slope_blur_grayscale` | 18 | 145,664/145,664 | 0 |
| `slope_blur.sbs/slope_blur_grayscale_2` | 18 | 145,664/145,664 | 0 |

All five include the four original renders, the [four-format comparison and independent 32² case](#four-format-inventory-comparison). For each Mosaic, the totals are 119,808 U8, 5,120 U16, 5,120 F16 and 12,544 F32 pixels. For each Slope Blur, the counts are the same except for 15,616 F32 pixels.

For the eight additional Mosaic renders, use the [modular input field](#stored-input-precision-in-box-and-directional-blur). For source and effect in that order, use channel zero for grayscale and add 31 to the effect's seed. The two root indices i are 227 and 228. Use seed 11003+13i and add 0, 1, 2, 2, 2, 2, 6 and 7. Start with 16² U16 input, F32 output, graph defaults. Use 32² floating input and output with one sample and zero intensity. Next use 32² U16 input with four samples and intensity .25 in U8/U16/F16/F32. Finish with eight samples and intensity .6875, using 128×64 floating inputs at a 64² parent and 32×64 U8 inputs at a 32² parent. Both outputs are F32. Store the floating fields as 1.5 times the normalized value minus .25.

For Slope Blur, use the same field and channel conventions with seed 18211+17i for root indices 484–486. Add 0, 1, 2, 2, 2, 2, 6, 7 and 8. Use 16² U8 inputs, F32 output and defaults, followed by 32² floating inputs and output with one sample, zero intensity and mode 0. At 32², use U16 inputs with four samples, intensity 1.375 and mode 1 in all four output formats. For the last three F32 outputs use parent 64×32, 32×64 and 64². Use respectively 64×32 floating inputs with eight samples, intensity 2.5 and mode 2, 64² U8 inputs with three samples, intensity .71875 and mode 0, and 64×128 floating inputs with twelve samples, intensity .25 and mode 1.

The displacement uses the current gradient texel minus its immediate left and upper neighbors. With equal input and output dimensions W×H and S=min(W,H), multiply those differences by intensity×S²/256 to get the source offset in output pixels. On a 32² unit impulse at intensity one, the center moves four pixels in both directions, the texel to its right moves four pixels left, and the texel below moves four pixels up. Each axis respects its own tiling flag. Nearest and bilinear filter the primary image at the displaced coordinates. A gradient up to twice the output size per axis can supply its texels directly. The difference then spans one stored gradient texel, scaled by the gradient-to-output size ratio on that axis.

| Standalone controls | Renders | Exact pixels / compared | Maximum error |
|---|---:|---:|---:|
| Planar sources, impulses, ramps and signed fields | 180 | 215,040/215,040 | 0 |
| Rectangular and square coordinate controls | 40 | 34,816/34,816 | 0 |
| Color and gray fields, native input sizes | 80 | 219,520/219,520 | 0 |
| Unequal inputs with an explicit output size | 32 | 50,176/50,176 | 0 |

For the 180 planar renders store the X and Y coordinate ramps (x+.5)/N and (y+.5)/N as separate F32 sources. Use a constant .5 map at 16², then a step map .5[x≥N/2]+.25[y≥N/2] at 16², 32² and 64². At 32² also use a unit center impulse, a ramp (x+2y)/128, and a border map .25[x<2 or y<2]+.5[x=N−1 or y=N−1]. Each bracket is one when true. For the four output formats at 32², use source (((11x+17y+3xy) mod N)+.5)/N and map (((11x+19y+7xy+23) mod 97)−21)/64. Cross these fields with intensities 0, 1, 4, .5, 8, −2.5, 8, 8, 8 and 31. Use nearest for the fifth setting and bilinear for the others. The seventh through ninth use no tiling, X only and Y only. The other settings tile both axes. Start at seed 10009 and add 19 per map/size/output setting. For the 40 coordinate controls, cross both ramps and the step and impulse with nearest and bilinear at 16², 32², 32×16, 16×32 and 64×32. Use intensity one, XY tiling and seed 18121+W+H.

For the mixed-size fields use the same modular field as the Mosaic roots above. Start at seed 17327+29j, add 13 for the gradient and add 0, 1, 1, 1, 1, 5, 6 and 7 for the eight renders. Use parent 16² F32, 32² in U8/U16/F16/F32, 64×32 F32, 32×64 F32 and 64² F32. The size powers in the table multiply the corresponding parent dimension by two to that power. The first series inherits the source dimensions at the warp. Repeat configurations 5–8 with the output explicitly set to the parent dimensions.

| j | Source / gradient storage | Source / gradient size powers | Intensity | Filter and wrapping |
|---:|---|---|---:|---|
| 0 | U8 gray / U8 | (0,0) / (0,0) | .0625 | Bilinear XY |
| 1 | U8 RGBA / U8 | (0,0) / (0,0) | .0625 | Bilinear XY |
| 2 | U16 RGBA / U16 | (0,0) / (0,0) | 3.25 | Bilinear XY |
| 3 | F16 gray / F16 | (0,0) / (0,0) | .41875 | Bilinear XY |
| 4 | F32 gray / F32 | (0,0) / (0,0) | .71875 | Nearest XY |
| 5 | F32 gray / F32 | (1,0) / (0,1) | .8375 | Bilinear XY |
| 6 | U16 RGBA / F32 | (−1,0) / (1,−1) | .0625 | Bilinear X |
| 7 | U16 gray / U8 | (2,1) / (1,2) | −1.375 | Bilinear Y |
| 8 | F32 RGBA / U16 | (−1,1) / (−2,−1) | 2.125 | Nearest, clamped |
| 9 | F32 gray / F32 | (0,0) / (0,0) | 1 | Bilinear, clamped |

Ordinary MaterialCraft saves of the five complete graphs in current-header SBS packages reopened in the reference. Their 43 additional renders reproduced all 61,696 output pixels, maximum error zero. The reference app rejected the two MaterialCraft saves whose inputs retained the older 0.1.3 package header. Three current-header standalone warp saves also matched all 42,496 pixels in 36 additional renders.

Ten other roots have a reachable gradient warp. Reuse the [square and rectangular generator controls](#square-and-rectangular-pattern-generators) for the first two Crystal roots, the [patch controls](#bitmap-stamp-filtering-and-compositing) for both patch roots and the [four-format inventory controls](#four-format-inventory-comparison) for the others. The color patch differs by one pixel in each of the four 64² formats. Grayscale matches U8 and differs by one pixel in U16, F16 and F32. Both independent 32² F32 patches differ by one pixel.

| Remaining root | Renders | Exact pixels / compared | Max U8 / U16 levels | Max F16 / F32 error |
|---|---:|---:|---:|---:|
| `crystal_1.sbs/crystal_1` | 17 | 144,060/145,664 | 1 / 55 | 0.000244140625 / 0.000106841326 |
| `crystal_2.sbs/crystal_2` | 17 | 144,468/145,664 | 0 / 19 | 0.000244140625 / 3.05473804e-05 |
| `fluid.sbs/fluid` | 9 | 131,984/132,096 | 0 / 201 | 0 / 0 |
| `make_it_tile_patch.sbs/make_it_tile_patch` | 9 | 132,090/132,096 | 1 / 93 | 0.00146484375 / 0.00212101638 |
| `make_it_tile_patch_grayscale.sbs/make_it_tile_patch_grayscale` | 9 | 132,092/132,096 | 0 / 47 | 0.000732421875 / 0.00105287135 |
| `microscope_view.sbs/microscope_view` | 9 | 131,217/132,096 | 0 / 61 | 0.000244140625 / 1.52606517e-05 |
| `noise_crystal_2.sbs/crystal_2` | 9 | 131,037/132,096 | 0 / 46 | 0.00048828125 / 1.52587891e-05 |
| `noise_fluid.sbs/fluid` | 9 | 129,723/132,096 | 1 / 71 | 0.00048828125 / 0.00016784668 |
| `noise_liquid.sbs/liquid` | 9 | 127,452/132,096 | 1 / 268 | 0.0009765625 / 0.000885009766 |
| `noise_microscope_view.sbs/microscope_view` | 9 | 130,081/132,096 | 1 / 48 | 0.000183105469 / 0.000137329102 |

### Pattern backgrounds and center culling

A connected FX-Map background with matching channels is resized at the wider of its input and output storage precisions. The resize proceeds one power-of-two level at a time. A connected input of the opposite bitmap type produces a zero background in the requested type. With the input disconnected, the authored background color applies. A grayscale map uses the mean of its three color components.

| Comparison with the reference | Renders | Exact pixels / compared | Maximum stored-component error |
|---|---:|---:|---:|
| Background alone, mixed sizes and storage | 84 | 150,528/150,528 | 0 |
| Square over a matching background, plus opposite-type controls | 105 | 188,158/188,160 | 2.98023224e−8 RGBA32F |
| Connected opposite type and disconnected, with and without a square | 60 | 52,224/52,224 | 0 |
| Independent 32² U16 field reduced to 8² | 3 | 192/192 | 0 |
| Center at or across the edges of the unit and custom tiling regions | 84 | 172,032/172,032 | 0 |
| Same custom regions with center culling disabled | 60 | 122,880/122,880 | 0 |
| Reversed, fractional and negative tiling regions | 60 | 61,440/61,440 | 0 |

For the background rows use configurations 0 and 2 through 12 of the [mixed-size blur matrix](#stored-input-precision-in-box-and-directional-blur), retaining the neutral Levels source, dimensions, formats, seeds and tiling. Connect that source to the pattern background. A single resize of the original texture matched only 90,461/150,528 pixels in the first row. For the second row, draw an additive square of size (.25,.3125) at (.3125,.65625), global opacity .625, with gray .421875 or RGBA (.5625,.3125,.8125,.71875). Also render configurations 2, 8 and 10 with the opposite output type and an empty pattern. All 138,768 pixels outside the square in the twelve matching-type configurations agree. The two differing pixels lie inside the square. For the third row keep those three opposite types, set the constant to (.125,.375,.8125,.25), and cross connected/disconnected with square/empty at 16² F32 and 32² in all four formats. The disconnected gray value is .4375. For the independent reduction use U16 codes ((17x+113y+59xy+19)×7919) mod 65536 at seed 15709. At 8² U8, pixels (2,0) and (5,0) contain 146 and 140.

When culling is enabled, the original stamp center must lie inside both tiling intervals before any copies are drawn. Each interval includes its lower endpoint and excludes its upper endpoint, including on axes with wrapping disabled. Use a square of size (.484375,.40625). In order, place its center at (.5,.5), (.1875,.1875), (.2490234375,.625), (.25,.625), (.75,.625), (.7509765625,.625), (.625,.125), (.625,.875), (.75,.875), (.125,.0625), (−.0625,.765625) and (1.0625,.4375). Rotate the tenth square .125 turns. With index j starting at zero, render at 64² when j is divisible by three and 32² otherwise, cycle F32/U16/F16/U8, and use seed 8209+17j. Use unit regions with XY wrapping and culling off and on. Then enable culling with tiling (.25,.75,.125,.875) and each of the four wrap masks, followed by XY wrapping with render region (−.5,1.5,−.25,1.25). Repeat those five custom configurations with culling off.

For the 60 endpoint controls, use render region (−1,1.5,−1,1.5), a .5-square, and tiling regions (.75,.25,.125,.875), (.25,.75,.875,.125), (.75,.25,.875,.125), (.1,.3,.15625,.84375) and (−.8125,−.1875,−.75,.5). Use respective wrap masks XY, none, X, none and XY. Test centers (.5,.5), (0,.625), (.75,.625), (1,.5), (.25,.875), (.5,.125), (.125,.5), (.3125,.5), (.625,.0625), (−.8125,−.75), (−.5,.25) and (−.1875,.25). All are 32² RGBA32F, seed 9833+17j. A reversed interval uses [0,1) on its axis.

Five ordinary MaterialCraft saves reopened in the reference and reproduced all 53,952 pixels in 39 renders, maximum error zero. They cover X-only culling, two reversed intervals, the 32² to 8² reduction and opposite-type backgrounds in both directions. The Paths Polygon root retained 5,120/5,120 exact buffer pixels across its 19 renders. The position debug root retained all 136,192 pixels in its seven equal-resolution controls and all 256 pixels in its separately matched preview. The Bevel Smooth debug position root matched 246,784/246,784 across 13 renders. [Polygon controls](#color-temperature-range-reductions-sky-parameters-and-polygon-paths) · [Matched position preview](#tinted-stamp-offsets-and-alpha)

The second Black and White Spots 1 root now matches 41,745/132,096 pixels over its nine controls, compared with 35,010 in the earlier run. U8 matches 36,930/118,784 with maximum error 16 levels, U16 matches 834/4,096 with maximum 147 levels, F16 matches 3,501/4,096 with maximum .002197265625, and F32 matches 480/5,120 with maximum .00223374367. The 64² four-format sweep uses seed 42, scale 2, disorder .37, speed .6, anisotropy .3, angle .125 turns, roughness .6 and tile offset (.17,.29). [Archived and independent controls](#empty-size-dependent-pattern-branches)

### Tiling axes and pattern copy bounds

A square stamp now wraps independently on each enabled axis. A repeated copy contributes only when its bounding box overlaps the tiling interval on every wrapped axis. A copy that only touches an endpoint contributes zero. Extending the render window beyond the tiling rectangle exposes the portions of admitted copies that extend into the larger window. A reversed tiling interval uses [0,1] on that axis. The tests exercise all four masks and both edges of a tangency.

| MaterialCraft compared with the reference | Renders | Exact pixels / compared | Maximum stored-component error |
|---|---:|---:|---:|
| Axis-aligned square, two regions, grayscale and RGBA | 112 | 200,704/200,704 | 0 |
| Default, expanded and reversed regions, RGBA32F | 22 | 90,112/90,112 | 0 |
| Rotated square, paraboloid, Gaussian and tangent squares, F32 | 81 | 160,564/163,328 | 2.14576721e−6 |

For the first row use maximum blending, an axis-aligned square and a black background. For each of masks 0, X, Y and XY, first use unit render and tiling regions, then render (−.25,1.25,0,1) with tiling (.125,.875,.25,.75). Regions are in left, right, top, bottom order. Render these seven settings, in order, with seed 7949+19j.

| Output | Center offset from (.5,.5) | Stamp size |
|---|---|---|
| 16² F32 | (−.46875,−.4375) | (.25,.3125) |
| 32² U8, U16, F16, F32 | (.421875,.453125) | (.375,.25) |
| 64² F32 | (−.734375,−.546875) | (.5625,.375) |
| 64² U16 | (.671875,−.59375) | (.3125,.125) |

At 64² F32 the covered pixel counts for 0, X, Y and XY are 27, 324, 72 and 864 in the unit region and 117, 216, 312 and 576 in the custom region. For the second row, draw the centered half-size square at seed zero and the first row's 16² geometry at seed 1511, both at 64². With render (0,2,0,1) compare the unit and matching tiling regions, tiling disabled and the tiling region omitted. Also render (0,2,0,2) with unit tiling, and (−.5,1.5,0,1) with X tiling. Finally use a unit render window and tiling (.25,.75,.125,.875), reversing X, Y and then both axes. Repeat the last setting with tiling disabled.

For the final row, render each F32 shape separately at the following settings. Capture X, Y and XY tiling and the individual untiled copies offset by integer multiples of the tiling interval. The pixelwise maximum of the admitted untiled reference copies reproduces all 32,256/32,256 pixels in the 18 tiled reference renders, maximum zero. The cross-application counts above include the separate shape renders. Small radial-profile differences remain.

| Shape | Output | Center | Size | Turns | Tiling region | Render region | Seed of tiled image |
|---|---:|---|---|---:|---|---|---:|
| Square | 64×64 | (0.0625, 0.21875) | (0.34375, 0.40625) | 0.125 | (0.125, 0.875, 0.25, 0.75) | (-0.25, 1.25, -0.5, 1.5) | 3322 |
| Square | 16×32 | (1.390625, 0.8125) | (0.53125, 0.6875) | -0.1875 | (-0.375, 1.125, -0.125, 0.75) | (-0.75, 1.5, -0.5, 1.25) | 3393 |
| Paraboloid | 32×64 | (-0.234375, 0.109375) | (0.21875, 0.3125) | 0.25 | (0.3125, 0.6875, 0.25, 0.625) | (-0.25, 1.25, -0.375, 1.375) | 3464 |
| Gaussian | 32×64 | (0.21875, -0.15625) | (0.5625, 0.4375) | 0 | (-0.375, 0.125, -0.625, 0.125) | (-0.875, 0.625, -1, 0.625) | 3535 |
| Square | 32×32 | (0.125, 0.875) | (0.25, 0.5) | 0 | (0.25, 0.75, 0.125, 0.875) | (0, 1, 0, 1) | 3606 |
| Square | 32×32 | (0.875, 0.125) | (0.25, 0.5) | 0 | (0.25, 0.75, 0.125, 0.875) | (0, 1, 0, 1) | 3677 |

### Stored input resizing and grayscale Levels controls

Directional noise at 16² now matches all 256 F32 pixels after its 8×16 U16 Blend. Before the fix, the Blend matched 128/128 and the following Levels matched 59/256, with a maximum F32 error of 8.49366188e−6. The remaining first difference at the 32² default is in a U16 FX-Map, with 31/32 pixels equal and a one-level maximum error. On the 64² F32 case, 4,037/4,096 final pixels match with maximum error 3.36766243e−5. The maximum across the earlier comparison was 2.94148922e−5. The larger exact count and the higher maximum are both retained below. [Square and rectangular generator inputs and seeds](#square-and-rectangular-pattern-generators)

| Root | Renders | Previously exact | Current exact pixels / compared | Max U8 / U16 levels | Max F16 / F32 error |
|---|---:|---:|---:|---:|---:|
| Black and white spots 3 | 27 | 139,099 | 139,314/185,856 | 1 / 3 | 0.00048828125 / 5.4359436e-05 |
| Clouds 1 | 17 | 131,293 | 144,125/145,664 | 1 / 11 | 0.00048828125 / 0.000111579895 |
| Crystal 2 | 17 | 104,017 | 104,128/145,664 | 14 / 12040 | 0.052734375 / 0.136126161 |
| Directional noise | 23 | 97,505 | 145,135/147,712 | 1 / 3 | 0.00048828125 / 3.36766243e-05 |

Across 300 direct comparisons, all 1,421,760 pixels match in their declared storage, with maximum component error zero. A color pixel counts once when all four components agree. The change uses the stored input precision at destination pixel centers. Intermediate resizes proceed in steps of up to a factor of two. A Levels enlargement stores at the wider input or output precision before applying its curve. The final reduction samples the previous level directly. In grayscale, each of the five Levels controls averages red, green and blue as ((R+G)+B)/3 before any integer endpoint clamp. Fixed Levels and Blend controls receive both resolved tiling axes.

| Direct comparison | Renders | Exact pixels / compared |
|---|---:|---:|
| Original gray and color fields, changed size and precision | 84 | 150,528/150,528 |
| Independent sizes, tiling axes and changed curve | 105 | 188,160/188,160 |
| Isolated scalar endpoints | 12 | 49,152/49,152 |
| Independent RGB controls, grayscale and RGBA | 60 | 245,760/245,760 |
| Complete normalized 16-bit ramp, two curves, four outputs | 8 | 524,288/524,288 |
| Same ramp through a half input, two curves, two float outputs | 4 | 262,144/262,144 |
| Two widths and small integer ramps | 11 | 704/704 |
| Four tiling masks, Levels and Blend, U16 and F32 | 16 | 1,024/1,024 |

The first two field series previously matched 83,073/150,528 and 73,501/188,160 pixels. To reproduce the resize fields, calculate H=((131x+367y+29xy+41c+17xc+seed) mod 2053)/2052 for component c starting at zero. For byte and word input, round H to the nearest stored level. For a floating input use 1.5H−.25 and apply the stated bridge format. Compare 16² F32, 32² U8/U16/F16/F32 and 64² F32/U16 in that order. The seed for field j is 11677+11j plus offsets 0,1,1,1,1,5,6. The changed curve has input low .0705882013, midpoint .522988021, high 1, output low .0134100001 and high .955938995 on RGB. Alpha uses the neutral curve. Intermediate clamping is off.

The source size is the output size multiplied by (2ˣ,2ʸ) for the exponents below. The optional neutral Levels stage has the source dimensions and format. Tiling masks 0,1,2,3 enable neither axis, X, Y and both. In the second group every field includes the neutral stage and uses the changed curve.

| Fields j | Source and dimensions | Controls |
|---|---|---|
| 0–2 | RGBA32F (0,0), RGBA16 (−1,0), LUM16 (−1,0) | Direct, neutral, mask 3 |
| 3–5 | LUM16 (−1,0), (−1,0), (0,0) | Neutral stage, neutral curve for 3 and changed curve for 4–5, mask 3 |
| 6–8 | LUM16 (0,−1), (1,0), (2,1) | Neutral stage and curve, mask 3 |
| 9–11 | RGBA8, RGBA16F and RGBA32F, all (−1,0) | Neutral stage and curve, mask 3 |
| 91–94 | LUM16 (−1,−1) | Masks 3,0,1,2 |
| 95–99 | LUM16 (−2,−1), (−2,−2), (−1,1), (1,1), (2,2) | Mask 3 |
| 100–102 | RGBA16F (−1,−1), RGBA32F (−1,−1), RGBA8 (−2,−1) | Mask 3 |
| 103–105 | RGBA16F, RGBA32F and RGBA8, all (−1,0) | Masks 0,0,3 |

For the 256² ramp, use U16 code 256y+x or store (256y+x)/65535 through an F16 bridge. Compare neutral and changed curves, using seeds 6347 and 6348 for U16, 6733 and 6734 for F16. The 64² independent-control input uses U16 codes round(65535((41x+17y+13xy) mod 4093)/4092). Set the first eight codes to 0,1,2,3,65532,65533,65534,65535. One unclamped grayscale control uses input low (−.5,.25,1.625,−2), mid (−.3,.53,1.25,.1), high (.601,−.375,1.875,2.5), output low (−.125,−.875,1.625,−.5) and high (1.875,1.125,−.375,2.125). At source level 45,308 the result is U16 level 55,876 or F32 bits 0x3f5a44c2.

For the tiling control, enlarge a 4² grayscale word field with code (4y+x)×4096 to 8². Use a neutral Levels or a foreground-only Blend at full opacity. In mask order 0,1,2,3, the top-left U16 levels are 0, 3072, 12288 and 15360 in both operations. Four regression tests cover source filtering, half precision before the curve, grayscale control conversion and independent tiling axes through the evaluator and its cache.

### Saved filter graphs and scalar Levels controls

A grayscale Levels graph with an exposed scalar expression matches **4,096/4,096 pixels in 16 renders**, maximum zero. Bind a uniform .5 F32 input at 16². Set the lower input control to a float4 swizzle of (amount−.25). With the default swizzle, the expression occupies red and the other RGB components are zero. At amounts .75 and .5, the raw F32 outputs are .40000003576278687 and .4545454680919647. An explicit (0,0,0,0) swizzle instead broadcasts the scalar and produces 0 and .3333333432674408. Repeat both graphs at amounts .9375 and .25. Use seeds 9217 through 9220 in that order and render both U16 and F32. The engine regression also changes the input, undoes it, redoes it and reloads the saved package.

Five original graphs saved through MaterialCraft's file commands reopened in the reference with **52,224/52,224 pixels identical to the reference renders of the source files**, maximum zero. Both the imported and written packages use schema 1.1.0.202502. Compare the seven gray Y-only and seven RGBA X-only [custom-region square renders](#tiling-axes-and-pattern-copy-bounds), seven box blur renders at index 0 and seven directional renders at index 15 and .25 turns in the [input-resize matrix](#stored-input-precision-in-box-and-directional-blur), and the eight default-swizzle Levels renders above. The original procedural fields, exposed values, output formats and dimensions were the same on both sides of each comparison.

### Repeated seeds in dirt and pattern generators

At 16² F32 and seed 9802, Dirt 5 produces two reference values at pixel (12,4) on repeated evaluations of the same graph. The other 255/256 pixels agree in all eight observations. With the intermediate images included, the output was 0.21309643983840942 three times and 0.21314725279808044 five times. With only the output observed, the counts were seven and one. The maximum difference in both series is 0.0000508129597. The U16 FX-Map at that coordinate alternates between levels 36,829 and 36,830. The directly connected F32 Levels image also alternates. In the original 16² MaterialCraft render the pixel is 0.21309643983840942. The recorded overall root counts still include the comparison against the original reference observation. [Square and rectangular pattern generators](#square-and-rectangular-pattern-generators)

Each series evaluates the seeds S + (0,0,1,0,2,0,0,19,0,0,1,0) in order, keeping the original procedural inputs and reading raw images after each graph evaluation. The table compares the eight visits to S. Each row counts a pixel once if every component has identical stored bits in all eight visits.

| Root | Side | Seed S | Stable final pixels / compared | Maximum variation in final output |
|---|---:|---:|---:|---:|
| Dirt 5, intermediate images included | 16 | 9802 | 255/256 | 0.0000508129597 |
| Dirt 5, output only | 16 | 9802 | 255/256 | 0.0000508129597 |
| Dirt 5 | 32 | 9803 | 1,024/1,024 | 0 |
| Dirt 1 | 16 | 9776 | 256/256 | 0 |
| Dirt gradient | 16 | 9828 | 256/256 | 0 |
| Crystal 1 | 16 | 9633 | 256/256 | 0 |
| Crystal 2 | 16 | 9646 | 256/256 | 0 |
| Alveolus | 16 | 8359 | 256/256 | 0 |
| Directional noise | 32 | 9764 | 1,024/1,024 | 0 |

Use settings A at 16² and B at 32² from the linked generator table. All observed intermediate pixels also repeat in the 32² Dirt 5 and Directional noise cases and the Dirt 1, Crystal and Alveolus cases. Dirt gradient has three varying intermediate pixels, each at (14,7) in a separate image. The U16 Blend spans 29,406–29,407, the U16 FX-Map spans 11,843–11,844, and the F32 Levels spans 0.3331910967826843–0.3332066237926483. The Dirt gradient final pixel repeats in all eight visits. Its other measured differences with MaterialCraft remain open.

### Square and rectangular pattern generators

Nineteen roots account for 194 new renders and 368,384 output pixels. Capturing the intermediate filters in the same renders places the first difference at an FX-Map in 18 of the 19 roots. Brick 2 matches all eight new outputs and every intermediate image. Its earlier U16, F16 and F32 differences are reproducible. The combined columns include the archived defaults, original changed controls and the earlier four-format and independent cases compared again with the current evaluator. All counts require equality across every stored component.

| Inventory index | Root | New renders | New exact pixels / compared | Combined renders | Combined exact pixels / compared | Max U8 / U16 levels | Max F16 / F32 error |
|---:|---|---:|---:|---:|---:|---:|---:|
| 19 | Alveolus | 15 | 31,608/32,000 | 41 | 332,280/334,336 | 0 / 1000 | 0 / 3.05175781e-05 |
| 57 | Black and white spots 1 | 15 | 25,692/32,000 | 34 | 157,008/204,288 | 1 / 5 | 0.00048828125 / 6.47306442e-05 |
| 58 | Black and white spots 3 | 8 | 10,283/13,568 | 27 | 139,099/185,856 | 1 / 3 | 0.00048828125 / 5.4359436e-05 |
| 59 | Brick 1 | 15 | 31,858/32,000 | 34 | 202,490/204,288 | 0 / 1 | 1.52587891e-05 / 1.52587891e-05 |
| 60 | Brick 2 | 8 | 13,568/13,568 | 27 | 185,560/185,856 | 0 / 1 | 6.10351562e-05 / 1.52662396e-05 |
| 61 | Brick 3 | 15 | 31,829/32,000 | 34 | 200,505/204,288 | 0 / 3 | 0 / 1.52587891e-05 |
| 67 | Cells 1 | 8 | 13,525/13,568 | 17 | 144,983/145,664 | 0 / 3 | 0.00048828125 / 3.67760658e-05 |
| 78 | Clouds 1 | 8 | 7,054/13,568 | 17 | 131,293/145,664 | 1 / 11 | 0.00048828125 / 0.000111401081 |
| 79 | Clouds 2 | 8 | 13,456/13,568 | 16 | 128,005/129,280 | 1 / 3 | 0.00048828125 / 2.29477882e-05 |
| 114 | Creased | 8 | 13,483/13,568 | 17 | 143,706/145,664 | 0 / 29 | 0.00048828125 / 1.53183937e-05 |
| 117 | Crystal 1 | 8 | 1,403/13,568 | 17 | 99,250/145,664 | 52 / 27622 | 0.206420898 / 0.361802135 |
| 118 | Crystal 2 | 8 | 1,996/13,568 | 17 | 104,017/145,664 | 14 / 12040 | 0.052734375 / 0.136126161 |
| 127 | Directional noise | 15 | 15,078/32,000 | 23 | 97,505/147,712 | 1 / 2 | 0.0009765625 / 2.94148922e-05 |
| 128 | Dirt 1 | 8 | 13,565/13,568 | 17 | 144,472/145,664 | 0 / 304 | 0 / 1.52587891e-05 |
| 129 | Dirt 3 | 8 | 13,535/13,568 | 17 | 145,297/145,664 | 0 / 1 | 1.52587891e-05 / 1.52736902e-05 |
| 130 | Dirt 5 | 8 | 13,558/13,568 | 16 | 129,218/129,280 | 0 / 4 | 0.000122070312 / 5.0842762e-05 |
| 131 | Dirt 6 | 8 | 9,039/13,568 | 16 | 117,822/129,280 | 0 / 8 | 0.133789062 / 0.140006408 |
| 132 | Dirt gradient | 8 | 0/13,568 | 16 | 538/129,280 | 115 / 50249 | 0.453125 / 0.877987966 |
| 353 | Pattern Alveolus | 15 | 31,436/32,000 | 33 | 201,876/203,264 | 0 / 2 | 0 / 1.52587891e-05 |

All 19 roots generate their own input. For square renders use defaults at 32² F32, setting A at 16² F32, B at 32² in U8/U16/F16/F32, then C at 64² F32 and U16. With the zero-based inventory index i in the table, use seed 8111+13i plus offsets 0, 1, 2, 2, 2, 2, 3 and 4 in that order.

| Roots | Parameters in tuple order | A | B | C |
|---|---|---|---|---|
| Both Alveolus | Tiling, filled cells, interstice width | (3,off,.0625) | (1,on,.21875) | (2,off,.375) |
| Both spots, Clouds 1, Dirt 1 and Dirt 3 | Disorder | .15625 | .46875 | .84375 |
| Brick 1 and 2 | Tiling, edge smoothness, interstice width | (3,.15625,.21875) | (1,.546875,.0625) | (2,.3125,.375) |
| Brick 3 | Tiling, roundness, luminance variation, luminance seed, disorder | (2,.125,.15625,7,.125) | (1,.34375,.6875,13,.3125) | (3,.03125,.3125,31,.5625) |
| Cells 1 | Distance, disorder | (3,12.5) | (11,37.5) | (17,68.75) |
| Creased and Crystal 1 | Warp intensity, disorder | (0,.15625) | (12.75,.46875) | (27.5,.84375) |
| Crystal 2 | Warp intensity | 0 | 12.75 | 27.5 |
| Clouds 2, directional noise, Dirt 5, Dirt 6 and Dirt gradient | Graph seed and dimensions | Defaults | Defaults | Defaults |

For Pattern Alveolus enable non-square expansion in A and C and disable it in B. Both Alveolus roots, black and white spots 1, Bricks 1 and 3, and directional noise also have seven rectangular renders. Use 64×32 F32 at defaults, 32×64 in all four formats at A, 128×32 U16 at B and 32×128 F32 at C. Use seed 8623+19i plus offsets 0, 1, 1, 1, 1, 5 and 6. Width precedes height. The output dimensions were checked against both applications.

The new stage captures compare 2,140,795/2,835,040 pixels across 1,452 images, including output bridges. The first discrepancy in the 16² directional noise Levels output occurs after a matching 8×16 U16 blend. The blend matches all 128 input pixels, while 59/256 Levels samples match with maximum F32 error 8.49366188e−6. In the 64² Dirt 1 render, all 4,096 U16 pattern pixels match, followed by a 16² U8 blur with 255/256 exact pixels and maximum error one level. In the 16² Dirt 5 case, all 256 F32 Levels samples match and the output bridge differs at one pixel by .0000508129597. Dirt gradient first differs in three separate pattern maps. [Previous spot and brick controls](#spot-and-brick-pattern-generators) · [Previous radial stamp controls](#radial-stamp-profiles-and-pattern-generators)

| Stage storage | Images | Exact pixels / compared | Maximum component error |
|---|---:|---:|---:|
| LUM16 | 1,148 | 1,777,298/2,364,864 | 57516 |
| LUM16F | 50 | 53,124/63,488 | 0.453125 |
| LUM32F | 188 | 250,004/341,504 | 0.877987966 |
| LUM8 | 66 | 60,369/65,184 | 115 |

### Horizontal stamp bounds at fractional heights

A 24-pixel-wide horizontal gradient on a 64² canvas gives one of two F32 horizontal slopes as only its height and vertical position vary. For each of 21 heights h from 14.75 to 17.25 in eighth-pixel steps, render at centers (32,c), where c=32+j/8 for j from 0 through 7. In these 168 original renders the two possible slopes separate exactly by the integer distance D=floor(c+1.5h+.5)−floor(c−.5h+.5). Where D is 29, 31, 32 or 35, the slope is the nearest F32 to 2/24. Where D is 30, 33 or 34, the slope is two F32 steps below that value. For h=16.25 the eight offsets give slope differences [0,−2,−2,−2,−2,0,0,0]. The calculation establishes the effect of independently rounding the two vertical endpoints. The general slope calculation for other widths remains unresolved.

Follow these renders with widths [12,20,24,40], heights [15,16,17,24,33] and vertical offsets [0,.5] in nested order, with horizontal center 32.5. All 208 controls use F32, zero rotation and seed 59771+23i, starting at i=0. The current release matches 798,890/851,968 raw pixels, maximum 2.38418579e−7. The [single-stamp slope measurements](#horizontal-stamp-slope-and-height) describe how to recover the candidate slope from the stored values.

### Horizontal stamp slope and height

An original single horizontal gradient stamp changes its F32 slope when only its height changes. For example, use size (24,16) in pixel units, center (64,64), zero rotation, zero roughness, opacity 1 and disabled tiling on a 128² canvas. Repeat with heights 33 and 63.25. Against the nearest F32 to 2/24, the only compatible F32 horizontal slopes are respectively 0, −2 and 0 representable steps away. Each nonempty row within each image has identical stored pixels. Moving a 48.125×32 stamp by .5 pixel horizontally also changes its slope by one representable step. These height and origin controls constrain the coordinate calculation for the [tiled radial stamps](#tiled-radial-stamp-coordinates).

To recover the slope, use every interior sample of the horizontal ramp. For each candidate F32 slope s near 2/width, bound the intercept b at every pixel center x by requiring the stored ramp to equal F32((trunc32(sx+b)+1)/2). The intersection of those bounds identifies the possible slopes. The 198 F32 renders below give **3,304,944/3,428,352 exact pixels** with the current renderer, maximum 2.38418579e−7.

For the first 55 renders, use sizes in pixels and offsets from the canvas center. For each width in [12,20,24,40,48], keep height 16 and visit sides 64, 128 and 256 in order. At side 128 render widths [12,24,40,48] for each (horizontal offset, height) in [(7,16),(.375,16),(0,33),(0,63.25),(11.25,48)]. At height 32 place the left edge of widths 24, 48 and 96 at x in [−2,0,1.125,3.5]. Finally, offset a 48.125×32 stamp horizontally by [0,.25,.5,.75,1,1.5,7.5,32]. Use seed 31237+19j in the order given, starting at j=0.

The next 143 renders use side 128 and seed 41249+17j. Center width 24 and set height to [4,5,6,7,8,9,10,11,12,13,14,15,15.125,15.75,15.984375,16,16.015625,16.25,16.875,17,18,19,20,21,22,23,24,25,31,32,33,34,39,40,47,48,63,64,80,96,121]. For each width/height pair [(24,16),(24,17),(24,16.25),(24,16.875),(24,32),(48.125,32),(32,16)], use offsets [(0,.25),(0,.5),(0,.75),(0,1),(.5,.5),(.125,−3.5)]. Finally center every width in [20,30,32,40,48,64] at each height in [8,16,17,24,31,32,33,40,48,64]. The general slope calculation remains open.

### Tiled radial stamp coordinates

An original graph draws a single Gaussian stamp with opacity .995 or a paraboloid with opacity 1. Use maximum blending, size (3,1.5), and center (.25,.25) or (.75,.75). At 32×64 and 64×128 F32, the per-pixel maximum of nine separate untiled renders matches the tiled render. Offset each copy by (dx,dy) with both components in {−1,0,1}. The same result holds for the Gaussian at 64×128 U16. Across the two shapes this gives 40,960/40,960 F32 and 16,384/16,384 U16 pixels, maximum error zero. Seeds are 83 and 84 at the smaller size and 223 and 224 at the larger one.

The following calculation also predicts those ten tiled images. In each axis of length N, let L=N(center+offset−size/2), D=N×size and B=max(0,L). K is the next F32 toward zero from F32(2/D). At pixel center p inside (L,L+D], evaluate the local coordinate as trunc32((p−B)×K+trunc32(−1+(B−L)×K)), where trunc32 rounds toward zero. Preserve full precision inside the two truncations. Apply the [radial profiles](#radial-stamp-profiles-and-pattern-generators) and take the maximum over the copies.

In 52 further renders, this coordinate calculation predicts only 104,640/219,136 F32 pixels and 218,428/219,136 U16 pixels. The respective maximum errors are 8.34465027e−7 and one level. Both shapes use width N/2 and height N, with seed 4729+17j for row j starting at zero. Render each row in F32 and U16. These independent cases keep the general coordinate calculation open.

| N | Center | Size |
|---:|---|---|
| 32 | (.25,.25) | (3,1.5) |
| 256 | (.75,.75) | (3,1.5) |
| 32 | (.3125,.6875) | (3,1.5) |
| 128 | (.1875,.8125) | (3,1.5) |
| 64 | (.5,.5) | (1,1) |
| 128 | (.5,.5) | (.75,.375) |
| 64 | (.375,.625) | (1.5,.75) |
| 64 | (.625,.3125) | (.6875,1.125) |
| 64 | (.5625,.4375) | (1.4,1.4) |
| 128 | (.50013,.49967) | (.71875,1.6875) |
| 128 | (.3125,.6875) | (2,.5) |
| 64 | (.1875,.3125) | (.618034,.953125) |
| 256 | (.5,.5) | (4,2) |

For all 26 Gaussian renders in this table, acquiring the same package and capturing its outputs took 352.37 seconds in a long-running reference session and 24.01 seconds in a fresh one. Both captures produced byte-identical outputs at all 219,136 pixels. A second check repeated 13 color renders and obtained byte-identical readbacks at 62,976/62,976 pixels across 39 images. Each run restored the prior graph settings.

### Fixed integer arguments and the UV debug marker

The UV debug graph now matches all 217,088 pixels in its original 32 renders. The [square-root measurements](#uniform-square-roots-and-the-uv-debug-marker) previously had 194,522 exact. Sixteen additional renders with signed channel offsets match all 10,240 pixels. Six independently wired parents add 56,828/56,832 exact pixels. Compute Irradiance and Compute Ray Direction each retain all 217,088 exact pixels across 32 renders.

| Debug hit UV storage | Renders | Exact RGBA pixels / compared | Maximum component error |
|---|---:|---:|---:|
| U8 | 13 | 120,832/120,832 | 0 |
| U16 | 9 | 18,429/18,432 | 1 level |
| F16 | 9 | 18,432/18,432 | 0 |
| F32 | 59 | 126,463/126,464 | .750555232 |

For the 16 direct renders, use field channel c=((37x+41y+7xy+53×19+61c+5xc) mod 509)/508 as F32, with c from 0 through 3. Set the debug position to (.1875,.8125), sample index to 0 and count to 1. Sweep integer offsets −2 through 5 at 16² and 32² F32. Use seed 6481+17j+7t for offset index j and size index t, starting at zero.

For the wired parents, drive the index and count through value processors. Use the following settings in row order. Within each row render the same field at 16² F32, 32² in U8/U16/F16/F32, 64² F32 with a 64×32 field, and 32² F32 with field values scaled by 2.5 and shifted by −.75. Use seed 4591+17j+13t for row j and render t.

| Index | Count | Offset | Debug position |
|---:|---:|---:|---|
| 0 | 1 | −1 | (.1875,.8125) |
| 0 | 4 | −1 | (.671875,.328125) |
| 2 | 8 | −1 | (.25,.625) |
| 1 | 8 | 0 | (.421875,.765625) |
| 3 | 4 | 1 | (.71875,.15625) |
| 0 | 1 | 0 | (.375,.6875) |

At (13,24) in the fourth row's 32² F32 output, the reference stores gray .1476394385. MaterialCraft also covers that pixel with the marker color (.43350938,.89819467,0,1). An isolated rectangle centered at (.42769217,.83190984), size (.13307922,.01), rotated .23604888 turns reproduces the extra pixel. In the final row at 32² U16 the reference stores red 15,899 at (9,19), (10,20) and (11,21). MaterialCraft stores 15,898. Both apps store 15,898 when an independent rectangle receives the literal red value .24259558. Integer storage also agrees on all 14,336 output pixels from 56 solid rectangle and uniform-color controls around half-level thresholds.

Four independent color and grayscale rectangle series compare coverage at 16², 32², 64² and 128², using the two marker geometries above and offsets of up to a quarter pixel. A forward-transformed quadrilateral reproduces all 197,632 coverage decisions in 88 renders. In 80 additional edge-focused renders it matches 137,175/137,216. Snapping its screen vertices to the nearest 1/256 pixel gives 137,166/137,216. MaterialCraft's inverse-transform test gives 137,185/137,216. The maximum binary coverage error for each method on this second set is one.

For the second set, position one edge of a rotated rectangle at an interior pixel center. Vary its length in [.1,.57], its signed width among −.03125, .009765625, .09765625 and .25, and its rotation among −.38200337, −1/6, .03125, .09375, .125, .21249999, .23604888, .3125 and .4782 turns. Shift each center axis by up to one F32 spacing. The 80 cases include both agreement and disagreement between the analytical coverage tests. They preserve a distinction that the first two geometries alone would miss.

A folded scalar integer is ordered and converted to F32 using its unsigned 32-bit value. A runtime scalar integer has signed ordering and conversion. For example a folded −5 becomes 4,294,967,296 in F32. With a runtime input of −5 it becomes −5. Compare −32,768 with 2, −5 with −2 in both orders, −1 with 0 in both orders, equal operands, opposite signs and −2³¹ with 1,024. Test less, less-or-equal, greater and greater-or-equal with two literals, two runtime inputs, either operand fixed, and an equivalent literal subtraction. Both scalar integer encodings and the converted values are covered. All 84/84 returned F32 tuples match. Integer vectors of two, three and four lanes preserve signed conversion in all 12/12 additional typed controls.

For the scope controls pack the four comparison flags into bits 0 through 3 and use bit 4 for float(left)>1,000,000. Divide by 32. With (−5,2), both fixed yield .875, a fixed left operand and runtime right operand yield .59375, and a runtime left operand with fixed right yields .09375. Use a property function, pixel processor, pattern function and value processor at 16² F32 and 32² in four formats with a positive and a negative argument. All 39,168 image pixels and 15 typed values match.

An enclosing instance passes the fixed status of its scalar integer arguments through nested graphs and reusable functions. A value wire or exposed runtime control keeps its runtime status. Four original current-format packages with fixed, directly bound, computed and nested fixed inputs were saved by MaterialCraft and reopened in the reference app. All 20/20 typed F32 tuples match before and after saving and across applications, with error zero.

Self-assignment remains open. Two separate programs assign an integer to the same identifier they read in the assignment, then read it again. With distinct graph identifiers and instruction IDs, 40/60 typed tuples match across those programs and four controls. The maximum F32 error in that capture was 2.65040882e25. Repeating the five identical seeds and inputs in the runtime self-assignment reproduced only one reference tuple. Assigning from a different input and assigning a constant to a local variable reproduced all 20/20 tuples, with error zero.

### Uniform square roots and the UV debug marker

The UV debug marker now cooks all **32** previously measured cases. The 27 cases that already cooked still match every pixel. Across all 32 renders **194,522/217,088 RGBA pixels** match. Maximum errors are 253 U8 levels, 0 U16 levels, 0 F16 and 1 F32. The five formerly rejected renders contribute 218/22,784 exact pixels. At 16² with index 0, count 1 and offset −1, the first output pixel has gray level .360237986 in the reference app and zero in MaterialCraft. At index 1, count 4 and offset 0, both store zero there. The input's first RGB and alpha components differ at this position. Both sibling roots, Compute Irradiance and Compute Ray Direction, retain 217,088/217,088 pixels each across 32 renders each, with error zero. [Wired sample inputs and seeds](#environment-rotation-and-wired-irradiance-samples)

A uniform square root returns positive zero for negative values, either sign of zero, positive subnormals and NaN. Positive normal values use the ordinary F32 square root. For example, sqrt(1.375) has bits `3f9617e3` in a uniform function and `3f9617e2` in a runtime pixel expression. Positive infinity propagates in the comparisons. A literal square root preserves negative zero and accepts a positive subnormal. These distinctions apply before the output is stored. [Runtime pixel square roots](#runtime-pixel-square-roots)

| Original controls | Renders | Exact stored pixels | Exact typed floats | Maximum absolute error |
|---|---:|---:|---:|---:|
| Runtime roots and equality predicates | 136 | 26,112/26,112 | 34/34 | 0 |
| Quotients by zero, literal predicates and positive controls | 52 | 9,984/9,984 | 13/13 | 0 |
| Raw literal roots | 28 | 5,120/5,376 | 7/7 | 0 |
| Pattern dimensions, offsets, rotation and color | 35 | 43,520/43,520 | — | 0 |

For the first three rows evaluate the expression independently in a node parameter, pattern, Pixel Processor and Value Processor. The three images are 16² LUM32F. For the first row read the raw square root r and the separate expression [r=0]+2[r=r]. Use −.25, −1, −.00003125, −0, +0, the least positive subnormal, the least positive normal, .0625, .31875, .81234568, 1, 1.375, 2, 3, 7.03125, 1337.125 and 2⁻⁹⁸. Start at seed 4319 and add 11 per entry.

For the quotient controls compute d=a/0 for a=0,−1,1 and output [sqrt(d)=0]+2[sqrt(d)=sqrt(d)]+4[sqrt(d)=d]. The three uniform results are 3, 3 and 6. For the literal predicate pack four equalities into bits 0 through 3. Compare the roots of F32 bits bf800000, 80000000 and 00000001 to zero, and the root of 3f801000 to 3f800800. The other positive and subnormal controls take F32 bits 3f801000, 40300000, 3fe20000, 007fffff, 00800001, 7f000010, 00800000, 3fb3d23a and 3f8081cd. Each series starts at seed 4457 and advances by 13. For the raw literals use −.25, −0, 00000001, 007fffff, 3f801000, 0/0 and −1/0, selected by a runtime index with seeds 4549+13j. The typed results have bits 00000000, 80000000, 1a3504f3, 1fffffff, 3f800800, 00000000 and 00000000. Only the grayscale maximum-blend square with literal −0 differs in the sign of zero at all 256 pixels. The reference image stores +0. The numeric maximum error is zero and the table counts bitwise matches.

For the last row draw a parent square with size (.375,.5), color (.75,.25,.5,1), transparent black background and no tiling. Draw a second square on its upper-left child with local size (.25,.25), color (.25,.75,.125,1), maximum blend and zero roughness. Bind sqrt(t) to the parent's width with height .3125, the X pattern offset, the rotation, the X branch offset or the red channel, one graph per property. Keep Y offsets zero. Render t=.0625 and −.25 at 16² F32, then −.25 at 32² in all four formats, then .25 at 64² F32. Use seeds 4271+13j within each series. Before the square-root fix, the 25 negative-input cases reported a nonfinite function result. Both stamps now evaluate without an error. At 16² a negative input to width leaves the four child pixels visible.

### Paths, caustics and cell fields after pattern sampling

| Root | Renders | Exact pixels / compared | Maximum U8 / U16 levels | Maximum F16 / F32 error |
|---|---:|---:|---:|---:|
| Cells 4 | 29 | 215,500/215,552 | 0 / 53,506 | 0 / 0.491874591 |
| Cells 4, second package | 29 | 215,516/215,552 | 223 / 20,688 | 0.315917969 / 0.667963699 |
| Paths Trace | 19 | 58,213/58,624 | 120 / 30,822 | 0.756835938 / 1.22619528 |
| RT Caustics Base | 19 | 152,777/172,288 | 0 / 1 | 0.000122070312 / 6.67572021e-06 |

The ten new path and caustic renders use the following (output side, input height, seed, format, setting). Use (16,16,4069,F32,default), (32,32,4093,F32,A), four at (64,64,4127,B) in U8/U16/F16/F32 order, (128,128,4159,F32,C), (32,16,4177,F32,A), (32,32,4201,F32,D) and (64,64,4229,F32,B). The four archived controls and five inventory cases are also included. An independent input repeated for stage inspection counts once. [Four-format inventory comparison](#four-format-inventory-comparison)

For Paths Trace use the transformed five-point path for default, A and D and the seven-point path for B and C. The background stores ((37x+41y+7xy+583+61c+5xc) mod 509)/508 in channel c, starting at zero. In settings A/B/C/D enable corner tracing for A and B, vertex tracing for B and C, and set thickness to .75/1.625/2.75/.25. All four instrumented stages match on default, both A renders and D. The six remaining renders first differ in the FX-Map that draws the path. At 64² in U8 the output differs at 18 pixels, including (0,0), (1,0) and (63,0). [Path buffer coordinates](#marked-variable-reads-in-path-and-mipmap-graphs)

For RT Caustics Base supply height and noise H=((37x+41y+7xy+53j) mod 509)/508 at j=11 and 13. Store the normal field at j=12, using the normal and alpha construction below. For A/B/C/D use photon grid size 1/1/2/0, height scale .0625/.3125/.625/0, height position .125/.75/.375/.5, index of refraction 1.125/1.375/1.625/1, photon size 2/5.5/7.25/3 and jitter 0/.3125/.75/0. Place the light at (.3125,.6875) for A and B and (.75,.25) for C and D. The first differing stage is the photon FX-Map. At 64² in U16 it differs at (1,7) by one level and the final image matches. Across both matrices all 122,880 U8 output pixels match. U16 matches 8,185/8,192, F16 matches 8,181/8,192 and F32 matches 13,531/33,024. [Procedural normal fields](#normal-maps-panoramas-and-primitive-shapes)

For the eight extra Cells 4 renders use the gray band value ((floor(x/3)+2floor(y/5)) mod 5)/4. Run defaults at 16²/71, four output formats at 64²/193, 128² F32/317, a 64×32 input at 64² F32/449 and a 64² F32 repeat at seed 503. The three changed settings use (scale, disorder, color source, pseudo-random seed) of (9,.21875,1,17), (17,.75,2,255) and (6,.4375,1,41) for the 64², 128² and rectangular inputs. Set disorder speed to .75/1.625/.3125 where exposed and disable non-square expansion for the rectangular input. The first package matches 41,216/41,216 band pixels. The second matches 41,206/41,216. Their earlier default and modular images still have differences. [Cell-field inputs and differing scatter stages](#tile-random-cell-fields-and-noise-upscaling)

The debug hit UV root matches **194,304/194,304 RGBA pixels in 27 successful renders**, maximum error zero in each format. Five further renders with index 0, count 1 and index offset −1 produce a local nonfinite function error. The reference renders all five. Their 22,784 pixels are excluded from the successful-render total. [Wired sample inputs and seeds](#environment-rotation-and-wired-irradiance-samples)

The position debug root's seven archived equal-resolution cases match all 136,192 pixels and the matched 128² preview adds 256/256 pixels at its 16² output. The spline preprocess root retains six exact integer counts and four 2048×16 renders with 131,072/131,072 exact RGBA pixels, maximum error zero. The zero-count reference timeout still has no comparable image. [Position debug preview](#tinted-stamp-offsets-and-alpha) · [Spline preprocessing with wired counts](#spline-preprocessing-with-wired-counts)

### Integer and oblique directional blur

| Root | Renders | Previous exact pixels | Exact / compared | Maximum error |
|---|---:|---:|---:|---:|
| `anisotropic_blur.sbs/anisotropic_blur` | 29 | 145,109 | 200,192/200,192 | 0 |
| `anisotropic_blur.sbs/anisotropic_blur_grayscale` | 29 | 165,764 | 200,192/200,192 | 0 |
| `blur_hq.sbs/blur_hq` | 18 | 65,482 | 171,264/171,264 | 0 |
| `blur_hq.sbs/blur_hq_grayscale` | 18 | 128,880 | 171,264/171,264 | 0 |

On the 128² anisotropic color and grayscale inputs at angle .0625, intensity 17.5, anisotropy .625 and high quality, the first directional pass operates at intensity 10.5. Look at the raw U8 output of that pass. Before the change, 191/16,384 color and 2,125/16,384 grayscale pixels matched. On both inputs all 16,384 now match. A sheared sampling grid crosses the image boundary at a different position on the other axis. Wrap each tap at its own stored mip level and bilinearly sample the transverse shift. Recomputing the padded edge from the original bitmap still misses 675 color and 290 grayscale pixels, with a maximum error of two byte levels.

An integer diagonal blur stores the first shear and each reduction at the destination depth. In the window normalization, adding half of 1/256 before the U8 store reproduces the measured byte controls. The measured U16 offset is approximately .4 of one stored level. To isolate the rounding, generate a 32² image whose rows divisible by eight contain 1/255, with all other rows zero. Set angle .125 and both axes to repeat. At intensity 19 the U8 output at x=0 for the first eight rows is `0,0,1,0,0,0,1,1`. At intensity 48 the U16 levels are `30,30,30,30,52,30,30,30`. For the edge test use the 128² signed [impulse-and-stripes field](#tiled-diagonal-directional-blur), angle .0625 and intensity 10.75. Pixel (0,65) is RGBA8 (0,0,57,116), (127,52) is (0,0,0,88), and (64,64) is (17,26,61,99).

All four roots include the [archive and four-format comparisons](#tiled-diagonal-directional-blur), the independent 32² input and nine further fields. For those nine use `H=((37x+41y+7xy+61c+5xc) mod 509)/508` for color channel c. Use c=0 for grayscale RGB with opaque alpha. Render defaults at 16²/43. At 64²/157 render all four formats at intensity 5.25 with high quality. At 128²/293 use intensity 18.5 with high quality off. At 32²/463 use signed input `2.5H−.75` and intensity 10.25 with high quality. Apply the signed transformation to every channel. Repeat the 128² settings at 32²/599 with a 32×16 input and the 64² settings at seed 751. On the anisotropic roots the three changed intensity settings use anisotropy/angle .6875/.0625, .3125/.15625 and 1/−.21875.

The anisotropic roots include twelve additional fields. Use `H=((73x+37y+11xy+157+43c+3cx) mod 1021)/1020`, with c=0 for the grayscale image. Columns for each case below give square side, seed, intensity, anisotropy, angle in turns, quality and output. Apply `2.5H−.875` to both signed inputs.

| Side | Seed | Intensity | Anisotropy | Angle | High quality | Output |
|---:|---:|---:|---:|---:|---|---|
| 16 | 2833 | 10 | .5 | 0 | Off | F32 |
| 32 | 2953 | 10 | .5 | 0 | Off | U8 |
| 64 | 3001 | 24 | .8125 | .125 | Off | U8, U16, F16, F32 |
| 128 | 3049 | 17.5 | .625 | .0625 | On | F32, signed |
| 32 | 3119 | 5.375 | .375 | −.1875 | Off | F32, signed |
| 32 | 3203 | 0 | 1 | .03125 | On | F32 |
| 64 | 3251 | 27 | 0 | −.375 | On | F32 |
| 64 | 3271 | 24 | .8125 | .125 | Off | F32 |
| 32 | 3313 | 13 | .75 | .1875 | On | F32 |

### Direct blur controls and remaining differences

| Procedural control | Renders | Previous exact pixels | Current exact / compared | Maximum U8 / U16 / F16 / F32 error |
|---|---:|---:|---:|---|
| Tiled eighth turns | 96 | 1,046,248 | 1,158,974/1,351,680 | 0 / 1 / 0.00048828125 / 3.57627869e-07 |
| Tiled .0625 and .21875 turns | 24 | 2,625 | 280,184/337,920 | 1 / 74 / 0.00146484375 / 0.00263102353 |
| Untiled .125 turns | 12 | 5,523 | 5,523/168,960 | 192 / 49,199 / 1.09570312 / 1.09612465 |
| Independent stripes and projected axial controls | 64 | 134,690 | 160,571/163,840 | 0 / 0 / 3.81469727e-06 / 5.96046448e-08 |
| Code patches, low pulses and translated row fields | 80 | 1,200,293 | 1,454,692/1,495,040 | 0 / 1 / — / 1.78813934e-07 |

The [impulse and stripes controls](#tiled-diagonal-directional-blur) specify the input, strength and seed for the first three rows. In the 96 tiled eighth-turn renders, all 163,840 RGBA8 pixels match. Of 163,840 RGBA16 pixels, 90 still differ by one level. Off-axis filtering has further differences at .21875 turns. Untiled boundaries and non-square processing grids still use the direct sampling path.

For the independent stripes use U8 inputs at (side, radius) (32,2.375), (64,3.125), (64,6) and (32,5.375). Set R=(53x+29x²+19) mod 256, G=(31y+7y²+17) mod 256, B to 208 or 23 according to the parity of x+y, and A to 127. The second field uses 1 or 2 by x parity, 9 or 2 by y parity, (x+3y) mod 17 and 127+(x(y+1) mod 3). Set intensity to 256×radius/side at .125 turns. For the first field also render horizontal and vertical controls with radius .5+(radius−.5)/√2, across all four formats at seeds 3359 through 3362.

For the code patches, assign k=16⌊y/16⌋+⌊x/16⌋ on a 256² field and store channels (k,k+1,k+67,k+129) modulo 256. The other inputs are 64² row fields or a 32² one-level pulse across four base levels. One row field uses `((31y+7y²+17) mod 128)+32+c`. The other uses `U=((117y+53y²+17) mod 251)`, with channels U, (U+12) mod 256 and the transposes of both. In the pulse field add one to (0,4,45,184) every eighth row. For the U16 inputs, scale each code by 257 and add 29c below code 255. Use the same four radii at .125 turns and seeds 3433–3436 in field order. Store each U8 source in U8, U16 and F32, and each U16 source in U16 and F32. All 299,008 U8 output pixels in this set match.

The 38 instrumented color and grayscale anisotropic renders compare 304 graph images, including the input and output bridges. 2,560,000/2,560,000 pixels match, maximum stored-component error 0. Six other roots with a reachable directional blur retain all 790,528 exact pixels in 52 renders.

Four additional large and rectangular-source anisotropic stage captures match 278,528/278,528 pixels in 32 images. Reloading the package for each color or grayscale pair took 506.09 seconds in total. Repeating the same four captures with the open package took 4.14 seconds. All raw readbacks were byte-identical. The graph size, seed, numeric controls, source bindings and output format were restored between captures.

The high-quality color and grayscale blur graphs each have 65 observed images across their 128² changed input and four archived controls. The final outputs match 131,072/131,072 pixels in each graph. Across all observed images, color matches 1,703,770/1,703,936 and grayscale 1,703,876/1,703,936. At the default 256² input, four directional passes and one intermediate blend differ by at most one U8 level in each graph.

### Bitmap stamp filtering and compositing

A quadrant with an image pattern uses automatic bilinear mip filtering when `imagefiltering` is omitted or zero. Mode 1 selects the original level with bilinear filtering. Mode 2 selects the nearest texel. Filter taps outside the bitmap use transparent black. Intermediate reductions store each level at the source precision, which matters even when the final output is F32. A grayscale stamp needs a grayscale image and a color stamp needs a color image. Wiring the opposite kind leaves the background intact.

For an original 64² image, set the byte at `(x,y)` to `(47x + 31y + 7xy + 13) mod 256`. For the signed F32 input, divide that field by 255, multiply by 1.75 and subtract 0.375. Keep the input at 64² and render the quadrant at 32², 64² and 128². A centered stamp of size (0.375, 0.375) produces F32 bits `3f4ea0cf`, `3f5e5ade` and `3f62e2e3` at pixel (32,32) of the 64² output for modes 0, 1 and 2. To distinguish horizontal and diagonal mip selection, use size (0.17, 1), offset (0.07125, −0.0390625) and rotations of 0.0625 and 0.125 turns. The selected levels are 2 and 3 at 64². Bilinear modes use the measured four-tap weights in [stored texture filtering](#multi-directional-warp-and-intermediate-storage).

| Original procedural control | Renders | Previous exact pixels | Exact / compared | Maximum U8 / U16 / F16 / F32 error |
|---|---:|---:|---:|---|
| Hash, ramp, checker and flat stamps from 16² to 128² | 150 | 496,615 | 683,609/695,040 | 221 / 56,797 / 0.866622925 / 1 |
| Fixed 64² byte and signed F32 inputs, eight stamp transforms | 192 | 902,752 | 1,221,604/1,228,800 | — / 65,535 / — / 1.66078436 |
| Twelve anisotropic transforms, three modes and the omitted setting | 144 | 752,204 | 1,029,747/1,032,192 | — / — / — / 0.996078432 |
| Tint, opacity, three blend modes and premultiplication | 90 | 49,150 | 92,132/92,160 | 0 / 0 / 0 / 5.96046448e-08 |
| Matching and crossed grayscale and color sockets | 60 | 29,249 | 60,864/61,440 | 0 / 1 / 0 / 5.96046448e-08 |
| Four image pins with fixed selectors | 30 | 5,632 | 7,680/7,680 | — / — / — / 0 |

The automatic mip estimate selected the observed level in 81 angle and scale combinations, 128 additional independently chosen transforms, and 24 rectangular cases with sides of 32, 64 and 128. The corresponding F32 bit comparisons give 331,582/331,776 (maximum 0.00355535746), 523,951/524,288 (maximum 0.00379943848), 73,702/73,728 (maximum 0.000717163086) respectively. Oblique sample positions still have pixel-level differences. To isolate the nearest-filter error, replace the stamp with a horizontal gradation, then rotate it by a quarter turn and swap its dimensions to measure the other coordinate. Sampling the original bitmap with the two measured coordinates and a zero border reproduces **408,576/408,576 F32 pixels** in 57 renders of 19 geometries, maximum error zero.

For the tint tests, the 32² input channel `c` is `1.5 × (((47x + 31y + 7xy + 13 + 69c + 3xc) mod 256) / 255) − 0.25`. Use opacity 0.375, tint (0.75, 0.375, 1, 0.625) and background (0.25, 0.375, 0.625, 0.8125). Grayscale applies the mean RGB tint. Color mode 0 adds the premultiplied RGB contribution and accumulates alpha. Modes 1 and 2 composite over the background. Signed alpha and premultiplied input are covered. The 28 F32 alpha differences in this measurement were each 0.0000000596046448. The [signed blend controls](#signed-bitmap-blend-precision) subsequently close all 28. The fixed four-pin renders now match the raw F32 samples as well as the slot selections.

| Root | Renders | Exact / compared | Maximum U8 / U16 / F16 / F32 error |
|---|---:|---:|---|
| Make It Tile Patch | 9 | 98,161/132,096 | 2 / 318 / 0.00476074219 / 0.00485236943 |
| Make It Tile Patch Grayscale | 9 | 117,553/132,096 | 2 / 150 / 0.00228881836 / 0.00228885561 |
| Arc Pavement | 27 | 206,904/211,200 | 0 / 0 / 0 / 9.87946987e-06 |

For both patch roots, retain the [four-format parameter matrix](#four-format-inventory-comparison), the independent 32² render and the four original 128²/256² controls. The changed case uses mask size 0.8, precision 0.7, warping 0.1, pattern width/height 120/160, disorder 0.25, size variation 0.15, two octaves and rotation 0.125. Arc Pavement previously matched 202,429/211,200 pixels in the [same cases](#uniform-arc-tangents-and-pavement-rotations). All 123,904 U16 pixels, 8,192 U8 pixels and 8,192 F16 pixels now match. The soft, bell, pyramid and capsule patterns at seed 547 and three image choices at seed 607 account for the 4,296 remaining F32 differences.

Both patch graphs first differ in a fixed 32² U16 mask. One Gaussian stamp, size (1.4, 1.4) and opacity 1.00343001, matches 1,020/1,024 pixels. The four differences are one level each at (24,7), (11,11), (7,24) and (24,24). The color graph also has an independent U8 directional blur difference upstream. At the default 128² graph size, this blur matches 4,087/4,096 pixels, maximum one level.

Six independently authored files with format header `1.1.0.202502` passed through ordinary `file.saveAs` and reopened in the reference app. Four files covered omitted and explicit filtering, and two covered tinted color with and without premultiplication. Comparing original and saved output in that app gives **96,256/96,256 exact pixels in 22 renders**, maximum zero. See the [earlier file handoff](#image-selection-in-saved-pattern-graphs) for the older-header restriction.

The two Alveolus roots use procedural Gaussian stamps. In each root, all eight stage comparisons place the first differing pixel in the FX-Map. On the default 128² and 256² renders its half-width U16 output matches 8,096/8,192 and 32,548/32,768 pixels respectively, maximum one stored level. Isolate the two stamps at (0.25, 0.25) and (0.75, 0.75), local size (3, 1.5), opacity 0.995 and maximum blending. Independently storing the reference F32 result as U16 reproduces the observed U16 values. The coordinate calculation remains open for both roots.

### Signed bitmap blend precision

The bitmap stamp accumulator retains three guard bits below the aligned single-precision significands. A weighted component and its addend each keep a low bit when discarded digits are nonzero. When cancellation removes two or more leading bits, the accumulator drops the low guard bits before storing the result. In additive mode a zero source contribution preserves the background's zero sign. In the over modes a transparent source scales the background by its remaining coverage.

| Procedural inputs | Renders | Previous exact pixels | Exact / compared | Maximum error |
|---|---:|---:|---:|---:|
| Constant tint and background, grayscale and color | 90 | 92,132 | 92,160/92,160 | 0 |
| Signed color over a varying background | 96 | 94,568 | 98,304/98,304 | 0 |
| Independent exponent ramps and near cancellation | 54 | 66,248 | 96,768/96,768 | 0 |
| Positive and negative zero with opposite signs | 6 | 5,532 | 6,144/6,144 | 0 |

The [90 tint and opacity controls](#bitmap-stamp-filtering-and-compositing) include U8, U16, F16 and F32 output. For the 96 two-image controls use the same 32² signed RGBA stamp. Set background channel c to `3.55 × fract(((53x + 29y + 11xy + 37) mod 257)/256 + .117c + .005xc) − 1.35`. Cross all three blend modes and both premultiplication settings with four output formats. Use opacity/tint pairs .427/(.487,.683,.237,.563), .375/(.75,.375,1,.625), .923/(1.375,.125,−.125,1.1875) and 1.333/(−.375,1.875,.439,.271). For pair j and blend b use seed 1787+13j+b.

For the 54 independent F32 renders use sides 16, 64 and 32, and seeds 2683+17i+7j+b for size index i, tint index j and blend b. Before cancellation, source channel c is `(((31x+83y+23xy+47c+13) mod 1021)/509−1) × 2^(((3x+7y+11c) mod 19)−9)`. The background is `(((73x+41y+17xy+31c+67) mod 1019)/503−1) × 2^(((5x+7y+3c) mod 23)−11)`. Use opacity/tint pairs .63125/(.4375,−.8125,.733125,.93375), .14875/(1.21325,−.796875,.34375,.713125), and 1/(1,1,1,1). At 32² set source alpha to `1.5 × (((7x+59y+13) mod 509)/508) − .3125`. With F32 arithmetic, place the background at `−V/(1−A+1e−25) × (1+2^−12)` where `V` is the premultiplied, tinted source contribution, `A` is its coverage and `abs(1−A)>1e−5`. At smaller denominators use the original background RGB and alpha .5. The six separate signed-zero renders use a 32² repeating table (+0,−0,1,−1,.5,−.5,2,−2), all three blends, both premultiplication settings and seeds 2729+b. Equality for floating outputs includes the sign bit.

A single nearest-filtered premultiplied texel at full opacity reproduces the former rounding error. Use F32 source bits `baf97631 c061d2bc bc4a2f47 be8c58b1` and background `3ac3d70f 4031485b 3c1eb9a8 3e5c5bb0`. The four over-mode output words are `34f98400 3a61d400 364a3400 388c5c00`. The additive alpha word is `388c6000`.

Replaying 206 renders across 18 affected roots gives 2,131,360/2,246,400 exact pixels. The pattern-package [Tile Random Color matrix](#tile-random-image-and-soft-patterns) gains one F32 pixel and now matches **268,904/276,736** in 40 renders. Its maximum component errors are 163 U8, 59,666 U16, .0478515625 F16 and .90265286 F32. The image-rotation and texel-boundary differences remain. `brick_generator.sbs/resources/Tile`, `histogram_tools.sbs/histogram_compute` and `pattern_brick_generator.sbs/resources/Tile` retain 233,984/233,984 exact pixels in 40 renders combined.

### Tile Random image and soft patterns

The pattern-package Tile Random and Tile Random Color render the background by itself when the pattern selector is zero. At 64²/827 all 4,096 color output pixels match the original background. Use selector 1 for an image stamp. At 64²/2273, switching from filter 0 to 1 or from 1 to 2 changes 1,187 reference pixels in both roots. Both background and image pins contain procedural fields.

| Root | Renders | Exact / compared | Maximum U8 / U16 / F16 / F32 error |
|---|---:|---:|---|
| Tile Random | 32 | 238,215/243,968 | 165 / 60,395 / 0.0498046875 / 1 |
| Tile Random Color | 40 | 268,904/276,736 | 163 / 59,666 / 0.0478515625 / 0.90265286 |

The original 20 grayscale renders still match 176,384/176,384 pixels. The color root matches 209,152/209,152 across the same settings, eight additional selector-zero controls, and its archive and inventory cases together. The twelve additional renders per root expose the outstanding differences.

For grayscale use H=((73x+37y+11xy+k) mod 1021)/1020, with k=29 on the pattern pin and 72 on the background. For RGBA add 43c+3cx inside the modulus, with c=0,1,2,3, k=29 on the background and 72 on the pattern. Use (4,5) tiles, pattern-specific value .3125, rotation randomness .375, shape rotation .125, interstice .1875 and split threshold .625. Render all three image filters at 64²/2273 F32, plus nearest at 128²/2311 U16, bilinear at 64²/2333 U8, and automatic mips at 32²/2357 F16. Render at 64²/2371 with filter 1 and blend mode 1, and at 32²/2393 with filter 2, blend mode 1 and signed inputs 2.5H−.875. For the two color cases use tint (.75,.375,1,.625), tint randomness (.25,.125,.375,.25) and background (.25,.375,.625,.8125). At 64²/2203 use bell (5), Gaussian (6) and pyramid (8). At 128²/2221 use brick (9). Maximum F32 errors for the three soft patterns are 2.14576721e−6 across the two roots. The U16 brick cases differ by one level at 70 grayscale and 216 color pixels.

A coordinate-coded color stamp isolates the effect of rotation on image sampling. Store R=x+1, G=y+1, B=128 and A=255 in a 64² U8 image and wire transparent black to the background. At 64²/2273 with the settings above and nearest filtering, both cooks draw the same 1,187 pixels. Of those, 594 sample a different quarter-turn of the image. Removing only the random pattern rotation gives 4,096/4,096 exact F32 pixels, maximum zero. With zero interstice and zero size randomness, all four fixed pattern rotations match for (1,1), (1,2) and (2,1) tiles. With (4,5) tiles and fixed rotation zero, 1,560 pixels differ by one source texel horizontally. Across the 30 rotation controls the result is 108,653/122,880 exact F32 pixels, maximum component error 0.247058838.

### Connected bitmap pattern roots

Repeat the four original root settings and the [five inventory cases](#four-format-inventory-comparison) after the [bitmap filtering change](#bitmap-stamp-filtering-and-compositing). The table counts complete pixels across all output channels and gives maximum stored-component errors. A dash marks formats with no completed comparisons. The three dirt roots exceeded the pattern execution budget in all five additional cases. Their counts cover the four completed original renders. Arc Pavement, the two patch roots and the two pattern-package Tile Random roots have their wider results above.

| Package / graph | Completed renders | Exact / compared | Maximum U8 / U16 / F16 / F32 error |
|---|---:|---:|---|
| `noise_dirt_1.sbs/dirt_1` | 4 | 94,914/98,304 | — / 538 / — / — |
| `noise_dirt_1_v2.sbs/dirt_1` | 4 | 98,263/98,304 | 1 / — / — / — |
| `noise_dirt_2.sbs/dirt_2` | 4 | 97,737/98,304 | — / 21 / — / — |
| `paths_tools.sbs/paths_trace` | 9 | 18,297/18,432 | 120 / 30,822 / 0.62109375 / 1.22619528 |
| `pattern_splatter_circular.sbs/splatter_circular` | 9 | 130,917/132,096 | 223 / 57,311 / 0.874511719 / 0.921568625 |
| `pattern_splatter_circular.sbs/splatter_circular_color` | 9 | 130,678/132,096 | 254 / 65,535 / 0.996078491 / 0.996078431 |
| `pattern_tile_generator.sbs/tile_generator` | 9 | 130,546/132,096 | 0 / 1 / 0 / 0 |
| `pattern_tile_generator.sbs/tile_generator_color` | 9 | 128,338/132,096 | 0 / 1 / 0 / 0 |
| `pattern_weave_2.sbs/weave_2` | 9 | 130,222/132,096 | 1 / 59 / 0.0009765625 / 0.000900268555 |
| `radial_blur.sbs/radial_blur_grayscale` | 9 | 109,921/132,096 | 1 / 14 / 0.00048828125 / 0.000137329102 |
| `splatter.sbs/splatter` | 9 | 132,084/132,096 | 0 / 91 / 0 / 1.52587891e-05 |
| `splatter_circular.sbs/splatter_circular` | 9 | 130,443/132,096 | 222 / 57,311 / 1 / 1.00000006 |
| `splatter_circular.sbs/splatter_circular_color` | 9 | 130,794/132,096 | 247 / 65,535 / 0.96862793 / 0.996078431 |
| `splatter_color.sbs/splatter_color` | 9 | 132,066/132,096 | 0 / 59 / 0.000122070312 / 0.000137329102 |
| `tile_generator.sbs/tile_generator` | 9 | 129,318/132,096 | 0 / 1 / 0.00048828125 / 4.88758087e-06 |
| `tile_generator_color.sbs/tile_generator_color` | 9 | 127,043/132,096 | 0 / 1 / 0.0009765625 / 4.88758087e-06 |
| `tile_random.sbs/tile_random` | 9 | 105,004/132,096 | 223 / 57,311 / 1 / 1.00000006 |
| `tile_random.sbs/tile_random_color` | 9 | 105,004/132,096 | 250 / 65,535 / 1 / 1.00000006 |

### Multi-directional warp and intermediate storage

The color and grayscale multi-directional warps each match **175,360/175,360 pixels** across the 19 [root cases](#directional-warp-orientation-and-input-precision), maximum error zero. Each root contributes 122,880 U8, 8,192 U16, 8,192 F16 and 36,096 F32 pixels. The two non-uniform blur sample roots retain the same exact counts.

For each multi-directional root, inspect the four original renders, the 64² changed F32 render, A in U8 and F32, B in F32 and signed D in F32. Compare the 27 raw graph images per render, including the input and output nodes. Across both roots all **7,796,736/7,796,736 pixels in 486 images** match, maximum zero. This includes 4,620,288 pixels in 288 individual directional warps. Before the change, one grayscale warp at angle .625 and intensity 4 differed by one byte at four 64² pixels. This propagated into all four output formats.

Compute horizontal direction from the sine of the angle in radians plus a quarter turn. Round the radian sum to single precision before evaluating the sine. The measured root cases agree when narrowing each sine toward zero and fusing the displacement with the pixel center. A constant warp intensity can exceed the previous hard slider limit of 1,088. Its magnitude changes sampling coordinates and leaves the tap count fixed.

| Direct warp measurements | Renders | Exact pixels / compared | Maximum error |
|---|---:|---:|---|
| All source, map and output format combinations | 64 | 262,144/262,144 | 0 |
| Unequal dimensions with inherited warp size | 70 | 412,160/412,160 | 0 |
| Unequal dimensions with explicit warp size | 70 | 340,480/340,480 | 0 |
| Sixty size controls and thirty X/Y position ramps | 90 | 953,472/953,472 | 0 |
| Eight direction and wrapping configurations | 224 | 1,089,513/1,089,536 | 1 U8, 48 U16, .00241088867 F16, .005859375 F32 |
| Amplified X/Y position ramps | 72 | 274,432/294,912 | 1.96875 F32 |

For the 64 format combinations, use the [orientation source and mask](#directional-warp-orientation-and-input-precision) at 16×32 and 32×16 respectively. Round integer fields to the nearest stored level. Use 1.75H−.375 for floating source images and M for floating masks, rounding the F16 file to half precision. Render at 64² with intensity 17, angle .125, both axes tiled and seed 961. Independently cross the source, mask and output in U8, U16, F16 and F32. At each resampling step use the wider of that input's format and the output, in the order U8, U16, F16, F32. With F32 source and map, output bytes at (0,0), (7,13), (14,5), (32,32), (63,63) are 0, 172, 114, 0 and 1.

The [large-displacement inputs](#directional-warp-tiling-and-unequal-input-sizes) now use all three authored intensities. The remaining five amplified Y ramps differ at −.3125, .9375 and 2.073125 turns. The ordinary 224-render matrix retains 14 differing pixels at angle zero and nine at −.3125. These primitive results delimit the root measurements above.

### Directional warp tiling and unequal input sizes

A graph with horizontal-only tiling previously clamped both axes when the warp angle and intensity were constants. At (0,0) in the 64² byte-field probe the F32 value was .02745098. Passing both graph tiling flags through the evaluator gives the measured .532509327. All 28 horizontal-only renders now match. The full [single-warp matrix](#directional-warp-orientation-and-input-precision) matches 1,089,472/1,089,536 pixels across 224 renders. The maxima are 1 U8, 288 U16, .00439453125 F16 and .0048828125 F32.

The warp applies the displacement to the output pixel center with one fused multiply-add per axis. A source whose dimensions each lie between the output dimension and twice that dimension supplies the taps directly. Other sources pass through stored factor-two levels. An axis within one step of its target waits until both axes can take their final step together. For example, an 8×32 source at 128² goes through 16×64, 32×64, 64×64 and 128×128. The mask follows the same size rule and is sampled at output pixel centers.

| Measurement | Renders | Exact pixels / compared | Maximum U8 / U16 / F16 / F32 |
|---|---:|---:|---|
| Unequal original source and map, warp size inherited from source | 70 | 392,072/412,160 | 13 / 723 / .012451171875 / 0 |
| Same inputs with the warp size relative to the graph | 70 | 319,070/340,480 | 43 / 10,985 / .012451171875 / 0 |
| X and Y position ramps, square and rectangular | 30 | 79,872/79,872 | F32 0 |
| Unequal dimensions with a unit mask and F32 output | 60 | 873,600/873,600 | F32 0 |

For the 60 unit-mask renders, use the original grayscale byte field from the [orientation probes](#directional-warp-orientation-and-input-precision). Bind a white mask at the source dimensions, set angle to .125 turns and intensity to 17. Set both axes to repeat. In order, test source dimensions 8×16, 8×32, 4×32, 128×16, 128×32, 128×8, 32×256, 256×128, 8² and 256² at output sides 8, 16, 32, 64, 128 and 256. Use seeds 947 through 956 in source order. For the two 70-render matrices, use the five source/map pairs and seeds 863+17j+n, with pair index j from zero and requested side n. Both U8 and F32 sources match in F32 output. Cases combining higher-precision sources with narrower outputs still differ.

All ten additional root inputs now match for both multi-directional warps. Across the full 19 cases, color matches **175,339/175,360** with a one-level U8 maximum and zero elsewhere. Grayscale matches **175,344/175,360**, with four changed-case pixels differing in each format. Maxima are 1 U8, 257 U16, .00390625 F16 and .00392156839 F32. Both non-uniform blur roots retain **175,360/175,360** exact pixels each.

To amplify small direction differences, bind separate 64² F32 fields returning x and y with an F32 unit mask and both axes tiled. For angles 0, .25, .5, .75, .125, .375, .625, .9375, .073125, −.3125, 1.1875 and 2.073125 turns, use intensities 4,096, 262,144 and 2,097,152. The 72 renders match 24,576/294,912 pixels, maximum 61.34765625. The resolved MaterialCraft intensity is 1,088 for all three authored values. At .625 turns and intensity 2,097,152 the reference center (32,32) reads x=56.53125 and y=56.34375. The two coordinates distinguish the direction components independently.

### Directional warp orientation and input precision

Both non-uniform blur sample roots now match **175,360/175,360 pixels each**, maximum error zero. Previously their nine baseline cases matched 96,971/132,096 grayscale and 50,664/132,096 color pixels. Directional warp now samples positive angles toward increasing image Y, scales distances in pixels by the shorter image side divided by 256, and bilinearly filters the image at its stored precision.

| Root | Renders | Exact pixels / compared | Maximum error by storage |
|---|---:|---:|---|
| Non-uniform blur grayscale sample | 19 | 175,360/175,360 | 0 in all four formats |
| Non-uniform blur color sample | 19 | 175,360/175,360 | 0 in all four formats |
| Multi-directional warp color | 19 | 175,329/175,360 | 1 U8, 257 U16, .00390625 F16, .00392156839 F32 |
| Multi-directional warp grayscale | 19 | 175,339/175,360 | 1 U8, 257 U16, .00390625 F16, .00392156839 F32 |

Each row includes the archived renders, the four 64² formats, the independent 32² input and ten additional comparisons. For those ten, use an original source with channel c = ((41x+53y+7xy+29+37c+11cx) mod 1021)/1020 and mask M = ((11x−17y+3xy+401) mod 509)/508. Use c=0 for gray. Round both to the nearest byte except for the signed case, which uses 1.75c′−.375 with c′ the source channel value, and M at F32. All color channels including alpha count toward a matching pixel.

| Case | Side, format, seed | Blur parameters (intensity, samples, angle, asymmetry, anisotropy, inner rotation) | Multi-directional parameters (intensity, angle, mode, directions) |
|---|---|---|---|
| Default | 16, F32, 137 | Defaults | Defaults |
| A | 64, all four formats, 251 and again F32 at 827 | (2.5, 6, .1875, .375, .5, .28125) | (7.25, .1875, 0, 4) |
| B | 128, F32, 353 | (1.75, 4, −.125, .125, .75, .0625) | (3.5, −.125, 5, 2) |
| C | 32, F32, 479 | (7, 2, .3125, .625, .25, −.125) | (11.75, .3125, 6, 1) |
| D | 32, signed F32, 557 and 64, byte source with F32 output, 733 | (4, 8, −.0625, .5, .375, .15625) | (4.25, −.0625, 10, 4) |

For a single directional warp, bind the same gray field and mask in U8, U16, F16 and F32. Use signed source values for float storage. For each source format and parameter row j, use a constant .625 mask at 16² F32, then varying masks at 32² F32, 64² U8/U16/F16/F32 and 128² F32. Seeds are 751+11j+d+17i with d the source format (0–3) and i the size/format index (0–6). Test (angle in turns, intensity) of (0,0), (0,7), (.25,11), (.125,17), (−.1875,−5.75) and (.073125,11.25) with both axes tiled, then (.1875,25) with neither tiled and (−.3125,21.5) with only X tiled. The 224 renders match **1,086,456/1,089,536 pixels**. Maximum errors are 255 U8, 65,535 U16, 1.070373535 F16 and 1.415079325 F32. The single-axis graph setting accounts for 3,016 differing pixels.

With a constant unit mask and separate F32 images returning x and y, all **79,872/79,872** pixels match across 30 renders, maximum error zero. Use intensity 16 and angles 0, .25, .125 at 64², 32×16, 16×32, 128×32 and 32×128. The measured diagonal shifts are .70703125 pixels at the two 16-pixel short sides and 1.4140625 at the 32-pixel short sides. For unequal source/map dimensions use (32×16,64²), (128×64,32×16), (64²,16×32), (128²,32²), (16²,128²), U8 or F32 inputs, angle .125 and intensity 17. The seven output requests above, using the varying mask throughout, match **294,268/412,160 pixels** across 70 renders. The corresponding maxima are 255, 65,535, 1.201248169 and 1.202106290.

Six other inventory roots contain the same primitive. Replaying their archived outputs, precision sweeps and independent inputs gave the following results. For Messy Fibers 4 the archive has three renders. Each other row has four. Integer maxima below are stored levels.

| Root | Renders | Exact pixels / compared | Maximum U8 / U16 / F16 / F32 |
|---|---:|---:|---|
| creased.sbs/creased | 9 | 80,098/132,096 | 1 / 637 / 0.00048828125 / 0.0001525878906 |
| fluid.sbs/fluid | 9 | 2,715/132,096 | 7 / 16338 / 0.02526855469 / 0.02534788847 |
| messy_fibers_4.sbs/messy_fibers_4 | 8 | 56,296/115,712 | 1 / 354 / 0.00439453125 / 0.01023879647 |
| noise_creased.sbs/creased | 9 | 91,206/132,096 | 1 / 782 / 0.00048828125 / 1.531839371e-05 |
| noise_fluid.sbs/fluid | 9 | 5,620/132,096 | 5 / 11756 / 0.01708984375 / 0.01704069972 |
| pavement_1.sbs/pavement_1 | 9 | 59,168/132,096 | 255 / 65535 / 1 / 1 |

## 2026-11-18 root comparisons

### Pixel Processor products and sums

Use the signed RGBA32F field and position ramp specified under [3D linear gradient](#3d-linear-gradient). Each arithmetic probe has a nearest-sampled RGBA input, seed zero and a 128×128 float output. Compare the raw output in the 16.0.7 Direct3D 11 reference app with the MaterialCraft CPU cook. A pixel counts when all channels match. Integer errors in this log use stored levels. Floating errors use absolute differences.

| Pixel expression | Exact pixels / total | Maximum error |
|---|---:|---:|
| `R*G+B`, `B+R*G`, `R*G-B`, `B-R*G` | 16,384/16,384 each | 0 |
| `R*G+B*A`, `R*G-B*A` | 16,384/16,384 each | 0 |
| Runtime product plus .375, runtime channel times .3 minus B, literal product plus R | 16,384/16,384 each | 0 |
| Shared product read directly and in a sum, shared product used only in sums, vector times scalar plus vector | 16,384/16,384 each | 0 |

Rounding each multiplication separately matched 11,640 additions, 11,721 left subtractions, 12,098 two-product additions and 12,052 two-product subtractions out of 16,384 each. The largest difference was 1.907349e-6. The CPU evaluator now fuses the dynamic product into its immediate sum in pixel scope. A literal product or a product also read as a standalone value retains its rounded value. When both operands are products, the left product fuses.

Repeat the November 17 **3D linear gradient** cases with that change. At 256² and 512², the default UV, default world and oblique world gradients each match 65,536/65,536 and 262,144/262,144 LUM32F pixels, maximum error zero. Seed 42 at 128² matches all 16,384 default UV pixels and all 16,384 oblique pixels in each of LUM8, LUM16 and LUM16F. Contrast .6 still differs. Exact counts are 24,206/65,536 and 96,824/262,144 with a maximum error of 2.384186e-7. For the signed input above, a direct `1/G` probe matches 14,182/16,384 float pixels. All finite differences in that probe are one unit in the last place.

With the original shared lookup described under [basic and ridged 3D Perlin](#basic-and-ridged-3d-perlin), **basic 3D Perlin** now matches 65,536/65,536 pixels at 256² and 16,384/16,384 in each of the other six cases, including changed LUM16, LUM16F and LUM32F. All maximum errors are zero. With the applications' respective stock lookups, only 173/16,384 and 663/65,536 LUM8 pixels match. Both maxima are 203 levels.

### Runtime reciprocal and 3D gradient contrast

The twelve wired 3D linear gradient cases listed under [3D linear gradient](#3d-linear-gradient) now match **1,376,256/1,376,256 pixels**, maximum error zero. This includes all 65,536 and 262,144 LUM32F pixels at contrast .6 and all four measured output formats. An exposed contrast of .6 gives the float32 denominator `0x3ecccccc`. Its runtime reciprocal is 2.5. With a literal expression, the reciprocal is the next float above 2.5.

For a standalone Pixel Processor, nearest-sample a linear RGBA32F input and output the reciprocal of one channel at its original dimensions. Five fields match **671,744/671,744 LUM32F pixels**, maximum error zero. Use seed zero. For the 128² case take G from the signed field under [3D linear gradient](#3d-linear-gradient). Construct the remaining four fields from float32 bits at row-major pixel index `i`, repeated in RGB.

| Dimensions | Input bits | Exact pixels / total |
|---|---|---:|
| 256² | `0x3f800000 + 128*i` | 65,536/65,536 |
| 256² | `(125 << 23) + floor(3*2^23/5) - 32768 + i` | 65,536/65,536 |
| 512² | Use `h=(747796405*i+2891336453) mod 2^32` as below | 262,144/262,144 |
| 512² | Use `h=(1013904223*i+341937) mod 2^32` as below | 262,144/262,144 |

For the 512² fields, use the low 23 bits of `h` as the fraction, `7 + (37*((h >> 22) XOR i)) mod 241` as the biased exponent, and bit 2 of `i` as the sign. MaterialCraft derives a quadratic for each of 128 mantissa intervals from the mathematical reciprocal. The measured fixed-point square retains partial products through fractional bit 15. The literal division path retains its original rounding. A separate 32² RGBA32F probe emits contrast, denominator, reciprocal and half the reciprocal minus one half. All nine runs match 1,024/1,024 pixels each, including exposed values .3, .35, .51, .6, .625, .875 and .99, an image input and a literal.

With Position disconnected, both apps return zero in U8 and U16 and quiet NaN in floating storage. At 32² with seed zero, the 1,024 raw samples are respectively `0x00`, `0x0000`, `0x7fff` and `0x7fffffff`. At 64² with seed 42 the F32 result has the same NaN bits at all 4,096 pixels. The five disconnected cases match **8,192/8,192 bit patterns**. Maximum finite error is zero for the two integer runs. The float runs contain only NaNs, for which a numerical maximum error is undefined. Across the 12 wired and five disconnected cases the root matches **1,384,448/1,384,448 pixels**.

Four independent 32² image functions distinguish output storage from scalar parameter validation. Bind an RGBA32F field with `R=(x mod 7)-3`, `G=1` on every fourth row starting at zero and `G=0` elsewhere, `B=0` and `A=1`. Output, in turn, B/B, A/B, R/B and R/G. In U8, U16, F16 and F32 each run matches 1,024/1,024 stored samples, **16,384/16,384** overall. The mixed R/G float images each contain 256 finite values, 120 quiet NaNs, 288 positive infinities and 360 negative infinities. Every special value matches bit for bit and the maximum finite error is zero. Integer storage maps NaNs and negative infinities to zero and positive infinities to white.

### Resizing images bound to graph inputs

Bind a 128² linear image directly to a graph input, set the input bridge to the indicated storage format, and nearest-sample its output at pixel centers into RGBA32F. The CPU cook and the reference app match **4,534,272/4,534,272 pixels across 72 cases**, maximum component error zero. All four channels count toward each pixel.

| Source | Bridge | Output sides | Cases | Exact pixels / total |
|---|---|---|---:|---:|
| Original RGBA8 atlas field | Color in U8, U16, F16 and F32 | 64, 128, 256 | 12 | 344,064/344,064 |
| Original RGBA16 and signed RGBA32F fields | Color in U8, U16, F16 and F32 | 32, 64, 128, 256, 512 | 40 | 2,793,472/2,793,472 |
| Same RGBA16 and signed RGBA32F fields | Grayscale in U16 and F32 | 32, 64, 128, 256, 512 | 20 | 1,396,736/1,396,736 |

Use the RGBA8 formula under [atlas crop with wired value inputs](#atlas-crop-with-wired-value-inputs) and the signed RGBA32F field under [3D linear gradient](#3d-linear-gradient). For the RGBA16 field, map each signed input component `v` to the nearest integer to `65535*(v+4)/8`. Use seed 42 below 128² and zero otherwise. The grayscale bridge result was also read directly before expansion to RGBA.

Each factor-two step filters four clamped texels at the source storage precision, then stores the result in the bridge format. For grayscale, the first step filters the three color channels before computing the equal-weight mean. Further steps read stored grayscale. Before the fix, the 256² RGBA8 source case with an F32 bridge matched 4/65,536 pixels, maximum error 7.629395e-6. Filtering the same RGBA16 field after conversion to gray matched only 53,819/65,536 gray U16 pixels at 256².

The 256² atlas wrapper now matches 65,281/65,536 at default rotation, maximum .250003815. Its 255 differing pixels lie on the first row and column of the lower-right quadrant. Rotation .125 matches 65,089/65,536, maximum 1. The directly bound controls above and this wrapper exercise different graph connections.

With the inputs and settings under [panoramic LOD](#panoramic-lod), the four 64² seed-42 format runs now match 4,096/4,096 each, maximum zero. The 128² opaque, alpha and changed-seed outputs each match 16,128/16,384, maximum 125 byte levels. At 256² the opaque image matches 64,768/65,536, maximum 126. Both 128² flat controls match 16,128/16,384, maximum 116.

The rotated 3D planar projection runs still differ. At 128² the grayscale output matches 16,335/16,384 in LUM16, maximum 226 levels, and color matches 16,378/16,384 in RGBA8, maximum one level. At 64² both roots match 4,094/4,096 in U8 and 4,093/4,096 in each of U16, F16 and F32. The respective maxima are 1, 32, .00048828125 and .000488288701. All three default and changed-seed runs for each root match 98,304/98,304 pixels. With the [shared lookup](#basic-and-ridged-3d-perlin), ridged 3D Perlin matches all four eight-bit cases. Its changed U16, F16 and F32 runs match 16,337, 16,380 and 5,160 of 16,384 pixels, with maxima 1, .00048828125 and 1.78813935e-7.

### Wired graph textures and pixel sampling

An original two-graph package returns two outputs from the child. One child output returns its color input directly, the other reads it using a Pixel Processor. The parent supplies the 128² RGBA8 atlas field above at a fixed 128². Give the child input a parent-relative size and set its precision explicitly. Read the stored bridge from the direct output and the sampled value from an RGBA32F output. Each pixel comparison includes all four components and checks the reported dimensions and depth.

| Controls | Output comparisons | Exact pixels / total | Maximum error |
|---|---:|---:|---:|
| Nearest and bilinear at pixel centers, U8/U16/F16/F32 bridge, sides 64/128/256, seed zero | 48 | 1,376,256/1,376,256 | 0 |
| Bilinear at 32² seed 42 and 512² seed zero, same four bridge formats | 16 | 2,105,344/2,105,344 | 0 |
| At 256² and F32, change the tiling on the parent input, child input or processor to none, horizontal or vertical | 18 | 1,179,648/1,179,648 | 0 |

The directly returned bridge resizes through stored factor-two levels using its own horizontal and vertical tiling flags. The sampled color input retains the source dimensions and precision inside the graph instance. For example, at 64² the first red value is 48/255 when sampled nearest and 24/255 through the direct U16 bridge. At 256², both axes wrap for the bilinear sampler. The first atlas texel receives a green contribution of 16,384/65,535 from the opposite edge.

For a separate single-graph control, nearest- or bilinear-sample the same fixed 128² input at `$pos + (−.3125,.1875)`. Set the processor output to RGBA32F, side 256 and seed 42. Repeat with its tiling set to none, horizontal, vertical and both. All **524,288/524,288** pixels match, maximum zero. The explicit image read repeats both axes in all eight cases.

In the [atlas crop wrapper](#atlas-crop-with-wired-value-inputs), default rotation at 256² now matches **65,536/65,536** RGBA32F pixels, maximum zero. With .125 turns the 256² case matches 65,408/65,536, maximum .994125247. At 128² it matches 16,320/16,384 with seed zero or 42, maximum 1. At 64², 4,064/4,096 match, maximum .996078432 in the authored float output. The remaining rotation differences follow one edge of the sampled shape.

For the [spline distance controls](#height-distance-texture-mapping-skew-and-weave), both wired counts now match 1,024/1,024 at 32² seed 42 and 4,096/4,096 at 64² seed 137 in LUM32F. At 16² seed zero, count 1 matches 256/256 and count 2 matches 252/256, maximum 5.96046448e-8.

### Value Processor coordinates

Both maximum-coordinate roots return the same Float2 as the reference in **12/12 typed comparisons each**. All maximum component errors are zero. Before the fix, reading the position variable stopped both cooks. A Value Processor starts at (0,0) and passes this position to nested function calls.

| Root | Defaults, changed input and seed | Four requested output formats | Four isolated peaks |
|---|---:|---:|---:|
| `pbr_render.sbs/get_max_coords/get_max_coord` | 4/4 | 4/4 | 4/4 |
| `pbr_render2.sbs/get_max_coords/get_max_coord` | 4/4 | 4/4 | 4/4 |

Use the byte field (17x+31y) mod 256 at 128² and 256² with seed zero, and at 128² with seed 42. The changed field is 32+((17x+31y) mod 160) at 128² with seed zero. At 64² seed 42, requesting U8, U16, F16 and F32 at the upstream processor yields the Float2 (.1015625,.109375) in all four cases. These roots expose no ordinary parameters. For the original single-peak field use (11x+13y+3xy) mod 79, and set the specified pixel to 255.

| Size | Seed | Peak pixel | Typed result in both roots |
|---:|---:|---|---|
| 16² | 7 | (1,2) | (.09375,.15625) |
| 32² | 91 | (21,7) | (.6640625,.234375) |
| 64² | 113 | (57,48) | (.890625,.75) |
| 128² | 251 | (122,17) | (.953125,.125) |

Separate original value graphs read `$pos`, `$size`, `$sizelog2`, `$depth`, `$depthpow2` and `$number`. At 16² and 128² with seed zero and 64² with seed 42, the position and three depth or iteration values are zero. The size values follow the canvas and its base-two logarithm. Is Defined reports true for the position, size and three depth or iteration names and false for an unbound probe name. Unbound numeric and Boolean reads return a typed zero. These scalar and vector controls match **50/50 typed values** with maximum error zero.

A Value Processor also averages a sampled color’s three channels in the order measured under [grayscale sampling](#grayscale-sampling-of-color-inputs). Using the same four RGBA fields, sample at (39.5+.1875,77.5+.5625)/128. Repeat with a uniform (.25,.5,.75,1). Compare the single Float or Float4 as IEEE float32 components in nearest and bilinear modes. Before correcting normalization and filtering, 23/32 values matched across the two input fields, with maximum component error 5.84125519e-06.

### Value Processor texture filtering

The 32 scalar and color comparisons above now match **32/32 typed values**. Two independent 128² coordinate sets add **192/192 Float4 values**. Two more sets at 2² and 32² give **16/16 Float4 values**. Maximum component error is zero across all 240 comparisons. Both maximum-coordinate roots still match all 24 previously measured Float2 results.

A value function wraps both texture axes even when output tiling is disabled. For bilinear sampling, multiply the UV by the stored dimensions, subtract .5 and keep the full float32 fractional texel position. In each row compute a×(1−t)+b×t with two rounded products and a sum, then do the same vertically. For U8 or U16, interpolate stored integer channel levels as float32 and finally multiply by the float32 reciprocal of 255 or 65,535. A nearest read normalizes the selected stored level with the same reciprocal. Floating textures already carry their sample values.

Use the four original color fields described under [grayscale sampling](#grayscale-sampling-of-color-inputs), both filtering modes and output tiling disabled. For the first set choose (x,y,a,b) from (7,13,0,0), (83,127,48,144), (42,78,19,61), (17,93,128,128), (0,62,−93,181), (126,7,201,113), (77,0,−171,−35), (128,67,47,23), (−2,132,105,151), (31,40,1,255), (4,1,77,99), (76,30,45,85). The UV is (x+.5+a/256,y+.5+b/256)/128. For the second set use (((227j mod 1447)−153)/1009, ((281j mod 1867)−413)/1031) for j=0…11. In both sets alternate seeds 0 and 42.

For the small independent field, take row-major RGBA byte tuples (19,67,129,251), (85,222,17,9), (255,128,74,66) and (3,102,39,109). For U16, multiply each level by 251. For F16 and F32, use (level−97.125)/13.375. Sample at (−.317,1.093), seed 17, directly from a 2² bridge and again after the bridge resizes the field to 32². With the 2² byte field, the four bilinear component bit patterns are `3e8013a0 3f2bd646 3e0f43a5 3e744cb7`.

### Luminosity, axial gradients, color lookup and luminance merge

The five roots below each match **165,120/165,120 pixels in 15 cases**. Replaying the four first-pass captures gives 114,688 exact pixels per root. Four 64² producer formats at seed 42 give 16,384, and a separate 32² F32 case at seed 113 gives 1,024. Six further original cases give 33,024. Their sizes are 16² F32 at seed 17, 64² U8/U16/F16/F32 at seed 71 and 128² F32 at seed 137. Floating comparisons include the stored bits of every channel. Maximum finite error is zero throughout.

| Root | Seed 42 settings | Independent controls at seeds 17, 71 and 137 |
|---|---|---|
| `blend.sbs/luminosity` | Opacity .67 with foreground, background and mask | Disable alpha blending, set opacity to .375 at 16² and 64² or .8125 at 128², and use the fields below |
| `gradient_axial.sbs/gradient_axial` and `gradient_axial_reflected` | Endpoints (.15,.3), (.7,.85), expansion disabled | Enable expansion. Use (.8125,.21875) and (.09375,.78125). At 16² set both endpoints to the first point |
| `hald_clut.sbs/hald_clut` | Identity lookup, input alpha control disabled | Use a new nonlinear lookup and vary the input alpha. Enable alpha control at 16² and 64², disable it at 128² |
| `hdr_merge.sbs/average_luminance_merge` | Original HDR field, exposure delta 2 | Use the HDR field below. Set exposure delta to −.75 at 16² and 1.375 otherwise |

For Luminosity, let t be 1 for background, 2 for foreground or 3 for mask. Generate C=(11x+17y+3xy+31t) mod 256. The mask is C. The color inputs contain (C, (41x−19y+11t) mod 256, (3x+79y+t) mod 256, 42×((13x+7y+31t) mod 7)). For the 512² lookup, use the low, middle and high six bits of the row-major index as R, G and B indices. Map each to the nearest byte using 255/63. The lookup stores (floor((R+2G)/3), 255−B, floor(R/8) XOR floor(G/4) XOR floor(B/2), 255). Feed it RGBA bytes ((11x+7y+3xy) mod 256, (43x+17y+28) mod 256, (5x+11y) mod 256, 23×((7x+3y) mod 11)). The original HDR input stores RGBA32F (((7x+23y+5xy) mod 303)/53, .09375+((19x+2y) mod 187)/51, ((x+8y) mod 103)/31, 1).

With coincident endpoints both gradients return the same quiet NaN, bit pattern `0x7fffffff`, at every one of the 256 pixels. All other outputs are finite. At 64² the five independent F32 images have respectively 4,092, 2,210, 2,053, 4,096 and 4,096 distinct stored pixels.

### Skipped loops and path buffers

A loop runs its initializer before testing its exit condition. If the condition is already true, or the iteration allowance is zero, its result is the zero of its declared result type. A loop that enters the body returns its last body value. MaterialCraft previously returned the initializer on a skipped loop. A Boolean initializer followed by a float body consequently stopped both path roots with a type error.

Use a standalone loop with a scalar body of 9 and an initializer of 7, true, (3,5,7,9) or an unwired initializer. Set the exit condition to true, false or pixel X ≥ .5, with iteration allowances 0, 1 or 2. Eight pixel functions at 16² seed 7 and 32² seed 91 match **10,240/10,240 LUM32F pixels**. The eight matching value functions give **16/16** exact typed results. Changing the result type to Boolean, integer pair or float pair with a true or false condition gives another **12/12** exact typed results. Three 32² uniform-parameter image probes match **3,072/3,072 pixels**. All maxima are zero. The regression also reads a variable assigned by the initializer after the loop. Direct Boolean or vector outputs from a grayscale Pixel Processor are a separate open conversion case.

| Path root | Inputs and controls | Exact pixels / total | Maximum error |
|---|---|---:|---:|
| `Impl/sort_by_blue` | Four original 16² RGBA32F cases with seed 0 or 42, including changed settings | 1,024/1,024 | 0 |
| Same | 64² seed 42, passthrough enabled, count 32, U8/U16/F16/F32 | 16,384/16,384 | 0 |
| Same | Passthrough disabled, count 7 at 16² seed 17; count 19 at 32² seed 71 in four formats and 64² seed 137 F32 | 8,448/8,448 | 0 |
| `Impl/compute_paths_lengths` | Four original 16² cases, including the 1×1 passthrough result, plus four 1×1 results at requested 64² seed 42 | 773/773 | 0 |
| Same | Five-point transformed path at 16² seed 17 F32, intermediate lengths disabled | 256/256 | 0 |
| Same | Intermediate lengths enabled and passthrough disabled, five or seven points, seed 17, 71 or 137 | Execution budget reached | Unavailable |

The sort holdout uses RGBA bytes ((17x+13y+5) mod 256, (29x−13y) mod 256, 23×((3x+7y) mod 11), (7x+5y+11) mod 256). Its outputs contain seven or nineteen distinct RGBA tuples. Across the fourteen sort cases **25,856/25,856 pixels** match. The length holdout starts with the original five- or seven-point closed path buffer and replaces each vertex (x,y) by (.0625+.625x+.1875y, .125+.0625x+.6875y). Set scale to (1.25,.8125). Both the resized 16² path buffer and independently sized buffers with header texel spacing 1/side reach the local execution limit with intermediate lengths enabled. The reference app returns a grayscale image. An error image supplies no comparable samples.

### Luminance EXR input

Generate a half- or single-precision EXR with luminance `(((67x+131y+5xy) mod 1024)−256)/128`. Write either Y alone or Y with a separate alpha channel `((11x+13y) mod 64)/64`. Bind each file to a grayscale input and sample at pixel centers with a Pixel Processor. At 32² use seed 113 and store U8, U16, F16 and F32. At 64² use seed 239 and store F32. These twenty original inputs and output-format combinations match **32,768/32,768 pixels**, maximum error zero. The previous decoder rejected the luminance files. MaterialCraft now decodes bounded Y and YA images through the existing shared image codec and retains linear signed and HDR luminance. The in-memory import regression checks both linear flag settings for both input precisions and layouts.

Repeating the nine independent input cases each for [color burn, color dodge](#color-burn-and-color-dodge) and [color temperature](#color-temperature-range-reductions-sky-parameters-and-polygon-paths) with genuine luminance resources gives **117,504/117,504 exact pixels**. The direct RGBA Divide and Switch probes at full and partial opacity match **163,840/163,840 pixels** with genuine luminance masks. The signed scalar endpoint cases still distinguish positive and negative zero. Full and partial Divide each match 63,488/65,536 for both F16 and F32. Full Switch matches 63,744/65,536 at F16 and 63,232/65,536 at F32. Partial Switch matches 65,280/65,536 at both precisions. Their maximum absolute error is zero.

### Grayscale sampling of color inputs

Feed an original 128² image to a Pixel Processor and read input zero with the grayscale sampler at pixel-center UV plus (.1875,.5625) texels. Use seed 42 and a LUM32F output. With nearest and bilinear filtering, RGBA8, RGBA16, RGBA16F and RGBA32F each match 16,384/16,384 pixels per mode. On the signed F32 input, reading only red previously matched zero pixels in both modes. Maximum errors were 4.95148039 and 3.83905315. Adding green and blue first, then red, and multiplying by the float32 reciprocal of three after sampling matches all eight color cases exactly.

For RGBA8 use ((17x+31y) mod 256, floor(255x/127), 255−floor(255y/127), (7x+13y) mod 256). For the floating formats use the signed four-channel input under [3D linear gradient](#3d-linear-gradient). For RGBA16 encode round((channel+4)×65535/8) from that field. The eight grayscale-bridge controls also match all pixels. Their U8 input is (61x+137y+7xy) mod 256. Their U16 input is q=(2029x+4889y+193xy) mod 65536. For floating storage, supply a linear RGB image with the same float32 value q/8191−3.25 in each channel and alpha one, and select a grayscale bridge. The single stored channel is sampled directly. The sixteen comparisons match **262,144/262,144 pixels**, maximum error zero. A separate luminance-only float EXR was readable in the reference app and the current MaterialCraft decoder returned an error for that file.

### Half-precision Crop area and input sizes

| Root | Renders | Exact stored pixels / compared | Maximum error |
|---|---:|---:|---:|
| Color Crop, both outputs | 48 | 377,344/377,344 | 0 |
| Grayscale Crop, both outputs | 48 | 377,344/377,344 | 0 |

Each root produced 256,000 exact U8 pixels, 26,624 U16, 30,720 F16 and 64,000 F32. The four archived renders each expose two image outputs. The remaining runs change storage at the producing node of Output or Area. The formerly mismatching enabled Area renders at 64² now match all 4,096 pixels each. The [earlier Crop controls](#crop-output-and-area) give the source fields, transforms and first exact counts.

For the additional twelve renders per port, use original bytes Hⱼ=((37+13j)x+(29−5j)y+(7+2j)xy+53j) mod 256. Use j=0…3 for RGBA and j=0 for gray. Run native defaults at 16²/seed 47 in F32. Enable Show Crop Area for the remaining cases. At 32²/139 use Input Size (32,64), matrix (.75,−.125,.375,1.25), offset (−.1875,.3125) and filter 0. At 64²/277 use (64,64), (.625,.25,−.375,1.125), (.125,−.25) and filter 1. Run both in all four output formats. At 32²/419 use 16×32 input, Input Size (16,32), identity, offset (.09375,.1875), filter 0 in F16 and F32. Repeat the first changed F16 case at seed 619. These inputs contribute 47,616 exact pixels per root.

With F16 blend output and equal physical extents, normalized integer inputs pass through half storage when the source and blend differ in whether their effective output size depends on a runtime expression. A plain size, a literal Int2 or an expression made solely of constants each use the static path. A getter or assignment sequenced before the same Int2 can change the rounding. A blend inheriting the background size also inherits that dependency. A pass-through node preserves it. The cook cache includes the distinction.

For a two-pixel reproduction, blend foreground bytes (71,55) over background (155,1) at opacity .4. Use a static (0,0) relative size or a sequence that reads the runtime size then returns (0,0). The half output expanded to F32 has bits 3ef3c000 and 3db58000 with both inputs static. Making just the foreground size dynamic changes them to 3ef3a000 and 3db56000. The regression varies both sources and blend inheritance with a shared cache.

| Standalone blend controls | Renders | Exact stored pixels / compared | Largest stored error¹ |
|---|---:|---:|---:|
| Output size read or assignment | 64 | 262,144/262,144 | 0 |
| Size inheritance through two image sources | 96 | 391,540/393,216 | 1 |
| Eleven expressions with byte, word or half inputs | 132 | 540,672/540,672 | 0 |
| Independent foreground, background and opacity sizes | 36 | 147,456/147,456 | 0 |
| Eleven expressions with full-float inputs | 44 | 180,224/180,224 | 0 |
| Different physical extents and equal-size controls | 96 | 293,493/393,216 | 5501 |

¹ Integer errors use stored levels. Floating errors use absolute component differences. For the separate source-depth controls, use a 64² linear field Hⱼ=(((19+16j)x+(13+28j)y+(7+6j)xy+43j) mod 1021)/1019−.03125 for foreground j=0 and background j=1. Change the stored precision, size inheritance and expression independently. Compare all four output formats. The 96 size-inheritance renders match all 98,304 F16 pixels. In that set, 1,674 F32 pixels differ by at most 1.19209290e−7 and two U16 pixels differ by one level. The physical-size controls also exercise 16, 32 and 128 texel inputs to a 64² blend. Those filter results have their own measurements above.

### Crop output and area

The color and grayscale Crop roots each match **229,376/229,376** pixels across both outputs in the four original cases. Use the default graph at 128² and 256² with seed zero, repeat 128² with seed 42, then disable Show Crop Area, set Input Size to (128,128), Matrix to (1.2,.25,−.125,.8), Offset to (.1,−.15), and Filtering to 1 at 128². The inputs are the original foreground or grayscale ramp from the first pass. At 64² seed 42, retaining the 128² original input, these altered settings match a further 32,768/32,768 pixels per root across the two outputs and all four storage formats. Every maximum is zero.

For the independent case, enable Show Crop Area, set Input Size to (64,128), Matrix to (.75,−.125,.375,1.25), Offset to (−.1875,.3125) and Filtering to 0. Use the original color or height field under [height distance, texture mapping and skew](#height-distance-texture-mapping-skew-and-weave) at the output dimensions. At 128² seed 71 F32 and 64² seed 137 in all four storage formats, each root’s Output matches 32,768/32,768 pixels. Area matches in U8, U16 and F32. At 64² F16, color Area matches 4,054/4,096 and grayscale Area matches 4,087/4,096. Both maxima are .00048828125. The last operation on both Area outputs is a blend.

### Tile edge masks and normalized spline distances

| Root | First four captures | Four 64² parent formats, seed 42 | Independent 32², seed 113 | Seven new cases | Total exact / compared |
|---|---:|---:|---:|---:|---:|
| `smart_auto_tile.sbs/pattern_find_edges_step` | 3,146,752 | 4,096 | 1,024 | 34,048 | 3,185,920/3,185,920 |
| `spline_tools.sbs/internal_tools/normalize_spline_distance` | 1,024 | 16,384 | 1,024 | 34,048 | 52,480/52,480 |

All 16 cases per root match at the stored precision, with maximum error zero. In the edge-mask root the default grid exponent is 10, so the first three results are each 1,024² at parent sizes 128² or 256² and seeds zero or 42. Setting the grid exponent to 5 produces a 32² result. The normalized distance root has a single grayscale input and no additional exposed settings. Its first four outputs are 16² LUM32F. The earlier precision and independent input definitions are in the [four-format inventory](#four-format-inventory-comparison).

The seven new cases per root use 16² F32 at seed 17, 64² U8/U16/F16/F32 at seed 71, 128² F32 at seed 137 and 32² F32 at seed 211. Use integer coordinates starting at zero, with N equal to the requested side. Set the edge-mask grid exponent to log₂(N), blur and smoothness to .25, combined threshold to .375 and per-channel thresholds for base color, normal and height to .1875, .3125 and .125. Enable per-channel thresholds except at 32². Disable height at 16², base color at 32² and normal at 128². Bind linear byte fields using z=(13x+7y+3xy) mod 256. Base color is (z,19x+3y,7x−23y,255−z), normal is (127+2x,91+3y,255−floor(z/5),255) and height is 19x+23y+7xy, reducing all byte channels modulo 256. The mask alpha is 255. Its RGB channels are 255 where (floor(x/4)+2floor(y/5)) mod 3 is nonzero, x < N/2 and y mod 7 ≥ 3, respectively, and zero otherwise. The 64² F32 output contains ten distinct RGBA tuples.

For normalized distance at 16² and 64² use a grayscale byte field (17x+7y+3xy) mod 256, with the rightmost column replaced by 13y mod 256. At 128² use 16-bit levels (311x+1861y+31xy) mod 65536 and zero the final column on rows y mod 3=0. At 32² store (((13x−31y+2xy) mod 253)−87)/32 in each RGB channel of a linear float image, alpha one, with the final column zero on rows y mod 4=0. The 64² F32 output has 3,330 distinct levels.

The [wired bridge distance](#height-distance-texture-mapping-skew-and-weave) root was also retested. With count two, 32² and 64² now match 1,024/1,024 and 4,096/4,096. The 16² case matches 252/256 with maximum 5.96046448e−8. All three count-one cases remain exact.

### Runtime pixel logarithms and powers of two

An original pixel function returns (log2(1+t), 2ᵗ, 2⁻ᵗ, log2((1+t)/2)). Across eight 1024² RGBA32F images all **8,388,608 pixels and 33,554,432 components** match bit for bit, maximum error zero. For image j=0..7, let t=(j×2²⁰+i)/2²³ with row-major index i, seed 251. Supply the four arguments as a linear RGBA32F input and sample at pixel centers with nearest filtering. This exhausts all float32 mantissas in [1,2) for the logarithm and all 23-bit fixed fractions in [0,1) for the exponential.

| Additional per-pixel controls | Cases | Exact / compared | Maximum finite error |
|---|---:|---:|---:|
| Logarithm base two and power of two on the 256² fields below | 22 | 1,441,792/1,441,792 | 0 |
| Natural logarithm, log2(x)×float32(ln(2)), and 2^(log2(x)×y) on signed and exceptional pairs | 6 | 393,216/393,216 | 0 |
| Both signs of zero, subnormals, smallest normals and NaN payloads, three operations at 16² | 3 | 768/768 | 0 |
| Exposed scalar passed directly to the pixel function, three operations | 28 | 7,168/7,168 | 0 |

For the 256² fields use the row-major 32-bit hash h from [the runtime trigonometry controls](#runtime-pixel-sine-cosine-and-atlas-rotation). At seed 239, set (x,y)=(1+t,t) with the 23-bit numerator of t equal to 128i, i, h mod 2²³, or 0x3a0000+i. For the signed field use x=4h/2³²−2 and y=12i/65535−6. The exceptional field cycles x through NaN, −∞, −2, −1, −0, +0, 2⁻¹⁴⁰, 2⁻¹²⁶, .25, .5, 1, 2, 2²⁰, +∞. Repeat each y for 17 pixels in the cycle NaN, −∞, −129, −126, −2, −1, −.5, −0, +0, .5, 1, 2, 3, 127, 128, +∞. Round inputs to float32 before sampling.

At seed 193, the full-range field combines exponent bits i mod 255 and mantissa h mod 2²³ for x, and uses y=300h/2³²−150. The power-boundary field starts at float bit pattern ((i mod 254)+1)×2²³ for x and at float32((i mod 255)−127) for y, then offsets their bit patterns by (floor(i/254) mod 17)−8 and (floor(i/255) mod 17)−8 respectively. For the near-one field offset the bit pattern of 1 by i−32768 and set y=(i−32768)/2³¹. A fourth field uses x=.5+1.5h/2³² and y=2h/2³²−1. The last cycles x through (.5,.7,.625,.2,.8,1,1.125,2,0,−1,−.7) and repeats each successive y in (0,1,2,3,4,5,6,7,.5,1.375,−1) for 13 pixels. For the 16² exceptional controls, cycle float32 input bits through 00000001, 007fffff, 80000001, 807fffff, 00000000, 80000000, 00800000, 80800000, 7fc12345 and ffc12345 at seed 17.

Both pixel operations evaluate a quadratic over one of 64 intervals. The coefficients come from the mathematical functions and the sampled rounding thresholds. The runtime logarithm of .5 has bits `bf7ffffe`, the literal result has bits `bf800000`. The pixel natural logarithm multiplies the base-two result by float32(ln(2)). Replaying the 30 existing sine and cosine comparisons after sharing the polynomial fitting routine gives 1,443,840/1,443,840 pixels, maximum zero.

For the exposed scalar controls use .5, .625, .7, 1, 1.125, 1.25, 2, 4 and 1.375 for either logarithm. For the exponential use −2, −1, −.625, −.5, −.125, 0, .5, 1, 1.375 and 2. Cycle seeds 71, 137 and 313 at 16². In a separate Value Processor, the same arguments match 5/9 typed base-two logarithms, 5/9 natural logarithms and 9/10 powers of two. Maximum errors are 5.96046448e−8, 2.98023224e−8 and 2.38418579e−7 respectively. See the [uniform power and logarithm measurements](#uniform-powers-of-two-and-volume-lighting) for the subsequent fix and the remaining logarithm error.

Literal-only pixel graphs at 16², seed 23, give 512/768 exact pixels for each logarithm on .5, .7 and 1. At .7 their maximum errors are 5.96046448e−8 for log2 and 2.98023224e−8 for ln. Literal powers of two match 768/768 at −.5, 1.375 and 2, maximum zero.

The shared-lookup ridged root still matches 5,160/16,384 at changed roughness in F32, maximum 1.78813934e−7. Its one-octave .7 control still matches 1,847/4,096, maximum 2.38418579e−7. For positive normal x on the signed field, the reference's general power agrees with exp(y×ln(x)) on all 32,614 pixels. On .7 with exponent 1 it returns `3f333334`. Further work on general power and natural exponential is tracked with the [octave controls](#ridged-noise-octave-isolation).

### Marked variable reads in path and mipmap graphs

| Root | Renders | Exact RGBA pixels / compared | Maximum error |
|---|---:|---:|---:|
| Facet link template | 18 | 57,600/57,600 | 0 |
| Facet stitch template | 18 | 57,600/57,600 | 0 |
| Vertex processor | 18 | 57,600/57,600 | 0 |
| Simple vertex processor | 18 | 57,600/57,600 | 0 |
| Color mipmap N from O | 18 | 287,232/287,232 | 0 |

A Get variable operation consumes one leading `%`. For an exposed Float named `angle`, Get `angle` and Get `%angle` both return the exposed value. Set `angle` to .6875 before the second read and it returns .6875. Set `%angle` instead and Get `%angle` still returns the exposed value, while Get `%%angle` returns .6875. Is Defined tests the literal name. A Get of `%$size` has the same result as a Get of `$size`.

Use original Node, Pixel Processor, Value Processor and FX-Map functions. For each function use a 16² default at seed 41 with angle .625, 32² at seed 131 with .1875, and 64² at seed 409 with .875. Test bare, marked, twice-marked and absent names, and the read/write combinations above. For Is Defined choose .875 for true and .1875 for false. For the size probe select X and multiply by 1/128. The Node scope stores the scalar before returning an Int2 size. Before the fix the marked exposed Float matched 0/3 typed values and 0/5,376 Pixel Processor pixels, maximum .875. It now matches all three values and every pixel.

| Direct scope | Cases | Exact pixels or typed values / compared | Maximum error |
|---|---:|---:|---:|
| Node | 51 | 91,392/91,392 | 0 |
| Pixel Processor | 51 | 91,392/91,392 | 0 |
| Value Processor | 51 | 51/51 | 0 |
| FX-Map opacity | 51 | 80,640/91,392 | 1 |

At FX-Map opacity, an unresolved direct Get of `%other` or `%%angle` yields opacity 1 in the reference app and 0 in MaterialCraft. All six such renders differ at every pixel. A literal Is Defined check and a read after assigning the marked name match. The table records the unresolved direct-read behavior separately from the five measured root graphs.

The nine additional renders for each root use the [modular input, resolution, precision and seed matrix](#normal-maps-panoramas-and-primitive-shapes). For both facet templates, set radius to .6875, 1.3125 and 2.125 for A/B/C, enable init only in B, set foreach to .375, .75 and 1.125, and use finally (.1875,.34375,.75,1). Each changed facet output is uniform on these inputs.

For each vertex root use the original five-point path for defaults and A and the seven-point path for B and C. For every valid position (x,y) in the path's second row use (.0625+.625x+.1875y, .125+.0625x+.6875y). Bind each remaining image pin in identifier order to its modular field. Set the per-vertex pair to (.1875,.8125), (−.25,.625) and (.78125,.3125), with image input counts 1, 2 and 3. On the full processor set previous/next fetch counts to (1,1), (0,2) and (1,0). Both 64² F32 outputs contain 383 distinct tuples.

For the color mipmap use (N,O)=(2,1),(3,0),(4,1), init .375, .75 and 1.125, blend .8125, finally (.75,.125,.6875,1), and empty (.1875,.3125,.625,.25). The 64² F32 output has 2,558 distinct tuples. The default 16² parent produces 16×32 and the 128² B parent produces 128×256. Counts use the stored output extents and precision for each render.

### Plane light, splatter, symmetry and waveform comparisons

| Root | Renders | Exact pixels / compared | Maximum U8 / U16 levels | Maximum F16 / F32 error |
|---|---:|---:|---:|---:|
| Panorama plane light | 18 | 170,458/171,264 | 0 / 0 | 0 / 1.21593475e-05 |
| Splatter blend, grayscale | 18 | 170,610/171,264 | 77 / 19923 | 0.303955078 / 0.780312974 |
| Splatter blend, color | 23 | 203,407/204,032 | 90 / 23114 | 0.352539062 / 0.566889435 |
| Symmetry slice | 18 | 156,022/171,264 | 7 / 1632 | 0.0249023438 / 0.0248995423 |
| Waveform v2 | 18 | 142,662/171,264 | 255 / 65535 | 1 / 1 |
| Waveform 1 | 18 | 142,630/171,264 | 255 / 65535 | 2 / 2 |
| Weave 4 | 18 | 170,018/171,264 | 0 / 1 | 0 / 0.765621424 |

Each root includes the archived outputs, the four-format sweep, an independent 32² render and nine further original renders. The color splatter count also includes five earlier changed-input controls. Use the [modular image fields, A/B/C settings, sizes and seeds](#normal-maps-panoramas-and-primitive-shapes) for the additional renders except symmetry slice. Number the changed settings k=1,2,3 for A/B/C.

| Root | Additional changed settings |
|---|---|
| Plane light | World position (−.75,.4375,1.25), UV (.3125,.28125), rotation (.0625k,−.125,.1875), scale .6875, size (.75,1.25). Use distance-from-origin mode in B and alternate up vector in C. Set color mode and pattern to k mod 3, color (.75,.34375,.625), temperature 4375 and exposure 1.25. Disable background input in C and enable ground clipping in B. |
| Splatter blend | Set pattern count to 3, 8 and 5, and color-input opacity to .375, .75 and 1. For grayscale set luminance adjustment −.1875 and luminance random .3125k. For color set random pattern assignment .375, HSL adjustment (.1875,−.125,.25), HSL random (.125,.25,.375). Use normal processing and the alternate input normal format in B. |
| Waveform v2 | Set sample count to 17, 31 and 24, function to k mod 3, scale to 4+k, amplitude limits .1875 and .8125, noise .125k, pattern to 3, 6 and 4, pattern variation .125k and disorder .15625k. |
| Waveform 1 | Use the same sample counts, noise and patterns, size limits .1875 and .8125 and wave number k+2. |
| Weave 4 | Use tiling 3, 5 and 7. Enable rotation in A and C. |

For symmetry, use the 16² default at seed 17, then the modular RGBA field at 64² in all four formats with seed 73, 128² F32/181, signed 32² F32/293, a 32×16 input at 32² F32/449, and the 64² F32 repeat at seed 661. In this order the changed (symmetry mode, transfer mode, blend, flip side, flip side 2) settings are (1,0,.25,true,false), (2,1,.71875,false,true), (0,1,.5,true,true) and (2,0,1,false,false). Repeat the first setting at seed 661. The first differing pattern buffer at 64² matches 3,840/4,096 U8 texels, maximum one level. At (28,0) MaterialCraft stores 114 and the reference stores 113.

The plane-light Pixel Processor at 64² F32 matches 4,020/4,096 pixels, maximum 1.21593475e−5. At (14,6) its red channel is 2.48438478 in MaterialCraft and 2.48438454 in the reference. In both splatter graphs the signed case first differs in the same 256² RGBA16 Pixel Processor. Every pixel differs, maximum 65,415 levels. At (0,0) MaterialCraft stores (51546,38154,36337,11593) and the reference stores (3600,46195,62868,47648).

In the 64² F32 waveform captures, the earliest differing stage is the pattern generator. Waveform v2 matches 3,908/4,096 pixels there, maximum 1. Waveform 1 matches 4,040/4,096, maximum 2. On the signed Weave 4 case, the first 8² U16 pattern differs at (7,0) and (3,4). Both reference texels are 65,535 and both MaterialCraft texels are zero.

### High-pass, anisotropic noise and triangle-grid controls

| Root | Renders | Exact pixels / compared | Maximum error |
|---|---:|---:|---:|
| Grayscale high-pass | 18 | 171,264/171,264 | 0 |
| Anisotropic noise v2 | 18 | 171,264/171,264 | 0 |
| Triangle grid base | 18 | 171,264/171,264 | 0 |

Each root includes four archived renders, the four-format sweep, the independent 32² case and nine additional renders. Use the [modular fields and size/seed matrix](#normal-maps-panoramas-and-primitive-shapes) for the nine. For the high-pass source use the gray field H and set radius to 3, 11.25 and 24 for A, B and C. The 64² F32 output contains 212 distinct gray values.

For anisotropic noise v2 use X/Y counts (5,13), (7,19) and (3,23). Enable rotation and Y count by resolution only in B, and non-square expansion only in A and C. Set smoothness/interpolation to (.625,.5), (.375,.1875) and (1,.75). Use disorder 0, .34375 and .6875 and disorder speed .8125. The 64² F32 render contains 764 distinct gray values.

For the triangle grid bind the procedural height, vector map and color inputs. For k=1,2,3 use X count 4+k, Y count 3+k, triangulation k mod 2, vector-map displacement .0625k and rotation .09375k. Set random position multiplier .375, rotation disorder .15625 and random height .3125. The color-output selector is an integer enum. Use 1 in A/B and 0 in C. The 64² F32 render contains 40 distinct RGBA tuples. An earlier probe passed a Boolean to this selector and was rerun with the declared integer type.

### Environment rotation and wired irradiance samples

| Root | Successful renders | Exact RGBA pixels / compared | Maximum U8 / U16 levels | Maximum F16 / F32 error |
|---|---:|---:|---:|---:|
| Rotate environment | 18 | 171,264/171,264 | 0 / 0 | 0 / 0 |
| Compute irradiance | 32 | 217,088/217,088 | 0 / 0 | 0 / 0 |
| Compute ray direction | 32 | 217,088/217,088 | 0 / 0 | 0 / 0 |
| Debug hit UV | 27 | 193,960/194,304 | 239 / 57568 | 0.878417969 / 1.6449104 |

Use the [procedural normal, position and modular fields](#normal-maps-panoramas-and-primitive-shapes), the same nine image sizes, seeds and destination formats, and the same A/B/C labels. For an input named hit_uv use (u,v,H,1). For environment rotation, set amount to .0625k, direction angle to −.1875, around-pole angles before and after to .125 and .3125, and exposure to .5, 1.25 and −.375 for A, B and C. Default and archived controls are included.

The irradiance graphs expose the sample index and count as scalar value pins. For nine direct cases leave these pins disconnected. In three additional original parent graphs, connect Float Value Processors to i and nb_samples with values (0,1), (1,4) and (5,16). Wire the same procedural images through the parent. For (1,4) use the 16² default, A at 64² in U16, F16 and F32, and the 64² F32 repeat. For the other two pairs use the 16² default, A at 64² U8, B at 128², signed C at 32² and the 32×16 B input. These 15 parent renders per root preserve the child’s declared F16 storage for irradiance and ray direction. Changing the connected pair from (0,1) to (5,16) changes every pixel at 16² and 128² in both graphs. Each standalone 64² F32 irradiance and ray-direction image has 4,096 distinct RGBA tuples.

For the debug root set debug_pos to (.3125,.6875), and use index offsets 0, 1 and 2 in the direct A, B and C settings. In the three parent graphs use index offsets −1, 0 and 1 respectively. All five cases with index 0, count 1 and offset −1 render in the reference app. MaterialCraft reports a nonfinite pattern-function result on those five and their fallback images are excluded from the pixel counts. The direct 64² F32 case isolates a second difference. The gray conversion, gradient and input match. The pattern output differs at four pixels on rows 46–49. At (20,46) the reference stores (.321308792,.787873089,0,1) and MaterialCraft stores (.811764717,.811764717,.811764717,1).

### Pixel quotients and volume-mask interpolation

The volume mask matches **1,029,632/1,029,632 pixels across 116 renders**. The maximum error is zero in each format. This includes all 640,512 F32, 315,392 U16, 36,864 F16 and 36,864 U8 pixels. The [shape and axis sweeps](#volume-mask-shapes-and-axis-sweeps) give the procedural position fields, parameters and seeds.

The first difference on the zero-rotation cylinder was an interpolation whose second endpoint is a quotient. Moving the pixel function's output to each operand showed that both endpoints matched. MaterialCraft had rounded the quotient before subtracting the first endpoint. The sphere also uses a quotient minus one when its radial extents differ. Reading 17 intermediate scalars on the 64² X sweep at seed 183, the 32² Y sweep at seed 226 and the 128² Z sweep at seed 140 gives 365,568/365,568 exact F32 values in 51 observations after the correction.

| Direct pixel expressions | Renders | Exact pixels / compared | Maximum error |
|---|---:|---:|---:|
| Quotient, reciprocal, sums, differences and interpolations | 91 | 652,288/652,288 | 0 |

Write q=R/G and p=R×G. Compare q, 1/G, q+B, B+q, q−B, B−q, lerp(q,B,A), lerp(B,q,A), lerp(R,B,A), lerp(p,B,A), lerp(B,p,A), lerp(B,A,q), and the four-channel output (q,q−B,lerp(B,q,A),q). Run each expression on the [signed 128² field](#3d-linear-gradient) with F32 at seeds 127 and 541. Use its top-left 64² texels in all four formats at seed 239. At 32² use F32 and seed 313 with channel c=((37x+71y+11xy+47c+3xc) mod 1021)/256−1.125. For every input replace G with .125 wherever |G| is less than .1. Sample at texel centers with nearest filtering.

The previous cook matched 603,372/652,288 pixels with maximum F32 error 0.0000152587890625. The measured sums round once when they consume a runtime quotient. For lerp(a,b,t), a quotient or product in b can combine with the subtraction b−a. The first endpoint, a quotient used as the weight, and a quotient also returned as a separate channel preserve their individual rounding.

### Volume-mask shapes and axis sweeps

The counts in this subsection were measured before the [pixel quotient correction](#pixel-quotients-and-volume-mask-interpolation). All 116 renders now match.

The shape menu assigns 0 to cube, 1 to cylinder and 2 to sphere. Cube is the default. Some earlier case names called shape 1 a box and called the default a sphere. The corrected shape names above and below use the actual control value.

| Control | Renders | Exact pixels / compared | Maximum float error |
|---|---:|---:|---:|
| Cylinder along X | 9 | 57,111/58,368 | 9.53674316e-07 |
| Cylinder along Y | 9 | 57,165/58,368 | 9.53674316e-07 |
| Cylinder along Z | 9 | 58,368/58,368 | 0 |
| Six settings on X, Y and Z, cube | 18 | 129,024/129,024 | 0 |
| Six settings on X, Y and Z, cylinder | 18 | 128,359/129,024 | 7.15255737e-07 |
| Six settings on X, Y and Z, sphere | 18 | 111,882/129,024 | 9.53674316e-07 |

Including the earlier 35 cases, **1,008,141/1,029,632 pixels** match in 116 renders. U8, U16 and F16 match every measured pixel. F32 matches 619,021/640,512 with maximum error 3.81469727e−6. The new cylinder cases with equal X/Y radii match in both radial directions. Unequal radial extents expose a float discrepancy with rotation set to zero. The axial Z sweep matches all 58,368 pixels. All 18 cube cases match, including tilted nonuniform cubes.

Use a linear RGBA32F position input. For side N and integer pixel coordinates (x,y), set one component to (x+Ny+.5)/N². Set the other position components to .5 and alpha to one. Choose world-space positioning and change the center and extent on the swept axis only. The other center components are .5 and the other extents are one. Run the following cylinder cases on each axis with zero rotation.

| N | Center | Extent | Scale | Feather | Seed | Outputs |
|---:|---:|---:|---:|---:|---:|---|
| 128 | .5 | 1 | .5 | .5 | 41 | F32 |
| 128 | .5 | .625 | .375 | .15625 | 71 | F32 |
| 64 | .3125 | .8125 | .34375 | .375 | 131 | U8, U16, F16, F32 |
| 32 | .6875 | .3125 | .75 | .21875 | 227 | F32 |
| 64 | .5 | .75 | .5 | .25 | 431 | F32, replace the swept component by .6875+(x+64y−2048)×2⁻²¹ |
| 64 | .3125 | .8125 | .34375 | .375 | 577 | F32 |

For each of the three shapes, repeat the linear sweep on each axis using these six F32 settings. Center the swept component at .5 for N=128, .3125 for N=64 and .6875 for N=32. The rotations are in turns.

| N | Size | Scale | Feather | Rotation | Seed |
|---:|---|---:|---:|---|---:|
| 128 | (1,1,1) | .375 | .15625 | (0,0,0) | 97 |
| 128 | (.625,.625,1) | .375 | .15625 | (0,0,0) | 140 |
| 64 | (.8125,1,.625) | .34375 | .375 | (0,0,0) | 183 |
| 32 | (1,.3125,.75) | .75 | .21875 | (0,0,0) | 226 |
| 64 | (.8125,1,.625) | .34375 | .375 | (0,0,.25) | 269 |
| 32 | (1,.3125,.75) | .75 | .21875 | (.0625,−.125,.1875) | 312 |

### Distance output channels, axis wrapping and cutoffs

| Original controls | Renders | Exact pixels / compared | Maximum U8 / U16 levels | Maximum F16 / F32 error |
|---|---:|---:|---:|---:|
| Unconnected source, both output kinds and both Combine settings | 64 | 294,912/294,912 | 0 / 0 | 0 / 0 |
| Connected source with the opposite output kind | 14 | 21,504/21,504 | 0 / 0 | 0 / 0 |
| Connected source with static axis wrapping | 56 | 86,012/86,016 | 28 / 7,248 | .110534668 / .110588416 |
| Single-center cutoff, both settings of Combine | 168 | 258,048/258,048 | — | — / 0 |

Without a source, grayscale returns the distance ramp and color returns the same ramp in RGB with alpha one. Connecting a grayscale source to color output, or RGBA to grayscale output, returns zeros in every requested channel. These 46 color or conflicting-source renders previously returned the wrong channel count. For the first row use 32² tiled and 64×128 open masks. Test empty, full and single-corner masks at strength zero, a sparse mask at 79.5, and a center site at 31. Use seed 179+37j, j from zero. Run F32 for each, and also U8, U16 and F16 for the sparse field. Use the [mask and source formulas](#connected-distance-sources-and-cellular-patterns).

For the next two rows, use 32² gray and 64×32 RGBA sources. Combine is on. Use empty, full, corner and sparse F32 masks, then repeat the sparse mask as F16, U16 and U8. For the conflicting kinds retain the connected controls' strengths 31, 31, 32 and 41.375 for those four mask layouts. For static controls use a matching source, constant strength 47.25 and the four wrapping settings. Use seeds 307+19j in each group. Static tiling previously matched 77,637/86,016 pixels. The four remaining differences are the same sparse gray texel (31,18) with Y wrapping alone. Its F32 values are .113228783 in MaterialCraft and .223817199 in the reference.

For the cutoff row place a site at the center of each source size and choose radii .25, 1, 2, 3, 4, 8 and 16 pixels at seeds 509+23j. Set strength to radius×256/min(W,H). Test Euclidean, Manhattan and Chebyshev distances with both axes open or both tiled. Exactly at the radius, the source comes from the current texel. For example, at (14,16) in the 32² Manhattan case with radius 2, the gray output with Combine off is −.109375 before gray-input conversion. With Combine on the stored F32 result is negative zero. Including the boundary in the spread previously matched 257,890/258,048 pixels with maximum error 1.1875.

The [connected-source and cellular measurements](#connected-distance-sources-and-cellular-patterns) retain 184,320/184,320 exact pixels on the matching-source format matrix. The mixed metric matrix now matches 630,768/630,784, maximum F32 error .096614778. The 16 residuals are on sparse wrapped masks. The radius sweep retains 950,270/950,272, maximum .765625. Directional distance still matches 342,528/342,528 across both outputs. The two Cells 4 roots still match 19,151 and 15,949 of 173,312 pixels respectively, with their first difference at the scatter map.

### Connected distance sources and cellular patterns

A connected distance source supplies the nearest qualifying site's texel inside the radius and the current texel beyond it. With Combine enabled the gray channel or the alpha channel is attenuated. RGB remains the sampled site's color inside the radius and the current texel's color outside. The 120 gray and RGBA controls below previously matched 64,706/184,320 pixels. Maximum errors were 255 U8 levels, 65,535 U16 levels and 1.859375 in F16 and F32.

| Original distance controls | Renders | Exact pixels / compared | Maximum error |
|---|---:|---:|---:|
| Matching gray and RGBA source formats | 120 | 184,320/184,320 | 0 |
| Three metrics and four tiling settings | 168 | 630,766/630,784 | 0.096614778 |
| Empty, isolated and full masks at small distances | 54 | 202,752/202,752 | 0 |
| Sparse-site radius sweep and connected-source cutoffs | 120 | 950,270/950,272 | 0.765625 |

Use both a grayscale source and a matching RGBA source. At pixel (x,y), use channel c=((13x+7y+5xy+19c) mod 127)/64−.375, with c=0 for luminance. Repeat the nine layouts and strengths from the [32² mask controls](#directional-distance-and-mask-propagation) with both axes tiled and Combine on and off. Also run 64×32 with both axes open and Combine on, and 32×64 with both tiled and Combine off. The two centered horizontal sites are at x=W/4 and 3W/4. Run each case in F32 at seed 431+29j from j=0, and repeat the single-center and sparse cases in U8, U16 and F16.

For the metric row use 64×128 mask-only, 64×32 RGBA and 32² grayscale. Combine is on. Use both open and both tiled with Euclidean, Manhattan and Chebyshev distance, then wrap each axis alone with Euclidean distance. Use F32, seed 197+29j, and strengths 31, 0, 47.25, 159.75, 41.375, 367 and 16. The masks are empty, a site at (1,2) twice, full, sparse, and centered twice. Use (13x+23y+11xy) mod 109=0 for the standalone sparse mask and (13x+11y+7xy) mod 67=0 for the sources. Two grayscale Manhattan cutoff pixels differ only in the sign of zero. The other 16 differences occur on sparse wrapped fields. All 114,688 RGBA pixels in this matrix match. For the small-distance row use both open and both tiled, the same three sizes, Euclidean distance and strengths 0, .125 and 2 at seeds 541, 578 and 615 for each mask.

For the last row, run the sparse standalone mask at radii 1.25, 2.875, 3.875, 4, 4.25, 5.5, 6.75, 7.875, 8, 8.125, 8.6875, 9.875, 10.34375, 11.5, 12.75, 14.125, 15, 15.875, 16, 16.125, 19.75 and 23.875 pixels at seeds 283+17j. Use 128² tiled Euclidean and 64×128 open fields for all three metrics. All 901,120 mask pixels match. The connected controls use Combine off on 64×32 RGBA and 32² gray, both open and both tiled. At seeds 653+17j use an empty mask at strength 31, a corner, empty and full mask at zero, a corner at 1, a sparse mask at 413 and .25, and a center at 4096. Two RGBA texels differ on the tiled rectangular sparse field at strength 413.

The open-field results require a final unit propagation step. The measured tiled square cases retain their original winners without it. At zero distance the mask gradient is black. For a positive subpixel radius, qualifying sites are white. When the source is connected and an open field is empty, the result copies its first texel. The 32 directional-distance root renders retain all 342,528 exact pixels.

| Cellular root | Renders | Exact pixels / compared | Maximum U8 / U16 / F16 / F32 error |
|---|---:|---:|---|
| Cells 4 | 17 | 19,151/173,312 | 231 / 65462 / 0.9066162109 / 1 |
| Cells 4 v2 | 17 | 15,949/173,312 | 253 / 58223 / 0.8881835938 / 1 |

For the additional cellular controls use the eight [banded-input cases](#panoramic-line-light-and-horizon-across-the-seam). Use defaults at 16². At 64², 128² and with rectangular input, respectively, set scale to 9, 17 and 6, disorder to .21875, .75 and .4375, source selection to 1, 2 and 1 and the additional random seed to 17, 255 and 41. Disable non-square expansion only for the rectangular input. The second version also sets disorder speed to .75, 1.625 and .3125. The totals include the original defaults, changed parameters, four formats and independent 32² fields.

Read the input bridge and each reachable filter's first output. At 64² in U16 the first version's two grayscale FX-Maps each match 3,935/4,096 before Distance. Their maxima are 63,743 and 65,535 levels. At 128² the maps match 15,914 and 15,823 of 16,384, both with maximum 65,535 levels. The second version's RGBA16 FX-Map is the first difference at both sizes, with 3,941/4,096 and 15,811/16,384 exact and maximum 65,535 levels. The input bridges match. Both cellular roots remain in progress.

### Directional distance and mask propagation

Directional distance matches **342,528/342,528 pixels** in 32 renders across the value and UV outputs. The maximum error is zero in all four storage formats. The previous batch matched 342,473 pixels. Its first differing buffer was the distance primitive receiving a mask with its source disconnected.

| Direct mask controls | Renders | Exact pixels / compared | Maximum error |
|---|---:|---:|---:|
| Square and rectangular fields, sparse through full coverage | 56 | 516,096/516,096 | 0 |
| Single-site threshold | 11 | 11,264/11,264 | 0 |
| Tiled 32² mask, nine layouts and distances | 9 | 9,216/9,216 | 0 |

For the first row use 64², 128×64 and 128² with both axes tiled, and 64×128 with both axes open. In each size use the following eight F32 masks in order, with seed 167+37j from j=0. A single center site uses strength 256√2. A site at (1,2) uses 37.25. Set sites where (13x+23y+11xy) mod 109=0 and use 17.375. Put sites on column floor(W/3) and row floor(H/2) and use 23.75. Alternate .5 with .50000762939453125 on (x+2y) mod 13=0 and use 9.75. Empty and full masks use 59 and 19. Repeat the sparse mask and strength at the last seed. Also store the center and sparse cases in U8, U16 and F16. Before the correction, 289,400/516,096 pixels matched. Maximum errors were 124 U8 levels, 32,019 U16 levels, .4881744384765625 in F16 and 1 in F32.

For the threshold row set a single F32 center texel on an otherwise black 32² mask. Use strength 31 and seeds 431+29j for values −.25, 0, 2⁻¹⁶, .0625, .25, .499, .5−.5/65535, .5, .5+.5/65535, .75 and 1. The first accepted value is .5. For the last row of the table use strengths 31, 32, 41.375, 127, 16, 31, 31, 33 and 41.375 at the same seed progression. The masks are a center site, a site at (1,2), (13x+11y+7xy) mod 67=0, sites at (8,16) and (24,16), a center site, empty, full, .4375 on (x+3y) mod 13=0, and the repeated sparse mask.

The radius in pixels is strength×min(W,H)/256. Seed locations use normalized texel centers rounded to the nearest unsigned 16-bit coordinate. At each power-of-two jump, compare the sites from the eight neighbors and the center. Halve the jump through one pixel. A Euclidean search with an extra refinement disagrees at (99,0), (64,30) and (39,55) on the tiled 128² sparse case. The observed F32 values there have bits 3e86a510, 3e260066 and 3e25ccdc. Attenuation uses the [runtime square root](#runtime-pixel-square-roots) and a fused multiply-add. Connected sources have a separate color and attenuation path and remain under comparison.

For the full graph, repeat the original four renders, the [four-format and independent comparisons](#four-format-inventory-comparison) for both ports, and the [modular field matrix](#normal-maps-panoramas-and-primitive-shapes) on each port. Number the gray fields in the order angle map, distance map, input. For settings A, B and C the output selector is (1,0,1), maximum distance (.1875,.3125,.0625), angle (.125,.25,.375), distance-map multiplier (.6875,.25,1) and angle-map multiplier (.25,.625,0). Set the angle-map offset to .375. Each port matches 39,168/39,168 pixels in its nine modular-field renders.

### Panoramic line light and horizon across the seam

| Root | Renders | Exact RGBA pixels / compared | Maximum error |
|---|---:|---:|---:|
| Panorama line light | 30 | 241,152/241,152 | 0 |
| Panorama plane light | 22 | 199,155/199,936 | 4.76837158e-07 |
| Straighten panorama horizon | 26 | 212,480/212,480 | 0 |

The [pixel quotient and interpolation change](#pixel-quotients-and-volume-mask-interpolation) also resolves the line-light and horizon differences in the [original panorama matrix](#normal-maps-panoramas-and-primitive-shapes). On the same release binary each root matches all 171,264 pixels in those 18 renders. In the eight banded-input renders below, the preceding binary matched 41,188/41,216 line-light pixels and 41,172/41,216 horizon pixels. The maximum errors were 2.56299973e−6 and 3.05175781e−5 respectively. Both now match all 41,216.

Number the input pins j in identifier order. At integer texel (x,y) set B=((floor(x/3)+2floor(y/5)+j) mod 5)/4. Gray inputs use B. For color use (u,v,B,A), with normalized texel centers (u,v). Set A=.875 where x<W/8 or x>W−W/8 and .375 elsewhere. Run 16² F32 at seed 71, all four formats at 64²/193, 128² F32/317, 64×32 input on a 64² F32 canvas/449, and repeat 64² F32/503.

For the eight line-light renders set the UV endpoints to (.9375,.40625) and (.0625,.71875), camera to (.125,−.0625,.1875), line rotation to −.15625 and thickness to .15625. Use color (.8125,.375,.1875), temperature 3975 and exposure .75. For 16², 64², 128² and the rectangular input, set both pattern and color mode to 0,1,2,0 respectively. Use distance-from-origin mode at 128², ground/ceiling for the other sizes, and up-vector mode 1 only for the rectangular input. For the horizon use defaults at 16² and horizon points (.9375,.34375) and (.0625,.71875) for the other cases. Use rotation .0625 at 128² and −.1875 elsewhere. Disable horizon correction only for the rectangular input.

For four further line- and plane-light renders use F32. At 64² select world, distance-from-origin and ground/ceiling positioning with seeds 239, 310 and 381 respectively. At 128² use world positioning with seed 452. Set color mode 2 and pattern 4 at 64², pattern 1 at 128². For the line use the 64² and 128² banded inputs above, with world endpoints (−.875,.4375,1.25) and (.5,−.3125,1.125). For the plane use the 64² A and 128² B [plane-light fields](#plane-light-splatter-symmetry-and-waveform-comparisons) with world position (−.75,.4375,1.25). All 28,672 line-light pixels match. On the plane, the four outputs match 4,084/4,096, 4,049/4,096, 4,079/4,096 and 16,247/16,384. Their first three maximum errors are 5.96046448e−8 and the last is 1.19209290e−7. The plane remains in progress.

### Normal maps, panoramas and primitive shapes

| Root | Renders | Exact pixels / compared | Maximum U8 / U16 levels | Maximum F16 / F32 error |
|---|---:|---:|---:|---:|
| Normalize normal map | 18 | 171,264/171,264 | 0 / 0 | 0 / 0 |
| Transform normal map | 18 | 171,264/171,264 | 0 / 0 | 0 / 0 |
| Panoramic 3D position | 18 | 171,264/171,264 | 0 / 0 | 0 / 0 |
| Panorama line light | 18 | 171,207/171,264 | 0 / 0 | 2.38418579e-07 / 7.86781311e-06 |
| Panorama shape | 18 | 171,264/171,264 | 0 / 0 | 0 / 0 |
| Panorama shape 2 | 18 | 171,264/171,264 | 0 / 0 | 0 / 0 |
| Straighten panorama horizon | 18 | 171,231/171,264 | 1 / 204 | 0.0029296875 / 0.00311285257 |
| Cube | 18 | 171,264/171,264 | 0 / 0 | 0 / 0 |
| Cube position, normal, depth and UV | 60 | 685,056/685,056 | 0 / 0 | 0 / 0 |
| Hemisphere | 18 | 171,264/171,264 | 0 / 0 | 0 / 0 |

The totals include the original defaults, original changed settings, four requested output formats, the independent 32² screen and nine additional renders per output. For the new fields let u and v be texel-center coordinates. In each input, number pins j in identifier order and channels c from zero. Use H=((37x+41y+7xy+53j) mod 509)/508. For ordinary color inputs add 61c+5xc to the numerator before the modulus. Gray inputs use H. For normal inputs use (.5+.5a,.5+.5b,.5+.5√(1−a²−b²),.25+.75H), with a=.625(2u−1) and b=.625(2v−1). The transform's normal input uses j=−1. Position inputs use (u,v,.25+.5H,1).

Use F32 inputs and the following output settings. Use defaults at 16²/seed 43. Use A at 64²/157 in U8, U16, F16 and F32. Use B at 128² F32/293 and with 32×16 inputs at 32² F32/599. Use C at 32² F32/463 with every input value v replaced by 2.5v−.75. Repeat A at 64² F32/751. Generators use the same sizes and seeds. Let k equal 1, 2 and 3 for A, B and C.

| Control | A, B and C |
|---|---|
| Normalization | No exposed numeric control. |
| Normal transform | Matrix (.8125,.34375,−.1875,1.125) for A/C and (0,−.75,1.25,0) for B. Offset (.1875,−.3125) for A/B and (−.5,.0625) for C. Normal format 1 only in B. Disable scale/skew compensation only in C. |
| Panoramic position and line light | Up vector 1 only in B. For the light, position mode 1 only in B, grid enabled only in C, UV endpoints (.1875,.25) and (.71875,.4375), height .625, distance 1.375, color mode k mod 3, temperature 4375 and exposure 1.25. |
| Panorama shape | Same matrices and offsets as the normal transform. Shape k mod 3, color (.75,.34375,.625), intensity 1.375, soft borders .1875, hotspot size .375 at (.25,.6875). Disable background only in C and texture only in B. |
| Panorama shape 2 | Same matrices, offsets, shape and color. Use color mode k mod 3, temperature 4375, exposure 1.25 and hardness .6875. Use image gamma 1 only in B and disable background only in C. |
| Straighten horizon | Points (.1875,.34375) and (.71875,.59375). Horizontal rotation .0625 in B and −.1875 in A/C. Disable horizon correction only in C. |
| Cube and four cube buffers | Orientation (.0625k,−.1875), size (.625,.875,1.125). Scale .34375 in B and .5 in A/C. Disable non-square expansion only in C. |
| Hemisphere | Scale .625 in B and .875 in A/C, size (.6875,1.125), rotation .09375k. Disable non-square expansion only in C. |

The four 64² F32 cube buffers contain 3,187 distinct position tuples, four normals, 2,969 depths and 3,187 UV tuples. The transformed normal map contains 4,094 distinct tuples. All four stored components count toward an RGBA pixel match.

The line-light differences begin in its pixel processor. On the signed field, pixel (6,2) stores 1.98653376 in the three color channels in the reference and 1.98652911 in MaterialCraft. For the horizon, the input matches. Its pixel processor differs at two texels in the 64² A case. Pixel (39,49) has reference red .756740689 and MaterialCraft red .753627837. Both discrepancies were resolved in the [panorama seam comparisons](#panoramic-line-light-and-horizon-across-the-seam).

### Tiled diagonal directional blur

A centered shear moves a periodic square image onto an axis-aligned sample grid. The blur reduces full interior texel runs at the mip level for their width, with separate fractional endpoint reads. An inverse shear samples the stored result back on the output grid. Both shears filter at the precision of their input. The integer comparisons use U16 intermediates.

Across 96 original diagonal renders, exact RGBA pixels improved from 35,337 to 1,046,248 out of 1,351,680.

| Output | Renders | Previous exact | Current exact / compared | Previous maximum | Current maximum |
|---|---:|---:|---:|---:|---:|
| RGBA8 | 16 | 4,953 | 117,809/163,840 | 200 | 1 |
| RGBA16 | 16 | 239 | 97,055/163,840 | 51479 | 1 |
| RGBA16F | 16 | 12,188 | 163,827/163,840 | 0.811889648 | 0.00048828125 |
| RGBA32F | 48 | 17,957 | 667,557/860,160 | 0.867096573 | 3.57627869e-07 |

Use a center impulse in R, a vertical stripe through the center in G and a horizontal stripe in B. For U8 and U16 set alpha to H=((97x+43y+7xy) mod 1024)/1023 and round the four channels to the nearest stored integer. Floating inputs use 3H−1.25 for alpha. In order, use turns .125, −.125, .375, .625 and .875 on F32, then .125 on U8, U16 and F16. For each, take (radius, square side) pairs (2,32), (2.375,128), (3.125,64), (5.375,128), (11.25,64), (7.5,256). Set intensity to 256×radius/side and seed to 479+37j+19k for zero-based angle/input index j and radius index k. Use all four output formats at radii 3.125 and 5.375, and F32 elsewhere.

At .125 and −.125 turns with F32 input and F16 output, all 40,960 pixels match. Differences in the other diagonal renders include fractional channel rounding and the projection of angles beyond the first quadrant. The untiled .125, tiled .0625, and tiled .21875 controls retain larger differences. Across these 36 renders the maxima are 192 U8 levels, 49,199 U16 levels, 1.095703125 in F16 and 1.09612465 in F32.

| Inventory root | Renders | Exact / compared | Maximum U8 / U16 | Maximum F16 / F32 |
|---|---:|---:|---:|---:|
| `anisotropic_blur.sbs/anisotropic_blur` | 8 | 112,190/115,712 | 1 / 0 | 0 / 0 |
| `anisotropic_blur.sbs/anisotropic_blur_grayscale` | 8 | 112,563/115,712 | 1 / 0 | 0 / 0 |
| `blur_hq.sbs/blur_hq` | 9 | 40,496/132,096 | 8 / 0 | 0 / 0 |
| `blur_hq.sbs/blur_hq_grayscale` | 9 | 96,534/132,096 | 4 / 0 | 0 / 0 |

The [four-format inventory](#four-format-inventory-comparison) gives the changed inputs and settings. Each of the four roots matches on its independent F32 input and its changed 64² four-format matrix. The differences above are in archived U8 cases. Six previously exact roots with reachable directional filters also matched all 790,528 pixels in 52 renders, maximum zero.

### Short oblique directional blur

| Direct footprint | Renders | Exact pixels before | Exact pixels after / compared | Maximum error after |
|---|---:|---:|---:|---:|
| 128² impulse and stripes | 10 | 27,936 | 163,840/163,840 | 0 |
| Rectangle, U8 output | 10 | 3,535 | 81,920/81,920 | 0 |
| Rectangle, U16 output | 10 | 1,113 | 81,906/81,920 | 1 |
| Rectangle, F16 output | 10 | 0 | 81,918/81,920 | 7.62939453e-06 |
| Rectangle, F32 output | 50 | 0 | 292,389/409,600 | 3.57627869e-07 |

A projected reach of at most one texel uses three taps. Multiply radius−.5 by the larger absolute sine or cosine of the angle. Divide each direction component by its image dimension, then divide both by the larger resulting magnitude to obtain the texel offsets. The first tap points toward positive coordinates on the original dominant axis. Filter the two outer taps at source precision. With reach d and center weight m=.5/(.5+d), weight each side by d×m. Multiply the first tap by its weight, accumulate the center with a fused multiply-add, then accumulate the last tap the same way.

Use the [128² impulse, stripes, signed alpha and radius sequence](#quarter-turn-directional-blur) at .125 and .0625 turns. The first five radii at both angles now match exactly. The fourteen wider renders match 4,832/229,376 RGBA32F pixels, maximum .750733346. On the 108 straight-angle controls all 1,769,472 pixels still match.

For rectangles use H=((37x+71y+11xy+47c+3xc) mod 1021)/1020 in channel c. Use 3H−1.125 for F32 inputs and round(255H) for U8. In order, the angles are .03125, .09375, .15625, .21875, −.0625, .3125, .375, .625, −.137 and .913 turns. Use 64×128 for indices 0, 2, 5, 7 and 9 and 128×64 elsewhere. Use U8 source at indices 3 and 6, F32 otherwise. Disable both tile axes at indices 2, 4 and 9. Use radii .5625, 1, 1.32, 1.5, 1.5625, 1.875, 2.25, 5.375 and 12.25, with intensity four times the radius and seeds 187+19j by zero-based radius index. Use F32 output for every radius and add U8, U16 and F16 at radii 1.32 and 5.375. The table covers only the 80 short cases. The other 70 rectangle renders match 61/573,440 pixels, with maximum errors of 192 U8 levels, 49,199 U16 levels, 1.056640625 F16 and 1.06214854 F32.

| Root | Renders | Exact pixels / compared | Maximum U8 / U16 levels | Maximum F16 / F32 error |
|---|---:|---:|---:|---:|
| Color anisotropic blur | 24 | 226,638/269,568 | 16 / 0 | 0 / 0.0235294104 |
| Grayscale anisotropic blur | 24 | 232,976/269,568 | 16 / 0 | 0 / 0.0196078718 |
| Color high-quality blur | 18 | 63,731/171,264 | 8 / 0 | 0 / 0.00784316659 |
| Grayscale high-quality blur | 18 | 125,647/171,264 | 4 / 0 | 0 / 0.00784316659 |

For the nine added root renders use the [modular fields and size/seed matrix](#normal-maps-panoramas-and-primitive-shapes). Set intensity to 5.25, 18.5 and 10.25 in A/B/C. Set quality to 1, 0 and 1. For anisotropic blur set anisotropy to .6875, .3125 and 1 and angle to .0625, .15625 and −.21875 turns. The 128² case differs in all four graphs. The direct wider impulse and rectangle probes reproduce the outstanding error in a single directional primitive.

### Quarter-turn directional blur

Anisotropic noise matches **171,264/171,264 grayscale pixels in 18 renders**, maximum error zero. The [noise settings and seeds](#color-conversion-gradient-height-and-histogram-controls) include the rotated 128² and 32² cases. At 128² the directional stage improved from 15,226/16,384 to 16,384/16,384. Every sampled stage matches on the 64², 128² and rectangular-input controls. The final outputs comprise 122,880 U16 pixels, 32,000 F32 pixels and 8,192 each U8 and F16.

A single directional filter on an original 128² linear RGBA32F field matches **1,769,472/1,769,472 pixels in 108 renders**. Use an impulse at (64,64) in R, its column in G and its row in B. Use 3H−1.25 for alpha, where H=((97x+43y+7xy) mod 1024)/1023. Test turns 0, .25, .5, .75, 1, −.75, −.25, −.5 and 1.25. For each angle use radii .5625, .75, 1, 1.25, 1.5, 2, 2.375, 2.5, 3.125, 4.375, 5 and 8.25, with seed 113+17j at zero-based radius index j. At this size set intensity to twice the radius.

For a nonzero whole-quarter turn, project radius−.5 by the F32 factor immediately below one before calculating weights. With projected reach d, the center weight is .5/(.5+d). The short-window side weight is d times that weight. For a wider window retain d−(ceil(d)−1) for the partial outer texel. At radius 1, the quarter-turn impulse stores F32 bits 3e7fffff one row above center. At radii 2 and 5, its central texel stores 3e800001 and 3dccccce. At zero turns the same three readings along the horizontal axis are 3e800000, 3e800000 and 3dcccccd.

Forty additional 64×128 quarter-turn renders on the original modular RGBA8 field, with both axes wrapped or both clamped, match **327,680/327,680 pixels**. For channel c round 255 times ((37x+71y+11xy+47c+3xc) mod 1021)/1020 to the nearest byte, with ties to even. Use radii .75, 1.25, 1.5, the next F32 above 1.5, 2, 2.5, the next F32 above 2.5, 3.125, 4.375, 9.5 and 15.5625. Multiply the radius by four for intensity. Use seed 97+23j and F32 output. At radii 1.25, 2 and 4.375 also use U8, U16 and F16. Each of those three formats contributes 49,152 pixels.

Before the [short oblique filter correction](#short-oblique-directional-blur), the same 128² impulse and stripes also differed at .125 and .0625 turns. At .125 turns, 32,768/196,608 pixels match, maximum .750733346. At .0625 turns, 0/196,608 match, maximum .433206394. That earlier measurement included both short and wide kernels.

### Alpha maximum and one-texel reductions

The alpha-maximum helper matches **36/36 RGBA output pixels in 33 renders**, maximum error zero. Its output has one texel for inputs through 256² and four for a 512² input. The 128² position and alpha field previously selected a different tuple with maximum component error .968627451. Reading the whole reduction chain on this input showed that the final processor received an exact 2×2 image. At requested exponent −1 it samples the lower-right texel. The earlier cook treated the one-texel allocation as logical exponent zero.

For the 128² field at seed 347, take u and v at texel centers and H=((59x+41y+5xy) mod 1021)/1020. Store (u,v,H,.125+.625H) as RGBA32F, replacing alpha at (3,125) by .9375. For fourteen further renders, use original RGBA8 fields with channels x mod 253, y mod 251, (31x+29y) mod 251 and (47x+59y+13xy) mod 191. Set a marked texel in quadrant j to (29+53j,211−47j,37+59j,255). Number quadrants in row-major order. Place the mark one eighth or seven eighths of the way across each axis. Use a 16² field at seed 73, four 128² fields at seeds 137, 173, 277 and 373, and a 64² southwest field in all four formats at seed 433. Also test 256² northeast/577, 512² southeast/661, 128×64 northwest/739 at a 128² parent, 128² with opposing northwest and southeast marks/809, and 128² southwest/953. These cases improved from 11/17 to 17/17 output pixels. The 128² northwest field returns RGBA8 levels (75,69,59,190) expressed as F32. The two-mark field returns (188,70,214,255).

| Direct controls | Renders | Exact pixels or typed values / compared | Maximum error |
|---|---:|---:|---:|
| Pixel size, logarithm and sample displacement | 72 | 3,333/3,333 | 0 |
| Pixel canvas, absolute, parent and first input | 12 | 86,420/86,420 | 0 |
| Filter size relative to a one-texel input | 16 | 1,088/1,088 | 0 |
| Uniform color function | 8 | 148/148 | 0 |
| Stamp opacity | 24 | 444/444 | 0 |
| Typed value function | 24 | 24/24 | 0 |

At parent sides 16, 32, 64, 128, 256 and 512, request offsets (−8,−8), (−5,−5), (−8,−2) and (−2,−8), seed 127. Return the two size components and the two pixel-center coordinates in RGBA32F. Repeat with the base-two size logarithm. For the third output, sample a linear F32 image at the pixel center plus (.25,.25) divided by the reported size. Set the source channels to (x/N,y/N,((17x+23y+9xy) mod 251)/250,1). The raster clamps each axis to one texel while the logarithm keeps the requested negative exponent. For exponents −1, −2, −3 and −4 the reported size is −2³¹, 2³⁰, 2²⁹ and 2²⁸. The position still uses physical texel centers.

The propagation controls use parents of 16, 64, 128 and 256 at seed 131. Follow the (−8,−2) processor with a transform whose width offset relative to input is −1, 0, +1 or +3. For example, at parent 128 and offset +3 the transform is four texels wide. Uniform color encodes the two logarithms as .5 + log₂(size)/16 and the two reported sizes as .5 + size/2³². Test relative (−8,−2) and absolute (−2,3). A separate diagnostic that assigns a variable inside the pixel-ratio function, then reads it in another parameter, matches 64/296 pixels in sixteen renders. The reference returns zero for that read. This side effect remains open.

### Material bundles and the eighth irradiance pass

| Root | Recorded renders | Exact output pixels / compared | Maximum stored error |
|---|---:|---:|---:|
| Material transforms, all five outputs | 74 | 856,320/856,320 | 0 |
| Two-material blender, three outputs | 46 | 513,792/513,792 | 0 |
| Four-material blender, three outputs | 46 | 513,792/513,792 | 0 |
| Eight-material blender, three outputs | 46 | 513,792/513,792 | 0 |
| Raytraced irradiance IS pass 8 | 18 | 136,704/136,704 | 0 |

The material roots use four archived settings, followed by a four-format and independent render of each output. Add nine renders per output using the procedural channels H and input-port ordering from the [color and histogram controls](#color-conversion-gradient-height-and-histogram-controls). Use default controls at 16²/seed 47, A in all formats at 64²/151, B at 128²/307, C on signed 32² input/467, B on 32×16 input at 32²/557, and A at 64²/719. The five transform outputs cover diffuse, specular, height, opacity and emissive. Use scale/rotation/offset (75,137.5)/.125/(.1875,−.3125) for A, (125,50)/.25/(−.125,.4375) for B and (62.5,150)/−.1875/(.5,.0625) for C.

For the blenders, let m number materials from one and k be 1, 2 or 3 for A, B or C. Set each material transform to (.25+.0625m, .0625(m+k), .25+.03125m, .6875−.03125m). Set the height range to (.0625m,.9375−.03125m) and specular range to (.03125m,.875−.015625m). The mask index is (floor(x/4)+3floor(y/5)) mod 8. The grayscale mask uses byte level (37×index) mod 256. The color mask uses its three low bits as red, green and blue, each either 0 or 255, with alpha 255. Compare all three composited outputs.

The eighth irradiance pass previously timed out. Finishing the original 128² default, seed 42 and changed settings plus the 256² default gave 114,688/114,688 RGBA16F pixels. These archive measurements used the signed-product implementation. The four 64² storage formats, the independent 32² case and nine additional cases matched 22,016/22,016 pixels both before and after the wide-blur and function-call changes. The combined storage totals are 119,040 RGBA16F, 8,960 RGBA32F and 4,352 each RGBA8 and RGBA16.

For those nine new cases, use u=(x+.5)/N, v=(y+.5)/N and H=((53x+31y+9xy+43) mod 257)/256. Write original F32 channels for position (u,v,H,1), normal (.5+.3(u−.5),.5+.3(v−.5),1,1), environment (.25+u,2v,H²,1), previous (v,H,u,.125+.75u), emissive (.75u,.375H,.5v,1), sample table (u,v,H,1−H), noise (H,H,H,1) and height (v,v,v,1). Use defaults at 16²/seed 37. For the rest set shape scale .8125, UV tiling 1.5 and displacement .125. Run shape 0/pass 1 at 32²/173 and repeat at seed 439. Repeat seed 173 with previous=(.5H,.25v,1−u,1). This changes every one of the 1,024 output pixels and both images match exactly. At 16² use shape 1/pass 3/seed 359 with black environment and emissive, and shape 2/pass 2/seed 251 in all four output formats.

### Literal pixel divisors and RGB to Lab

| Comparison | Renders | Exact stored pixels / compared | Maximum error |
|---|---:|---:|---:|
| RGB to Lab | 29 | 212,480/212,480 | 0 |
| Original quotient and contraction controls | 252 | 741,888/741,888 | 0 |

The RGB-to-Lab root matches 126,976 RGBA16, 12,288 RGBA8, 14,336 RGBA16F and 58,880 RGBA32F pixels. Before the change the root matched 152,444/212,480. All 58,880 full-float pixels had a differing component, at most 1.19209290e−7. The archived defaults, changed parameters and four-format cases are under [color conversion and related controls](#color-conversion-gradient-height-and-histogram-controls).

For each of the two Keep Alpha settings, run a 16² grid (u,v,(x XOR y)/15,1) at seed 31 with u=x/15 and v=y/15. In an original linear RGBA32F field, define Hⱼ=(((31+16j)x+(47−4j)y+(7+6j)xy+101j) mod 1021)/1020. Use H at 64²/163 in four output formats and repeat F32 at seed 587. Use 3H−1.125 at 32²/281 in F16 and F32. At 128²/373 use R,G,B=.04045+(((x+7jy) mod 67)−33)/65536 for j=1,2,3, with alpha H₃. Also pass 16×32 pixels at 32²/499 in F32. The input bridge stores RGBA8 in the measured reference cases.

A runtime numerator divided by a literal denominator uses the float32 reciprocal of that literal. A sampled denominator of the same value uses the [runtime pixel reciprocal](#runtime-reciprocal-and-3d-gradient-contrast). For 255 the two multipliers have bits 3b808081 and 3b808080. For numerator c34928a6, the quotients have bits bf49f299 and bf49f298. Adding offset befff6a8 through a single-use quotient gives bfa4f6f7 and bfa4f6f6. A separately consumed quotient rounds before addition. The regression covers a folded 510/2 denominator as well.

For direct controls, sample R,G,B,A from a linear RGBA32F field. For each of ten denominators (255,100,200,500,.3999999761581421,−3.625,10,1,7,3), put that value in G. Emit R divided by the literal, R/G, the literal quotient plus B, or (200R+127.5) divided by the literal. For 255 add a folded denominator and a graph returning both a quotient and a sum using it. Use 16²/61 and 32²/367 F32 plus 64²/211 in four formats. Start with the upper-left crop of the original signed 128² [four-channel hash field](#3d-linear-gradient), multiply R by 64, and at 32² replace R with ((37x+53y+5xy) mod 2047)/16−19.875. Set the first two pixels to (127.5,G,0,1) and (.375,G,.5,1). All 225,792 F32 pixels in 126 renders match bit for bit, as do the 172,032 pixels in each of U8, U16 and F16.

### Vector equality, Lab to RGB and SBS round trips

| Comparison | Renders | Exact stored pixels / compared | Maximum error |
|---|---:|---:|---:|
| MaterialCraft Lab-to-RGB root | 25 | 180,736/180,736 | 0 |
| Six independent comparisons using the edited functions | 36 per application | 50,688/50,688 per application | 0 |
| Same comparisons using the reference app’s saved and reopened copy | 36 per application | 50,688/50,688 per application | 0 |
| Reference app conversion with the edited file, then with its saved copy | 25 per file | 180,736/180,736 per file | 0 |
| Reference app comparisons after MaterialCraft Save As | 36 | 50,688/50,688 | 0 |
| Reference app conversion after MaterialCraft Save As | 25 | 180,736/180,736 | 0 |

The RGB and RGBA equality functions compared red twice when reading the first two lanes from each operand. Four selectors in the existing function library now read red and green. The Lab-to-RGB root bypasses conversion only for fully transparent black. A green-only texel with zero alpha previously took that bypass. The old root matched 180,157/180,736 pixels. The largest errors were 255 in U8, 65,535 in U16 and 1 in F16 and F32. The corrected root matches all 123,904 U8, 9,216 U16, 10,240 F16 and 37,376 F32 pixels.

Use the archive, four-format inputs and independent field under [color conversion and related controls](#color-conversion-gradient-height-and-histogram-controls). For ten new renders, use the modular fields, sizes, formats and seeds in [RGB to Lab](#literal-pixel-divisors-and-rgb-to-lab). At 128² set L to (0,.04,.08,.08000001,.07999999,.5,1) indexed by x mod 7, adding 1/8192 where y mod 4 equals 1. Set the two chroma channels to .5+((x mod 16)−8)/128 and .5+((y mod 16)−8)/128.

For six green-channel and transparency cases set mask=(x+3y) mod 16 and z=1+((37x+11y+7xy) mod 255). Byte channel j is 1+((z+43j−1) mod 255) when bit j is set, zero otherwise. Run 16²/seed 73 F32, 32²/193 in four formats and 64²/379 F32. For direct function tests compare the first two, three or four lanes against (0,0,0,0) or (.25,−.375,1,2). Add the same byte field divided by 256 to the chosen offset and output one for equal, zero otherwise. The regression enumerates all eight three-lane masks and all sixteen four-lane masks. With the previous file, the three-lane comparisons matched 7,392/8,448 pixels for each offset and the four-lane comparisons matched 7,920/8,448.

For the file check, open a byte-identical copy of the edited functions package in reference app 16.0.7, save a copy there, then load the saved file. Both RGB first-pair selectors read (0,1) before and after saving. The RGBA function retained both (0,1) selectors and both (2,3) selectors. Run the direct comparisons above using a relative dependency on each file. For the native conversion check, nearest-sample the byte RGBA bridge, compare all four lanes to zero with the selected file, and otherwise combine the public Lab conversion of RGB with the original alpha. Run the same 25 render cases. Substituting the pre-fix file in that graph reproduces the original 180,157/180,736 count.

For the MaterialCraft-to-reference handoff, open the edited function library in MaterialCraft and use Save As to write a separate SBS file. Open that saved file in reference app 16.0.7. It exposes 201 resources. Both three-lane selectors and the first pair from each four-lane operand read (0,1). The second pairs read (2,3). Set the equality and conversion graphs’ relative dependency to that specific Save As file and repeat all 61 renders above. Both comparisons are pixel-exact. Repeating the same Save As and reopen with the pre-fix library gives 180,157/180,736 conversion pixels, maximum 65,535 U16 levels.

The separate complete color-quantization package did not open in the reference app with either function library, including when supplied with an original procedural lookup image. It also did not open after saving from MaterialCraft with its existing 0.1.3 format header. A separate original constant graph opens with a current header and fails to open with 0.1.3. Changing only the complete package’s header still left its import unresolved. The round-trip results in the table cover the edited function file and the measured consumers.

### Color conversion, gradient, height and histogram controls

| Root or output group | Recorded renders | Exact pixels / compared | Maximum stored error |
|---|---:|---:|---|
| sRGB to ACEScg | 18 | 171,264/171,264 | 0 |
| Linear gradient 3 | 18 | 161,419/171,264 | U16 2, F32 3.05175781e−5 |
| Radial gradient | 18 | 171,264/171,264 | 0 |
| Height blend, both outputs | 32 | 342,528/342,528 | 0 |
| Histogram scan | 18 | 171,264/171,264 | 0 |
| Histogram and cumulative distribution | 32 | 4,608/4,608 | 0 |
| Mean of least variance, color iteration | 18 | 171,264/171,264 | 0 |
| Mean of least variance, grayscale iteration | 18 | 171,264/171,264 | 0 |
| Color median filter | 18 | 171,264/171,264 | 0 |
| Anisotropic noise | 18 | 167,393/171,264 | U16 17, F32 .000274673104 |

Each single-output root includes four archived renders, four 64² storage formats, an independent 32² render and nine additional settings. For the two-output roots each archive render captures both outputs, then the format and additional cases select one output at a time. The median filter also matched 131,072/131,072 pixels in a separate repeat of the four original settings and the four storage formats. Those repeats are excluded from the table. The [earlier input and format matrix](#four-format-inventory-comparison) identifies the original fields.

For the nine additional settings, number pins j in identifier order and channels c from zero. Define H=((47x+67y+13xy+29c+11cx+71j+3jy) mod 1021)/1020. Use rounded 16-bit PNG levels, taking c=0 for grayscale. At 16² use default controls, seed 37 and F32 output. At 64² use setting A, seed 173 and U8/U16/F16/F32. At 128² use B/307 and F32. At 32² use C/439 with 2.5H−.75 in F32. Also run B on 32×16 input at 32²/571 and repeat A in F32 at seed 701. Generators render at the stated output size.

| Root | A | B | C |
|---|---|---|---|
| Linear gradient 3, tiling/position/rotation | 3/.3125/1 | 7/.8125/2 | 2/.4375/3 |
| Radial gradient, shape/non-square expansion | 1/off | 2/on | 3/off |
| Height blend, position/contrast/mode/opacity | .3125/.6875/1/.8125 | .625/1/0/.4375 | .1875/.375/1/.75 |
| Histogram scan, position/contrast/invert | .3125/.6875/off | .625/1/on | .4375/.375/off |
| Histogram resolution exponent | 6 | 5 | 7 |
| Least variance, intensity/smoothness/criterion/Gaussian | 10/.125/1/off | 16/.75/0/on | 24/.5/1/off |
| Median, kernel/filter/affect alpha | 1/1/off | 0/0/on | 1/1/off |
| Anisotropic noise, X/Y counts, smoothness/interpolation | 5/13, .625/.5 | 7/19, .375/.1875 | 3/23, 1/.75 |

For the radial gradient set points to (.28125,.40625) and (.75,.84375). The color least-variance control affects alpha only in B. For anisotropic noise enable rotation and Y amount by resolution only in B, and disable non-square expansion only in B. The color conversion has no exposed numeric control. At 64² F32 its output has 4,038 distinct RGBA tuples. The two height outputs have 3,210 distinct heights and 2,324 mask values. The 64-bin histogram has counts from 7 to 85 and its cumulative distribution has 64 distinct values.

For linear gradient 3, the first differing stage is its grayscale 16-bit FX-Map. At 64² A the stamp matches 3,840/4,096 texels, maximum one level. At 128² B it matches 16,249/16,384, also by one level. The root's U8 outputs match 8,192/8,192. F16 matches 7,936/8,192, maximum .00048828125. In anisotropic noise the upstream field matches. At B the first difference is the directional blur, 15,226/16,384 at 128², maximum 18 levels before the final output. The 32² B blur misses one pixel by one level. The root's U8 outputs match 8,164/8,192, maximum one level, and F16 matches 7,796/8,192, maximum .00048828125.

### Wired spline helpers and shape mapping

| Root | Renders | Exact pixels / compared | Exact integers / compared | Maximum integer-image / F32 error |
|---|---:|---:|---:|---:|
| Shape mapper | 29 | 310,784/310,784 | — | 0 / 0 |
| 2D spline transform | 64 | 87,552/87,552 | 18/18 | 0 / 0 |
| Bridge distance | 33 | 54,528/54,528 | — | 0 / 0 |
| Initial spline length | 27 | 43,776/43,776 | — | 0 / 0 |
| Spline tangents | 27 | 43,296/43,776 | — | 63578 / 1.95677358 |

The spline cases add three original coordinate and tangent buffers, with counts 1, 3 and 2 supplied through Integer value processors wherever the graph accepts a count. At 16² (circle) or 32×16 (line and signed curve), populate rows 0 to 2 and leave later rows zero. Let θ=2π(x+.5)/width. The circle and signed coordinates are (.5+(.25−y/32)cos θ, .375+(.1875+y/64)sin θ+y/16, 1, −1), with tangent (−sin θ,cos θ,0,1). For the signed curve, transform its x component to 2.5x−.875 and negate its y component in row 1. For the line use D=(x+.5)/width−.5 and slope S=.5 in the first half, −.25 in the second. Its coordinates are (.5+D,.375+S×D+y/16,1,−1), tangent (1,S,0,1).

The spline transform uses matrix (1.125,−.25,.375,.75) on the circle, and (0,−1,1,0) on the other buffers. Set offset (.1875,−.3125) and switch on reverse direction for both curved inputs. For distance and length, feed parent size (2,1) with non-square correction enabled on curved buffers, or (.75,1.5) with correction disabled on the line. Use tangent mode 1 on curved inputs and 0 on the line. Render 16² F32/seed 89, 32²/197 with four requested formats and 64² F32/439. For the integer port use one render per size. Its nine wired results are three 1s, three 3s and three 2s. The transformed 32² line contains 97 different stored coordinate tuples.

The transform's wired image ports and bridge distance retain F32 storage inside their wrapper. Their direct four-format runs verify U8, U16, F16 and F32. The six earlier [wired bridge distance cases](#height-distance-texture-mapping-skew-and-weave) now match all 10,752 pixels. With three splines the tangent helper matches 7,971/8,448 on the line, including 955/1,024 in F32 at 32². Its 64² F32 line reaches error 1.95677358. Both curved inputs match 8,448/8,448 on this helper.

For shape mapper, the five earlier comparison sizes and four-format inputs are listed in the [inventory comparison](#four-format-inventory-comparison). Add an F32 coordinate grid with R=x and an alternating-column pattern x mod 2 at 128², plus coordinate fields at 16², 256², 64×32 into 64² and a 128² repeat. At 64² store .1875x−2.125 in all four output formats. The seeds for 16², 128² grid, 128² binary, 256², signed 64², rectangular input and the repeat are 89, 131, 233, 331, 421, 509 and 601. Use defaults at 16² and otherwise radius .3125, width .6875, seven patterns, five segments, rotation .1875 and alternate flipping. Use shape 0 for the rectangular case and 1 elsewhere, nearest filtering for the binary case.

For ten further shape mapper renders use H=(((59x+41y+5xy) mod 1021)/1020+u+2v)/4, where u,v are source texel centers. Use defaults at 16²/83, four formats at 64²/167, F32 at 128²/347, a 32×16 input at 32²/499, signed 32²/631 with 3H−1.375, 64²/787 and 64²/827. Set radius .375, width .71875, rotation .09375, nine patterns and seven segments. Use shape 0 and disable non-square expansion for the rectangular input. Use shape 1 elsewhere. Link segments and pattern count for the signed input. Disable alternate flipping and select nearest filtering at seed 787. Together the ten smooth-field renders match all 43,264 stored pixels.

### Multiply blending and pre/post composition

The color pre/post composite matches **166,400/166,400 RGBA pixels in 17 renders**, maximum error zero. The two Levels inputs to the final blend matched on all 4,096 pixels in the changed 64² case before the fix. The multiply blend itself previously matched 2,514/4,096 in the additional 64² F32 field. The complete matrix improved from 162,255/166,400. The final counts are 122,880 U8, 8,192 U16, 8,192 F16 and 27,136 F32.

Use the four archive cases, changed 64²/42 with opacity .37 and mode 3 in all four formats, and the independent 32² case. For eight further cases, take the [composite matte field](#unbound-property-links-and-composite-matte) Hⱼ. Set foreground to (H₁,H₂,H₃,(x mod 5)/4) and background to (H₄,H₅,H₆,(y mod 7)/6). Test 16² opaque/default, 16² with opacity .375 and mode 0, 64² in all four formats with .6875 and mode 3, 32² opaque with 1 and mode 0, then 128² with .3125 and mode 1. Use seeds 103,164,225,286,347 respectively.

Independent multiply controls match **2,990,080/2,990,080 pixels in 130 renders**, including the three signed-zero fields below. The measured source-alpha, ignore-alpha and premultiplied paths preserve separate rounding of the color and coverage weights. For the original byte foreground (116,38,60,36), background (158,163,10,68), mask 44 and opacity .6875, source-alpha returns F32 bits (3f1d2bec,3f214e9d,3d1e9201,3e8ed249). Ignoring foreground alpha for color gives (3f145cda,3f131eb2,3d120e5e,3e8ed249). A separate straight-alpha control remains at 14,395/40,960 across ten renders, maximum F32 component error 377.020249 on signed HDR input.

Use the [independent blend inputs](#color-burn-and-color-dodge) with mode 3 and opacities 1, .6875 and .37. At 64² use varied, opaque, white-mask, constant, black-foreground and transparent-background cases with U8 and F32 sources. Capture the varied case in all four formats. For float sources add the signed field 3H−1 to foreground and background. At .6875 vary the color interpretation and toggle coverage. For grayscale use the 256² byte-pair matrix, reversed pair order and the signed 16-value grid at opacities 1 and .6875. Floating-point comparisons include zero’s sign bit.

For a direct zero test, set V=(+0,−0,1,−1,.5,−.5,2,−2) with indices modulo eight. At 64² set foreground=(V[x],V[x+3],V[x+5],V[y]) and background=(V[floor(x/8)],V[floor(x/8)+1],V[floor(x/8)+4],V[floor(y/8)]). At seeds 137,180,223 use mask 1, V[x+y] and .6875 respectively. The channel product starts at positive zero. Subsequent weighting preserves a negative zero when both terms are negative zero. All 12,288 RGBA32F pixels now match bit for bit, up from 9,779/12,288 with maximum numerical error zero.

### Unbound property links and composite matte

| Comparison | Renders | Exact pixels / compared | Maximum error |
|---|---:|---:|---:|
| Composite matte | 19 | 168,448/168,448 | 0 |
| Levels property controls | 48 | 141,312/141,312 | 0 |
| Blend opacity controls | 48 | 196,608/196,608 | 0 |

The matte graph’s grayscale Levels output-high property directly reads an absent user input. The reference app reports a stored property value of (1,1,1,1), which remains in effect. With foreground byte 64 and background byte 191, Levels outputs 64 and the final F32 result is .8120107650756836. Before the fix Levels returned zero and the blend produced .7490196228027344. On the original nine renders, the root improved from 659/132,096 to 132,096/132,096 exact pixels. The ten additional renders match all 36,352 pixels. The combined totals are 122,880 U8, 8,192 U16, 8,192 F16 and 29,184 F32.

Use the 128² default, reversed-input and seed-42 fields, the 256² default and the 64² four-format and independent 32² cases. For the ten additional cases use F32 values (.25,.75) and (.75,.25) for foreground/background at 16², then (0,1) at 32². At 64² in four formats and 128² in F32, fill each input with Hⱼ=(((37+4j)x+(61−2j)y+(7+2j)xy+37j) mod 1021)/1020 using j=1,2. Use seeds 103,164,225,286 and 347 respectively. Also connect just the .25 foreground or just the .75 background at a 32² parent, seed 419. The root exposes no matte-opacity control.

For independent Levels controls use (H₁,H₂,H₃,(x mod 5)/4) as linear RGBA input. Compare the default output-high, a scalar literal .375, an absent scalar, an available scalar, adding .1875 to either scalar, and packing either scalar into all four lanes. Use 16², 64² and 32² at seeds 89,168 and 247 with bound values .375,.75 and 0. At 64² capture all four formats, elsewhere F32. Only the direct absent scalar keeps the stored output-high. Before the fix this control matched 0/17,664 pixels.

On an independently authored blend with scalar inputs H₁ and H₂, compare the default opacity, an absent getter with and without a leading percent sign, that same getter with a disconnected literal, the getter plus .1875, a bound getter, a bound zero and a local write of .3125 followed by a read. Use the same three input sizes and formats, seeds 71,168,265 and bound values .375,.75,.125. The blend’s output is fixed at 64² for these controls. Each direct missing-variable case improved from 24/24,576 to 24,576/24,576 exact pixels. A missing variable used in arithmetic contributes zero, and a zero-valued binding is honored.

### Stored integer value outputs

The four-integer root matches **41/41 typed vectors**, maximum absolute component error zero. A stored graph input of (−31,0,1,2147483647) cooks to (−31,0,1,−2147483648). Independent scalar, two-, three- and four-component processors matched **132/132 typed results** across immediate constants and direct input reads. In 32 input cases per width, the reference app’s value constructor and graph property preserved all 320 requested integer lanes. At the processor and graph output ports, all 320 matched rounding to binary32 followed by conversion back to signed 32-bit integer. The positive endpoint rounds to 2³¹ and stores −2³¹. For example, 16777217 stores 16777216, −16777219 stores −16777220, and 2147483583 stores 2147483520. The regression checks uncached and cached outputs at all four widths. A MaterialCraft save and reload also preserves all ten authored constant values across scalar and vector integers, floats, Boolean and text. After reopening, the authored integer 16777217 is intact and its cooked output is 16777216. The authored pair (−2147483648,2147483647) is intact and both cooked lanes are −2147483648.

For the 32 boundary cases let T=2²⁴ and M=2³¹. In order, take 0, 1, −1, 257, 65537, −65537, 2²³+1, T−1, T, T+1, T+2, T+3, −T−1, −T−3, 2²⁵+1, 2²⁵+3, 2³⁰+1, M−258, M−193, M−192, M−191, M−129, M−128, M−65, M−64, M−63, M−3, M−2, M−1, −M, −M+1 and −M+65. In case j, fill lane c from index (j+7c) mod 32. Cycle parent sides 16/32/64/128 and requested U8/F16/U16/F32 with seed 29+37j. All outputs report their integer type. Also cook defaults at 16² and the first d components of (M−1,T+1,−T−3,−M+1) as a literal at 32², seed 83. The three existing scalar, two- and three-integer root cases per width also still match.

Arithmetic inside a processor has a separate measured difference. Inserting modulo 257 before the output gives 26/132 exact typed results, maximum component error 257. Across scalar and four-component addition and subtraction controls at offsets −7 through 7 from (T,−T,2²⁹,−2³⁰), 48/94 results match, maximum error 8. A fully literal (T+1)−T returns 1 in both applications. A direct graph input set to T+1, then reduced by T, returns 0 in the reference app and 1 in this cook.

### Unconnected function arguments

Create an original function that returns its scalar argument and declares .1875 as the source default. Call it from a node parameter, pixel processor, stamp parameter and value processor. For each scope test five variants with a wired argument, an open argument, an open argument sharing the outer variable name, and an open argument declared connectable or explicitly non-connectable. Supply .625 at 16²/seed 79 and .3125 at 32²/seed 197. Read LUM32F for the three image scopes and a typed float for the value scope.

Each image scope matches 2,560/6,400 pixels and the value scope matches 4/10 typed floats. Maximum error is .1875 in every scope. Wired arguments match, as does the open connectable declaration. For the remaining open declarations MaterialCraft reads the authored .1875 and the reference returns zero. MaterialCraft also uses published source defaults across open documents, including when the author changes the source value. This difference remains open.

For a separate typed control at 16²/seed 487, pass true, integer 7, integer vectors (−2,3), (9,−8,7) and (4,3,2,1), float .1875, and float vectors (−.75,.5), (.625,−1.5,2) and (−1.375,3,2,−.25). Read bool as 0 or 1 and convert the first component of each numeric argument to float. With the value wired, 9/9 typed floats match exactly. With the value declared as the source default and the call port open, 0/9 match, with maximum error 9. The [literal-argument comparisons](#literal-arguments-inside-pixel-function-calls) use explicit wires.

The release engine tests also cover scalar Levels controls before and after saving a package. Their expected alpha is one for an input function returning .25, consistent with the [measured endpoint controls](#integer-levels-endpoints-and-channel-mixing).

### Literal arguments inside pixel function calls

The color-temperature root now matches all 42,496 RGBA pixels across its 19 renders. All 29,440 F32 pixels and all 4,352 pixels in each of U8, U16 and F16 are bit-exact. The previous F32 result differed by up to 1.1920929e−7. The root exposes a scalar temperature, with a default of 3,000. The [temperature cases and scalar fit](#volume-density-dithering-and-color-temperature-controls) give the original settings and varying-input controls.

| Independently constructed F32 controls | Renders | Exact before / compared | Exact after / compared | Maximum error after |
|---|---:|---:|---:|---:|
| Nested sum and difference, signed hash field | 24 | 134,270/141,312 | 141,312/141,312 | 0 |
| Nested arithmetic, logarithm and sine, modular fields | 48 | 247,296/282,624 | 282,624/282,624 | 0 |
| Temperature fit with warm, cool, unit or varying coefficients and full RGB | 25 | 98,576/113,920 | 113,920/113,920 | 0 |

Create a separate four-input scalar function for each expression a×b+c×d, a×b−c×d, ln(a) or sin(a). Give each function a distinct identifier. Bind the first pair, the second pair, all four or none of the arguments to literal values (.1875, 1.25, −.375, 2.75). For the remaining arguments sample the corresponding channels of an original RGBA32F image at pixel centers. Use 16²/seed 83, 128²/311 and 32²/577. For the first row use 4H/2³²−2 with the [row-major hash](#runtime-pixel-sine-cosine-and-atlas-rotation) at index 4(yN+x)+channel. For the second row use ((73x+93y+7xy+47c+7cy) mod 1024)/256. Double this and subtract four at 32². Inlining the same expressions also matches on all 529,920 control pixels.

When a and b are literals, their product rounds first and the other product can fuse with the sum. At (2,0) of the 128² hash field, c and d have F32 bits be7ff6a8 and bf68ddb9. The expected sum ends in 3eec6a9d. Rounding c×d first gives 3eec6a9c. A literal read inside a function also uses literal arithmetic for its logarithm and sine. The function compiler carries that information through explicit argument connections and accounts for assignments that can replace their values.

Omitting a connection is a separate case. With the same declared defaults and both first-pair ports left open, the hash-field sum/difference agrees on 0/35,328 pixels, maximum .234375238. The four modular-field expressions agree on 0/70,656 pixels, maximum 1.67397642. The measured color-temperature root uses explicit connections for its fit.

### Wide blur windows and bevel blur

The bevel blur matches 311,552/311,552 grayscale pixels across all 26 renders, maximum error zero. The [bevel intensity controls](#five-texel-blur-kernels) previously matched 160,049 pixels. The current totals are 122,880 U8, 24,576 U16, 24,576 F16 and 139,520 F32. These include all three successive blur stages at the larger intensities.

| Original controls with radius above 2.5 texels | Renders | Exact RGBA pixels / compared | Maximum error |
|---|---:|---:|---:|
| Separable impulse, stripes and signed channel | 48 | 1,081,344/1,081,344 | 0 |
| Zero-angle directional impulse and stripes | 16 | 262,144/262,144 | 0 |
| Separable modular four-channel fields | 48 | 172,032/172,032 | 0 |
| Zero-angle directional modular fields | 24 | 86,016/86,016 | 0 |
| Four shifted impulses | 42 | 1,204,224/1,204,224 | 0 |
| Rectangular and square RGBA fields, separable | 128 | 3,801,088/3,801,088 | 0 |
| Rectangular and square RGBA fields, directional | 64 | 1,900,544/1,900,544 | 0 |

Interior runs aligned to a power of two read a single filtered texel from the corresponding one-dimensional mip level. Reduce adjacent pairs at the source precision, storing each level at the destination precision. At the boundary of a footprint with radius r, let e=ceil(r−.5) and f=r−(e−.5). An endpoint aligned with the start of a pair combines the fractional texel with its full neighbor. For example, the left sample lies 1/(1+f) texels from that endpoint and has weight 1+f. A lone endpoint has weight f. Accumulate the weighted runs from right to left horizontally and top to bottom vertically, then multiply by .5/r. On a clamped axis, coverage beyond the image contributes one weighted read of the original border texel. A 256² impulse at column 128 and radius 4 reads .0625 at column 124, 513/4096 at 125 and 255/4096 at 132.

Use the [three-texel impulse and stripe inputs](#three-texel-blur-kernels) and the [four-channel and shifted-stripe inputs](#five-texel-blur-kernels). At seed 347 run the separable blur at intensity 17.5 on 64², 6.25/10/16.5/24.75 on 128² and 5.25 on 256². Run the zero-angle directional filter on the four 128² cases. For the modular fields use radius 2.875 at 64²/seed 797, 3.0625 at 32²/839 and 3.375 at 64²/877, with all four destination formats at 3.375. For the shifted stripes use radii 2.75, 3, 3.125, 3.5, 4, 5.25 and 8.25, seed 239, at 64², 128² and 256².

For the final two rows use 64×128, 128×64 and 256×256. Channel c is H=((37x+71y+11xy+47c+3xc) mod 1021)/1020. Supply rounded U8 levels or F32 values 3H−1.125. In order, use intensities 6.25, 9, 13, 17.5, 23.75, 30, 31.5, 48 and 62.75 at seeds 109+58j with j starting at zero. Output F32 except at 17.5, which uses U8, U16, F16 and F32. Run the separable filter with both axes tiled and both clamped, and the horizontal directional filter tiled. The radius scales from the shorter dimension on both axes. The table counts only radii above 2.5. Oblique directional kernels are outside these measurements.

### Five-texel blur kernels

| Original controls with 1.5 < radius ≤ 2.5 | Renders | Exact RGBA pixels / compared | Maximum error |
|---|---:|---:|---:|
| Separable impulse, stripes and signed field | 40 | 851,968/851,968 | 0 |
| Zero-angle directional blur on the same fields | 12 | 147,456/147,456 | 0 |
| Separable four-channel field | 80 | 253,952/253,952 | 0 |
| Zero-angle directional four-channel field | 40 | 126,976/126,976 | 0 |
| Shifted stripes, separable and directional | 36 | 1,032,192/1,032,192 | 0 |

At this radius, sample the center directly and combine the texels on each side with a filtered read at offset 2−1/(radius−.5) texels. The center weight is .5/radius. Each side has weight (radius−.5)×center weight. Filter at the source storage precision, then accumulate in F32 from left to right horizontally or bottom to top vertically. The separable blur stores its horizontal image at the destination precision. On the 128² stripe at radius 2, the outer and inner samples are .12451171875 and .25048828125. The regression also checks the raw F32 channels on the signed field.

Use the [three-texel inputs](#three-texel-blur-kernels) for the first two rows. For the next two, define H=((33x+87y+5xy+71c+17cx+13cy) mod 1021)/1020 per channel. Use H rounded to the nearest integer level for normalized inputs and 3H−1.375 for floating inputs. Run the separable filter both tiled and untiled. Use radius and seed (1.5009765625,97), (1.71875,137) and (2.375,571) at 32². At 64² use (1.5625,109), (1.9375,211), (2.09375,389) and (2.4990234375,683). Output F32 except at radius 1.9375, which uses all four formats. For the last row, fill the four F32 channels with vertical stripes at x=1, N/8+1, N/2 and N−3. Use 64², 128² and 256², seed 239, and radii 1.50390625, 1.75, 2, 2.0625, 2.25 and 2.5.

The bevel blur now matches all 154,880 pixels of the original 17 renders, including 128² at intensity 9. Its three blur stages each have radius 2.25 at that setting. Nine further renders extend the range. Define H=((37x+29y+11xy) mod 509)/508. At 32² use intensity 0/seed 53. At 64² use intensity 16/137 with 2.5H−.75. Both match exactly. At 128² run intensity 12.5/307 in all formats and repeat F32 at seed 797. At 256² use intensity 10.75/487. For a 64×32 source at 64² use intensity 24.5/653.

| Bevel blur with the extended range | Exact pixels / compared | Maximum error |
|---|---:|---:|
| LUM8 | 106,545/122,880 | 3 |
| LUM16 | 8,192/24,576 | 356 |
| LUM16F | 8,192/24,576 | 0.005859375 |
| LUM32F | 37,120/139,520 | 0.00733691454 |

At radius 3.125, shifting a single vertical impulse by one column exchanges the two measured edge weights .09953125 and .099999994. Those larger footprints still differ. The maximum F32 difference on the extended bevel matrix occurs at 256².

### Uniform powers of two and volume lighting

| Graph or operation | Cases | Exact RGBA or grayscale pixels / compared | Maximum error |
|---|---:|---:|---:|
| Uniform power of two, 128 independently evaluated columns per image | 80 | 1,310,720/1,310,720 | 0 |
| Uniform base-two logarithm, 128 columns per image | 9 | 145,536/147,456 | 2.38418579e−7 |
| Direct tests of the two uniform operations in four storage formats | 32 | 51,200/51,200 | 0 |
| Volume environment light | 39 | 7,898/31,488 | U8 4, U16 829, F16 .0126953125, F32 .0560382009 |
| Volume direct light | 46 | 43,038/43,264 | U8 0, U16 0, F16 .0000152587890625, F32 .00000370293856 |

Use a node function to compute x=o+i×d for each column i=0..127, store each result in a scalar variable, then read the variable for the current column into a 128² F32 output. At seed 257 use (o,d) for the power of two of (0,1/128), (−1,1/128), (−8,1/8), (−.5,1/32768), (.625,1/16384), (−4,1/37), (−3,1/19), (32,1/64) and (−31,1/64). For the logarithm use (1,1/128), (.5,1/256), (1,1/32768), (.125,1/32), (2,1/64), (8,1/16), (1.75,1/16384), (2⁻¹²⁶,2⁻¹²⁹) and (.7,1/4096). The subnormal step flushes to zero. The logarithm differs in 15 of the 1,152 sampled columns. In a second power-of-two series, use (j/64,1/8192) for j=0..63, followed by (0,−1/4096), (−.5,1/4096), (.5,1/4096), (16,1/256), (−16,1/256), (32,1/128) and (−32,1/128). Set the seed to 311+29j. For the four-format controls, run exponents −.984375, −.015625, .9375 and −2 or logarithm arguments .625, 1.375, 1.75 and 2 at sides 16, 32, 64 and 32, seeds 277, 318, 359 and 400.

The uniform exponential clamps the argument to [−126,126]. For its fractional part f, evaluate the degree-seven Taylor polynomial of exp(f×float32(ln 2)) in Horner order, rounding each operation to float32. Scale by the integer power of two. An exposed −.125 gives bits 3f6ac0c3. Both a literal and a uniform .9375 give 3ff52576, while a runtime pixel argument gives 3ff5257e. The negative smallest normal gives 3f7ffff4, the negative smallest subnormal gives 3f800000, and a NaN intermediate gives 00800000. Uniform log2 computes the uniform natural logarithm divided by float32(ln 2). Direct readbacks of intermediate +∞, −∞ and NaN give natural-log bits 42b17218, 00000000 and 00000000. The [per-pixel comparisons](#runtime-pixel-logarithms-and-powers-of-two) cover the separate pixel evaluations.

For temperature-to-RGB use 1200, 1733, 2718.5, 3000, 3975, 5555, 6500, 7111.25, 9900.5 and 18000. For the scalar fit use .1875, .3125, .5625, .875, 1, 1.375, 2.25, 4.125, 7.5 and 13.125 with coefficients (.1875,1.25,−.375,2.75) and (2.75,−.25,1.375,−.625). For each series use seed 151+29j, a 32² canvas when j is a multiple of three and 16² otherwise. Each of the three operations gives 5,632/5,632 exact pixels in the node scope, 5,632/5,632 in the pixel scope and 10/10 exact typed values in the Value Processor. All maxima are zero.

All 7,680 direct-light density pixels in the [density controls](#volume-density-dithering-and-color-temperature-controls) now match. Twelve further direct-light renders match 13,056/13,056. Use the same thin, thick and varying-density inputs. Run (a) thin defaults, (b) thick at 2700 K, (c) thick at 9900.5 K and 64², (d) thin with explicit light RGB (.1875,.5,.8125), (e) thin with ambient intensity 1.75 and RGB (.3125,.75,.1875), and (f) varying density with ambient intensity 1, RGB (.375,.7,.2), explicit light RGB (.4375,.1875,.75), UV (.375,.6875) and light intensity 2.25. Use seeds 47, 118, 189, 260, 331 and 402 respectively. Use four formats for (b) and (f), F32 elsewhere. The complete direct-light matrix is exact at U8 (7,424/7,424) and U16 (7,424/7,424). F16 has 10,751/10,752 exact pixels and F32 has 17,439/17,664.

Read out named values immediately after initialization and after the first volume step on the 16² thin, 16² dithered, 32² thick and 32² varying-density F32 cases. For environment light the initial ray position, ray direction, noise input, dithering, step size and accumulated depth are each exact at 2,560/2,560. After one step the position and depth each match 1,280/2,560, maximum 1.96305132. Scattered light and transmission match 1,280/2,560, maximum .0574402809. For direct light the initial light color improved from 0/2,560 to 2,560/2,560 exact. Direction, ambient color, noise, step size, first position and first depth are also exact. First-step scattering and transmission match 2,560/2,560, maximum 0. The two light roots remain in progress.

### Undefined quotients in a unit interval and panorama spot lighting

| Output | Renders | Exact stored pixels / compared | Maximum error |
|---|---:|---:|---:|
| Panorama spot | 36 | 189,696/189,696 | 0 |
| Eight intermediate spot outputs, F32 | 24 | 24,576/24,576 | 0 |
| Inlined and called scalar intervals | 120 | 168,960/168,960 | 0 |
| Operand order, shared reads, signed zero and computed bounds | 66 | 92,928/92,928 | 0 |
| Single minimum or maximum, called and inline | 24 | 33,792/33,792 | 0 |

The panorama spot matches 165,120 F32 pixels and 8,192 pixels in each of U8, U16 and F16. The preceding build matched 188,736/189,696, maximum 3.91726637 in F32. With a zero-size hotspot, its numerator matches at every pixel. Dividing it by zero and then evaluating max(0,min(x,1)) produced a weight of one in MaterialCraft and zero in the reference. Those 1,024 intermediate weights now match. For example, at (5,3) in the 32² border case the red output changed from 6.26762629 to 2.35035992. At hotspot size 2⁻¹² and .4375 the same intermediate was already exact.

In a pixel function, a scalar clamp with literal bounds zero and one saturates NaN to positive zero. Returning the inner min(NaN,1) separately still gives one. Inline operands and explicitly bound function arguments both exhibit this behavior. The .25 to .75 interval returns .75 for NaN in the same operation order. Runtime bounds supplied through the texture also keep their individual minimum and maximum behavior.

For the 36 spot renders, start with the four archive and five inventory comparisons. Define channel c of the original F32 texture as H=((37x+41y+7xy+61c+5xc) mod 509)/508. Use the defaults at 16²/43. At 64²/157 set the position to (.21875,.3125), scale .8125, color (.84375,.375,.59375), temperature 4,750, intensity 1.875 and border softness .34375. Set hotspot intensity 3.125, size .4375, falloff .65625 and position (.65625,.1875). Enable the input and the background, with color (.0625,.15625,.3125,.875). Render all four formats and repeat F32 at seed 751. The additional F32 settings are as follows.

| Size / seed | Changes from the 64² case |
|---|---|
| 128² / 293 | Position (.984375,.4375), scale 2.625, hotspot (.03125,.84375) |
| 32² / 347 | Position (.5,0), hotspot (.5,1), softness .9375, background disabled |
| 32² / 431 | 32×16 source, softness, hotspot size and falloff zero, texture disabled |
| 32² / 577 | Background and texture disabled |
| 32² / 661 | Scale .125, hotspot size and falloff 1, temperature 8,800, texture 2.5H−.75 |

For the final sixteen renders use the 32²/431 case. With a 32×16 source run all eight combinations of softness 0/.34375, size 0/.4375 and falloff 0/.65625. Repeat the all-zero and all-positive combinations with a 32² source. From the all-zero combination, change just one control per render. Set softness, size or falloff to 2⁻¹², disable the background, set hotspot intensity to zero, or set shape intensity to zero.

For the independent clamp controls, choose a numerator and denominator by (x+3y+seed) mod 12 from (0,0), (1,0), (−1,0), (7,2), (−7,2), (.375,1), (.75,1), (−.25,1), (−0,−0), (1,−0), (−1,−0), (.5,2). Use 16²/83 and 64²/311 F32, and 32²/157 in all four formats. Try max(lower,min(q,upper)) and min(upper,max(q,lower)), unit and .25/.75 literal limits, and swapped operand orders. For varying limits choose (0,1), (.25,.75), (−.25,1.5), (−1,0), (0,1), (0,1), (.25,.5), (−.25,.25) by (5x+7y+seed) mod 8. Include zero and one formed by subtracting a texture component from itself and adding one. Separately expose the minimum or quotient in a second comparison, swap each bound, use negative zero, compute bounds from literals and form 0/0 with literals. For the single-extremum controls replace zero denominators with one wherever the numerator is nonzero. This retains NaNs and gives finite stored results after the selected minimum or maximum.

### Reverse polar sampling in color and grayscale

The color and grayscale roots each match **306,688/306,688 pixels in 28 renders**, maximum error zero. Each matrix includes 126,976 U8, 12,288 U16, 12,288 F16 and 155,136 F32 pixels. Both use the same coordinate function and a bilinear image read. The coordinate function packs the radius-times-cosine and radius-times-sine products into two lanes before adding (.5,.5). In 19 color-field controls the build with scalar product forwarding matched 174,522/174,592 pixels, maximum .00390630960. The build with [independent vector product rounding](#vector-products-in-the-direct-light-pass) and the current build each match all 174,592.

The first 18 renders include the four archive cases, five inventory comparisons and nine [modular, signed and rectangular fields](#segment-mapping-and-normal-graph-rechecks). Add ten original coordinate and stripe inputs. Fill color channels with (x,y,x mod 2,y mod 2) for integer pixel indices, or just x for grayscale. Run F32 at 16²/seed 89, 128²/131 and 601, 256²/331 and 64²/509 with a 64×32 source. At 128²/233 replace the four channels with x mod 2, y mod 2, floor(x/2) mod 2 and floor(y/2) mod 2. At 64²/421 apply .1875v−2.125 to the coordinate field and render all four formats. Both roots expose only their input image.

### Scalar products used by sums inside a called function

| Measured output | Renders | Exact pixels / compared | Maximum error |
|---|---:|---:|---:|
| Two-point panorama gradient | 19 | 143,104/143,104 | 0 |
| Linear panorama gradient | 19 | 143,104/143,104 | 0 |
| Intermediate outputs of the two gradients, F32 | 38 | 97,280/97,280 | 0 |
| Independent called and inlined expressions | 156 | 459,264/459,264 | 0 |
| Earlier scalar call controls | 60 | 176,640/176,640 | 0 |

Each panorama root now matches 127,744 F32 pixels and 5,120 pixels in each of U8, U16 and F16. The two-point gradient previously matched 138,704 of these pixels and the linear gradient matched 138,579. The original direction, color, contrast, seam, pole and coincident-point cases are listed under [panorama gradient weights](#panorama-gradient-weights).

When a pixel function passes a scalar product or quotient to a second function, additions and subtractions inside that function can use the original factors. At texel (1,0) of the 32² linear gradient, the dot product has F32 bits 3f01187e and the width is .6875. The range helper returns bits 3f5de345. Returning the quotient in a second output channel rounds it to 3f3bc689 and changes the range result to 3f5de344. The regression exercises both graphs.

For the 156 renders, use the [signed RGBA field, six output settings and seeds](#vector-products-in-the-direct-light-pass). Let Q be R/G or R×G and B be the blue channel. Evaluate (Q+1)/2 and (B−Q)+Q for both terms. For division also evaluate (Q−B)/3, (B+Q)×.375, (Q+B)−Q and (B−Q)−Q. For both terms repeat (Q+1)/2 while returning Q in a separate channel, and subtract max(Q,B) from the half-range within the same function. For division also evaluate (B−Q)+Q². Render each expression as a separate function with explicit arguments, then inline it. The earlier build matched 446,883/459,264. The 120 controls for [two shared products](#two-sums-sharing-the-same-scalar-products) still match all 353,280 pixels.

### Panorama gradient weights

| Root | Renders | Exact RGBA pixels / compared | Maximum U8 / U16 / F16 / F32 error |
|---|---:|---:|---|
| Two-point panorama gradient | 19 | 138,704/143,104 | 0 / 1 / 0 / 1.78813934e−7 |
| Linear panorama gradient | 19 | 138,579/143,104 | 0 / 1 / 0 / 5.96046448e−8 |

Both roots generate their own coordinates. Alongside the four archive renders and five [inventory comparisons](#four-format-inventory-comparison), run the ten settings below. For the two-point gradient set colors to (.8125,.1875,.5625) and (.0625,.6875,.375). The second contrast is half the first. For the linear gradient use the first position, the same two colors as top and bottom, and the first contrast. All 5,120 U8 and 5,120 F16 pixels match for each root. The two-point root has 5,116/5,120 exact U16 and 123,348/127,744 exact F32 pixels. The linear root has 5,117/5,120 and 123,222/127,744 respectively.

| Size | Seed | First and second positions | First contrast | Output |
|---|---:|---|---:|---|
| 16² | 43 | Graph defaults | Default | F32 |
| 32² | 157 | (.15625,.34375), (.8125,.71875) | .3125 | U8, U16, F16, F32 |
| 32² | 347 | Same positions | .3125 | F32 |
| 64² | 317 | (.125,0), (.875,1) | .3125 | F32 |
| 32² | 431 | (.984375,.1875), (.015625,.4375) | .8125 | F32 |
| 16² | 577 | (.5625,.375) twice | .3125 | F32 |
| 16² | 661 | (.25,.5), (.75,.5) | 0 | F32 |

In both roots, at 32²/157 and 64²/317, the three-component directions for the texel and first point, their dot product and its quotient each match all 5,120 pixels when exported. Both constant linear RGB endpoints of the linear gradient also match. The next function adds one to the quotient and divides by two. Its unclamped output matches only 652/1,024 and 2,496/4,096 F32 pixels, maximum 1.19209290e−7 in both cases. Let D be the stored dot product and R the F32 reciprocal of 1−.3125, bits 3fba2e8c. Rounding D×R+1 once, then multiplying by .5, reconstructs both the unclamped and clamped linear weights at all 5,120 pixels. At (1,0) of the 32² case D has bits 3f01187e and the stored quotient is 3f3bc689. Separate rounding gives 3f5de344 for the weight and the reference gives 3f5de345. The same arithmetic followed by clamping and subtracting from one reconstructs the first two-point weight at all 5,120 pixels. The two-point weighted color sum still differs at 238/1,024 and 384/4,096 pixels in the measured implementation.

### Constants returned from pixel functions

| Comparison | Renders | Exact pixels / compared | Maximum error |
|---|---:|---:|---:|
| Second color-selection root | 23 | 179,456/179,456 | 0 |
| Nine intermediate rotation values at 32² | 9 | 9,216/9,216 | 0 |
| Returned and inlined arithmetic with pure results | 84 | 247,296/247,296 | 0 |
| Literal and sampled division controls | 252 | 741,888/741,888 | 0 |

The second color-selection root matches all 123,904 U8, 9,216 U16, 9,216 F16 and 37,120 F32 pixels in the [selection matrix](#scalar-arithmetic-across-a-call-argument). The 26 outstanding pixels used the perceptual distance. Its rotation helper multiplies the measured exponential by 60, then by π/180. π is returned by a zero-argument function. Dividing that constant by 180 gives bits 3c8efa35. The same numeric numerator read from a runtime variable gives 3c8efa36. Recognizing a pure constant result at the call boundary brings the radians from 17/1,024 to 1,024/1,024 and the complete perceptual distance from 998/1,024 to 1,024/1,024. The other intermediate comparisons are described under [shared scalar products](#two-sums-sharing-the-same-scalar-products).

For the 84 independent renders, use the [signed field, six sizes/formats and seeds](#vector-products-in-the-direct-light-pass). Return a scalar V and store (R×(V/180), V/180, sin(V), sqrt(V)). Use π, 1.125×2.375+.4375, an explicitly passed π, an explicitly passed abs(G)+.625, π from a function with an unused varying argument, π through a second zero-argument call and π forwarded through a second single-argument call. Repeat each expression inline. Before the change the same graphs matched 194,555/247,296 pixels. Both the called and inlined varying case still match all 35,328 pixels.

Two additional controls return an assignment of π or sequence an assignment of the varying argument before returning π. Inlined and called versions together match 32,768/70,656 in 24 renders, maximum 5.96046448e−8 in F16 and 8.74227766e−8 in F32. All U8 and U16 pixels match. These results were the same on both measured revisions.

### Two sums sharing the same scalar products

| Comparison | Renders | Exact pixels / compared | Maximum error |
|---|---:|---:|---:|
| Second color-selection root | 23 | 179,430/179,456 | 3.57627869e−7 F32 |
| Normalized lightness difference at 32² | 1 | 1,024/1,024 | 0 |
| Complete perceptual distance at 32² | 1 | 998/1,024 | 1.19209290e−7 F32 |
| Original graphs with two shared products | 120 | 353,280/353,280 | 0 |

In the [color-selection matrix](#scalar-arithmetic-across-a-call-argument), 41 more root pixels match after preserving the same unrounded product across two sums. All 123,904 U8, 9,216 U16 and 9,216 F16 pixels match. The remaining 26 differences are in the 32² F32 perceptual-distance case. In the normalized lightness term, the numerator and denominator each matched at all 1,024 pixels when exported separately. Exporting their quotient previously gave 926/1,024. The sum and difference share two lightness values scaled by 100. Both operations contract the same scaled value when used together.

For independent controls use the [direct-light signed field, sizes, formats and seeds](#vector-products-in-the-direct-light-pass). Form P=R×B and Q=G×A, then export P+Q and Q−P in two channels. Repeat with P=100R and Q=100G, swap the output channels, and reverse either sum's operand order. With the four varying factors, also expose P or Q as a third channel, divide the two results, and reverse the declaration order while keeping the output connections. These 20 graphs each have six renders. At input (bfd8e681,c00ec5b1,c05f9255,c020d2bc), written as F32 bits, (P+Q,Q−P) gives (41386778,bea0a9ae). Requesting the difference first gives (bea0a9ab,41386779). Exposing a product separately preserves its rounded value.

The remaining rotation contribution differs at 66/1,024 pixels, with maximum 3.87430191e−7. The hue in degrees, its offset from 275, scaled width, negative square and exponential each match all 1,024 intermediate values. The conversion to radians matches only 17/1,024, maximum 1.19209290e−7. With the measured exponential E, float32(60E) multiplied by float32(π/180) reconstructs every reference radian value. MaterialCraft’s effective factor is one F32 step higher. The square-root chroma term is exact. The complete root remains in progress.

### Scalar arithmetic across a call argument

The second color-selection root matches **179,389/179,456 grayscale pixels in 23 renders**. Its nine archive, changed four-format and independent cases are now exact. The 14 additional renders match 47,293/47,360. All 123,904 U8, 9,216 U16 and 9,216 F16 pixels match. In F32, 37,053/37,120 match, maximum absolute error 5.06639481e−7. The 67 differing pixels all use the perceptual color-difference mode at 32². Observing that distance directly gives 949/1,024 exact F32 values, maximum 1.78813934e−7.

For the additional inputs use N×N texels, u=(x+.5)/width, v=(y+.5)/height and Hⱼ=(((41+6j)x+(73−4j)y+(7+2j)xy+47j) mod 1021)/1020. Store (H₀,u,v,H₁) and (H₂,H₃,1−u,v) as linear RGBA32F. Take the eight rows below with seeds 79+71j, j starting at zero. At 16² use defaults. For the other rows use fixed color (.17,.57,.89) and distance weights (.875,.625,1.4375).

| Parent | Distance mode | Use color input | Threshold | Fade | Outputs |
|---|---:|---|---:|---:|---|
| 16² | Default | Default | Default | Default | F32 |
| 64² | 1 | Yes | .3125 | .4375 | U8, U16, F16, F32 |
| 32² | 0 | Yes | .75 | .125 | F32 |
| 64² | 2 | No | .625 | .3125 | F32 |
| 32² | 3 | Yes | .5 | .6875 | U8, U16, F16, F32 |
| 64² | 4 | Yes | .8125 | .5 | F32 |
| 128² | 1 | Yes | .4 | .75 | F32 |
| 32² with 16×32 inputs | 1 | Yes | .3125 | .4375 | F32 |

On the changed 64² case and the additional 64² mode-1 field, the RGB distance, selected distance, threshold and quotient each matched 8,192/8,192 before the fix. The next helper subtracts that quotient from one. Its output improved from 2,932/8,192 to 8,192/8,192. A scalar product or quotient with a single reader can round together with the addend inside a simple called addition or subtraction. A second reader of the product retains the rounded result. Original arguments still evaluate in their wired order.

Nine independent graphs match **158,976/158,976 pixels in 54 renders**, maximum error zero. Use the top-left 16², 64² and 32² of the original linear RGBA32F [signed hash field](#3d-linear-gradient), multiply green by .75, and set the first two texels to (3.25,.3,1,.0625) and (−3.4875,3.625,.875,−.625). Use seeds 47,223,449. Test F32 at each size and U8, U16 and F16 at 64². Compare an inline B−R/G, that same operation through a two-argument helper, the reverse subtraction, addition, B−RG, a literal divisor 3.625, two readers of the quotient, two readers of the addend, and a wholly literal 7.75/3.625 passed to the helper. For R,G,B with F32 bits c022f465,bfb54d77,3f4cdf41, B−R/G returned from the helper stores bf7f5030. Returning the quotient alongside it gives bf7f502f and 3fe617b8.

A tenth helper that both subtracts the quotient and adds it back internally still matches 16,011/17,664 pixels, maximum one U16 level or .000751018524 in F32. The measured correction covers the single-operation helper.

### Vector products in the direct-light pass

| Comparison | Renders | Exact RGBA pixels / compared | Maximum error |
|---|---:|---:|---:|
| Volume direct light, final output | 46 | 43,264/43,264 | 0 |
| Vector products returned through calls | 96 | 282,624/282,624 | 0 |
| Sums of two, four and eight returned products | 6 | 3,840/3,840 | 0 |

All 7,424 U8, 7,424 U16, 10,752 F16 and 17,664 F32 direct-light pixels match. Use the [volume light fields, seeds and parameters](#uniform-powers-of-two-and-volume-lighting). The last differing pixel was red at (2,60) in the changed 64² F32 render. At that pixel the secondary lighting processor now stores .007045745849609375 in F16. Forcing that stage to F32 gives bits 3be6ffff in both engines. The final red now has bits 3c198a4c.

For the independent graphs, take the upper-left 16², 64² and 32² corners of the signed 128² [four-channel hash field](#3d-linear-gradient), multiply green by .75, and replace the first two texels with (3.25,.3,1,.0625) and (−3.4875,3.625,.875,−.625). Use seeds 47, 223 and 449 respectively, selecting all four formats at 64² and F32 elsewhere. Have a function return ((a,b)×(c,d),d). Call it with (R,G,B,A) and (B,A,G,R), then sum the two returned vectors. Repeat with vector-by-scalar multiplication by c, with the two product lanes swapped, and with an assignment returning d in the third lane. For each version, export the sum, then expose each returned product’s first lane in turn, then expose the first call’s third lane. At the second texel the red sum has F32 bits 3df6666c. Exposing the first product changes the red sum to 3df66660. Reading the other component keeps the first value. Each single-consumer component contracts with its addend using its evaluated factors, including when its sibling performs an assignment. All 594,432 pixels in the [scalar packing and call controls](#pixel-products-carried-through-vector-lanes-and-calls) also match.

The eight isolated ambient samples explain the original rounding discrepancy. For sample i let aᵢ be its stored alpha and let uᵢ be the F32 product of (1−aᵢ) and channel coefficient 3e5b2d9a, 3ea31892 or 3f800000 (F32 bits). With background b=.03310475870966911, every isolated color channel equals the rounded b×uᵢ. Start the eight-term sum with FMA(b,u₀,round(b×u₁)), apply FMA(b,uᵢ,sum) for i=2 through 7, then multiply by .125. This reconstructs every reference RGB value at 64². MaterialCraft’s normal F16 secondary stage now matches all 12,288 RGB components. At forced F32, 4,088, 4,090 and 4,089 of 4,096 components match in R, G and B.

A separate difference survives inside the secondary stage. Its normal F16 alpha matches at 3,109/4,096 pixels, maximum .05224609375. In the observed mean, reference alpha equals the eighth ambient sample at all 4,096 pixels. A two-call control returning a pair (set a variable to x, read that variable) reproduces this ordering difference. Explicitly sequencing the write before the read in each call gives matching results. The final direct-light output matches in all 46 cases above.

### Pixel products carried through vector lanes and calls

| Comparison | Renders | Exact stored pixels / compared | Maximum error |
|---|---:|---:|---:|
| Scalar products and sums through vector components or reusable function calls | 108 | 594,432/594,432 | 0 |
| Volume ambient-direction helper, fixed sample count | 18 | 78,336/78,336 | 0 |
| Same helper, sample count read from a texture | 9 | 38,670/39,168 | F32 5.96046448e−8 |
| Volume direct light | 46 | 43,263/43,264 | F32 .00000370293856 |
| Volume environment light | 39 | 7,898/31,488 | U8 4, U16 829, F16 .0126953125, F32 .0560382009 |

A varying scalar product used only in a pixel addition rounds once with the addend, including when the product passes through a vector constructor, a component selector or a function return. Reading a different component preserves that behavior. When the product itself is also output, both consumers see its rounded value. A product of two literals rounds before addition. For R, G and B with F32 bits bfd8e681, c03e5cec and c05f9255, R×G+B gives 3fc60290 and the separate product and sum give 3fc60292. MaterialCraft retains the factors already evaluated in each call. The regression checks both operand orders and the next random draw after a sibling component consumes the first draw.

Start with the signed 128² [four-channel hash field](#3d-linear-gradient). Use the upper-left 16²/seed 53, 64²/241 and 128²/373. Render F32 at all three sizes and U8, U16 and F16 at 64². Form R×G directly, in the first lane of (R×G,A), in a scalar function return, and in a function returning the same pair. Add B in either operand order. Also output the product alongside the sum, read the sibling lane when present, and repeat with literal factors 5 and .618034005. These 18 graphs previously matched 545,872/594,432 pixels. The largest errors were one U16 level, 9.53674316e−7 in F16 and 1.90734863e−6 in F32.

For the ambient-direction helper, encode sample index i, noise and sample count N in the first three F32 channels. Use the direct-light iteration-four noise at 16²/0 and 64²/42 with i=4 and N=32. For other cases use i=(x+3y) mod N and noise=((37x+71y+11xy) mod 1021)/1020. Use (side,seed,N) of (32,93,32), (64,271,32), (128,509,37) and (32,719,19). For the last case apply 3×noise−.75. Render all four formats at 64²/271 and F32 elsewhere. Repeat the six inputs with a literal sample count of 32, and with both i=4 and N=32 held literal. The varying-index phase uses (i+1)×float32(.618034005)+noise with one rounding. With literal i, the product rounds first. The remaining 498 mismatches are in the two F32 cases with a changing sample-count input.

The [volume light inputs and format matrix](#uniform-powers-of-two-and-volume-lighting) give 7,424/7,424 pixels each in U8 and U16, 10,752/10,752 in F16, and 17,663/17,664 in F32 for direct light. At iteration prefixes five and six, separately read ray position, depth, and scattering/transmission at 16² with defaults and 64² with changed albedo (.3,.6,.9). These twelve images match 26,112/26,112 pixels. At prefix six the ambient direction, origin, step, maximum distance, distance along the ray, visibility and iteration index each match 4,352/4,352. A separate cosine threshold differs by 4.37113883e−8. In the changed 64² F32 case, the complete iteration outputs at prefixes 9, 17, 25 and 33 match 16,384/16,384. The final stored output differs in red at (2,60), with bits 3c1999d4 in MaterialCraft and 3c198a4c in the reference. The difference is also present in the background term. The environment-light pass uses a random draw in its first dithered step. Its per-pixel [random starting state](#pixel-processor-random-draws) remains unresolved.

The node release suite, formatting, strict workspace Clippy, layers and the ten WebAssembly checks passed. The same executable reproduced all 180,736 Lab-to-RGB pixels, 212,480 RGB-to-Lab pixels and 42,496 blackbody pixels.

### Volume density, dithering and color-temperature controls

| Root | Renders | Exact RGBA pixels / compared | Maximum U8 / U16 levels | Maximum F16 / F32 error |
|---|---:|---:|---:|---:|
| Volume environment light | 39 | 7,898/31,488 | 4 / 829 | 0.0126953125 / 0.0560382009 |
| Volume direct light | 34 | 25,144/30,208 | 0 / 1 | 1.52587891e-05 / 3.70293856e-06 |
| Blackbody | 19 | 34,048/42,496 | 0 / 0 | 0 / 1.1920929e-07 |

The two light passes each have twelve new controls in addition to the [surface and volume comparisons](#3d-surface-and-volume-render-passes). Set the position to RGBA8 (128,128,128,255) and the noise to zero. Bind original 128² F32 distance and density fields. The environment pass also uses a flat RGBA8 environment (64,128,192,255) and a 16² RGBA32F sample table (.25,.25,1,1). Unless specified, use distance −.125, dithering 0, absorption 1 and iteration 0. Store F32.

| Side | Seed | Density | Change |
|---:|---:|---:|---|
| 16 | 53 | 0 | Empty volume |
| 16 | 109 | 1/1024 | Three renders with the stated defaults, dithering 1, or distance +.125 |
| 16 | 109 | 1/16 | Stronger density |
| 32 | 211 | 1/128 | Absorption 2, four output formats |
| 32 | 373 | (u+v)/64 | Dithering .5, absorption .75, iteration 1. Use 128² atlas texel centers (u,v). |
| 32 | 377 | 1/128 | Absorption 2, field of view 37, camera UV (.1875,.6875) |
| 16 | 691 | 1/1024 | Repeat of the zero-dither thin volume |

For environment light, 6,400/7,680 new pixels match. The 16² thin volume is exact with dithering 0 and changes to 0/256 with dithering 1, maximum .00543540716. The varying density case matches 0/1,024, maximum .0560382009. Direct light matches 4,516/7,680 new pixels. On the thick volume it matches 1,023/1,024 at U16, maximum one level, and 291/1,024 at F32, maximum 2.98023224e−8.

The blackbody root retains the [temperature and format comparisons](#color-temperature-range-reductions-sky-parameters-and-polygon-paths). Original pixel graphs isolate the scalar fit and the RGB conversion. In an N² F32 texture, set input x=start+(row×N+column)×step. Use (N,start,step,seed) of (16,1200,37.75,127), (64,1300,4.125,281), (32,2500,.1875,337), (128,9000,.6875,433) and (32,.1875,1/1024,701). For coefficients driven by a texture, use a=((17+23×column+7×row) mod 256)/64−.5, b=((31+13×column+19×row) mod 256)/128+.125, c=((5+3×column+17×row) mod 256)/64+1 and d=.375.

| Five F32 renders per operation | Exact / compared | Maximum error |
|---|---:|---:|
| Fit with (.1875,1.25,−.375,2.75) | 18,799/22,784 | 5.96046448e-08 |
| Fit with (2.75,−.25,1.375,−.625) | 22,447/22,784 | 4.76837158e-07 |
| Fit with (1,1,1,1) | 22,784/22,784 | 0 |
| Fit with texture coefficients | 22,784/22,784 | 0 |
| RGB conversion with alpha 1 | 11,762/22,784 | 1.90734863e-06 |

For the first, second and texture-coefficient sets, an independently assembled pixel graph predicts all 68,352 reference fit outputs. Evaluate p using the reusable power function on (x,d), and output (a×b+c×p)/(b+p). When that graph exposes p, numerator, denominator and quotient as four channels, MaterialCraft also matches every component of all 68,352 pixels. The fixed-coefficient discrepancies persist when calling the fit as a nested function.

### Quad, swirl, trapezoid and vector morph comparisons

| Root | Renders | Exact pixels / compared | Maximum U8 / U16 levels | Maximum F16 / F32 error |
|---|---:|---:|---:|---:|
| quad transform | 23 | 204,032/204,032 | 0 / 0 | 0 / 0 |
| quad transform grayscale | 23 | 204,032/204,032 | 0 / 0 | 0 / 0 |
| swirl | 18 | 171,264/171,264 | 0 / 0 | 0 / 0 |
| swirl grayscale | 18 | 171,264/171,264 | 0 / 0 | 0 / 0 |
| trapezoid transform | 18 | 171,262/171,264 | 1 / 0 | 0 / 0.00343325734 |
| trapezoid transform grayscale | 18 | 171,262/171,264 | 1 / 0 | 0 / 0.000473022461 |
| vector morph | 18 | 171,264/171,264 | 0 / 0 | 0 / 0 |
| vector morph grayscale | 18 | 171,264/171,264 | 0 / 0 | 0 / 0 |

Every row includes the four original defaults, seed and parameter renders, the 64² four-format comparison, the independent 32² field and nine additional controls at 16² to 128². Use the [polar-field formula, rectangular edge pattern, signed range and seeds](#segment-mapping-and-normal-graph-rechecks). For color vector morph, number the image first and the vector field second. For grayscale vector morph, number the vector field first. Each quad row also includes the five [previous quad controls](#height-distance-texture-mapping-skew-and-weave). The 128² changed corners that formerly differed at six pixels now match all 16,384.

| Root pair | Controls after the default case |
|---|---|
| Quad | p00=(.0625,.125), p01=(.1875,.84375), p10=(.90625,.078125), p11=(.75,.9375). Disable tiling only for the signed input. |
| Swirl | Matrix (1.25,.1875,−.25,.875), offset (.15625,−.3125), amount 2.75. At 128² use −1.375. |
| Trapezoid | Top stretch .3125 and bottom stretch −.1875. At 128² use top stretch −.5. For signed input use bottom stretch .75. |
| Vector morph | Amount 3.125, or −1.375 for signed input. |

The trapezoid pair each has two remaining differences across the archived and new comparisons. The [four-format inventory](#four-format-inventory-comparison) lists the original changed settings and port names.

### Conditional pixel draws and White Noise v2

White Noise v2 matches **183,808/183,808 pixels in 76 renders**, with maximum error zero. The original 19 renders improved from 104,036/143,872 to 143,872/143,872. Across the full matrix, all 122,880 U8, 8,192 U16, 11,264 F16 and 41,472 F32 values match. This generator has no input image.

| Root controls | Renders | Exact pixels / compared | Maximum error |
|---|---:|---:|---:|
| Defaults and previous disorder, speed and format variations | 19 | 143,872/143,872 | 0 |
| Uniform and Gaussian at three seeds | 33 | 8,448/8,448 | 0 |
| Uniform, Gaussian and Triangle at four additional seeds | 24 | 31,488/31,488 | 0 |

Use the [original settings](#per-pixel-random-initialization) for the first row. For the second row use 16² F32 at seeds 0, 1 and 137. At each seed use uniform with disorder and speed zero. For Gaussian use (disorder,speed) of (0,0), (.25,0), and speed 1 with disorder 0, .25, .5, .75, 1, 1.25, 1.5 and 2. For each distribution in the third row, render 16² F32 at seed 509 with (−.25,.6875), 32² in all four formats at seed 317 with (.125,.6875), 64² F32 at seed 709 with (.875,1.5), and 32² F16 and F32 at seed 1021 with (1.5,.25).

Ten intermediate scalar outputs each matched 1,024/1,024 F32 pixels when selected individually. They covered the unit draw shared by both generators, radial and angular inputs, angle conversion, cosine, logarithm, squared radius, unnormalized and normalized Gaussian values, and the Gaussian result. Use 16² at seed zero with (disorder,speed) of (0,0) and (.25,1), seed one with (.5,1) and seed 137 with (.75,1). In the complete graph with disorder and speed zero, the Gaussian branch uses the third unit draw for its radius and the first for its angle. Evaluating the Gaussian expression on its own changes that order.

For an original conditional pixel expression, evaluate its condition first. A scalar draw shared between both arms or also used outside the conditional consumes its sample before draws private to the selected arm. Multiple shared draws follow first use in the true arm, then the false arm. Nested conditionals and draws supplying another draw's range use the same cached samples.

| Original condition and return value | Exact pixels before | Exact RGBA pixels after / compared |
|---|---:|---:|
| (T ? A : B, C, A, B) | 2 | 9,472/9,472 |
| (T ? A : B−A, C, A, B) | 2 | 9,472/9,472 |
| (T ? (A,A) : (B,A), C, C) | 4,736 | 9,472/9,472 |
| (T ? A : A−B, C, A, B) | 4,738 | 9,472/9,472 |
| (S ? (A,A) : (D,A), C, C) | 4,736 | 9,472/9,472 |
| (T ? A : (S ? B−A : B+A), C, A, B) | 2 | 9,472/9,472 |
| (T ? A : (S ? B−A : D), C, A, B) | 2 | 9,472/9,472 |
| (T ? A−B : B−A, C, A, B) | 4,736 | 9,472/9,472 |
| (T ? B−A : A−B, C, A, B) | 4,736 | 9,472/9,472 |
| (T ? E : B−E, C, A, B) | 3 | 9,472/9,472 |

A, B, C and D are separate unit-range scalar draw nodes. E is a draw with range A. Repeated letters reuse the same node. T is normalized texel-center x < .5 and S is B < .5. Render each expression at 16² F32/seed 0, 32² F32/17, 32² in four formats/137 and 64² F32/613. All 94,720 pixels match every channel, with maximum error zero. For the third row the F32 words at (24,0) of the 32²/137 render are 3e87bb00, 3f596540, 3f22ab7c and 3f22ab7c. The [standalone vector-typed draw](#scaled-random-draws-and-offsets) remains open.

### Packed random draws in flood-fill color

| Root | Renders | Exact pixels before | Exact RGBA pixels after / compared | Maximum error after |
|---|---:|---:|---:|---:|
| Flood Fill 2 color | 19 | 143,858 | 143,872/143,872 | 0 |
| Flood Fill 2 random color | 19 | 143,487 | 143,872/143,872 | 0 |

Use the [flood-fill cell maps, color fields and HSL settings](#per-pixel-random-initialization). Both roots now match 119,808 U16, 5,120 U8, 5,120 F16 and 13,824 F32 pixels apiece. Before the correction, the random-color root differed at 375 U16 and ten F32 pixels. The color root differed at five U16 and nine F32 pixels. At an absolute 32², seeds 137 and 733, and at 16², seed 839, each input bridge matches 2,304/2,304 pixels on both revisions. Each U16 random map improved from 2,299/2,304 (one level) to 2,304/2,304 (zero). All 2,304 processor and output pixels now match for each root.

Store a scalar unit draw once per evaluation of its range. Independent lanes can then contract each scaled sample with its own offset. When a lane supplies another output that reads the rounded product, the sum uses that rounded value. At (1,0) of a 16² graph at seed zero, pack consecutive draws of ranges .999 and −1.731053, then add (.001,.352111). The two F32 result words are `3ea12485` and `bf63be1d`. Return the sum together with its input pair and the four words are `3ea12485 bf63be1e 3ea0a173 bf9ef108`.

| Original RGBA control, seven renders each | Exact pixels before | Exact pixels after / compared | Maximum error after |
|---|---:|---:|---:|
| Packed pair plus offsets | 5,637 | 9,472/9,472 | 0 |
| Packed sum and both rounded draws | 9,472 | 9,472/9,472 | 0 |
| Sum of two single-use draws | 9,472 | 9,472/9,472 | 0 |
| One draw with two offsets | 5,846 | 9,472/9,472 | 0 |
| Shared draw with another single-use draw | 9,472 | 9,472/9,472 | 0 |
| First draw supplies the next range | 9,472 | 9,472/9,472 | 0 |

Let A=rand(.999), B=rand(−1.731053) and C=rand(1). For the packed control, repeat the two-channel sum as RGBA. For the shared version, return the sum followed by (A,B). The four other graphs return (A+B,C,A+B,C), (A+.001,.352111−A,A+.001,.352111−A), (A+.001,B−A,A+.001,B−A) and (rand(A)+B,C,rand(A)+B,C), respectively. Repeated expressions reuse the same graph node. Use 16² F32/seed zero, 32² F32/1, 32² in four formats/137 and 64² F32/557. All 56,832 complete pixels match. The [earlier scalar controls](#scaled-random-draws-and-offsets) and the 337 initialization controls retain their results. The separately declared vector-typed random instruction remains open.

### Scaled random draws and offsets

At (2,0) on a 16² graph with seed 17, evaluate `r=rand(.999)` and return `r+.001`. The raw F32 result has bits `3f3563cb`. Return the sum and r together and their bits are `3f3563ca` and `3f352241`. At (4,0) with the same settings, returning `.001−r` alone gives `beb19951`. A scalar pixel draw consumed by a single addition or subtraction multiplies its unit sample by the range and adds the offset in one rounding. A reader of the scaled draw receives its rounded value. The graph's evaluation order determines which sample each draw receives.

| Original direct control, eight renders each | Exact pixels before | Exact pixels after / compared | Maximum U16 levels | Maximum F32 error |
|---|---:|---:|---:|---:|
| Two sums, draw on left | 6,081 | 10,496/10,496 | 0 | 0 |
| Two sums, draw on right | 6,081 | 10,496/10,496 | 0 | 0 |
| Range minus offset | 7,433 | 10,496/10,496 | 0 | 0 |
| Offset minus range | 7,433 | 10,496/10,496 | 0 | 0 |
| Return each sum and its scaled draw | 10,496 | 10,496/10,496 | 0 | 0 |
| Vector-typed random instruction | 0 | 0/10,496 | 65430 | 1.73066464 |
| Horizontal coordinate as range, vertical as offset | 9,648 | 10,496/10,496 | 0 | 0 |
| Sum and difference with both scaled draws returned | 10,496 | 10,496/10,496 | 0 | 0 |

For each control, use 16² F32/seed 17, 32² F32/0, 32² in all four formats/137, 64² F32/413 and 32² F32/673. Use ranges .999 and −1.731053, offsets .001 and .352111. In the two-sum and subtraction controls, repeat the first two results as the other two RGBA channels. In the coordinate control, return `rand(u)+v` in grayscale, using the normalized texel center. In the sum-and-difference control, return the second scaled draw minus the first, their sum, then both individual draws. All **73,472/73,472 pixels in the seven scalar graphs** match. The eighth control explicitly declares a two-component random instruction with range (.999,−1.731053). Its reference output returns zeros for the draw, while the two scalar draws packed into a vector have distinct values. MaterialCraft's vector-typed result still differs. The [337 initialization and draw-order controls](#per-pixel-random-initialization) also retain zero error.

| Root | Renders | Exact pixels before | Exact pixels after / compared | Maximum U16 levels | Maximum F32 error |
|---|---:|---:|---:|---:|---:|
| Volume environment light | 39 | 31,387 | 31,488/31,488 | 0 | 0 |
| Flood Fill random color | 19 | 143,501 | 143,872/143,872 | 0 | 0 |
| Flood Fill random grayscale | 19 | 143,795 | 143,872/143,872 | 0 | 0 |
| Flood Fill 2 color | 19 | 143,858 | 143,858/143,872 | 1 | 1.42455101e-05 |
| Flood Fill 2 grayscale | 19 | 143,871 | 143,872/143,872 | 0 | 0 |
| Flood Fill 2 random color | 19 | 143,487 | 143,487/143,872 | 1 | 1.52587891e-05 |
| Flood Fill 2 random grayscale | 19 | 143,800 | 143,872/143,872 | 0 | 0 |

Use the same [root fields, parameters, seeds and output storage](#per-pixel-random-initialization). The density ramp now matches 1,024/1,024 F32 pixels, and the 39 environment-light renders include 11,264 F32 pixels. The grayscale pass and the first flood-fill package now match their complete recorded matrices. The two open color roots pack scalar random draws into a vector before applying their offset. Their residuals are bounded by one U16 level and .0000152587891 in F32. All U8 and F16 pixels in this root table match. The Worley, Dissolve, diffusion, splatter and fast white-noise reruns retain their previous counts. The later white-noise generator retains the differences measured for its Gaussian distribution.

### Per-pixel random initialization

The first four calls to `rand(1)` at pixel (0,0) of a 32² graph give (6046590,4572138,703596,3572932) / 2²³ at seed zero. At seed one they give (3086392,1966280,4339094,3323762) / 2²³. At (16,1) of a 32×16 graph, seed 42 gives (3941566,2846122,6888364,1390724) / 2²³. These observations distinguish the graph seed, position, resolution and draw order.

Start a 32-bit hash at zero. In order, mix the integer graph seed and the F32 bit patterns of the normalized texel center ((x+.5)/width,(y+.5)/height). For each word, set `t=(h+word)×1025` and `h=t XOR (t >> 6)`. Finish with `t=h×9` and `s=(t XOR (t >> 11))×32769`. All products and sums wrap to 32 bits. Use the low 24 bits of s to start the [measured pixel draw sequence](#pixel-processor-random-draws). This is the word-wise form of the public one-at-a-time mixing operation. [Bob Jenkins, Hash Functions](https://burtleburtle.net/bob/hash/doobs.html)

All **95,512/95,512 RGBA32F pixels in 337 direct renders** match, maximum component error zero. The earlier 13 four-draw images and 16 fan-out and scaling controls account for 53,568 pixels. For 288 new controls, set absolute width×height to 1×1, 1×2, 1×8, 1×32, 1×256, 2×1, 2×2, 8×1, 8×2, 8×8, 32×1 or 256×1. Use parent side 16 and seeds 0–5, 7, 8, 16, 17, 31, 32, 42, 127, 128, 255, 256, 257, 4095, 65535, 65536 and 16777216. Repeat seed zero at parent sides 32 and 64. The last 20 controls use 64×8, 8×64, 16×128 and 128×16 at seeds 0, 73, 983, 879201 and 16777217.

| Root | Renders | Exact pixels / compared | Maximum U8 / U16 levels | Maximum F16 / F32 error |
|---|---:|---:|---:|---:|
| Volume environment light | 39 | 31,387/31,488 | 0 / 0 | 0 / 0.000266551971 |
| 3D Worley noise | 34 | 312,832/312,832 | 0 / 0 | 0 / 0 |
| Dissolve | 28 | 183,040/183,040 | 0 / 0 | 0 / 0 |
| Diffusion RGBA pass | 19 | 143,872/143,872 | 0 / 0 | 0 / 0 |
| Diffusion UV pass | 19 | 143,872/143,872 | 0 / 0 | 0 / 0 |
| Diffusion grayscale pass | 19 | 143,872/143,872 | 0 / 0 | 0 / 0 |
| Diffusion normal pass | 19 | 143,872/143,872 | 0 / 0 | 0 / 0 |
| Flood Fill random color | 19 | 143,501/143,872 | 0 / 1 | 0 / 1.52587891e-05 |
| Flood Fill random grayscale | 19 | 143,795/143,872 | 0 / 1 | 0 / 1.52587891e-05 |
| Flood Fill 2 color | 19 | 143,858/143,872 | 0 / 1 | 0 / 1.42455101e-05 |
| Flood Fill 2 grayscale | 19 | 143,871/143,872 | 0 / 0 | 0 / 5.96046448e-06 |
| Flood Fill 2 random color | 19 | 143,487/143,872 | 0 / 1 | 0 / 1.52587891e-05 |
| Flood Fill 2 random grayscale | 19 | 143,800/143,872 | 0 / 1 | 0 / 1.52587891e-05 |
| Shape Splatter blend | 18 | 171,264/171,264 | 0 / 0 | 0 / 0 |
| Shape Splatter blend color | 23 | 204,032/204,032 | 0 / 0 | 0 / 0 |
| Shape Splatter to mask | 4 | 114,688/114,688 | — / — | — / 0 |
| White Noise fast | 18 | 127,488/127,488 | 0 / 0 | 0 / 0 |
| White Noise v2 | 19 | 104,036/143,872 | 165 / 39351 | 0.600250244 / 0.751115441 |

The root counts include each root's archived and four-format measurements where available. The earlier measurements specify the [Worley position fields](#3d-volume-mask-and-worley-noise) and [Dissolve inputs](#segment-mapping-and-normal-graph-rechecks). The [volume](#volume-density-dithering-and-color-temperature-controls) and [splatter](#plane-light-splatter-symmetry-and-waveform-comparisons) comparisons give the additional held-out fields for those roots. The remaining 101 environment-light differences are F32. In the 32² density ramp, 987/1,024 pixels match and the maximum error is .000266551971. The mask row covers its four archived F32 renders.

For Worley, Dissolve, the four diffusion passes, six flood-fill outputs and both white noises, add ten renders apiece. At 16² use defaults and seed 17. Use variant A at 32² in U8, U16, F16 and F32 with seed 137. Use variant B at 64²/413 and with a 32×16 field on a 32² graph/673. Use A with a constant 32² field/733, C with a signed 16² field/839, and A again at 32²/1237. All six single-format cases use F32. The two noise generators have no image input. For image pin j (one-based in declared probe order) and zero-based channel c, use H=((79x+53y+11xy+113j+73c+3xc) mod 1019)/1018. A grayscale pin uses channel zero. The constant is (.375,.625,.1875,.8125) and the signed field is 2.5H−.875. Worley uses normalized texel center (u,v) and position (u,v,.5+.25 sin(2π(u+v)),1). For flood-fill cell input, let sx=max(2,floor(width/5)), sy=max(3,floor(height/3)). Store (((x mod sx)+.5)/sx,((y mod sy)+.5)/sy,sx/width,sy/height) and zero cells where (floor(x/sx)+3floor(y/sy)) mod 7 is zero. Constant and signed cases replace these fields. Dissolve pin order is foreground, background, mask. For diffusion it is input/reference/mask/direction (RGBA), input (UV), input/reference/intensity (grayscale) and input/reference (normal). Flood Fill 2 color and grayscale take the cell input first and the color or grayscale field second.

| Parameter | A | B | C |
|---|---|---|---|
| Worley scale, mode, style, invert | 9,1,0,off | 13,2,1,off | 3,3,2,on |
| Dissolve opacity, alpha blending | .5625,on | .3125,off | .8125,on |
| Diffusion iter, nbIter, dither, normal | 2,5,on,off | 4,7,off,off | 0,3,on,on |
| Flood Fill 2 color/grayscale filtering | 1 | 0 | 1 |
| White Noise v2 distribution, disorder, speed | 1,.375,.625 | 0,.8125,1.25 | 1,.1875,.375 |

For Worley also set size (.75,1.25,1.5), Minkowski number .6875, border width .1875 and roundness .3125. Set optional diffusion controls where exposed. For Flood Fill 2 color use HSL adjustment (.125,−.1875,.25) and random range (.375,.625,.1875). For its grayscale equivalent use luminance adjustment −.1875 and random range .625. The random flood-fill outputs and fast white noise have no additional exposed numeric changes in these controls. Each residual flood-fill difference is at most one U16 level or .0000152587891 in F32. Uniform noise selects the measured per-texel stream. The later [conditional draw comparison](#conditional-pixel-draws-and-white-noise-v2) resolves the Gaussian differences in White Noise v2.

### Sampling integer bitmap inputs through wider bridges

An input node can have distinct readback and sampling precision. With a constant byte level 9 in a 32×17 grayscale PNG, a 32² F16 input bridge reports 3d108000 expanded to F32. A directly connected half output gives the same bits. A separate F32 filter sampling that bridge at a texel center returns 3d109091, the F32 value of 9/255. The filter also uses integer interpolation for off-center reads. A word bitmap converted to half supplies its stored half texture. A word bitmap through F32 still interpolates the normalized word texture.

| Original controls | Renders | Exact output pixels / compared | Maximum error |
|---|---:|---:|---:|
| Two source depths and four bridge formats through a translated bilinear transform | 48 | 49,152/49,152 | 0 |
| Centered and offset filters through a half bridge | 144 | 147,456/147,456 | 0 |
| Off-center reads through U8, F16 and F32 bridges | 72 | 73,728/73,728 | 0 |
| Direct output wired to the same three bridge formats | 36 | 36,864/36,864 | 0 |

For the first row use a 32² F32 output, seed 469, identity transform, bilinear filtering and offset (15/1024,−21/1024). Use grayscale or RGBA PNG files with zero-based channel c. On a 32×17 image use level 9 when x+c is even and 243 otherwise. On a 17×23 image use (47x+73y+13xy+31c+113) mod 256. On a 47×33 image use (11x+7c+223) mod 256. For 16-bit files, expand each level b to 257b and add (7x+3y+11c) mod 251 when b < 255. On the alternating grayscale image, the F32 word at (0,0) is 3eee4eee for the 8-bit source through all four bridges. For the word source, the same pixel gives 3eee8eef, 3eee92ef, 3eee8000 and 3eee92ef through U8, U16, F16 and F32 respectively.

For the second row use each half bridge with an identity transform at texel centers, a translated nearest transform, a blur at intensity zero, and a bilinear pixel sampler using the translation above. For each combination of filter and grayscale or RGBA, use the [original PNG fields](#source-image-depth-and-the-three-noise-upscalers) at all three sizes for bytes. For words use the 32×17 constant with levels (9,89,142,199)×257 and the 17×23 and 47×33 word hash. Add 32×17 constants at byte levels 1, 9, 31, 127, 143 and 250 and at their word equivalents. The fields use seed 157 and the constants use seed 379. For the last two rows use the three byte fields, the 32×17 word field and the constant 9 at both depths. Compare a translated bilinear transform, an offset bilinear pixel sampler and a direct wire. The filters render at F32. For the wire, read the declared output storage.

All 106 earlier integer-to-half cases also match at the downstream output, **108,544/108,544 pixels**, including all 26,624 byte pixels. The half readbacks match at the same locations. The two curvature roots and three noise upscalers retain all 2,444,800 pixels in 103 renders. The [float-source integer conversion](#enlarging-bitmap-rows-and-converting-integer-images-to-half) still differs at 18 U8 and 14 U16 pixels on the signed and stepped controls, by at most one level each.

### Enlarging bitmap rows and converting integer images to half

| External source and destination | Renders | Exact bridge pixels / compared | Exact F32 output pixels / compared | Maximum F32 output error |
|---|---:|---:|---:|---:|
| Signed F32 fields, original sizes | 78 | 79,872/79,872 | 239,616/239,616 | 0 |
| Signed F32 fields, 17 additional growth ratios | 51 | 147,456/147,456 | 466,944/466,944 | 0 |
| Six byte PNG fields read through half | 6 | 6,144/6,144 | 1/6,144 | .000486373901 |
| Thirty word PNG fields read through half | 30 | 30,720/30,720 | 30,720/30,720 | 0 |
| Twenty byte PNG constants read through half | 20 | 20,480/20,480 | 4,096/20,480 | .000411689281 |
| Fifty word PNG constants read through half | 50 | 51,200/51,200 | 51,200/51,200 | 0 |

A one-column input reproduces the previous [float growth difference](#float-bitmap-reduction-and-half-output). Resize 49 rows to 64. Put F32 words beae712e and 3f904fe4 in rows 5 and 6, with zeros elsewhere. Row 7 has bits 3c737400. During enlargement, map the output index and half-texel offset separately in F64, as i/s + .5/s for s=destination/source. The original 35×49 grayscale and RGBA fields now match at the bridge and at the square output.

For the 51 additional controls, the source width and bridge width are 32. Use source heights 21, 35, 43, 45, 47, 49, 51, 55, 57 and 61 with destination height 64. Use heights 67, 71, 87, 91, 93, 97 and 105 with destination height 128. Render at the destination height squared, sampling the bridge at normalized texel centers. At each height generate three original RGBA32F images, with zero-based x, y and channel c.

- At seed 709, use (((17x+53y+31xy+19c) mod 2047)/1024−1) × 2^((3x+7y+c) mod 8−4).
- At seed 710, reinterpret 3f800001 plus (37x+53y+13xy+31c) mod 256 as F32 bits. Negate where x+3y+c is odd.
- At seed 711, use (((21x+29y+11c+13xy) mod 1021)/1020−.5) × 2^((x+y) mod 13−5).

For the PNG fields use the [byte and word recipes](#source-image-depth-and-the-three-noise-upscalers). For constants, bind 32×17 grayscale and repeated RGBA images to a 32² half bridge, seed 881. Byte levels are 0, 1, 9, 31, 127, 128, 143, 196, 250 and 255. Word levels are 0, 1, 3, 7, 9, 13, 16, 127, 128, 129, 251, 513, 1023, 2117, 2313, 4118, 8193, 15427, 22001, 32079, 32767, 39114, 49197, 65529 and 65535.

Filter both axes at the source integer precision. A byte value b reports the half of b/255 rounded toward zero at the bridge. At byte level 9, that is 3d108000 expanded to F32. A word source uses the [three-row diffusion weights](#non-power-of-two-float-image-inputs-and-curvature) in word units with one new random sample per interleaved channel. Start the generator at 1, multiply by 48271 modulo 2147483647, and divide by 2147483647 for a unit draw r. Truncate original word + received error + 16(r−.5) and clamp to the word range. Values 0 through 2 read and reconstruct as zero. Value 3 reads as 2/65536 and reconstructs as 3. For larger values q, store the half of q/65535 rounded toward zero, and reconstruct as min(65535,65536×stored). Propagate original minus reconstructed. For word level 2313 the first three expanded channel words are 3d100000, 3d102000 and 3d10c000.

The reference F32 consumer of these byte sources reads the unrounded filtered byte divided by 255. Its half bridge readback and the consumer value are distinct. This discrepancy accounted for all 22,527 differing downstream pixels in the two byte rows. The [subsequent sampling controls](#sampling-integer-bitmap-inputs-through-wider-bridges) resolve them. The float-input to U8 and U16 controls retain 18,414/18,432 and 18,418/18,432 exact pixels, with maximum errors of one stored level. Both curvature roots retain 176,384/176,384 across 20 renders each. Each of the three noise upscalers retains 697,344/697,344 across 21.

### Float bitmap reduction and half output

| Bound input and bridge storage, read through F32 | Renders | Exact pixels before | Exact pixels after / compared | Maximum F32 error after |
|---|---:|---:|---:|---:|
| Signed F32 images read through F32 | 78 | 238,988 | 239,608/239,616 | 9.31322575e-10 |
| F32 images through U8 | 18 | 18,400 | 18,414/18,432 | 0.0039215982 |
| F32 images through U16 | 18 | 18,399 | 18,418/18,432 | 1.52597204e-05 |
| F32 images through F16 | 18 | 4,979 | 18,432/18,432 | 0 |
| Byte and word images through U8 | 36 | 36,854 | 36,864/36,864 | 0 |
| Byte and word images through U16 | 36 | 36,846 | 36,864/36,864 | 0 |
| Byte and word images through F16 | 36 | 6,605 | 6,605/36,864 | 0.359375 |
| Byte and word images through F32 | 36 | 36,846 | 36,864/36,864 | 0 |

Use the [original float fields and shapes](#non-power-of-two-float-image-inputs-and-curvature) and [byte and word fields](#source-image-depth-and-the-three-noise-upscalers). Every channel must match. On the signed 35×49 field at seed 277, the 16×64 bridge retains one different texel in each of the grayscale and RGBA measurements. At (5,7), channel zero has bits `3c737400` in the reference and `3c737401` in MaterialCraft. Reading these bridges at the parent square size gives the eight differing pixels in the first row. Integer-to-half conversion also retains differences.

| Additional direct control | Renders | Exact pixels / compared | Maximum finite error |
|---|---:|---:|---:|
| Constants in grayscale and RGBA | 48 | 49,152/49,152 | 0 |
| Midpoints, underflow, overflow and signed fields | 37 | 37,888/37,888 | 0 |
| Four different constants in one RGBA image | 1 | 1,024/1,024 | 0 |
| Close row values and simple row controls | 43 | 11,008/11,008 | 0 |

For a filter axis, let s=target/source, k=min(1,s), and c=(output+.5)/s. At input i the triangle weight is k×max(0,1−k×abs(i+.5−c)). Normalize the surviving weights separately, left to right, in F64. Retaining the outer k changes midpoint decisions on reduction. At width N=47, 65 or 67, bind a one-row float image with bits `3f000000` plus (13x+7j+3xj+j²) mod 64, where j is 15, 14 or 14 respectively. With a 16×1 F32 bridge, samples 11, 9 and 4 have bits `3f00001d`, `3f00001d` and `3f00001f`. For the other 40 rows, use widths 19, 25, 29, 35, 37, 41, 47, 67, 73 and 91. Store x/16−1, ((31x+5x²+9) mod 61)/51, (((19x+7x²+5) mod 32)−17)/8, or (x mod 7)−3. The table counts the 16² final outputs. The row regressions also check the 16×1 bridge.

For an F32 source and half output, resize both axes at F32 storage first. Visit the output pixels in row-major order. Add the accumulated error to each original channel value, clamp at zero and round to nearest half with ties upward. Propagate original minus stored value to neighboring pixels using the [three-row diffusion weights](#non-power-of-two-float-image-inputs-and-curvature). A constant (1+2⁻¹¹,−1,.125,.12505) on a 32×17 input at 32² gives (1+2⁻¹⁰,0,.125,.125) at (0,0), (1,0,.125,.125) at (1,0), and (1+2⁻¹⁰,0,.125,.125+2⁻¹³) at (2,0).

For 48 constant renders, use the same dimensions with values −10, −1, −.01, 0, .0001, .001, .01, .035294117647, .051234, .125, .12505, .21875, .241234, .501234, .813579, .998, 1, 1.125, 1.312345, 1.999, 2.345678, 8.12345, 32.6789 and 76.234 in grayscale and repeated RGBA. The 37 grayscale boundary renders cover four sequences. Use each entry as a constant, then the full sequence indexed by (3x+5y) mod length.

- 65504, 65505, 65519, 65520, 65521, 65535, 65536, 10¹⁰, −10¹⁰.
- 2⁻²⁶, 2⁻²⁵, 3×2⁻²⁵, 2⁻²⁴, 2⁻¹⁴−2⁻²⁵, 1+2⁻¹¹, 1+3×2⁻¹¹.
- 0, −.5, 1.5, .375, .625, .12505, .999, 1.001, 4.
- −10⁻¹², 9×10⁻⁹, .00034, .99, −2.75, 0, 5.5, .1875.

The boundary comparison includes the positions of infinite samples. At a constant 65520, the first five half values are positive infinity, zero, zero, positive infinity and zero. Both curvature roots retain 176,384/176,384 pixels across 20 renders apiece, and each of the three noise upscalers retains 697,344/697,344 across 21 renders. Their [word-depth comparison](#source-image-depth-and-the-three-noise-upscalers) locates the earlier source and mask differences.

### Source image depth and the three noise upscalers

| Root | Renders | Exact grayscale pixels before | Exact grayscale pixels after / compared | Maximum error after |
|---|---:|---:|---:|---:|
| Noise Upscale 1 | 21 | 695,768 | 697,344/697,344 | 0 |
| Noise Upscale 2 | 21 | 695,877 | 697,344/697,344 | 0 |
| Noise Upscale 3 | 21 | 693,752 | 697,344/697,344 | 0 |

Use the [upscaler inputs, offsets and seeds](#tile-random-cell-fields-and-noise-upscaling) and the [Min blend comparison](#min-and-max-blend-weights). Each matrix includes 491,520 U8, 32,768 U16, 32,768 F16 and 140,288 F32 pixels. On the 32×17 word source at seed 977, the source and mask bridges now agree on 1,024/1,024 U8 texels apiece and each of the three 64² F32 final outputs agrees on 4,096/4,096 pixels.

A PNG's original channel depth matters even when its decoded numbers are identical. Bind a 32×17 grayscale PNG containing 9 at eight bits or 2,313 at sixteen bits, and cook a 32² U8 input. The byte file gives six 9s at the start of the first row. The word file gives 8, 8, 9, 9, 9, 8. Binding the byte file again restores all six 9s. The graph's image declaration, exposed ports and authored resources retain their original representation.

When the source has fewer levels than the destination, filter each axis at source depth. When reducing word samples to bytes, use the original word, add the received error in byte units multiplied by 257, then add the centered random fraction multiplied by 256. Take the high byte after saturation. For diffusion, measure the error between the normalized original and stored sample. The order of random draws, error weights and separable filter are specified in [Non-power-of-two float image inputs and curvature](#non-power-of-two-float-image-inputs-and-curvature).

| Bridge storage, read through F32 | Renders | Exact output pixels before | Exact output pixels after / compared | Largest absolute output error after |
|---|---:|---:|---:|---:|
| U8 | 36 | 16,716 | 36,854/36,864 | 0.00392156839 |
| U16 | 36 | 19,962 | 36,846/36,864 | 0.00392156839 |
| F16 | 36 | 6,605 | 6,605/36,864 | 0.359375 |
| F32 | 36 | 19,962 | 36,846/36,864 | 0.00392156839 |

For all 144 controls, use 32×17, 17×23 and 47×33 inputs on a 32² graph at seed 157, in grayscale and RGBA. Define W=((97x+71y+13xy+157+79c+5xc)×197 +11y²+53x) mod 65536. Use (W mod 256) at eight bits, the identical decoded field (W mod 256)×257 at sixteen bits, and W at sixteen bits. Three further sixteen-bit fields are constant (15427,32079,39114,49197), constant (9,89,142,199)×257 and an endpoint field alternating W mod 512 for even W with 65535−(W mod 512) for odd W. Grayscale uses channel zero. Counts compare every output channel. A small number of reduction-boundary differences remain at byte and word working precision. The wider independent controls also retain discrepancies in half-float conversion.

### Non-power-of-two float image inputs and curvature

The final F32 output on the 32×17 stepped normal field now matches all 1,024 pixels for both roots. The previous RGBA16 input matched 201/1,024 within one level. The corrected input matches 1,024/1,024 with zero error. Each root now matches all archived, four-format, independent and held-out cases. The normal fields, seeds and parameter settings are specified in [Curvature from original normal fields](#curvature-from-original-normal-fields).

| Root | Renders | Exact pixels before | Exact pixels after / compared | Maximum error after |
|---|---:|---:|---:|---:|
| Curvature | 20 | 175,774 | 176,384/176,384 | 0 |
| Curvature smooth | 20 | 176,030 | 176,384/176,384 | 0 |

Each root covers 8,192 U8, 122,880 U16, 8,192 F16 and 37,120 F32 grayscale pixels. The 32×17 RGBA32F input exercises a distinct bitmap resampling path. At each output texel center filter with a triangle of radius max(1, source length / destination length) in source texel units. Drop out-of-bounds taps and normalize the remaining weights. When the width grows, filter vertically first. Otherwise filter horizontally first. Write the intermediate in its working precision. A unit impulse at the center of a 3×3 or 35×21 F32 input, filtered to 16², gives respective F32 bits 3f524000 and 3dbac261 at (8,8).

For normalized output, quantize the F32 input before filtering. Scan from top left in channel order. For each channel advance s=48271s mod 2147483647 from initial state 1. Center s/2147483647 on zero. For U8, round that fraction to the nearest 1/255 before centering. Add the received quantization error and centered noise in output-level units, then round and saturate the sample. Spread the difference between the original and stored level using the Sierra weights (5,3) to the right, (2,4,5,4,2) on the next row and (2,3,2) two rows below, all divided by 32. Discard errors beyond an edge. For constant RGBA (.21875,.25,.5,.75) at 32×17, the first two output texels at 32² have U16 levels (14335,16383,32768,49152) and (14336,16384,32767,49151).

| Independent input controls | Renders | Exact output pixels before | Exact output pixels after / compared | Largest absolute output error |
|---|---:|---:|---:|---:|
| F32 center impulses and steps | 6 | 2,402 | 3,072/3,072 | 0 |
| Signed F32 fields | 78 | 24,464 | 238,988/239,616 | 9.53674316e-7 |
| F32 input through U8 storage | 18 | 4,730 | 18,400/18,432 | 1/255 within F32 rounding |
| F32 input through U16 storage | 18 | 3,230 | 18,399/18,432 | 1/65535 within F32 rounding |
| F32 input through F16 storage | 18 | 4,979 | 4,979/18,432 | 34.25 |

For the signed field use H=((97x+71y+13xy+11s+79c+5xc) mod 1021)/1020 then (3H−1.25) × 2^((x+3y) mod 11−5). Use RGBA and the first channel alone. Source sizes are 17×19, 23×11, 9×7, 17², 31×33, 19×47, 67×21, 73×63, 35×49, 41×27, 91×55 and a 16×32 control. Use seed 277 and repeat 23×11 with seed 571. Compare 32², 64×16 and 16×64 bridge outputs through an F32 identity read at the parent square size.

For each stored-format group, use the constant above at 32×17. Use H with 11s replaced by 214 and 79c by 23c at 23×17, 31×17, 47×73 and 41×27. Use (floor(11H)+2)/16 at 17×32, and the signed field with seed 307 at 23×19, 17×49 and 9×7. Read all results through F32 at 32². Residuals remain on input storage boundaries and on float reductions. The F16 input conversion still differs, particularly for negative values.

### Min and Max blend weights

Noise Upscale 2 now matches 695,877/697,344 pixels across all 21 [upscaler cases](#tile-random-cell-fields-and-noise-upscaling). The remaining difference occurs on the 32×17 source. Maximum errors at U8, U16 and F16 are zero and the maximum F32 error is 0.00439214706. Applying the corrected blend to the measured intermediate textures reproduces all 168,960/168,960 pixels across twelve original parameter cases.

For Min and Max in source-alpha, ignore-alpha and premultiplied interpretations, multiply the selected foreground color by its weight, then accumulate the background product with a fused multiply-add. Read the mask and participating foreground alpha through the complement of their transparencies. For example, with foreground 46/255, background 79/255 and opacity .56, Min gives F32 bits 3e730783.

| Independent controls | Renders | Exact pixels before | Exact pixels after / compared | Maximum error after |
|---|---:|---:|---:|---:|
| Gray and RGBA, interpretations 0, 1 and 3 | 96 | 92,110 | 135,168/135,168 | 0 |
| Straight RGBA, interpretation 2 | 24 | 12,244 | 12,244/33,792 | 1 level at U8/U16, .00048828125 at F16, 2.38418579e-7 at F32 |

Run both modes with opacity .56, with and without a mask. Use 16²/191 and 64²/541 at F32, and 32²/307 in all four output formats. For RGBA source j starting at 1 and channel c starting at 0, use ((97x+71y+13xy+11s+57j+79c+5xc) mod 1021)/1020. For a gray source or mask use ((97x+71y+13xy+11(s+79i)) mod 1021)/1020 with zero-based pin index i. The color pin order is foreground, background, mask and the gray pin order is foreground, background, mask. Use U8 input bridges for the gray blend and F32 for the color blend. The straight-color controls remain open.

### Tile Random, cell fields and noise upscaling

After the [pattern seed](#local-seed-expressions-in-pattern-graphs) and [blend](#filtering-unequal-inputs-to-a-blend) changes, the pattern-package Tile Random matches all **176,384/176,384** pixels across 20 renders. Its [four-format inventory cases](#four-format-inventory-comparison), independent 32² case and four archived cases contribute 132,096 pixels. Eleven new inputs add 44,288. The two 64² F32 circle renders at seeds 251 and 947 contain 507 and 510 distinct stored levels respectively. Counts below cover the end-to-end cases in this matrix.

| Root | Renders | Exact pixels / compared | Maximum U8 / U16 error (levels) | Maximum F16 / F32 error |
|---|---:|---:|---:|---:|
| Dirt 5 | 19 | 156,858/156,928 | 0 / 4 | 6.10351562e-05 / 5.0842762e-05 |
| Cells 1, second package | 20 | 158,598/173,312 | 1 / 0 | 0 / 1.96695328e-06 |
| Cells 4 | 21 | 174,284/174,336 | 0 / 53,506 | 0 / 0.491874591 |
| Cells 4, second package | 21 | 174,310/174,336 | 223 / 0 | 0 / 0.667963699 |
| Noise Upscale 1 | 21 | 695,768/697,344 | 0 / 0 | 0 / 0.00784313679 |
| Noise Upscale 2 | 21 | 678,381/697,344 | 0 / 1 | 0 / 0.00439214706 |
| Noise Upscale 3 | 21 | 693,752/697,344 | 0 / 0 | 0 / 0.00769269466 |
| Tile Random, pattern package | 20 | 176,384/176,384 | 0 / 0 | 0 / 0 |

For Tile Random use H=((73x+37y+11xy+k) mod 1021)/1020 in the pattern and background pins, with k=29 and 72. Start at defaults at 16²/137. At 64²/251 choose pattern 3 and (5,3) tiles, and render all four formats. At 128²/353 use pattern 2 and (9,7). At 32²/479 use a 32×16 U16 source, pattern 3 and (3,7), disabling non-square expansion. At 32²/557 use 2.5H−.875, pattern 2, (1,3), independent size randomness .125 and .625 and position randomness .1875. Use constant .375 with pattern 3 and (3,1) at 32²/733. At 64²/827 select pattern 0 (no stamp), (4,5) and filter 0. The [image-stamp comparison](#tile-random-image-and-soft-patterns) covers the image selector and all three filters. Repeat the 64² circle field at seed 947. The changed settings share pattern-specific value .3125, rotation randomness .375, shape rotation .125, interstice .1875 and split threshold .625.

For the seven other roots use the same H with k=19 on the first pin and 132 on the second. Renders use defaults at 16²/71, four formats at 64²/181, F32 at 128²/269, a 32×16 U16 input at 32²/383, a signed 32² field at seed 487, a constant .375 field at 32²/617, the 64² field at seed 701, and a 32² impulse at (floor(w/3),floor(h/2)) with seed 883. For roots with image inputs add a 32×17 U16 field at 32²/977. Use unsigned H on the signed case's mask. Dirt 5 has no input pins or exposed numeric controls. At 64²/181 its first differing pattern map matches 4,094/4,096 U16 pixels within one level.

For the cell roots use (scale, disorder)=(5,.6875) on the 64² fields, (13,.6875) on the 128² and rectangular fields, (1,.125) for the flat input and (9,.125) for the signed and impulse inputs. Disable non-square expansion on rectangular inputs. Select disorder speed .625 where exposed. Cells 1 uses angle .0625, angle randomness .1875, anisotropy .25 at .125 turns, size (.875,1.125), scale .8125, luminance randomness .375 and offset (.1875,−.3125). Select pattern 4 on the large and rectangular cases and 3 on the others. For Cells 4 use color source 1 except on the flat case, and pseudo-random seed 17 on the two 64² modular fields and 37 on the others.

Set the two offset pairs on the first and second upscalers to (.1875,−.4375) and (−.3125,.125) on the 64² fields, (−.5625,.3125) and (.6875,−.1875) on the large and rectangular cases, and (0,0) and (.5,−.5) on the other changed cases. The third upscaler has source and mask inputs only. The first and third match all pixels in 20 of 21 renders. Both first differ on the 32×17 U16 source at their U8 image inputs. The source matches 702/1,024 within two levels, and the third upscaler's mask matches 679/1,024 within one. For the second upscaler the final Min blend also differs on power-of-two inputs. At 64²/181 both of its 128² U8 inputs match but 2,126 final F32 pixels differ by at most 5.96046448e-8.

### Emboss lighting and physical pixel size

An original height impulse at (16,16), intensity 1, light angle zero and a .5 gray base gives .625 at the impulse and .375 immediately to its right. The pixel to its left equals .5. For a general height H use dx=H(x,y)−H(x−1,y), dy=H(x,y)−H(x,y−1). A tiled axis wraps and an untiled axis clamps. Scale intensity by min(width,height)/256. Multiply cosine and sine of the angle in turns by the respective physical pixel sizes, then by that scale. Use F32 radians. Round the x product and accumulate the y product with one fused multiply-add. A right-angle rotation retains the small computed cosine.

For each RGB channel, accumulate light×highlight on the base when light is positive, or light×(1−shadow) when negative, with one fused multiply-add. Preserve the base alpha. A gray base uses the arithmetic mean of the three highlight or three shadow components before calculating the signed contribution. At 32²/277 use a .5 base, the impulse above, highlight (.75,.25,.125,.5) and shadow (.125,.875,.375,.25). The impulse and its right neighbor have raw F32 values .546875 and .4322917. Swapping single-channel red, green and blue tints leaves the gray output identical.

| Original controls | Renders | Exact pixels before the corresponding change | Exact pixels after / compared | Maximum error after |
|---|---:|---:|---:|---:|
| Gray ramps, steps, impulses and modular fields | 192 | 86,649 | 330,240/330,240 | 0 |
| RGBA bases and independent colored tints | 128 | 207,552 | 425,984/425,984 | 0 |
| Rectangular outputs and four tiling modes | 96 | 17,223 | 147,456/147,456 | 0 |
| Colored tints over a gray base | 32 | 31,026 | 81,920/81,920 | 0 |
| Independent tint and base values | 32 | 31,363 | 81,920/81,920 | 0 |

For the first 192 renders use the horizontal and vertical ramps x/(w−1), y/(h−1), a center impulse, a horizontal step and H=((53x+71y+19xy+23) mod 509)/508. Use .5+(H−.5)/32 for the low-amplitude field. Bases are .25, .5, .75 and H. Compare default intensity 5 and 0, .25, .625, 1, 1.375 and 2. Use axial angles 0, .25, .5 and .75 turns and oblique angles .125 and −.1875. Vary both physical axes between .1 and 4 and use (.5,2) to distinguish them. Compare 16², 32² and 64² with seed 97+29j in field order. Read F32 throughout and U8, U16 and F16 on the 64² H base. Use F32 and U16 heights. For the rectangles use 32×64 and 64×16, the .5 base and H height, angles 0, .25 and .1875, physical size (.5,2), tiling masks 0 through 3, four output formats and seed 563.

For RGBA use the [independent modular color field](#filtering-unequal-inputs-to-a-blend) with s=193 and j=2. Compare the step, impulse, H and low-amplitude heights at 32² F32 and at 64² U16, F16 and F32. Use intensity 2, angles 0 and .1875, and the default, custom, reversed and alpha-zero tints. For gray tint tests repeat the impulse at 32² and the low-amplitude field at 64² in all four formats. Use seed 277 with the constant base, or 457 with H and four independent tint pairs. One such pair is (.147,.913,.718,.0625) and (.796,.344,.521,.875). The 480 original controls match **1,067,520/1,067,520 raw pixels**, maximum error zero.

Curvature now matches **175,774/176,384** final pixels across the [20 normal-field renders](#curvature-from-original-normal-fields), up from 2,025. All U8, U16 and F16 samples and 36,510/37,120 F32 samples match. The largest F32 difference is .0001068115234375. The two emboss branches use a dynamic physical pixel size of 204.8/side. Applying the measured lighting to the reference's two incoming height fields reproduces all 88,576 U16 branch pixels from the eleven additional cases. The one remaining end-to-end difference occurs on the 32×17 stepped input at 32². The first different stage is its RGBA16 image input, which matches 201/1,024 pixels within one level, just as in [smooth curvature](#filtering-unequal-inputs-to-a-blend).

### Filtering unequal inputs to a blend

An original 2×2 grayscale field with row-major values (0,.25,.75,1) distinguishes direct 8× enlargement from three successive 2× filters. Connect it to one port of a 16² Copy blend, seed 317. Read the foreground at full opacity, the background at zero opacity, or a mask between white and black. At (2,7) and (6,12) the U16 foreground produces levels 26,112 and 42,495. An F32 mask produces levels 102 and 165 in U8. The comparable U16 background produces F16 values .3984375 and .64794921875.

For larger jumps, each axis takes a factor-two step while its distance to the destination exceeds one doubling or halving. The final pass filters both axes to the destination. For example, an 8×16 input to 64² visits 16×32 then 32×32 then 64×64. In weighted blends each resized port uses the higher precision of that port and the destination, including the final filtered image. The order observed is U8, U16, F16, F32. An unmasked grayscale Copy at full opacity, a background at zero opacity and the selected input of a full-opacity Switch use destination storage for the copied input. Rounding the unrelated port to the same higher precision gave a different result in the independent mixed-format controls.

| Original controls | Renders | Exact pixels before | Exact pixels after / compared | Maximum error after (integer levels or native float units) |
|---|---:|---:|---:|---|
| Individual foreground, background and mask impulses | 3 | 68 | 768/768 | 0 |
| Five layouts and individual ports | 152 | 59,404 | 164,864/164,864 | 0 |
| Independent rectangular gray and RGBA fields | 160 | 33,516 | 174,592/174,592 | 0 |
| Ports with different source formats | 48 | 19,980 | 49,152/49,152 | 0 |
| Copy, Add, Add Sub and Switch at three opacities | 144 | 93,665 | 147,455/147,456 | LUM16 1 |
| Inputs passed through intermediate nodes at different sizes | 96 | 293,493 | 381,372/393,216 | LUM16F 0.083984375, LUM32F 0.0839429498, LUM16 5501, LUM8 22 |

For the gray controls, set H=((97x+71y+13xy+11s) mod 1021)/1020 with s=107. Use 2.5H−.75 for signed fields. Compare 8²→32², 64²→16², 8×16→64², 64×8→32² and 32²→32². Repeat the 8²→32² and 64²→16² foregrounds at seed 391. In the independent inputs use 4×32→64², 64×4→16², 128×32→8², 8²→32² and 16×64→4², opacity .375 and seed 431. For RGBA output, add 57j+79c+5xc inside the numerator of H for port j and channel c. Masks and gray controls use s=431+17j. Compare all four source and destination formats. At 8²→32² additionally set the resized and other port formats to (U8,F32), (F32,U8), (U16,F16) and (F16,U16). Use s=551+73j at opacity .375. For the 144 mode controls use U16 and F32 sources, seeds 613 and 719, three output formats, and opacity 0, .375 or 1. One U16 pixel still differs by one level. The 96 rerenders with intermediate nodes have residuals on 32² and 128² inputs to a 64² blend.

The following root results include both the blend change and the [local pattern seed change](#local-seed-expressions-in-pattern-graphs). Use the [spot, brick, cell and cloud inputs](#random-draws-in-pattern-properties-and-iterations) and the [normal fields](#curvature-from-original-normal-fields).

| Root | Renders | Exact pixels before both changes | Exact pixels after / compared | Maximum U8 / U16 error (levels) | Maximum F16 / F32 error |
|---|---:|---:|---:|---:|---:|
| Black and white spots 1 | 19 | 27,157 | 131,316/172,288 | 1 / 5 | 0.00048828125 / 6.47306442e-05 |
| Black and white spots 3 | 19 | 34,974 | 124,148/172,288 | 1 / 3 | 0.00048828125 / 5.4359436e-05 |
| Brick 1 | 19 | 170,632 | 170,632/172,288 | 0 / 1 | 1.52587891e-05 / 1.52587891e-05 |
| Brick 2 | 19 | 171,992 | 171,992/172,288 | 0 / 1 | 6.10351562e-05 / 1.52662396e-05 |
| Brick 3 | 19 | 51,064 | 168,676/172,288 | 0 / 3 | 0 / 1.52587891e-05 |
| Cells 1 | 19 | 171,491 | 171,491/172,288 | 0 / 3 | 0.00048828125 / 3.67760658e-05 |
| Clouds 1 | 19 | 122,064 | 136,408/172,288 | 1 / 11 | 0.00048828125 / 0.000112771988 |
| Clouds 2 | 18 | 138,259 | 154,536/155,904 | 1 / 3 | 0.00048828125 / 2.30669975e-05 |
| Creased | 19 | 112 | 243/172,288 | 99 / 40,002 | 0.387939453 / 0.483289391 |
| Crystal 1 | 19 | 146 | 99,700/172,288 | 46 / 11,996 | 0.183105469 / 0.451787651 |
| Crystal 2 | 19 | 113 | 113/172,288 | 151 / 58,015 | 0.592163086 / 0.821276069 |
| Curvature smooth | 20 | 54,757 | 176,030/176,384 | 0 / 0 | 0 / 3.05473804e-05 |

Smooth curvature matches all samples in 19 of its 20 renders. Its remaining 354 differences occur only on the 32×17 stepped input rendered at 32². The first different stage is the RGBA16 input bridge, where 201/1,024 pixels match and the largest channel error is one level. At (0,0) the first two channels read 14,335 and 16,383 in the reference app and 14,336 and 16,384 in MaterialCraft. All 139,264 U8, U16 and F16 final samples match. In Brick 3 the complete U8 and F16 outputs also match. Creased and Crystal 2 still diverge at the first pattern stage.

### Curvature from original normal fields

| Root | Exact pixels / compared | Maximum U8 / U16 error (levels) | Maximum F16 / F32 error |
|---|---:|---:|---:|
| Curvature | 2,025/176,384 | 22 / 5,756 | 0.087890625 / 1.70635545 |
| Curvature smooth | 54,757/176,384 | 1 / 7 | 0.00048828125 / 6.10351562e-05 |

For each root combine the four archive cases and five [inventory renders](#four-format-inventory-comparison) with eleven original normal-map renders. At centered UV coordinates let a=.5sin(2πu)+.25cos(4πv), b=.5cos(2πv)−.25sin(4πu) and z=sqrt(max(0,1−a²−b²)). Encode (a,b,z) as .5+.5×component in RGBA32F. Alpha is ((19x+43y+7xy) mod 255)/254. The stepped field substitutes a=((11x+5y) mod 13−6)×.09375 and b=((3x+7y) mod 9−4)×.125. The flat field sets a=b=0. In the signed case multiply encoded RGB by 2.5 and subtract .75. Read every final grayscale sample at its declared storage precision.

| Input and output | Size / seed | Curvature intensity / invert green / normal format |
|---|---|---|
| Wave, F32 | 16² / 53 | Defaults |
| Wave, all four output formats | 64² / 157 | 1.375 / false / 1 |
| Wave, F32 | 64² / 157 | .625 / true / 0 |
| Steps, F32 | 128² / 239 | 2.75 / true / 1 |
| Flat, F32 | 32² / 389 | Defaults |
| Signed wave, F32 | 32² / 467 | .1875 / false / 0 |
| 32×17 stepped input, F32 | 32² / 523 | 2.75 / true / 1 |
| Wave, F32 | 64² / 787 | 1.375 / false / 1 |

Curvature smooth exposes only the normal format. Both roots match all 1,024 flat-control samples. In the 64²/157 F32 curvature graph, both transformed normal maps and both grayscale channel extractions match all 4,096 pixels apiece. The uniform background matches all 1,024 pixels. Both emboss outputs differ at every pixel, with U16 errors up to 673 and 2,876 levels respectively. This isolates the first difference before their final blend.

In the smooth graph, both 64² channel extractions match exactly. A 16² U16 blend of a transformed channel with the 64² grayscale source matches 240/256 pixels, maximum one level. Both inputs to this blend match before resizing. At this setting the final F32 output matches 2,440/4,096 with maximum error 4.57763672e-5. Both roots remain open.

### Local seed expressions in pattern graphs

In a clean-room control, a single original square records three successive opacity draws in RGB with alpha 1. Give its quadrant, iterate or switch a local integer seed of 43. A visit with seed 7 starts the stamp and child property streams at 50 when it inherits the incoming seed and at 43 when it resets. The first opacity draw has F32 bits 3f4fd146 and 3e98dd54 respectively. A random draw inside the local seed expression itself uses the incoming visit seed. Its update is s=(16,294,893×s+10,493,233) mod 2²⁴ and its value is s/2²⁴. The first draw at 7 has bits 3ed93158. An assignment in this expression is visible to subsequent properties on the same stamp.

For the first group, render a 16² F32 square at seeds 0, 2, 7, 43, 317, 65,537 and 1,048,575. Set the local seed after recording the three expression draws and use inheritance both ways. On a nested graph use quadrant slot 3 or the repeated iterate child, at 64² and seeds 0, 2, 7, 43 and 317. At the same five seeds compare literal and computed 43 on the original 16² stamp. In separate controls read the integer variable `$randomseed` or test whether it is defined inside the expression and the opacity. The getter returns zero and the binding is absent on the tested pattern visits. A positive constant of 43 and two distinct defined-variable marker values distinguish these observations from a failed expression. Read all four channels from the raw output.

| Original F32 controls | Renders | Exact pixels before | Exact pixels after / compared | Maximum error before | Maximum error after |
|---|---:|---:|---:|---:|---:|
| Seed function on a quadrant, iterate or switch | 42 | 8,736 | 10,752/10,752 | 0.93937701 | 0 |
| Nested visits and literal or computed seeds | 60 | 151,520 | 168,960/168,960 | 0.959448397 | 0 |
| Assignment read on the same stamp | 10 | 1,600 | 2,560/2,560 | 1 | 0 |
| Integer seed getter on the stamp and repeated child | 40 | 7,456 | 10,240/10,240 | 0.870084286 | 0 |
| Constant, integer getter and defined-variable controls | 30 | 6,432 | 7,680/7,680 | 317 | 0 |

All 182 renders match 200,192/200,192 pixels. The 18 independent iteration cases in four output formats and 72 property controls also retain all 129,024 exact pixels. In Brick 3 at 64²/223, the two 32² U16 pattern maps improved from 64 to 1,024 and from 840 to 996 exact pixels. The maximum errors after the change are zero and one integer level. At the same setting the Crystal 1 pattern map improved from 0 to 4,067/4,096, with maximum error one level. Both roots have remaining differences downstream.

### Random draws in pattern properties and iterations

A pattern property starts its own random sequence at the visit seed. For each draw use s=(A×s+C) mod 2²⁴, then return floor(s/2)/2²³. Three draws recorded as the R, G and B channels of an original square stamp distinguish the following sequences. Seeds 0 and 1 constrain the missing low bit of each candidate state. Seeds 2, 5, 42, 223, 743 and 65,537 independently reproduce all three channels.

| Property | A | C |
|---|---:|---:|
| Opacity | 3,626,811 | 51,137 |
| Branch offset | 14,154,961 | 50,989 |
| Pattern offset | 1,823,937 | 50,707 |
| Pattern size | 5,812,301 | 50,411 |
| Rotation | 3,470,325 | 50,131 |
| Variation | 12,382,630 | 50,647 |
| Iteration count | 9,501,597 | 50,591 |
| Switch | 14,464,988 | 50,111 |

For offset, size, rotation and variation, assign the three draws to a float4 variable with alpha 1 and return the property's ordinary typed value. Read the variable from the opacity of the child square in the upper-left quadrant of a 16² F32 image. A preceding opacity function can consume three further draws without changing the offset, size or rotation sequences. Use the same assignment on the iteration count and switch, return 1 or false, and read the initial or left child.

The initial iterate output receives the visit seed. Each repetition advances it once with A=8,532,665 and C=14,533. Starting from zero the first four repeated children receive 14,533, 4,831,522, 11,773,527 and 8,285,092. To read each child's seed, draw a square with three independent opacity draws. Set its width to .75/count, height to .5 and horizontal offset to (ordinal+.5)/count−.5. Test the root and initial output and 1, 2, 4, 16 and 64 repetitions in F32 at 64² with seeds 0, 1, 2, 42, 223 and at 128² with seed 743. Repeat the initial, four- and sixteen-child cases at 64² with seed 7 in F32 and 317 in four formats, and at 128² with seed 1,297 in F32. Test quadrant slots 0, 3 and the nested pair 1 then 2, a reset to 317, and a computed count of floor(rand(1))+4, using 32²/11, 64²/107, 64²/65,537 and 64²/1,048,575.

| Original pattern controls | Renders | Exact pixels before | Exact pixels after / compared | Maximum error after |
|---|---:|---:|---:|---:|
| Iteration output and counts 1–64 | 42 | 170,496 | 258,048/258,048 | 0 |
| Iteration format and seed controls | 18 | 69,120 | 110,592/110,592 | 0 |
| Quadrant placement and property combinations | 48 | 264,865 | 294,912/294,912 | 0 |
| Nested quadrants, initial output and seed reset | 20 | 41,600 | 66,560/66,560 | 0 |
| Three recorded draws per property | 72 | 17,408 | 18,432/18,432 | 0 |
| Count and switch properties, including a computed count | 20 | 11,392 | 17,408/17,408 | 0 |

All 37 archive renders across eight previously matched roots that use iteration or property functions still match, totaling **827,392/827,392 pixels** with maximum error zero. These include both brick tile helpers, position debug, Checker 2, histogram compute, paths polygon, Weave 3 and White Noise.

The following roots use their own procedural fields. The [spot and brick settings](#spot-and-brick-pattern-generators) give the first five matrices. For the other six, add ten renders to the archive and [four-format inventory](#four-format-inventory-comparison). Use defaults at 16²/83, then 32²/139, 64²/223, 64²/743, 128²/293, 32²/419 and 32²/577. Render F32 throughout and all four formats at 64²/223. The two 64² seeds share their parameters. In order of the five changed configurations, Cells 1 uses (distance, disorder) of (3,9.375), (7,37.5), (19,68.75), (1,0) and (31,100). Set disorder to .125, .4375, .6875, 0 and 1 on Clouds 1, Creased and Crystal 1. Set warp intensity to 1.75, 11.5, 37.25, 0 and 63.5 on Creased and both crystals. Clouds 2 exposes no numeric setting.

| Root | Renders | Exact pixels before | Exact pixels after / compared | Maximum U8 / U16 error after (levels) | Maximum F16 / F32 error after |
|---|---:|---:|---:|---:|---:|
| Black and white spots 1 | 19 | 108 | 27,157/172,288 | 17 / 5,193 | 0.0673828125 / 0.081654489 |
| Black and white spots 3 | 19 | 122 | 34,974/172,288 | 1 / 857 | 0.00048828125 / 0.00813311338 |
| Brick 1 | 19 | 170,632 | 170,632/172,288 | 0 / 1 | 1.52587891e-05 / 1.52587891e-05 |
| Brick 2 | 19 | 171,992 | 171,992/172,288 | 0 / 1 | 6.10351562e-05 / 1.52662396e-05 |
| Brick 3 | 19 | 51,048 | 51,064/172,288 | 145 / 38,763 | 0.569335938 / 0.730708778 |
| Cells 1 | 19 | 143 | 171,491/172,288 | 0 / 3 | 0.00048828125 / 3.67760658e-05 |
| Clouds 1 | 19 | 113 | 122,064/172,288 | 1 / 11 | 0.00048828125 / 0.000112771988 |
| Clouds 2 | 18 | 106 | 138,259/155,904 | 1 / 3 | 0.00048828125 / 4.58359718e-05 |
| Creased | 19 | 108 | 112/172,288 | 139 / 38,215 | 0.544921875 / 0.545176744 |
| Crystal 1 | 19 | 164 | 146/172,288 | 237 / 64,404 | 0.929046631 / 0.929198672 |
| Crystal 2 | 19 | 87 | 113/172,288 | 151 / 58,015 | 0.592163086 / 0.821276069 |

Cells 1 matches all 8,192 U8 pixels. At 64²/223 the U16 pattern map matches 4,077/4,096 with maximum error one level. At the same setting the three spot-1 pattern maps match 229/256, 3,731/4,096 and 967/1,024 with maximum one level each. The final additive blend combines 16² and 64² inputs. Starting with the reference's two intermediate images, two successive bilinear doublings reconstruct its 4,096/4,096 output levels exactly. Direct 4× bilinear enlargement reproduces 847/4,096, maximum error 2,867 levels. Bricks 1 and 2 retain their measured [edge-coordinate differences](#brick-edge-distances-and-storage-precision).

For Clouds 1, the three pattern maps at 64²/223 match 1,023/1,024, 254/256 and 4,074/4,096 U16 pixels, all within one level. For Clouds 2 they match 1,022/1,024, 4,080/4,096 and 256/256. The first pattern maps of Creased and both crystals still differ substantially at the same seed. Crystal 1 also loses 18 exact pixels across its root matrix. All eleven roots remain open.

### Brick edge distances and storage precision

To isolate the one-level differences in the [brick renders](#spot-and-brick-pattern-generators), read the pattern map at F32 before the final transform. Compare the same node at U16 with its normal output wiring. Use the 32²/139, 64²/223 and 128²/293 smooth settings in the linked matrix.

| F32 pattern map | 32² output, 32² map | 64² output, 32² map | 128² output, 64² map | Largest F32 error |
|---|---:|---:|---:|---:|
| Brick 1 | 940/1,024 | 584/1,024 | 2,180/4,096 | 5.72204590e−6 |
| Brick 2 | 992/1,024 | 620/1,024 | 2,072/4,096 | 9.53674316e−7 |

For both implementations separately, the standard U16 storage conversion of these F32 maps predicts 12,288/12,288 stored pixels, maximum zero. Clamp to [0,1], discard bits below 2⁻²⁰, multiply by 65,535 and round to the nearest integer with ties down. At (7,1) in the 32² map of the 64² first brick, the two F32 edge values are 3cdff221 in the reference and 3cdff1c5 in MaterialCraft. Those values store as levels 1,792 and 1,791.

An original single-stamp graph separates coverage from placement. Use an unrotated grayscale maximum-blended stamp with smoothness .375. Render the following (width, height, center offset) tuples at 16²/seed 83, 32²/139, 64²/223 and 128²/293. Offsets are from (.5,.5).

| Width | Height | Center offset |
|---:|---:|---|
| .921875 | .421875 | (0,−.25), (.5,.25) or (0,0) |
| .9609375 | .2109375 | (0,−.375) |
| .4609375 | .2109375 | (−.075,.125) |
| 1 | 1 | (0,0) |

Keep each footprint and replace the beveled stamp (shape 9) with a pyramid (shape 8). Read horizontal edge distance as the maximum of each pyramid column and vertical distance as the maximum of each row. For each axis let t=clamp(distance×float32(1/.375),0,1), s=float32(t²), c=float32(s×t) and evaluate 3s−2c with a rounded 3s. Multiplying the two axis fades reconstructs **130,560/130,560 F32 bevel pixels**, maximum zero. Computing s×(3−2t) instead gives 112,795/130,560. MaterialCraft already uses the measured edge polynomial. Its local stamp coordinates are the remaining difference in these single-stamp renders.

### Spot and brick pattern generators

Each root generates its own grayscale input. The 19 renders per root combine four archive cases, the five [inventory renders](#four-format-inventory-comparison) and ten further settings. Across the 19, each root produced 8,192 U8, 122,880 U16, 8,192 F16 and 33,024 F32 pixels.

| Root | Exact pixels / compared | Maximum U8 / U16 error (levels) | Maximum F16 / F32 error |
|---|---:|---:|---:|
| Black and white spots 1 | 108/172,288 | 166 / 48,142 | 0.650512695 / 0.782735884 |
| Black and white spots 3 | 122/172,288 | 144 / 54,770 | 0.564941406 / 0.67150037 |
| Brick 1 | 170,632/172,288 | 0 / 1 | 1.52587891e-05 / 1.52587891e-05 |
| Brick 2 | 171,992/172,288 | 0 / 1 | 6.10351562e-05 / 1.52662396e-05 |
| Brick 3 | 51,048/172,288 | 145 / 38,763 | 0.569335938 / 0.730708778 |

| Size / seed | Spots, disorder | Bricks 1 and 2, tiling / smoothness / gap | Brick 3, tiling / roundness / luminance variation / luminance seed / random position / disorder |
|---|---:|---|---|
| 16² / 83 | Default | Defaults | Defaults |
| 32² / 139 | .15625 | 1 / .125 / .0625 | 1 / .1875 / .25 / 7 / 1 / .0625 |
| 64² / 223 and 743 | .4375 | 2 / .375 / .15625 | 2 / .375 / .625 / 17 / 2 / .3125 |
| 128² / 293 | .8125 | 3 / .6875 / .25 | 3 / .625 / .8125 / 67 / 4 / .6875 |
| 32² / 419 | 1 | 2 / 0 / .3125 | 2 / 0 / 0 / 31 / 2 / 0 |
| 32² / 139, flat control | 0 | 1 / 0 / 0 | 1 / 0 / 0 / 1 / 1 / 0 |

Render F32 for every row and seed. Also render U8, U16 and F16 at 64²/223. Read each pattern map before its first downstream image operation. At this setting the three spot-1 maps match 0/256, 0/4,096 and 93/1,024 U16 pixels respectively. The three spot-3 maps match 0/1,024, 2/65,536 and 21/256. All three maps already differ with their default settings at 16².

At 64²/223 the 32² U16 pattern maps before the final transform of bricks 1 and 2 match 952/1,024 and 1,016/1,024 pixels respectively, with a maximum error of one level in each. Both maps match all 256 default pixels at 16², and both match all 1,024 pixels in the flat control. The third brick graph has two pattern maps. At 64²/223 they match 64/1,024 and 840/1,024 U16 pixels, with maximum errors of 32,260 and 8,778. Both match all 1,024 pixels in the flat control. These readbacks separate the first two bricks' small edge differences from the third brick's variable placement and luminance.

### Beveled square, gradient and wave stamps

The pattern Tile Generator matches **217,222/226,048** pixels in 43 renders. Before the profile changes it matched 182,039/226,048. U8 matches 16,381/16,384 and U16 matches 162,929/164,608, each with maximum error one level. F16 matches 15,234/16,384 with maximum .0009765625. F32 matches 22,678/28,672 with maximum 3.33786011e−6. The root remains in progress.

Start with the [four-format inventory inputs](#four-format-inventory-comparison) and add 34 renders. Use defaults, then set X Amount and Y Amount to (1,1) and (2,2). For these three configurations run 16² U16, 32² F32 and 64² in four formats. With (2,2) select pattern identifiers 2, 3, 4, 5, 6, 8, 10 and 11 individually, at 32² F32 and 64² U16. In the order given, the eleven configurations use seed 111+19j starting at j=0.

| Single-stamp controls | Renders | Exact stored pixels / compared | Largest stored error¹ |
|---|---:|---:|---:|
| Beveled square | 119 | 451,766/578,816 | 1 |
| Horizontal and vertical gradients | 40 | 116,093/163,840 | 1 |
| Wave frequency at eight variations | 8 | 43,727/131,072 | 1.93715096e-06 |
| Translated and out-of-range bevel and wave | 24 | 74,072/98,304 | 1 |

¹ Integer errors use stored levels. The bevel’s maximum F32 error is 3.81469727e−6, the gradient’s is 5.96046448e−8 and the wave’s is 1.93715096e−6 in the first three sets, respectively.

For shape 9 the two axes fade independently. On each axis take t=clamp((1−abs(x))/variation,0,1) and apply 3t²−2t³, then multiply the axis results. A nonpositive variation produces a flat stamp. At local coordinates (−.984375,0) and (−.984375,−.984375) with variation .0625, the stored F32 values have bits 3e200000 and 3cc80000. The independent bevel controls use variations 0, .0625, .125, .25, .375, .5, .75, .875 and 1 with seeds 133+11j. Render a unit stamp at 16² and 32² F32, 64² in four formats and 128² F32. At 0, .25, .5 and 1 add a quarter-turn and a (.875,.625) stamp at .0625 turns with offset (.125,−.1875).

For shape 10, scale the horizontal ramp (x+1)/2 by 1−max(variation,0). In an additive F32 output, variation 1.25 produces a negative ramp. At 64², compare zero turns and .25 turns for variations −.25, 0, .0625, .1875, .25, .375, .5, .75, 1 and 1.25, seeds 101+7j. Under maximum blending the reference’s zero-variation ramp, scaled and clipped for each variation, predicts all 81,920 F32 texels. The full renderer comparison includes placement error.

For shape 11 the radial term is max(0,1−x²−y²). In eight 128² F32 unit-stamp fields, multiplying the measured column waveform by the reference radial stamp gives 131,072/131,072 exact pixels. Use variations 0, .0625, .1875, .25, .375, .5, .75 and 1, seeds 463+37j. The fitted waveform uses 1.5+6×max(variation,0) cycles per local x unit, a periodic triangle, and a parabolic correction. Local waveform rounding and absolute placement still differ. The separate 64² U16/F32 cases cover −.25 and 1.25, offsets (.0078125,.0078125) at .0625, .375 and .71875, and size (1.375,.6875) at offset (−.0625,.125) and variation .3125. Use seeds 181+19j.

The [radial profile comparisons](#radial-stamp-profiles-and-pattern-generators) give the previous placement controls. At 64² and 128², float32(2(i+.5)(1−2⁻²⁴)/N−1) predicts the unit gradient coordinates. Four of the 32 axis values at 32² still disagree. Rotated and translated coordinates remain open.

### Parquet 1 and outstanding inventory controls

Parquet 1 matches all 289,024 stored pixels across 25 renders. The maximum error is zero in every format.

| Output | Renders | Exact / compared |
|---|---:|---:|
| LUM8 | 8 | 140,288/140,288 |
| LUM16 | 4 | 25,600/25,600 |
| LUM16F | 4 | 25,600/25,600 |
| LUM32F | 9 | 97,536/97,536 |

Use the four original archive cases, the [64² format sweep and independent 32² case](#four-format-inventory-comparison), then the following sixteen original parameter cases. Each row lists (Tiling, Luminance_Variation), square output side and seed. The graph has no input pins.

| Tiling | Luminance variation | Side | Seed | Outputs |
|---:|---:|---:|---:|---|
| 1 | 0 | 32 | 31 | F32 |
| 1 | .125 | 64 | 191 | F32 |
| 2 | .375 | 32 | 653 | U8, U16, F16, F32 |
| 3 | .6875 | 128 | 467 | U8, U16, F16, F32 |
| 4 | 1 | 64 | 977 | U8, U16, F16, F32 |
| 5 | .53125 | 256 | 131 | F32 |
| 7 | .25 | 16 | 821 | F32 |

Two other roots pass the independent 32² control and retain differences in the nine-render comparison.

| Root | Exact / compared | Maximum U8 / U16 levels | Maximum F16 / F32 |
|---|---:|---:|---:|
| `pattern_arc_pavement.sbs/arc_pavement` | 132,092/132,096 | 0 / 65535 | 0 / 0 |
| `spot_emboss.sbs/spot_emboss` | 131,394/132,096 | 4 / 411 | 0.00634765625 / 0.00627452135 |

Arc Pavement differs at four pixels in the archived U16 output. Spot Emboss differs at two pixels in each of the four changed 64² outputs, as well as in its archived U8 cases.

The table below rechecks one control for each of the 149 roots marked Too hard before the directional stamp measurements. The 32² independent input, parameter values and seed are specified in the [four-format inventory](#four-format-inventory-comparison). The one 64² row uses that root’s changed F32 case. Results use local revision `25ddf9e` except for Arc Pavement, whose matched independent control was replayed at `9ab2e6d`. The three full comparisons above use the latter revision. Later measurements of a root supersede this dated control. Every pixel of an RGBA output must match in all four channels.

| Root | Control | Format | Exact / compared | Maximum absolute error or cook result |
|---|---|---|---:|---:|
| `directionnal_noise.sbs/directionnal_noise` | 32² independent | LUM32F | 191/1,024 | 2.13086605e-05 |
| `dirt_1.sbs/dirt_1` | 32² independent | LUM32F | 1,023/1,024 | 1.52587891e-05 |
| `dirt_3.sbs/dirt_3` | 32² independent | LUM32F | 1,023/1,024 | 1.52587891e-05 |
| `dirt_6.sbs/dirt_6` | 32² independent | LUM32F | 381/1,024 | 0.121068738 |
| `dirt_gradient.sbs/dirt_gradient` | 32² independent | LUM32F | 0/1,024 | 0.481860787 |
| `fibers_1.sbs/fibers_1` | 32² independent | LUM32F | 1,008/1,024 | 1.52587891e-05 |
| `fluid.sbs/fluid` | 32² independent | LUM32F | 0/1,024 | 0.546566069 |
| `fractal_sum_base.sbs/fractal_sum_base` | 32² independent | LUM32F | 1/1,024 | 0.115129322 |
| `fur_1.sbs/fur_1` | 32² independent | LUM32F | 885/1,024 | 3.05175781e-05 |
| `fur_2.sbs/fur_2` | 32² independent | LUM32F | 6/1,024 | 7.00354576e-06 |
| `fur_3.sbs/fur_3` | 32² independent | LUM32F | 773/1,024 | 3.05175781e-05 |
| `fur_4.sbs/fur_4` | 32² independent | LUM32F | 817/1,024 | 3.05175781e-05 |
| `gaussian_spots_2.sbs/gaussian_spots_2` | 32² independent | LUM32F | 27/1,024 | 3.96370888e-06 |
| `gradient_linear_1.sbs/gradient_linear_1` | 32² independent | LUM32F | 896/1,024 | 6.67572021e-06 |
| `highpass.sbs/highpass` | 32² independent | RGBA32F | 287/1,024 | 1.1920929e-07 |
| `make_it_tile_patch.sbs/make_it_tile_patch` | 32² independent | RGBA32F | 230/1,024 | 0.535881579 |
| `make_it_tile_patch_grayscale.sbs/make_it_tile_patch_grayscale` | 32² independent | LUM32F | 230/1,024 | 0.459052384 |
| `me_water_level.sbs/scratches_2` | 32² independent | RGBA32F | 0/1,024 | 0.300794378 |
| `mesh_1.sbs/mesh_1` | 32² independent | LUM32F | 0/1,024 | 0.856000617 |
| `mesh_2.sbs/mesh_2` | 32² independent | LUM32F | 48/1,024 | 0.984206913 |
| `messy_fibers_1.sbs/messy_fibers_1` | 32² independent | LUM32F | 339/1,024 | 4.6312809e-05 |
| `messy_fibers_2.sbs/messy_fibers_2` | 32² independent | LUM32F | 3/1,024 | 0.00293257833 |
| `messy_fibers_3.sbs/messy_fibers_3` | 32² independent | LUM32F | 1/1,024 | 1.16668423 |
| `messy_fibers_4.sbs/messy_fibers_4` | 32² independent | LUM32F | 0/1,024 | 0.775814459 |
| `microscope_view.sbs/microscope_view` | 32² independent | LUM32F | 0/1,024 | 0.398368992 |
| `moisture_noise.sbs/moisture_noise` | 32² independent | LUM32F | 19/1,024 | 7.79032707e-05 |
| `mosaic.sbs/mosaic` | 32² independent | RGBA32F | 0/1,024 | 0.694117658 |
| `mosaic.sbs/mosaic_grayscale` | 32² independent | LUM32F | 11/1,024 | 0.505882375 |
| `multi_directional_warp.sbs/multi_directional_warp_color` | 32² independent | RGBA32F | 56/1,024 | 0.200000018 |
| `multi_directional_warp.sbs/multi_directional_warp_grayscale` | 32² independent | LUM32F | 611/1,024 | 0.105882347 |
| `noise_bnw_spots_1.sbs/bnw_spots_1` | 32² independent | LUM32F | — | Pattern traversal budget |
| `noise_bnw_spots_1_v2.sbs/bnw_spots_1` | 32² independent | LUM32F | — | Pattern traversal budget |
| `noise_bnw_spots_3.sbs/bnw_spots_3` | 32² independent | LUM32F | — | Pattern traversal budget |
| `noise_bnw_spots_3_v2.sbs/bnw_spots_3` | 32² independent | LUM32F | — | Pattern traversal budget |
| `noise_cells_1.sbs/cells_1` | 32² independent | LUM32F | 241/1,024 | 4.76837158e-07 |
| `noise_clouds_1.sbs/clouds_1` | 32² independent | LUM32F | 928/1,024 | 1.52587891e-05 |
| `noise_clouds_1_v2.sbs/clouds_1` | 32² independent | LUM32F | 2/1,024 | 9.65595245e-06 |
| `noise_clouds_2.sbs/clouds_2` | 32² independent | LUM32F | 1,020/1,024 | 2.28881836e-05 |
| `noise_clouds_2_v2.sbs/clouds_2` | 32² independent | LUM32F | 11/1,024 | 0.00367540121 |
| `noise_creased.sbs/creased` | 32² independent | LUM32F | 0/1,024 | 0.451300859 |
| `noise_crystal_1.sbs/crystal_1` | 32² independent | LUM32F | 1,010/1,024 | 1.92448497e-05 |
| `noise_crystal_2.sbs/crystal_2` | 32² independent | LUM32F | 0/1,024 | 0.508284688 |
| `noise_directional_noise_1.sbs/directional_noise_1` | 32² independent | LUM32F | 800/1,024 | 2.67028809e-05 |
| `noise_directional_noise_1_v2.sbs/directional_noise_1` | 32² independent | LUM32F | 0/1,024 | 0.00392669439 |
| `noise_directional_noise_2.sbs/directional_noise_2` | 32² independent | LUM32F | — | Pattern traversal budget |
| `noise_directional_noise_2_v2.sbs/directional_noise_2` | 32² independent | LUM32F | — | Pattern traversal budget |
| `noise_directional_noise_3.sbs/directional_noise_3` | 32² independent | LUM32F | — | Pattern traversal budget |
| `noise_directional_noise_3_v2.sbs/directional_noise_3` | 32² independent | LUM32F | — | Pattern traversal budget |
| `noise_directional_noise_4.sbs/directional_noise_4` | 32² independent | LUM32F | 54/1,024 | 1.78813934e-06 |
| `noise_directional_noise_4_v2.sbs/directional_noise_4` | 32² independent | LUM32F | 46/1,024 | 2.32458115e-06 |
| `noise_directional_scratches.sbs/directional_scratches` | 32² independent | LUM32F | 698/1,024 | 5.12599945e-06 |
| `noise_directional_scratches_v2.sbs/directional_scratches` | 32² independent | LUM32F | 506/1,024 | 4.62494791e-06 |
| `noise_dirt_1.sbs/dirt_1` | 32² independent | LUM32F | — | Pattern traversal budget |
| `noise_dirt_1_v2.sbs/dirt_1` | 32² independent | LUM32F | — | Pattern traversal budget |
| `noise_dirt_2.sbs/dirt_2` | 32² independent | LUM32F | — | Pattern traversal budget |
| `noise_dirt_3.sbs/dirt_3` | 32² independent | LUM32F | 50/1,024 | 3.20076942e-05 |
| `noise_dirt_3_v2.sbs/dirt_3` | 32² independent | LUM32F | 23/1,024 | 2.25305557e-05 |
| `noise_dirt_4.sbs/dirt_4` | 32² independent | LUM32F | — | Pattern traversal budget |
| `noise_dirt_4_v2.sbs/dirt_4` | 32² independent | LUM32F | — | Pattern traversal budget |
| `noise_dirt_5.sbs/dirt_5` | 32² independent | LUM32F | — | Pattern traversal budget |
| `noise_dirt_5_v2.sbs/dirt_5` | 32² independent | LUM32F | — | Pattern traversal budget |
| `noise_dirt_gradient.sbs/dirt_gradient` | 32² independent | LUM32F | 0/1,024 | 0.257863402 |
| `noise_dirt_gradient_v2.sbs/dirt_gradient` | 32² independent | LUM32F | — | Pattern traversal budget |
| `noise_fluid.sbs/fluid` | 32² independent | LUM32F | 0/1,024 | 0.332752407 |
| `noise_fractal_sum_base.sbs/fractal_sum_base` | 32² independent | LUM32F | 0/1,024 | 0.150255591 |
| `noise_fractal_sum_base.sbs/fractal_sum_base_2` | 32² independent | LUM32F | 0/1,024 | 0.121705964 |
| `noise_fractal_sum_base_v2.sbs/noise_blend` | 32² independent | LUM32F | 82/1,024 | 0.392156869 |
| `noise_fractal_sum_base_v2.sbs/noise_upscale` | 32² independent | LUM32F | 0/1,024 | 0.228064887 |
| `noise_fur_1.sbs/fur_1` | 32² independent | LUM32F | 2/1,024 | 1.6450882e-05 |
| `noise_fur_2.sbs/fur_2` | 32² independent | LUM32F | 4/1,024 | 1.35451555e-05 |
| `noise_fur_3.sbs/fur_3` | 32² independent | LUM32F | 3/1,024 | 1.21295452e-05 |
| `noise_gaussian_noise.sbs/gaussian_noise` | 32² independent | LUM32F | 46/1,024 | 3.57627869e-07 |
| `noise_gaussian_noise_v2.sbs/gaussian_noise` | 32² independent | LUM32F | 44/1,024 | 3.57627869e-07 |
| `noise_gaussian_spots_2.sbs/gaussian_spots_2` | 32² independent | LUM32F | 23/1,024 | 1.37388706e-05 |
| `noise_gaussian_spots_2_v2.sbs/gaussian_spots_2` | 32² independent | LUM32F | 2/1,024 | 1.42455101e-05 |
| `noise_liquid.sbs/liquid` | 32² independent | LUM32F | 0/1,024 | 0.209913433 |
| `noise_messy_fibers_1.sbs/messy_fibers_1` | 32² independent | LUM32F | 18/1,024 | 2.97576189e-05 |
| `noise_messy_fibers_1_v2.sbs/messy_fibers_1` | 32² independent | LUM32F | 17/1,024 | 1.94311142e-05 |
| `noise_messy_fibers_2.sbs/messy_fibers_2` | 32² independent | LUM32F | 6/1,024 | 4.4375658e-05 |
| `noise_messy_fibers_2_v2.sbs/messy_fibers_2` | 32² independent | LUM32F | 4/1,024 | 5.65648079e-05 |
| `noise_messy_fibers_3.sbs/messy_fibers_3` | 32² independent | LUM32F | 37/1,024 | 1.16653473 |
| `noise_messy_fibers_3_v2.sbs/messy_fibers_3` | 32² independent | LUM32F | 3/1,024 | 1.1268341 |
| `noise_microscope_view.sbs/microscope_view` | 32² independent | LUM32F | 0/1,024 | 0.103792138 |
| `noise_moisture_noise.sbs/moisture_noise` | 32² independent | LUM32F | — | Pattern traversal budget |
| `noise_moisture_noise_1_v2.sbs/moisture_noise_1` | 32² independent | LUM32F | — | Pattern traversal budget |
| `noise_moisture_noise_2.sbs/moisture_noise_2` | 32² independent | LUM32F | — | Pattern traversal budget |
| `noise_perlin_noise.sbs/perlin_noise` | 32² independent | LUM32F | 996/1,024 | 3.05771828e-05 |
| `noise_perlin_noise_v2.sbs/perlin_noise` | 32² independent | LUM32F | 97/1,024 | 5.66244125e-07 |
| `noise_plasma.sbs/plasma` | 32² independent | LUM32F | 1,005/1,024 | 1.52587891e-05 |
| `non_uniform_blur.sbs/non_uniform_blur_grayscale_sample` | 32² independent | LUM32F | 669/1,024 | 0.00784315169 |
| `non_uniform_blur.sbs/non_uniform_blur_sample` | 32² independent | RGBA32F | 247/1,024 | 0.00784315169 |
| `normal_to_height_2.sbs/normal_to_height` | 32² independent | LUM32F | 270/1,024 | 0.305256728 |
| `paper_path.sbs/paper_path` | 32² independent | LUM32F | 7/1,024 | 0.0057874918 |
| `parquet_1.sbs/parquet_1` | 32² independent | LUM32F | 1,024/1,024 | 0 |
| `paths_tools.sbs/paths_trace` | 32² independent | RGBA32F | 901/1,024 | 1.66666675 |
| `pattern_arc_pavement.sbs/arc_pavement` | 32² independent | LUM32F | 1,024/1,024 | 0 |
| `pattern_brick_1.sbs/brick_1` | 32² independent | LUM32F | 712/1,024 | 2.08616257e-06 |
| `pattern_brick_2.sbs/brick_2` | 32² independent | LUM32F | 444/1,024 | 1.02072954e-06 |
| `pattern_fibers_1.sbs/fibers_1` | 32² independent | LUM32F | 1,008/1,024 | 1.52587891e-05 |
| `pattern_fibers_2.sbs/fibers_2` | 64² changed | LUM32F | 3,976/4,096 | 3.05175781e-05 |
| `pattern_mesh_1.sbs/mesh_1` | 32² independent | LUM32F | 0/1,024 | 0.856000617 |
| `pattern_mesh_2.sbs/mesh_2` | 32² independent | LUM32F | 48/1,024 | 0.984206913 |
| `pattern_polygon_2.sbs/polygon_2` | 32² independent | LUM32F | 1,012/1,024 | 1.52587891e-05 |
| `pattern_shape.sbs/shape_base` | 32² independent | LUM32F | 720/1,024 | 1.37090683e-06 |
| `pattern_splatter_circular.sbs/splatter_circular` | 32² independent | LUM32F | 1,003/1,024 | 0.921568625 |
| `pattern_splatter_circular.sbs/splatter_circular_color` | 32² independent | RGBA32F | 1,003/1,024 | 0.996078431 |
| `pattern_tile_generator.sbs/tile_generator_color` | 32² independent | RGBA32F | 0/1,024 | 4.99215698 |
| `pattern_tile_random.sbs/tile_random_color` | 32² independent | RGBA32F | 735/1,024 | 1 |
| `pattern_tile_random_2.sbs/tile_random_2` | 32² independent | LUM32F | 363/1,024 | 1 |
| `pattern_weave_1.sbs/weave_1` | 32² independent | LUM32F | 0/1,024 | 1 |
| `pattern_weave_2.sbs/weave_2` | 32² independent | LUM32F | 26/1,024 | 0.780254856 |
| `pavement_1.sbs/pavement_1` | 32² independent | LUM32F | 64/1,024 | 0.718337655 |
| `pbr_render_ortho.sbs/blue_noise` | 32² independent | RGBA32F | 0/1,024 | 1.99178356 |
| `pbr_render_ortho.sbs/ibl_shadow_pass` | 32² independent | RGBA32F | 0/1,024 | 28.3294125 |
| `pbr_render_ortho.sbs/sphere_displace` | 32² independent | RGBA32F | 1,022/1,024 | 1.52587891e-05 |
| `perlin_noise_zoom.sbs/perlin_noise_zoom` | 32² independent | LUM32F | 48/1,024 | 6.2584877e-07 |
| `quantize_color.sbs/kmeans_iteration` | 32² independent | RGBA32F | 4,080/4,096 | 5.96046448e-08 |
| `radial_blur.sbs/radial_blur_grayscale` | 32² independent | LUM32F | 315/1,024 | 0.00073248148 |
| `replace_color.sbs/replace_color` | 32² independent | RGBA32F | 0/1,024 | 0.244705796 |
| `rt_caustics.sbs/rt_caustics_base` | 32² independent | LUM32F | 199/1,024 | 0.0106643774 |
| `scratches_1.sbs/scratches_1` | 32² independent | LUM32F | 508/1,024 | 1.59740448e-05 |
| `scratches_2.sbs/scratches_2` | 32² independent | LUM32F | 9/1,024 | 1.24573708e-05 |
| `scratches_4.sbs/scratches_4` | 32² independent | LUM32F | 0/1,024 | 1 |
| `shape.sbs/shape_base` | 32² independent | LUM32F | 720/1,024 | 1.37090683e-06 |
| `shape_splatter_v2.sbs/shape_splatter_compute_normal` | 32² independent | RGBA32F | 0/1,024 | 0.979619235 |
| `slope_blur.sbs/slope_blur` | 32² independent | RGBA32F | 0/1,024 | 0.76862745 |
| `slope_blur.sbs/slope_blur_grayscale` | 32² independent | LUM32F | 12/1,024 | 0.32941179 |
| `slope_blur.sbs/slope_blur_grayscale_2` | 32² independent | LUM32F | 12/1,024 | 0.32941179 |
| `splatter.sbs/splatter` | 32² independent | LUM32F | 112/1,024 | 0.475806788 |
| `splatter_circular.sbs/splatter_circular` | 32² independent | LUM32F | 959/1,024 | 1.00000006 |
| `splatter_circular.sbs/splatter_circular_color` | 32² independent | RGBA32F | 879/1,024 | 0.996078431 |
| `splatter_color.sbs/splatter_color` | 32² independent | RGBA32F | 112/1,024 | 0.484412894 |
| `spline_tools.sbs/internal_tools/modify_spline_property_sub` | 32² independent | RGBA32F | 969/1,024 | 0.261176467 |
| `spline_tools.sbs/internal_tools/points_preview` | 32² independent | LUM32F | 1,024/1,024 on the exported output | External font unavailable |
| `spline_tools.sbs/internal_tools/resample_spline_data` | 32² independent | RGBA32F | 986/1,024 | 0.5 |
| `spline_tools.sbs/internal_tools/select_spline_sub` | 32² independent | RGBA32F | 30,720/32,768 | 0.5 |
| `spline_tools.sbs/internal_tools/spline_warp_sub` | 32² independent | RGBA32F | 992/1,024 | 0.181807004 |
| `spot_emboss.sbs/spot_emboss` | 32² independent | RGBA32F | 1,024/1,024 | 0 |
| `starburst.sbs/starburst` | 32² independent | LUM32F | 796/1,024 | 0.853360829 |
| `tile_generator.sbs/tile_generator` | 32² independent | LUM32F | 862/1,024 | 3.93390656e-06 |
| `tile_generator_color.sbs/tile_generator_color` | 32² independent | RGBA32F | 494/1,024 | 1 |
| `tile_random.sbs/tile_random` | 32² independent | LUM32F | 724/1,024 | 1.00000006 |
| `tile_random.sbs/tile_random_color` | 32² independent | RGBA32F | 148/1,024 | 1.00000006 |
| `tileable_material_to_mesh_material.sbs/tileable_material_to_mesh_material` | 32² independent | RGBA32F | 477/1,024 | 5.96046448e-08 |
| `vertical_noise.sbs/vertical_noise` | 32² independent | LUM32F | 215/1,024 | 2.12192535e-05 |
| `weave_1.sbs/weave_1` | 32² independent | LUM32F | 0/1,024 | 1 |
| `weave_2.sbs/weave_2` | 32² independent | LUM32F | 6/1,024 | 0.243778139 |
| `wood_fibers_1.sbs/wood_fibers_1` | 32² independent | LUM32F | 985/1,024 | 1.93715096e-05 |
| `wood_fibers_2.sbs/wood_fibers_2` | 32² independent | LUM32F | — | Pattern traversal budget |

Of the 149 controls, three match exactly and 125 have measured pixel differences. Twenty pattern traversals reached the execution budget. One text graph requires an external font. The 20 incomplete traversals produced no comparable pixels. The font case exported 1,024 matching pixels with a cook error on its text branch and stays open.

### Uniform arc tangents and pavement rotations

Arc Pavement now matches all 132,096 pixels in the nine [previous render comparisons](#quarter-turn-stamps-and-render-bounds). The four missing pixels at y=104 in the default 256² image now match. An additional 18 renders expose differences on continuous stamp profiles and image inputs. Across all 27 renders, 202,429/211,200 pixels match. The maximum errors by format are 0 U8 levels, 12,009 U16 levels, 0 in F16, and .197752327 in F32.

The full default graph draws 322 stamps. To compare its individual parameters, initialize a counter to zero before the first Iterate, increment it once per drawn stamp, and draw the selected parameter as a one-pixel square in a 32-column grid at 256² F32, seed 0. Record the index, both offset components, both size components and the rotation. Add two to signed offsets and rotations for these reads. Before the fix the two apps agreed on every index, offset and size, but only 14/322 rotations. The maximum difference was .00001859665 turns. All six fields now match, including all 322 rotations and all 393,216 stored pixels. Replaying the twenty [isolated edge controls](#quarter-turn-stamps-and-render-bounds) matches all 1,310,720 F32 pixels.

The rotation uses the arc tangent of a two-component vector. The first-octant approximation that already matched the pixel processor also fits node parameters, value processors and pattern parameters. These three scopes divide the smaller absolute component by the larger and round each polynomial product and sum separately. Pixel processing uses its reciprocal and fused dot-product arithmetic. At (1,.1316152) the uniform result has F32 bits `3e061f75` and the pixel result has bits `3e061f76`. At (1,1) both runtime results have bits `3f490fe1`. A literal (1,1) gives `3f490fdb` in all four scopes.

| Original angle controls | Cases | Exact image pixels | Exact typed scalars | Maximum error |
|---|---:|---:|---:|---:|
| Quadrants, axes and two literal vectors in four scopes | 100 | 44,544/44,544 | 25/25 | 0 |
| Signed zeros, exponent boundaries and an ordinary off-axis vector in four scopes | 60 | 11,520/11,520 | 15/15 | 0 |
| One angle per pixel on four pattern grids | 4 | 10,240/10,240 | — | 0 |

For the first row, test the eight signed permutations of (1,.1316152), the four diagonals, the four unit axes, zero, (.379,−.811), (−.812,.323), (.015625,.0068359375), (−.875,−.296875), (2⁻²⁴,−2⁻²²) and (128,4.875). Alternate sides 16 and 32, using seed 897 + 31i. Use literal (1,1) and (.379,−.811) at 16². For the second row use 16² and seed 967+i, with (0,−0), (−0,1), (−0,−1), (−1,−0), (−t,−t), (t,m), (m,t), (1,t), (m,1), (1,m), (m,m), (2m,m), (2¹²⁶,2¹²⁵), (2¹²⁷,2¹²⁶) and (1.25,.23243791), where t=2⁻¹²⁷ and m=2⁻¹²⁶. The large final power-of-two pair distinguishes scalar division from the rounded pixel reciprocal.

For the grids use sides 64, 32, 64 and 32 and seed 719+37j. The first grid evaluates (1,(i+.5)/4096). For the other three set b=1+((7i mod 29)/16) and a=b(i+f)/N². Use (b,a), (−a,b) and (−b,−a) with f=.25,.75,.25 respectively. Draw in row order with a single Iterate and square stamps of size .875/N.

| Additional Arc Pavement controls | Exact pixels / compared | Maximum error |
|---|---:|---:|
| Defaults, square controls and varied discs | 55,552/55,552 | 0 |
| Paraboloid, bell, pyramid and capsule | 7,819/10,240 | .00000987947 F32 |
| First image, filter controls 0, 1 and 2 | 6,210/12,288 | .197752327 F32 |
| Independent image | 752/1,024 | 12,009 U16 levels |

For defaults use 16², 32² and 128² F32 at seeds 101, 271 and 401. Set scale 1, five patterns, six arcs, square width .625 and height .8125 for 64² in all four formats at seed 829. For the discs set scale 2, nine patterns, minimum three, amount randomness .375, five arcs, width .78125, height .6875, pattern scale .875, width and height randomness .125 and .25, global width randomness .1875, height decrease .375 and color randomness .625. Use 64²/43, 128²/191 and 32²/503. Change this configuration to shape 4, 5, 8 or 15 with variation .375 at seed 547, alternating 64² and 32². For the first three image controls use the square configuration with image input, variation .25 and color randomness .3125 at 64² F32/607. Bind the independent grayscale field from the [four-format comparison](#four-format-inventory-comparison). For the last image use the varied configuration with filter 0 and the original 128² grayscale ramp at 32² U16/787.

These image tests also expose a separate file-handoff difference. Reopening the [saved Arc Pavement file](#integer-defaults-in-saved-graph-interfaces) and rendering the three image-filter cases produced black. The original file has 2,026 nonzero pixels in each. Its six previously reported saved-file comparisons exercised the default and disc controls.

### Other roots using uniform arc tangents

These measurements follow the [uniform arc-tangent correction](#uniform-arc-tangents-and-pavement-rotations). For each root, replay its archived defaults, seeds and changed controls, then the [four 64² formats and the independent 32² input](#four-format-inventory-comparison). For both Stripes roots also replay the nine [hard-edge and filtered controls](#facet-masks-and-stripe-boundaries). Integer maxima count stored levels.

| Root | Cases | Exact pixels / compared | Maximum U8 / U16 | Maximum F16 / F32 |
|---|---:|---:|---:|---:|
| Bevel Smooth debug position | 13 | 246,632/246,784 | 64 / 16384 | 0.25 / 0.25 |
| Dirt 4 | 9 | No comparable pixels | — | — |
| Dirt 4, second version | 9 | No comparable pixels | — | — |
| Non-uniform blur sample, grayscale | 9 | 96,971/132,096 | 4 / 771 | 0.01171875 / 0.0117647052 |
| Non-uniform blur sample, color | 9 | 50,664/132,096 | 4 / 771 | 0.0119628906 / 0.011764735 |
| Circular splatter, grayscale | 9 | 130,917/132,096 | 223 / 57311 | 0.874511719 / 0.921568625 |
| Circular splatter, color | 9 | 130,678/132,096 | 254 / 65535 | 0.996078491 / 0.996078431 |
| Pattern Stripes | 18 | 215,036/215,040 | 255 / 65535 | 1 / 1 |
| Irradiance hit UV debug | 8 | 115,708/115,712 | 0 / 0 | 0 / 7.4505806e-09 |
| Standalone Stripes | 18 | 215,012/215,040 | 0 / 65535 | 0 / 0 |

The remaining hard edge on Pattern Stripes is (23,59) at 128². MaterialCraft returns zero and the reference returns one at that pixel in each of the four output formats. Standalone Stripes matches every hard-edge control and retains 28 differences in the archived U16 renders. Both Dirt 4 versions reach the pattern traversal budget before producing an image in all nine cases. The additional [wired irradiance controls](#environment-rotation-and-wired-irradiance-samples) include nonfinite stamp parameters and remain open.

### Runtime selection across disconnected image sockets

All 66 two-input cases in [image selection in saved graphs](#image-selection-in-saved-pattern-graphs) now match **194,304/194,304 pixels**, maximum error zero. A selector read at runtime uses the first wired image socket at or above the requested index. Requests above the last socket use the last image. With only slots 0 and 3 wired, a request for 1 or 12 samples slot 3, and −1 samples slot 0. With no images the stamp contributes nothing. A literal selecting an empty socket still contributes nothing. This resolves the 8,832 differing pixels in the previous two-pin measurement.

Use the same four 16² constant fields with byte values 31, 79, 151 and 229. Wire all four, {0,2,3}, {0,2}, {1,3}, {2}, no sockets, {0,3}, {1}, or {0}. Read an exposed integer set to −1, 0, 1, 2, 3, 4 and 12, then repeat with a literal 12. Repeat the seven runtime selections with all four sockets and packed pattern types 256, 257 and 259. The runtime index takes precedence even at zero. Across the 93 F32 renders the selected byte values agree at **23,808/23,808 pixels**. The raw float results agree at **14,848/23,808**, maximum 2.98023224e−8. The thirty earlier fixed four-input cases still match 5,632/7,680 raw F32 pixels and all 7,680 selected bytes. The remaining raw error occurs when rendering the selected constant image.

Arc Pavement retains its separate measurements of 202,429/211,200 CPU-cooked pixels and 112,896/112,896 saved-file pixels. The earlier [file handoff](#image-selection-in-saved-pattern-graphs) gives the version and parameters.

### Image selection in saved pattern graphs

When opening Arc Pavement, the first image socket is `pattern0`. An omitted `imageindex` selects that socket. MaterialCraft previously wrote an explicit 1 when saving such a graph. The reference app then sampled an empty second socket. MaterialCraft now defaults the index to zero, and its numeric image selector and SBS writer use the same slot numbers. For automation that sets `imageindex` explicitly, use 5 for the sixth connected image. A nonzero positive selector also overrides the slot embedded in pattern codes 256 through 511. A negative selector draws nothing in the measured controls.

Use one full-frame grayscale image stamp. With one 64² pin, bind a constant byte 173 or a checker alternating bytes 0 and 255. Test a white square, generic image type 1 with selectors 0, 1, 2 and omitted, packed types 256 and 257, and a graph-controlled image type with selectors 0 or 1. All 73,728/73,728 F32 pixels match over 18 renders. With two inputs, bind the same constant to the first slot and the checker to the second. Test omitted, negative, zero, one and two, three packed forms, then a variable selector reading 0, 1 and 2. For each configuration compare 16² and 32² F32 and 64² U8, U16, F16 and F32, seed 859+13j+i.

| How the slot was specified | Renders | Exact pixels / compared | Maximum error |
|---|---:|---:|---|
| Two pins, fixed selections and runtime slots zero or one | 60 | 176,640/176,640 | 0 |
| Two pins, runtime slot two | 6 | 8,832/17,664 | 255 U8, 65,535 U16, 1 F16/F32 |
| Two pins, additional packed selections at 16² F32 | 12 | 3,072/3,072 | 0 |
| Four distinct constant pins at 16² F32 | 30 | 5,632/7,680 | 2.98023224e−8 |

For the four-pin case use byte levels 31, 79, 151 and 229. At seed 885, cross pattern types 1, 256, 257, 258 and 259 with indices 0, 1, 2, 3, 4 and −1. Rounding both float outputs back to bytes gives 7,680/7,680 identical socket selections. The remaining raw float differences are in the sampled value. With two pins a runtime request for slot two samples the last pin in the reference app, while MaterialCraft produces black. This explains all differences in the 66 two-pin renders.

For saved-file fidelity, start at format version `1.1.0.202502`, save and reopen through MaterialCraft's ordinary `file.saveAs`, then open the resulting file directly in the reference app and compare it with the original in that same app. Arc Pavement matches all 112,896/112,896 pixels across the six [original handoff cases](#integer-defaults-in-saved-graph-interfaces) and all 18 [expanded root cases](#uniform-arc-tangents-and-pavement-rotations), maximum error zero. This includes the three 64² F32 image inputs that previously returned black and the independent 32² U16 input. Spot Emboss again matches all 33,792/33,792 pixels across six cases. The two files saved with their original `0.1.3` header still have the documented load failure. The separate MaterialCraft cook comparison for Arc Pavement remains at 202,429/211,200 matching pixels.

### Integer defaults in saved graph interfaces

Two packages with a `0.1.3` format header failed to reopen in the reference app after MaterialCraft Save As. Changing only that field to `1.1.0.202502` in a temporary copy let the reference app open both packages. Spot Emboss then reproduced all six original renders. Arc Pavement reproduced its default render, but six scalar integer settings had no readable default on the saved graph. The [earlier round trips](#vector-equality-lab-to-rgb-and-sbs-round-trips) cover a separate function library and a package that failed to load.

A minimal graph declared three signed integer defaults of −3, 37 and 16,777,217. With `constantValueInt32` in each input declaration the reference app reported three unset defaults and a new instance reported 0, 0 and 0. With `constantValueInt1` it reported all three original integers both on the declaration and on the instance. A separate value processor reading the largest control returned 16,777,216 with either spelling. MaterialCraft now writes the one-lane spelling for scalar integer and enumeration defaults on graph interfaces. On Arc Pavement the public interface then reported `scale=1`, `pattern_amount=12`, `pattern_min_amount=1`, `arcs_amount=14`, `pattern=2` and `input_image_filtering=0`.

For the end-to-end check, set the input package header to `1.1.0.202502`, use MaterialCraft's ordinary `file.saveAs`, then open the resulting file directly in the reference app. Compare each saved file against the original graph cooked in that same app. Use the default controls at 128² U16, the [changed controls and original input fields](#quarter-turn-stamps-and-render-bounds) at 64² in U8, U16, F16 and F32 with seed 42, and the independent fields at 32² F32 with seed 113.

| Saved graph | Renders | Exact pixels / compared | Maximum channel error |
|---|---:|---:|---:|
| Spot Emboss | 6 | 33,792/33,792 | 0 |
| Arc Pavement | 6 | 33,792/33,792 | 0 |

Ordinary saves preserve the source format version. The two observed load failures with the original header remain open. The figures in this table measure saved-file fidelity within the reference app. MaterialCraft's own render comparisons are recorded under [quarter-turn stamps and render bounds](#quarter-turn-stamps-and-render-bounds).

### Empty size-dependent pattern branches

The two black-and-white Spots 1 graphs repeatedly visit a switch whose condition depends only on the output dimensions. At 32² its selected child is disconnected. The CPU cook now remembers that decision within the current cook and skips later copies of the empty branch. It includes the branch depth when normalizing roughness. Function assignments, random draws, image sampling and per-visit inputs stay on the evaluated path. The maximum number of evaluated visits, repeated children, stamp copies and pixel samples retain their existing limits.

| Original controls | Renders | Exact / compared | Maximum error |
|---|---:|---:|---:|
| Small repeated switch | 8 | 43,520/43,520 | 0 |
| Shared switch at two depths | 8 | 43,520/43,520 | 0 |
| 1,024 × 1,024 repeated switch | 1 | 256/256 | 0 |

In a grayscale pattern graph, use a switch with only its right output connected to a unit square. Its predicate is the X component of the log₂ output size ≥ 6. For the first row, use an Iterate with an initial unit square and two repeats of an inner Iterate that visits the switch three times. Use additive drawing, white stamps and roughness zero. For the second row, use roughness one. Draw a unit square on the root quadrant, connect slot 0 to the switch, and connect slot 3 to an empty quadrant whose slot 3 reaches the same switch. Use sides 16, 32, 64 and 128, seed 373 + side, and U16 and F32 output. For the final row use the first graph at 16² F32 and seed 389, with both repeat counts set to 1,024. Previously the first sixteen renders matched and the final render stopped at the execution limit. An additional CPU regression exercises a seed expression that increments a user variable at a disconnected switch.

| Root | Exact / compared | U8 exact / total · max levels | U16 exact / total · max levels | F16 exact / total · max absolute | F32 exact / total · max absolute |
|---|---:|---:|---:|---:|---:|
| `noise_bnw_spots_1.sbs/bnw_spots_1` | 3,216/132,096 | 3,040/4,096 · 1 | 176/118,784 · 5958 | 0/4,096 · 0.0029296875 | 0/5,120 · 0.00304281712 |
| `noise_bnw_spots_1_v2.sbs/bnw_spots_1` | 35,010/132,096 | 32,285/118,784 · 16 | 851/4,096 · 146 | 1,847/4,096 · 0.00231933594 | 27/5,120 · 0.0022289902 |

Each root now produces comparable output in all nine renders. Use defaults at 128² and 256² with seed zero and at 128² with seed 42. The changed cases use scale 2, disorder .37 and roughness .6. On the second version also set disorder speed .6, anisotropy .3, anisotropy angle .125 and tile offset (.17,.29). Run the changed settings at 128²/seed zero, at 64²/seed 42 in the four formats and at 32²/seed 113 in F32. These graphs have no image inputs. At 32² the first version matches 0/1,024, maximum .00304281712, and the second matches 8/1,024, maximum .000147402287.

Of the [twenty previously incomplete pattern controls](#parquet-1-and-outstanding-inventory-controls), eighteen still reach the traversal limit at 32² on this revision. No comparison pixels are available for those eighteen.

The seventeen repeated inventory matrices returned the same per-format counts and maxima as before. The matched-resolution Position Debug preview and sixteen additional Spot Emboss renders matched all 86,528 pixels.

### Quarter-turn stamps and render bounds

For an axis-aligned rectangle whose edges pass through pixel centers, the sample on the right or bottom edge belongs to the rectangle. Rotation by whole quarter turns and a negative stamp dimension preserve this ownership in image space. At 64², a .625 × .25 rectangle centered at (.5078125,.5078125) and rotated .25 turns covers columns 25 through 40 and rows 13 through 52, inclusive. The CPU cook now uses exact quarter-turn axes and measures these boundaries in image space. A reversed render-region interval selects [0,1] on that axis. Increasing intervals retain their supplied endpoints.

| Original F32 controls | Renders | Previous exact | Current exact / compared | Current maximum absolute error |
|---|---:|---:|---:|---:|
| Square and disc | 36 | 257,761 | 258,048/258,048 | 0 |
| Pyramid and gradient | 36 | 231,067 | 233,454/258,048 | 1.1920929e-07 |
| Signed dimensions and render bounds | 16 | 64,229 | 65,536/65,536 | 0 |
| Subregion, square | 8 | 28,684 | 32,768/32,768 | 0 |
| Subregion, gradient | 8 | 23,990 | 29,042/32,768 | 1.78813934e-07 |
| Rotated squares | 78 | 4,542,662 | 4,542,719/4,542,720 | 1 |

For the first two rows, render a single untiled stamp with roughness zero, size (.625,.25) and offset (1/128,1/128) from the image center. Use shapes 2, 3, 8 and 10, turns 0, .25, .5, .75, −.25 and 1.25, and sides 32, 64 and 128. With zero-based shape and angle indices, the seed is 577 + 71 × shape index + 17 × angle index + side.

For the third row, use side 64, the same offset, shape 2 and turns 0 or .25. First negate X, Y or both stamp dimensions. Then use positive dimensions and reverse each full render axis in turn or both together. Finally crop only X or only Y to [.25,.75]. The seed is 237 + 13 × configuration index + angle index. For the fourth and fifth rows, use [3/16,13/16] × [1/4,3/4] and [5/16,15/16] × [1/8,7/8]. Reverse neither axis, X, Y or both, in that order. Use shapes 2 and 10 at zero rotation, seed 717 + 13 × configuration index + shape.

For the final row, render the following (width, height, turns, X offset, Y offset) at sides 16, 32, 64, 128, 256 and 512. The seed is 361 + 47 × configuration index + side.

(0.375, 0.625, 0, 0, 0) (0.5, 0.75, 0.125, 0, 0) (0.5, 0.75, -0.125, 0.03125, -0.125) (0.375, 0.625, 0.0625, 0.109375, -0.1875) (0.433, 0.219, 0.071, 0.127, -0.192) (0.755, 0.391, 0.13125, -0.1113, 0.1437) (0.625, 0.25, 0.25, 0.0078125, 0.0078125) (0.75, 0.5, 0.125, 0.001953125, -0.00390625) (0.8193, 0.517, 0.31, 0.0682, -0.1083) (0.75, 0.75, 0.03125, -0.05, 0.081) (0.017578125, 0.013671875, 0.075, 0.123046875, 0.0673828125) (0.355, 0.28, -0.073, 0.412, 0.327) (0.87, 0.22, 0.3125, -0.2725, 0.405).

One oblique square pixel at 512² differs by 1.0. The 256² Arc Pavement root retains four U16 differences at (17,104), (110,104), (145,104) and (238,104), each 65,535 levels. For each pixel, take the contributing square's computed center, dimensions and angle from MaterialCraft, wrap the center into the image, and render it alone in both apps. Shift its center along the outward normal towards that pixel by 0, .0004, .0008, −.0008 and .002 pixels. Both apps return zero at the target at the first two offsets and one at .0008. All 1,310,720 pixels in the twenty F32 renders agree. The native full graph still covers the four target pixels. Its computed stamp parameters and tiling remain to be compared.

| Root | Renders | Exact / compared | Maximum U8 / U16 levels | Maximum F16 / F32 |
|---|---:|---:|---:|---:|
| Gradient Linear 1 | 47 | 435,120/435,968 | 1 / 1 | 0.000244140625 / 1.52587891e-05 |
| Gradient Linear 3 | 23 | 265,072/277,760 | 0 / 2 | 0 / 3.05175781e-05 |
| Spot Emboss | 25 | 218,368/218,368 | 0 / 0 | 0 / 0 |

All three rows include the four archive renders, the [four-format sweep and independent 32² control](#four-format-inventory-comparison). For both gradients, add eight (tiling, rotation, side) groups [(1,0,16), (3,1,32), (2,0,64), (5,2,128), (1,1,256), (4,3,64), (3,0,32), (1,1,32)]. Use F32, plus U8, U16 and F16 for the second and fourth groups. The seed is 197 + 53 × group index. Gradient Linear 1 additionally uses [(1,0,16), (1,1,64), (1,2,64), (1,3,64), (2,0,32), (3,1,128), (4,2,32), (5,3,64), (7,0,128), (8,1,256), (2,2,16), (3,3,32)], seed 1991 + 47 × group index. Add U8, U16 and F16 to groups 1, 2, 3 and 6 using zero-based indices. The remaining mismatches in Gradient Linear 1 occur at rotation 3. At tiling 4, rotation 3 and side 64 the earliest observed difference is in the U16 pattern image, with 4,000/4,096 exact pixels and a maximum of one level.

For Spot Emboss, the sixteen additional renders use the following (side, seed, light X, light Y, intensity) groups [(16,37,.5,.5,5), (32,907,.25,.75,2), (64,131,.75,.125,1), (128,521,.4375,.5625,4), (64,359,.5,.5,0), (32,977,.875,.875,3), (64,419,.0625,.1875,9)]. The first group uses all default parameters. The other groups use highlight (.875,.625,.25,.75) and shadow (.125,.25,.4375,.625). Use F32 throughout, adding U8, U16 and F16 for zero-based groups 1, 3 and 5. Generate 32² input fields for even groups and 64² for odd groups. With pixel indices x, y and group index j, the U16 height is (16381 × floor(x/4) + 271y + 37xy + 421j) modulo 65536. RGBA8 is (13x+11y+19j, 23y+5x, 9xy+17j, 3x+7y+29j), each channel modulo 256. All 25 Spot Emboss renders are exact, including the previously differing [archived cases and four changed renders](#parquet-1-and-outstanding-inventory-controls).

Twenty-one previously verified roots with a reachable pattern or directional blur filter match all 3,334,656 pixels in 164 equal-resolution renders. The [controlled 128² Position Debug preview](#tinted-stamp-offsets-and-alpha) supplies the equal-resolution comparison for the capped preview.

### Tinted stamp offsets and alpha

The directional gradient and hemisphere add signed variation to each channel after tint and opacity. The hemisphere applies the offset inside its support disk. Color blend modes 1 and 2 composite the premultiplied source over the background with its signed alpha. Mode 0 adds RGB and accumulates alpha as background alpha plus source alpha times the remaining background transparency. Grayscale modes 1 and 2 take the maximum. A constant RGBA background in a grayscale graph contributes the mean of its three color channels.

| F32 single-stamp controls | Renders | Previous exact | Current exact / compared | Maximum absolute error |
|---|---:|---:|---:|---:|
| Transparent RGBA | 18 | 5,076 | 14,042/18,432 | 7.30156898e-07 |
| RGBA over a fixed background | 12 | 2,661 | 11,182/12,288 | 1.1920929e-07 |
| Square, gradient and hemisphere over a fixed background | 30 | 2,296 | 28,924/30,720 | 5.96046448e-07 |

Use a centered unit stamp at 32² with roughness zero. For the 18 transparent controls set tint (.75,.375,1,.625). Use (blend mode, global opacity) of (0,1), (0,.5) and (1,.5), seeds 431, 468 and 505, for shapes 17 and 18 at variation 0, .25 and −.125. The next twelve controls use background (.5,.375,.75,.875), global opacity .5 and color blend modes 1 or 2 at those same three variations. For the last thirty controls use the same background, global opacity .5, all three modes, and run RGBA with that tint or grayscale with a white stamp. Use shape 2 at variation zero and shapes 17 and 18 at −.125 and .875. The seed is 943+9×shape+j, with j=0 for the first variation and j=1 for the second.

For the transparent gradient at opacity .5 and variation −.125, pixel (16,16) has F32 bits bb880000, bd844000, 3d140000 and 3d140000. The unit square at opacity .5 over the stated background returns (.734375,.4921875,1.0625,.9140625) in mode 0 and (.578125,.375,.828125,.9140625) in modes 1 and 2.

Across the earlier [260 profile renders](#directional-and-additive-stamp-profiles), exact pixels increased from 727,162 to 746,795 of 892,928. RGBA16 now matches 56,868/57,344, maximum one level. RGBA32F matches 28,361/57,344, maximum 2.42888927e−6. Earlier RGBA maxima were 12,544 levels and .191407166 respectively. Lower-coverage and transformed stamps retain coordinate differences.

Fifteen previously exact inventory roots with reachable pattern filters match 2,544,128/2,544,128 pixels in 112 equal-resolution renders. For [Position Debug](#position-debug), the 16² zero-distance control has two 4096² internal images in the full graph. Its full render matches 256/256. If only MaterialCraft caps those images at 128², 236/256 output pixels match the full reference, maximum 128 U8 levels. To compare the preview, explicitly set the reference pattern filter and its source pixel processor to 128² and keep the graph at 16². All 256 output pixels then match. All seven observed node images agree on 34,048/34,048 pixels including both 128² intermediates.

### Directional and additive stamp profiles

Eight built-in shapes were measured in original single-stamp graphs. Across 260 renders, the complete renderer improved from 297,007 to 727,162 exact pixels out of 892,928. Color requires all four stored channels to match. Integer maxima are channel levels.

| Shape | Renders | Previous exact | Current exact / compared | Maximum U8 / U16 | Maximum F16 / F32 |
|---|---:|---:|---:|---:|---:|
| 7 | 22 | 64,360 | 65,705/77,824 | 0 / 1 | 3.05175781e-05 / 4.76837158e-07 |
| 12 | 36 | 33,547 | 98,446/122,880 | 0 / 1 | 0.00048828125 / 1.4603138e-06 |
| 13 | 36 | 4,828 | 91,739/122,880 | 0 / 1 | 1.90734863e-06 / 8.94069672e-07 |
| 14 | 36 | 49,550 | 102,396/122,880 | 1 / 1 | 0.000122070312 / 1.08778477e-06 |
| 15 | 36 | 68,714 | 117,354/122,880 | 1 / 1 | 0.00048828125 / 4.24683094e-06 |
| 16 | 22 | 64,624 | 65,947/77,824 | 0 / 1 | 7.62939453e-06 / 1.78813934e-07 |
| 17 | 36 | 5,559 | 90,439/122,880 | 1 / 12544 | 0.00048828125 / 0.191406265 |
| 18 | 36 | 5,825 | 95,136/122,880 | 0 / 12544 | 0.00048828125 / 0.191407166 |

Start from a single centered unit stamp with no background, unit opacity, roughness zero and blend mode 1. Declare the graph output and stamp consistently grayscale or RGBA. For variation 0, .25, .5 and 1 (index j), compare grayscale 32² F32, then 64² U8, U16, F16 and F32. Use seed 177+size+j. At .25 also compare 64² RGBA16 and RGBA32F with tint (.75,.375,1,.625). For shapes 12–15, 17 and 18, add four controls at 32² F32 and 64² U16/F32, seed 491+73j+size. Use a unit stamp at variation −.25, a (1.375,.6875) stamp at .0625 turns with offset (.109375,−.1875) and variation .3125, a (.75,1.125) stamp at .25 turns with offset (−.0625,.125) and variation .71875, then a unit stamp at offset (.0078125,.0078125) and variation 1.25. Repeat the first additional control with the same 64² color tint.

Shapes 12 and 13 multiply a signed quartic bell by a horizontal mask. For 12 the mask is 1−3u²+2u³ with u=clamp(x/(1−max(variation,0)),0,1). At variation 1 the positive half is cut off. For 13 the mask is (1−|x|)². Shape 14 multiplies the same smooth mask, using (x+1)/2 horizontally, by the normalized ring 6.75t(1−t)², where t=min(x²+y²,1). Shape 15 uses max(0,1−20(y²+max(0,|x|−.77×max(variation,0))²)). Shapes 7 and 16 use the measured pixel square root for their conical falloff.

In the scalar profile controls, shape 17 adds variation to (x+1)/2. Shape 18 adds it to the hemisphere height inside the disk and returns zero outside. Their floating outputs exceed one in the positive-variation controls. At the 64² unit-stamp coordinates from the [radial controls](#radial-stamp-profiles-and-pattern-generators), both scalar profiles predict all 4,096 samples at each of four variations. Absolute stamp placement, signed blending and the order of variation and color tint affect the full-render results. For shape 17 at (0,0), variation .25, the tinted reference red/green channels are .253662109375/.2518310546875 and the current channels are .120849609375/.0604248046875.

| Inventory root | Renders | Exact / compared | Maximum U8 / U16 | Maximum F16 / F32 |
|---|---:|---:|---:|---:|
| creased.sbs/creased | 9 | 41,224/132,096 | 1 / 632 | 0.00048828125 / 0.000534534454 |
| crystal_2.sbs/crystal_2 | 9 | 102,021/132,096 | 13 / 4663 | 0.0494384766 / 0.0494220853 |
| fluid.sbs/fluid | 9 | 295/132,096 | 52 / 33520 | 0.20111084 / 0.201334268 |
| make_it_tile_patch.sbs/make_it_tile_patch | 9 | 12,322/132,096 | 196 / 47176 | 0.719848633 / 0.719859615 |
| make_it_tile_patch_grayscale.sbs/make_it_tile_patch_grayscale | 9 | 19,086/132,096 | 144 / 26181 | 0.399658203 / 0.459052384 |
| mesh_1.sbs/mesh_1 | 9 | 38,454/132,096 | 0 / 544 | 0 / 1.52587891e-05 |
| mesh_2.sbs/mesh_2 | 9 | 34,148/132,096 | 0 / 1102 | 0 / 1.52606517e-05 |
| messy_fibers_3.sbs/messy_fibers_3 | 9 | 126,928/132,096 | 1 / 3 | 0.0009765625 / 5.23328781e-05 |
| noise_creased.sbs/creased | 9 | 37,716/132,096 | 1 / 775 | 0.00048828125 / 0.000442534685 |
| noise_crystal_2.sbs/crystal_2 | 9 | 41,321/132,096 | 1 / 620 | 0.005859375 / 0.00588941574 |
| noise_directional_scratches.sbs/directional_scratches | 9 | 111,002/132,096 | 1 / 1 | 0.00048828125 / 5.30481339e-06 |
| noise_directional_scratches_v2.sbs/directional_scratches | 9 | 129,384/132,096 | 1 / 1 | 0.000122070312 / 6.22868538e-06 |
| noise_dirt_3.sbs/dirt_3 | 9 | 106,101/132,096 | 1 / 3 | 0.00048828125 / 3.20076942e-05 |

These nine-render comparisons include the original archived defaults, seed and changed setting, the 64² four-format sweep and the independent 32² case from the [inventory measurements](#four-format-inventory-comparison). In the 32² comparison Mesh 1 improved from 0 to 1,020 exact, Mesh 2 from 48 to 992, Messy Fibers 3 from 1 to 48 and Noise Creased from 0 to 136, each out of 1,024. All thirteen roots retain measured differences.

### Radial stamp profiles and pattern generators

A single stamp in an original grayscale pattern graph isolates the local profile. For each shape, use a centered stamp of size (1,1), then a quarter-turn, then size (1.375,.6875) at .0625 turns with offset (.109375,−.1875). Render at 16² F32/seed 433, 32² F32/127, 64² in U8, U16, F16 and F32/733, and 128² F32/983. The counts compare stored pixels for all 21 renders per shape.

| Shape identifier | Previous exact | Current exact / compared | Maximum U8 / U16 levels | Maximum F16 / F32 error |
|---|---:|---:|---:|---:|
| 4 | 66,880 | 66,884/102,144 | 1 / 1 | 0.00048828125 / 1.66893005e-06 |
| 5 | 25,043 | 61,752/102,144 | 0 / 1 | 0.00048828125 / 1.04308128e-06 |
| 6 | 52,799 | 61,127/102,144 | 0 / 1 | 0.000244140625 / 1.37090683e-06 |

Shape 4 uses max(0,1−t), where t is the squared radius computed with a rounded x² and a fused y×y addition. Shape 5 uses (1−t)⁴, evaluated as the expanded polynomial in Horner order inside the unit circle. Shape 6 uses the pixel base-two exponential of −t×float32(11.1×log₂(e)), subtracts .0000151, then clamps at zero. The old shape 5 reached errors of 64 U8 levels, 16,384 U16 levels and .250000358 in F32. It now matches all 12,288 U8 pixels.

A coordinate check isolates the remaining placement error. For a unit stamp, estimate each axis as float32((i+.5)×2×(1−2⁻²⁴)/N−1), with one final rounding. At 64² the quartic and Gaussian profiles each match 4,094/4,096 F32 samples with those coordinates. At 128² each matches 16,383/16,384. The regression checks include the 64² texels (0,32), (20,16), and (32,32). The stored output comparisons above include the complete renderer and translated and rotated stamps.

| Root | Renders | Exact / compared | Maximum U16 levels | Maximum F32 error |
|---|---:|---:|---:|---:|
| Alveolus | 26 | 300,544/302,336 | 1000 | 0 |
| Gaussian 1 | 18 | 168,522/171,264 | 2 | 1.52587891e-05 |
| Gaussian 2 | 18 | 169,796/171,264 | 1 | 1.52587891e-05 |
| Gaussian spots 1 | 9 | 1,573/132,096 | 65535 | 1.16203201 |
| Noise Gaussian spots 1 | 9 | 493/132,096 | 65518 | 0.905901405 |
| Noise Gaussian spots 1 v2 | 9 | 2,984/132,096 | 65153 | 1.05418956 |
| Pattern Alveolus | 18 | 170,440/171,264 | 2 | 0 |
| Pattern Gaussian 1 | 18 | 168,522/171,264 | 2 | 1.52587891e-05 |
| Pattern Gaussian 2 | 18 | 169,796/171,264 | 1 | 1.52587891e-05 |

Each generator includes defaults at 128² and 256²/seed 0, 128²/seed 42, changed settings at 128²/0 and 64²/42 in four output formats, and 32² F32/113. The three spots generators use disorder .3, scale 2 and disorder .37, and the same scale/disorder with speed .6, anisotropy .3, angle .125 and tile offset (.17,.29), respectively. Their spatial patterns still differ across formats. All nine roots generate their own inputs.

For the two Alveolus and four Gaussian generators, add nine controls. Use defaults at 16² F32/23, 64² in four formats/107, 128² F32/347, 32² F32/457 and /521, and 64² F32/587. Alveolus uses tiling 2, filled gradients and interstice width .125, with tiling 3, empty gradients and width .25 at seed 521. Gaussian uses tiling 2, raised to 3 at 128². Disable non-square expansion at seed 457 on the three pattern-package variants. The 26 Alveolus renders include eight fresh repeats of the original four renders and the four-format controls. Its F32, F16 and U8 pixels now all match. The remaining 1,792 pixels are U16. The [earlier placement comparison](#alveolus-quadrant-placement-and-falloff) gives the nonzero quadrant offsets and intermediate output.

### Three-texel blur kernels

The bevel Gaussian blur subgraph now matches 138,496/154,880 pixels across 17 renders. All 106,496 U8, 8,192 U16 and 8,192 F16 pixels match. Of 32,000 F32 pixels, 15,616 match. The remaining case is the 128² procedural field at intensity 9, with 0/16,384 exact and maximum error .00221335888. The previous run matched 108,965/154,880 and differed by up to 393 U16 levels. The original defaults at 128² and 256², the changed intensity and all four 64² format checks now match.

| Original three-texel controls | Renders | Exact RGBA pixels / compared | Maximum error |
|---|---:|---:|---:|
| Separable blur, tiled and untiled | 104 | 1,384,448/1,384,448 | 0 |
| Horizontal directional blur | 40 | 409,600/409,600 | 0 |

When .5 < radius ≤ 1.5, the center weight is .5/radius and each edge weight is (radius−.5) times the center weight. The separable blur starts with the left tap, fuses the center and then the right tap, and stores the horizontal result in the destination format. The vertical pass starts below the pixel and ends above it. The zero-angle directional blur starts at the right tap. For the signed field below, the 64² separable output at (32,32), intensity 5, stores 50 in U8, 12,958 in U16, 3e15c000 in F16 expanded to F32, and 3e15e059 in F32. The previous U16 result was 9,592. The 64² F32 directional result at that pixel is bdfdbf73.

For a square side N, fill R with an impulse at (N/2,N/2), G with the column x=N/2 and B with the row y=N/2. Let H=((97x+43y+7xy) mod 1024)/1023. The alpha input is H rounded to the closest stored level for U8 and U16, or 3H−1.25 stored at F16 or F32 precision. Use seed 347. Radius is intensity×N/256. At 32² use intensity 9.5. At 64² use 2.25, 3.25 and 5. At 128² use 1.125, 1.5, 2, 2.5 and 3. At 256² use 1. Use F32 output plus all four formats at 64²/intensity 5. Run the separable blur with both axes tiled and both untiled. The directional comparison uses the tiled 64² intensities 2.25 and 5, and the five 128² intensities. The [wider transform and mapping comparisons](#segment-mapping-and-normal-graph-rechecks) define the sizes and seeds for the nine bevel subgraph cases. Its single-channel input is ((19x+47y+5xy) mod 257)/256, with 2.5H−.75 for signed input and a single impulse at (7,13) for sparse input. Use default intensity at 16², 5 at 64², 9 at 128², 3.25 for sparse input and .25 for signed input.

Wider kernels still differ. For an F32 impulse blurred at 128² and intensity 4, the five horizontal stripe samples centered on the impulse are .12451171875, .25048828125, .25, .25048828125 and .12451171875. Both outer texels differ from a uniformly weighted window with fractional ends.

### Small graph parents and raytrace positions

| Comparison | Renders | Exact stored pixels / compared | Maximum error |
|---|---:|---:|---:|
| First PBR renderer, raytrace position | 26 | 142,340/142,340 | 0 |
| Second PBR renderer, raytrace position | 26 | 142,340/142,340 | 0 |
| Pixel centers and dimensions, runtime parent sizes | 44 | 9,552/9,552 | 0 |
| Saved parent sizes, including a 1×1 canvas in four formats | 12 | 693/693 | 0 |

The two raytrace-position roots each matched 142,080/142,080 RGBA pixels across their 18 renders at sizes 16² through 256². An earlier 8² request produced 16² in MaterialCraft. The parent-size command, package reader, writer and cook now preserve each axis down to a single texel. A width or height in pixels rounds up to the next power of two. An explicit log2 size sets the exponent directly.

For each raytrace position, wire the original offset grayscale field to Height and the levels range field to both Opacity and Blue Noise. Run native defaults at 128² and 256² with seed zero and at 128² with seed 42. For the changed 128² case set shape 1, shape scale .7, position (.1,−.2,.3), plane size (.7,1.3), displacement .2, UV tiling 2, sphere UV scale (2,3), plane and cylinder UV scales (2,1), cylinder length 1.5, radius .7, UV offset (.125,.25), rotation .125, direction .2 and rotation around that direction .3. Repeat the changed case at 64²/seed 42 in four storage formats and include the independent 32²/seed 113 F32 case. The [input-field definitions and independent cases](#four-format-inventory-comparison) give the pixel formulas.

For the modular cases use original grayscale inputs H=((37x+41y+7xy+53j) mod 509)/508, where j is 0 for Blue Noise, 1 for Height and 2 for Opacity. Store linear RGB (H,H,H), alpha 1. Run defaults at 16²/43. At 32²/157 use shape 1, scale .8125, position (.0625,−.125,.1875), plane size (.75,1.25), displacement .125, UV tiling 1.25, cylinder length 1.5, radius .625 and UV offset (.1875,−.3125). Run all four storage formats and repeat F32 at seed 751. Use shape 2 at 64²/293 and again with 16×8 inputs at 16²/599. At 16²/463 apply 2.5H−.75, set shape 0 and UV offset (−.5,.0625).

Eight more renders of each raytrace root use the 8² modular inputs. Request 8² F32 with defaults at seed 43 and 2² F32 with defaults at seed 499. Apply the changed shape-1 settings above at 4²/571 in all four formats, 8² F16/749 and 8² F32/827. Each root matched 260/260 pixels in these small renders. Across the complete matrix each matched 5,136 U8, 5,136 U16, 5,200 F16 and 126,868 F32 pixels.

For the standalone dimension check emit (u,v,width,height) using the pixel center and the function's reported size. Request (1,8), (8,1), (2,2), (2,8), (4,4), (4,16), (8,8), (3,5), (6,9), (17,31) and (32,32) in U8, U16, F16 and F32. Use seeds 108 through 398 in steps of 29. The three non-power-of-two requests give 4×8, 8×16 and 32×32. A separately saved 1×1 graph gives (.5,.5,1,1) in each format at seed 79. For the eight saved F32 packages, use independent parent exponents (0,0), (0,3), (2,0), (1,1), (2,3), (3,3), (4,2), (5,4) and a constant scalar pixel function with output bits 3fc5672a. A transient 1×1 request on the original coordinate graph with the default parent setting produced a 2048² reference output. Those four dimension mismatches are excluded from the exact-pixel counts above.

The core, formats, engine and UI release suites, engine panic hunt, formatting, layers, strict workspace Clippy and ten WebAssembly checks passed. The offscreen toolbar regression selected 1×8, and the saved frame displayed both dimensions in Properties. The full offscreen suite passed 45/47 tests. The material assignment frame and parameter preview cook timed out on isolated reruns of both the changed and cached pre-change UI executables.

### Irradiance with a connected pass index

| Comparison | Exact pixels or typed values / compared | Maximum error |
|---|---:|---:|
| First renderer, eighth pass, original 17 image observations | 120,232/120,320 | 1 U16 level, .015625 F16, .00390625 F32 |
| Nine image observations with the index wired | 4,537/4,608 | .001953125 F16 |
| Next index, 17 default and nine wired observations | 26/26 float values | 0 |

After the [vector product rounding change](#vector-products-in-the-direct-light-pass), the default 128², seed-42 128² and default 256² renders match 16,370/16,384, 16,370/16,384 and 65,481/65,536. In the changed 64² F32 case the first half-float processor now matches 4,094/4,096. The other two processors match all their pixels. Both blends carry the two differences through to the output. The first differing red at (28,20) is still 7.48046875 against 7.4765625. All nine [changed-shape and input cases](#first-renderer-eighth-irradiance-pass) retain exact images and index values.

For the wired cases, connect a scalar Value Processor returning the exposed start value to the root’s pass-index input. Connect its two outputs directly to the wrapper outputs. Use the seven original image fields from the preceding pass comparison, selecting its first matching size. Keep the wrapped graph’s exposed settings at their defaults. Run 16² with start −1 at seed 11 and 0 at seed 27, 32² with .5 at seed 43, 16² with 3 at seed 61 and 32² with 7.25 at seeds 211 and 401. Repeat the start-3 case with each of the four wrapper format requests. The measured inner output is RGBA16F on all nine renders. Both applications return 0, 1, 1.5, 4 and 8.25 for the five distinct inputs. The two first images match 256/256 each, the .5 case matches 991/1,024, the four start-3 cases match 255/256 each and both 7.25 cases match 1,007/1,024.

Across all 26 observations the image totals are 4,352/4,352 U8, 4,351/4,352 U16, 107,108/107,264 F16 and 8,958/8,960 F32. The largest errors by format are zero, one level, .015625 and .00390625. The image output remains in progress.

### First renderer eighth irradiance pass

| Stored output | Observations | Exact / compared | Maximum error |
|---|---:|---:|---:|
| RGBA8 | 2 | 4,352/4,352 pixels | 0 |
| RGBA16 | 2 | 4,351/4,352 pixels | 1 level |
| RGBA16F | 5 | 102,563/102,656 pixels | .015625 |
| RGBA32F | 8 | 8,957/8,960 pixels | .00390625 |
| Next pass index, float | 17 | 17/17 values | 0 |

The previously timed-out default 128², seed-42 128² and default 256² renders completed. They matched 16,368/16,384, 16,368/16,384 and 65,478/65,536 RGBA16F pixels. Their maximum errors were .015625, .015625 and .0078125. The other measurements use the four 64² storage formats at seed 42, the independent 32²/113 field and nine new fields. The two outputs are compared separately. All 17 index values were 1 with the pass-index input unwired.

For the nine new fields use the seven relevant [procedural irradiance inputs](#material-bundles-and-the-eighth-irradiance-pass), connecting H to the blue-noise port. Use default controls at 16²/37. Set shape scale .8125, UV tiling 1.5 and displacement .125 for the other eight cases. Use shape 0 at 32²/173, repeat at seed 439, then use the alternate previous image at seed 173. Use shape 1 at 16²/359 with black environment and emissive. Use shape 2 at 16²/251 in all four storage formats. These inputs matched all 4,608 pixels and all nine pass-index values.

In the changed 64² F32 render all seven input images matched at their stored precision. The first half-float pixel processor matched 4,093/4,096 pixels. Its red at (28,20) is 7.48046875 in MaterialCraft and 7.4765625 in the reference. The two other processors matched all 4,096 pixels each. The two blends carried the first processor’s three discrepancies to the output. The arithmetic behind those three pixels and the earlier default-case differences remains open.

### Voronoi distortion and shared noise inputs

| Root | Stock tables, exact / compared | Stock maximum F32 error | Shared table, exact / compared | Shared maximum U8 / U16 / F16 / F32 errors |
|---|---:|---:|---:|---:|
| 3D Voronoi | 146,688/149,760 | 0.304064572 | 5,632/5,632 | 0 / 0 / 0 / 0 |
| 3D fractal | 146,688/149,760 | 0.205578864 | 5,475/5,632 | 10 / 2,458 / .03759765625 / .0571730733 |
| 2D Voronoi | 146,688/149,760 | 0.368969858 | 5,632/5,632 | 0 / 0 / 0 / 0 |
| 2D fractal | 146,688/149,760 | 0.175076187 | 5,632/5,632 | 0 / 0 / 0 / 0 |

With the two applications’ stock tables, each root matched all 146,688 pixels in the 27 renders with zero distortion intensity. The remaining three 32² F32 renders per root differ at every pixel. They enable distortion. The seven additional renders use the same original noise table in both applications. All pixels match in three roots. The 3D fractal still differs in 157 pixels with this shared input.

The 30 stock cases include the eight original 16² to 256² renders, four changed 64² formats and the independent 32² field. Add ten cases with default settings at 16²/37, four formats at 32²/139, another F32 at 32²/457, distance mode 1 at 32²/257, mode 2 at 16²/347, mode 3 at 32²/521 and a distorted 32²/373 case. For the two 3D graphs use position (u,v,.25+.5H,1), where u and v are normalized pixel centers and H=((37x+41y+7xy) mod 509)/508. Use a 32×16 source for mode 1 and scale every channel by 2.5 then subtract .75 for mode 3. The two 2D graphs have no image inputs. For the changed cases set scale 3, size (.8125,1.125,.6875), offset (.1875,−.375,.0625), disorder .6875 in 3D or (.6875,.375,.8125) in 2D, distance scale 1.125, Minkowski exponent 1.625 and style 0. The fractal levels are 1 to 3, roughness .625, lacunarity 2 and opacity .8125. The distorted case uses distance mode 0, distortion intensity .3125, distortion scale multiplier 1.5, rounded curve .3125 and inversion.

For the other seven stock controls, hold the first 32² field and seed 373 fixed. Start with the undistorted mode-0 settings. Change only intensity, only the scale multiplier, only rounded curve and only inversion to their distorted-case values, then enable both distortion controls together, and finally enable rounded curve and inversion together. Each graph matches all 5,120 pixels in the five controls with zero intensity. Both positive-intensity controls differ at 1,024/1,024 pixels.

Replace the bitmap feeding the distortion calculation with the original 16-row [shuffled table](#basic-and-ridged-3d-perlin). Keep the distorted settings and render 16² F32/97 with intensity .1875 and multiplier .75, 32²/373 in all four formats, 32² F32/607, and 16² F32/461 with the signed input, intensity .5 and multiplier 2. In the 3D fractal’s 32² F32/373 case, reading the three noise terms gives 1,024, 994 and 1,024 exact values. The middle term’s maximum error is .4943066984. It differs only in columns 0 to 3 and rows 6 to 18. The five octave outputs match 996, 996, 995, 995 and 995 of 1,024 pixels. Their combined output matches 997/1,024, maximum .0374994278. The table substitution isolates the asset-dependent differences and leaves this 3D fractal discrepancy open.

### Eighth raytraced specular passes

| Renderer | Renders | Exact RGBA pixels / compared | Maximum U8 / U16 levels | Maximum F16 / F32 error |
|---|---:|---:|---:|---:|
| First | 26 | 142,061/142,340 | 0 / 0 | .0009765625 / .000030517578125 |
| Second | 26 | 142,337/142,340 | 0 / 0 | .000244140625 / 0 |

The four original renders that exceeded the earlier time limit finished for both roots. Set the input images and changed parameters from the [four-format inventory](#four-format-inventory-comparison) for the corresponding specular pass. At 128² with defaults, 256² with defaults, 128² with seed 42, and 128² with changed settings, the first renderer matched 16,344/16,384, 65,355/65,536, 16,344/16,384 and 16,369/16,384 RGBA16F pixels. The second matched 16,383/16,384, 65,535/65,536, 16,383/16,384 and 16,384/16,384. Its three differences occur in these default-setting renders.

Add the four changed 64² output formats and the independent 32² input. For the additional fields use the [normal, position and modular channels](#normal-maps-panoramas-and-primitive-shapes). Order the eight inputs by identifier. Use defaults at 16²/seed 43. At 32²/157 use shape 1, shape scale .8125, plane size (.75,1.25), rotation .0625, rotation direction −.125, displacement .125, UV tiling 1.25 and offset (.1875,−.3125). Test all four storage formats and repeat F32 at seed 751. At 64²/293 and with 16×8 inputs at 16²/599 use shape 2 and rotation .125. At 16²/463 use signed fields, shape 0, rotation .1875 and offset (−.5,.0625). The [small-parent comparison](#small-graph-parents-and-raytrace-positions) supplies the other eight renders per root, which matched a further 260/260 pixels per root.

Read the three pixel processors and two blends at changed 64²/42 and shape-2 64²/293. Every image input matched in the first renderer’s changed case, including all 4,194,304 environment texels. The first processor matched 4,090/4,096 and 4,094/4,096 in the two cases, maximum .00018310546875. The final blend matched 4,095/4,096 in each. Its differing green channel at (1,50) in the changed case reads −.04119873046875 in MaterialCraft and −.041168212890625 in the reference. In the second renderer, all five intermediates matched every pixel in both 64² cases. The three default-case F16 differences in the second renderer still need localization.

### Position-to-normal outputs in both renderers

| Root | Renders | Exact RGBA pixels / compared | Maximum error |
|---|---:|---:|---:|
| First PBR renderer, position to normal | 30 | 515,328/515,328 | 0 |
| Second PBR renderer, position to normal | 30 | 515,328/515,328 | 0 |

The first 18 cases per root include four archived renders, four output formats at 64², the independent 32² input and nine modular-field renders. They now match 171,264/171,264 pixels each. The earlier comparisons differed at 20 and five F16 pixels respectively. Twelve fresh renders per root repeat the four archived settings in the native precision, explicit F16 and explicit F32. They contribute 344,064 exact pixels per root. The fresh default F32 output includes the previously differing 256² coordinates.

Use the original byte position ramp with the varying-alpha background on the first renderer and the curved normal map on the second. Run defaults at 128² and 256², seed zero, and at 128² with seed 42. For the changed 128² case use seed zero, shape 1, scale .7, plane size (.7,1.3), UV tiling 2, sphere UV scale (2,3), plane and cylinder UV scales (2,1), cylinder length 1.5, UV offset (.125,.25), rotation .125, rotation direction .2 and rotation around that direction .3. Use normal format 1 and enable normal recomputation. The [byte input formulas and independent input](#four-format-inventory-comparison) specify all channels.

For the nine additional cases, use the [modular fields, sizes and seeds](#normal-maps-panoramas-and-primitive-shapes). The position pin uses the position field. The other pin uses the ordinary RGBA field or the normal field according to its identifier. Set shape to 1, 2 and 0 for A, B and C. Enable recomputation in A and B, use alternate normal format in B, UV tiling 1.25, sphere UV scale (.875,1.125), shape scale .8125 and plane size (.75,1.25). Use rotations .0625, .125 and .1875. Use UV offset (.1875,−.3125) for A and B, and (−.5,.0625) for C.

### Sobel normals and nadir projection

| Root | Renders | Exact RGBA pixels / compared | Maximum U8 / U16 / F16 / F32 error |
|---|---:|---:|---|
| First Sobel normal | 27 | 213,504/213,504 | 0 / 0 / 0 / 0 |
| Nadir extract | 18 | 174,336/174,336 | 0 / 0 / 0 / 0 |
| Nadir patch | 17 | 172,780/173,312 | 0 / 1 / 3.814697266e-06 / 0.0004730224609 |

The first Sobel normal now matches all 171,264 pixels in the [earlier Sobel matrix](#segment-mapping-and-normal-graph-rechecks), including the 232 differing pixels in that report. Nadir extract now matches all 132,096 pixels in its original, four-format and independent cases. The earlier binary matched 132,064 with maximum F32 error .00468292832. Its 32 differences were present before the [pixel quotient correction](#pixel-quotients-and-volume-mask-interpolation). Use the original linear byte foreground at 128² and 256² with seed zero, then 128² at seed 42. The changed render uses rotation .125, scale 3 and gamma mode 1 at 128². Repeat that setting in the [four-format and independent cases](#four-format-inventory-comparison).

For nine more renders per root, set D=((7 floor(x/3)+11 floor(y/5)) mod 19)/18. Use 16² F32 at seed 83, four output formats at 64²/179, 128² F32/277, 64×32 input on a 64² F32 canvas/379, 32×64 input on a 32² F32 canvas/479, and repeat 64² F32/577. For Sobel, use height −.375 on columns divisible by 13, 1.625 on the remaining rows divisible by 11, and D elsewhere. Use defaults at 16². At 64², 128², 64×32 and 32×64 use intensities .375, 2.625, 0 and −1.25 and normal formats 0, 1, 0 and 1 respectively. For nadir extract, use (u,v,D,A) at normalized texel centers with A=.875 on x<W/8 or y<2 and .3125 elsewhere. Use defaults at 16², then scale (2.625,8.125,.875,4.5), rotation (.1875,−.3125,.5,−.0625), and gamma mode (1,0,1,0) in the same size order. Each root matches all 42,240 pixels in these nine renders.

For nadir patch, use the eight [banded inputs and seeds](#panoramic-line-light-and-horizon-across-the-seam), with defaults at 16². At 64², 128² and with rectangular input use patch rotation .0625, .125 and .1875 and patch alpha modes 1, 0 and 1, respectively. Use frame thickness .09375, patch scale .4375, size (.8125,1.125), hardness .3125, source rotation offset −.09375, source position (.96875,.8125) and patch position (.03125,.96875). Show frames at 128² and disable the patch only for the rectangular input. The two input bridges match in the 64² and 128² F32 probes. The pixel processor is the first differing buffer, with 4,008/4,096 and 16,204/16,384 exact pixels and maxima 2.98023224e−7 and 3.57627869e−7.

### Segment, mapping and normal graph rechecks

| Root | Renders | Exact pixels / compared | Maximum error |
|---|---:|---:|---:|
| `jump_flood_segment_iteration` | 18 | 171,264/171,264 | 0 |
| `cartesian_to_polar` | 18 | 171,264/171,264 | 0 |
| `cartesian_to_polar_grayscale` | 18 | 171,264/171,264 | 0 |
| `blend_content` | 18 | 171,264/171,264 | 0 |
| `histogram_shift` | 18 | 171,264/171,264 | 0 |
| `non_uniform_rotation_color` | 26 | 206,336/206,336 | 0 |
| `non_uniform_rotation_grayscale` | 18 | 171,264/171,264 | 0 |
| `normal_sobel_2` | 18 | 171,264/171,264 | 0 |
| `panorama_rotation` | 26 | 206,336/206,336 | 0 |
| `panorama_sphere_light` | 18 | 171,264/171,264 | 0 |

Each row includes the archived default and parameter renders, four output formats, an independent 32² input and nine further procedural fields. The two rotation rows with 26 renders also include the eight [independent byte, float and rectangular fields](#color-temperature-range-reductions-sky-parameters-and-polygon-paths). All counts require matching dimensions, channel count, storage format and raw components. The historical segment-selection run differed at 101 pixels. All four of its original cases now match, including all 65,536 pixels of the 256² render.

For the jump-flood iteration, make an RGBA32F segment at each texel center (u,v) with endpoints (u,v) and (u+.0625 sin(2πv), v+.125 cos(2πu)). Use the default controls at 16² F32, seed 23. For four 64² outputs at seed 107 use distance mode 1, scale (1.125,.625) and iteration 1. At 128² F32, seed 347 use mode 0 and iteration 3. At 32² F32, seed 457 use mode 0, iteration 1 and endpoints −1 everywhere except (x+3y) mod 11=0. For seed 521 use mode 1, iteration 3 and multiply all four components by 2.5 then subtract .75. Repeat the 64² F32 settings at seed 587.

For the polar transforms and composite blend, define component c of input j as ((43x+29y+11xy+67c+(7+13j)x+13jy) mod 1024)/1023. Number pins from zero, in foreground, background, mask order for the blend. Use component zero for grayscale. Start at 16² F32/seed 97 with defaults. Use four 64² formats/139, 128² F32/307, a 32×16 input at 32² F32/451, signed 32² F32/557 with 3H−1.25, and a repeat of 64² F32/647. On the rectangular source use (x mod 2, y mod 2, x<width/2, (x+3y) mod 7=0). The polar graphs expose the image binding. The blend uses opacity .4375 and mode 3, changing to .8125 and mode 5 at 128² or mode 1 for signed input.

For the histogram, rotation, Sobel and sphere rows, use ((61x+53y+5xy+37c+11jx+17jy) mod 512)/511. The nine cases have the same dimensions and formats as the polar fields. Seeds in the same order are 71, 149, 263, 389, 487 and 661. The signed input is 3H−1.125. For color non-uniform rotation, pin order is rotation map, pivot map, input. For grayscale it is input, rotation map, pivot map. For sphere light it is background, sphere image.

| Graph | Exposed controls after the default render |
|---|---|
| Histogram shift | Position .4375, or .1875 for signed input |
| Non-uniform rotation | Multiplier .6875, offset .09375, pivot (.6875,.3125). Use multiplier −.28125 at 128² |
| Both Sobel normals | Intensity .8125, format 1. Use intensity 1.75 at 128² and format 0 in the signed case |
| Panorama rotation | Rotation .34375, direction .21875, pre-rotation −.15625 and post-rotation −.09375. Use post-rotation .46875 at 128² |
| Panorama sphere light | Up vector 1, world position (.75,1.5,2.25), UV (.375,.625), radius .875, exposure 1.25. Use radius 1.375 at 128² |

| Open root | Renders | Exact pixels / compared | Maximum integer error in levels | Maximum float error |
|---|---:|---:|---:|---:|
| Reverse polar transform, color | 18 | 171,242/171,264 | 1 | 0.003311216831 |
| Reverse polar transform, grayscale | 18 | 171,251/171,264 | 1 | 0.002349853516 |
| First Sobel normal | 18 | 171,032/171,264 | 1 | 3.337860107e-06 |
| Bevel debug position | 22 | 280,307/285,952 | 52428 | 31.25 |
| Dissolve blend | 18 | 112,533/171,264 | 63736 | 1 |

Both reverse polar graphs still differ on six of the 16,384 new 128² F32 pixels. The first Sobel normal differs on 135 pixels of its 128² field and five of its 64² U16 pixels. The second Sobel normal matches those same cases.

For the bevel debug and dissolve rows, use the same sizes, seeds and formats as the jump-flood iteration. The bevel input is (u,v,H₂,1) with H꜀=((19x+47y+5xy+71c+7cx) mod 257)/256. Change the debug pixel to (.3125,.6875), offset (1,1), mode on. At 128² use mode off and offset (.625,1.25). In the sparse case replace all components with zero except where (x+3y) mod 11=0. For dissolve, number the foreground, background and mask pins in graph order. For input j, component c uses H with index (j+c) mod 4, circularly shifted down 2j rows. The mask uses H₃ directly. Set opacity .6875, or .3125 for the large and sparse cases. Disable alpha blending for the signed case. Use defaults at 16² and 2.5H−.75 for the signed inputs of both roots.

### Rotated transforms and fractional texture coordinates

| Root | Renders | Exact pixels / compared | Maximum error |
|---|---:|---:|---:|
| Non-square color transform | 23 | 204,032/204,032 | 0 |
| Non-square grayscale transform | 23 | 204,032/204,032 | 0 |
| Safe color transform | 18 | 171,264/171,264 | 0 |
| Safe grayscale transform | 18 | 171,264/171,264 | 0 |

Four archived renders, four output-format changes, a second 32² image and nine further procedural fields cover each root. The non-square roots also match five integer-input renders with tile (3,2), offset (−.1875,.3125), rotation .3125 and tile-safe rotation disabled. Use 128² F32 at seed 71 and 64² in all four formats at seed 137. For the nine fields, use the [float image formula, dimensions, formats and seeds](#stored-precision-in-transform-mip-levels). The 16² case uses defaults. Otherwise use offset (.1875,−.3125), rotation .09375, background (.125,.375,.5625,.6875) or .375 for grayscale. For non-square set tile mode 1 and tile (3,2). For safe transform set tile 2, offset mode 1, symmetry 1 and automatic mip. At 128² use rotation .1875 with tile (2,3) or 3. The signed 32² case enables tile-safe rotation and uses manual mip 2 for safe transform. Its rectangular-source case uses symmetry 2.

A rotated two-color checker exposed a one-step difference in a bilinear weight. In the transform, first compute the pivot translation from the matrix columns. For the first coordinate it is .5−(.5m₀+.5m₂)+offsetₓ. Evaluate m₂v+m₀u with the second multiplication rounded and the first fused with the sum, then add the translation. The other coordinate uses m₁ and m₃. For bilinear reads, retain the signed fractional texture coordinate when forming the texel and half-texel offset. Wrap the integer neighbors individually. At 128 texels, input F32 bits bdfff801 on a repeating 0,1 stripe give .5. The previous result was .49609375.

Direct reads matched 360,448/360,448 F32 pixels across 22 renders. With integer x,y in a 128² field, let a=shift+(x+.5)/32768+(floor(y/4)−16)/128 in F32. On rows y mod 4 equal to 0 or 2, replace a with its next float toward negative or positive infinity, respectively. Use a and its reversed transpose as the two read coordinates and x mod 2 and y mod 2 as the channels to read bilinearly. Use shifts 0, −.25, −.5, −1, −2, −8, .25, .5, 1, 2 and 6, starting at seed 829 and incrementing by one.

Eleven matrices and offsets with manual mip 0 matched 405,504/405,504 RGBA32F pixels across 33 renders. The two checker renders use a 128² U8 source with channels 255 times (x mod 2, y mod 2, floor(x/2) mod 2, floor(y/2) mod 2) at 64² and 128². The third uses a procedural 128² byte field at 128². Seed is 773. One distinguishing matrix is (2.494419575,−1.666711092,1.111140728,1.662946343) with offset (.1875,−.3125). At (62,9) in its 64² checker output the component bits are 3eae00ae, 3f800000, 3f28ffa9, 3f800000. Four direct renders of matrices used by the roots matched 40,960/40,960. The 42 large-offset controls matched 688,128/688,128, and 118 square and rectangular resize controls matched 444,032/444,032. All maximum errors were zero.

### Centered transform resizing and untiled edges

| Root | Renders | Exact pixels / compared | Maximum error |
|---|---:|---:|---:|
| First horizon occlusion mip | 18 | 171,264/171,264 | 0 |
| Second horizon occlusion mip | 18 | 171,264/171,264 | 0 |
| First PBR environment rescale | 18 | 171,264/171,264 | 0 |
| Second PBR environment rescale | 18 | 171,264/171,264 | 0 |
| Orthographic PBR environment rescale | 18 | 171,264/171,264 | 0 |
| Orthographic PBR mip generator | 18 | 171,264/171,264 | 0 |

Both horizon occlusion helpers match the 64² input, the 16² U16 intermediate and the final 64² float expansion. The first helper previously matched 115/4,096 pixels on that F32 output. In the first PBR rescale, the 2,048×1,024 source for the transform was exact and the 64² output matched 0/4,096 pixels. The orthographic generator first differed along the top and bottom scanlines of a 2,048×1,024 manually filtered image. Reading the connected intermediates again gives bit-identical images in the 16² and 64² probes for both graphs and in the rescale's rectangular-source probe.

Each row combines four archived renders, four changed-format cases, the independent 32² input and nine new procedural fields. Use the [16² through 128² color fields, signed input, rectangular input and seeds](#stored-precision-in-transform-mip-levels). For grayscale use channel zero. The horizon helpers use the default mip at 16², mip 2 for the four 64² formats and seed repeat, mip 3 at 128², mip 0 for the rectangular input, and mip 4 for the signed input. The four environment and mip-generator roots expose only the image binding.

A centered automatic bilinear transform with a unit signed axis permutation resizes in factor-of-two steps. Determine the intermediate sizes working back from the destination in source-axis order. A 16×16 image resized to 64×32 passes through 32×16. Resizing it to 64×8 also passes through 32×16. Each pass reads at the source level's precision and stores at the destination precision. Quarter turns exchange the axes, including their wrap flags. Untiled edges extend the nearest edge texel. With a manual mip selection, the final centered read also extends the edge. Moving the transform off center restores the matte at the border.

| Original transform controls | Renders | Exact RGBA pixels / compared | Maximum error |
|---|---:|---:|---:|
| Square magnification and single-pass controls | 84 | 408,576/408,576 | 0 |
| Matrix, offset, matte and mip-mode controls | 60 | 237,568/237,568 | 0 |
| Signed axis permutations and mixed resize ratios | 50 | 37,440/37,440 | 0 |
| Rectangular sources | 20 | 26,880/26,880 | 0 |
| Centered and offset untiled edges | 66 | 118,272/118,272 | 0 |
| Nearest rectangular reads | 14 | 8,576/8,576 | 0 |

For the square controls, start with a fixed 16² field H=((97x+43y+7xy+143c+9xc) mod 1024)/1023. Round H to the nearest stored level for U8 and U16, and supply 3H−1.25 for F16 and F32. Use seed 631, 16², 32² and 128² F32 outputs, and 64² in all four formats. Run centered identity bilinear, centered nearest and matrix (1,.25,−.125,.75) with offset (.109375,−.1875) and horizontal tiling. The matrix controls add nonzero offsets including (1/64,0), horizontal reflection, diagonal scales 1±2⁻¹⁶, .5 and 2, independently toggled tiling axes, matte (.125,.375,−.5,1.25), and manual mip 0 or 1. Test the 16² field at 64×32, 8×16, 4×8, 64×8 and 16×64 with all eight signed axis permutations. The rectangular-source controls use a 32×16 field with coefficients (157,29,17,193,5) in H, and 4H−1.75 for float inputs, seed 773. Destinations are 64×16, 16×64, 8², 64² and 16×32.

### Surface irradiance and uniform power

Surface irradiance matches **47,360/47,360 RGBA pixels in 71 renders**, maximum error zero. The total includes repeated controls. There are 31,232 F32 pixels and 5,376 each in U8, U16 and F16. The eight original polar/index combinations, the four 64² format checks and the independent 32² field all match.

The mixed albedo (.27,.54,.88) isolated the green component. With the .54 control, the uniform expression ((c+.055)/1.055) raised to 2.4 produces F32 bits 3e8182ad. The logarithm and power controls each match 278,528/278,528 F32 pixels from 17 scalar inputs. MaterialCraft now range-reduces the uniform logarithm in base two and approximates the reduced natural logarithm with a polynomial fitted to the mathematical function. Products and sums round separately. On the broader field x=.5+i/16,384 for i=0…8,191, 8,104/8,192 distinct results match, or 1,037,312/1,048,576 pixels with each column repeated 128 times. Maximum error is 5.96046448e−8.

The [3D surface render inputs](#3d-surface-and-volume-render-passes) define the ramp. For the flat control, use position bytes (128,128,128,255), environment bytes (64,128,192,255) and zero noise. The axis control additionally uses an F32 normal (.5,.5,1,1) and sample table (.25,.25,1,1). The original 21 controls cross these three environments with seven combinations of index and albedo at 16². For nine further renders use the following settings.

| Input | Size / output | Seed | Index | Albedo |
|---|---|---:|---:|---|
| Ramp | 16² F32 | 31 | 3 | (.0625,.4,.75) |
| Flat | 32² in all four formats | 139 | 1 | (.375,.75,.8125) |
| Ramp | 64² F32 | 233 | 4 | (.9,.3,.6) |
| Axis | 32² F32 | 307 | 6 | (.27,.54,.88) |
| Flat | 16² F32 | 503 | 7 | (.031,.04045,.045) |
| Flat | 32² F32 | 239 | 1 | (.375,.75,.8125) |

For the final eleven F32 renders, keep the axis field and index 6. At 16², set each of the three channels of (.27,.54,.88) in isolation, then use each component as a neutral gray. At 32² use (.203125,.3125,.45625), (.6125,.776,.928), (.1546875,.478,.832), white and (.01,.03,.04). Seeds run from 473 through 483 in that order. All 6,656 pixels match.

**Function controls.** In 16² parameter, pixel, stamp and typed-value graphs, 18 exposed logarithm arguments give 13,824/13,824 exact image pixels and 18/18 typed floats. The 25 exposed power pairs give 19,200/19,200 pixels and 25/25 floats. Five literal logarithms and four literal powers give 6,912/6,912 pixels and 9/9 floats. All maximum errors are zero. Cases include signed zero, the nearest floats on each side of the minimum positive normal, negative bases with positive and negative integer exponents, negative bases with fractional exponents, and zero to the zeroth power.

At the subnormal boundary, runtime addition, subtraction, multiplication and division use signed zero for subnormal operands and results. On seven 32² F32 fields, those operations, a fused product and sum, a sum of two products and a two-channel dot product match 7,168/7,168 raw pixels, including signed zeros and nonfinite bit patterns. Maximum finite error is zero. Four further 16×16-output probes over sixteen scalar pairs match 16,384/16,384 pixel-processor outputs. For minimal discriminators use 2⁻¹²⁶ × .5, −2⁻¹²⁷ × 8, the smallest subnormal divided by itself and the product of 2⁻¹²⁶ and .5 added to 2⁻¹²⁶ in one fused operation. A plain stamp reading the twelve near-zero inputs still matches 1,792/3,072 F32 pixels, maximum 1.17549421e−38. Nonfinite typed values could not be compared through the JSON transport.

**Remaining roots.** The 3D volume mask matches 466,232/467,456 pixels in 35 renders. All integer and half outputs match, and 113,976/115,200 F32 pixels match with maximum error 3.81469727e−6. In eleven of these controls use an F32 position field ((x+.5)/N,(y+.5)/N,((11x+23y+3xy) mod 127)/126,1), scale .375, size (.75,1,.625) and world position (.375,.5625,.6875). The 64² cylinder at rotation (.125,−.1875,.0625), feather .15625 and seed 197 matches every U8, U16 and F16 pixel and 4,061/4,096 F32 pixels. The F32 repeat at seed 613 has the same count and maximum of 1.90734863e−6.

Blackbody matches 34,048/42,496 pixels in 19 renders, including all 4,352 pixels in each of U8, U16 and F16. F32 matches 20,992/29,440 with maximum error 1.19209290e−7. Use temperatures 1733, 2718.5, 5555, 6500, 7111.25, 9900.5 and 18000 at sizes 16, 32, 16, 64, 32, 16 and 16, with seeds 29, 197, 113, 137, 371, 89 and 83 respectively. Run all four formats at 5555 and F32 at every other temperature.

### Stored precision in transform mip levels

| Root | Cases | Exact RGBA pixels / compared | Maximum error |
|---|---:|---:|---:|
| Composite transform filter | 18 | 171,264/171,264 | 0 |
| First PBR renderer, panoramic LOD | 18 | 186,624/186,624 | 0 |
| Second PBR renderer, LOD | 18 | 171,264/171,264 | 0 |

The first PBR panorama matches all four archived renders, all four changed output formats, the independent 32² input and nine additional color fields. All measured intermediate images now match on the full, large and signed fields. Its 18 cases include 162,048 F32 pixels and 8,192 pixels in each other format. The earlier large-offset controls now match 688,128/688,128 pixels across 42 transforms. The [panorama comparison](#large-transform-offsets-and-additive-blends) gives the inputs, matrices, wrapping and seeds.

An equal-weight 2×2 reduction is enough to reproduce the previous discrepancy. Each mip level filters at the preceding level's storage precision, then stores the reduced image at the transform's output precision. Both nearest and bilinear final sampling use those same levels. For a float input, align the tap significands to the largest participating exponent before summing. The [floating texture sampling controls](#transform-edges-and-floating-point-filtering) describe the filter. For example, four F32 texels with bits 4012ab5d, bf9f3b55, 4075821b and c02f817b reduce to 3f090e53 in row-major order. A half source with F32 bits 3fa2e000, 3cd0c000, 404e0000 and c0180000 reduces to 3f092000.

| Direct transform controls | Cases | Exact RGBA pixels / compared | Maximum error |
|---|---:|---:|---:|
| One, two or three mip levels | 120 | 215,040/215,040 | 0 |
| Untiled matte after reduction | 24 | 98,304/98,304 | 0 |

For the 120 cases, use an identity matrix, zero offset, both axes tiled and a fixed 128² RGBA source. Run nearest and bilinear transforms at 64², 32² and 16². For each combination of four source and four destination formats, take the [signed-field hash](#3d-linear-gradient) per channel. Use the low eight or sixteen bits for integer inputs, or 8H/2³²−4 for F16 and F32, seed 557. For both floating-point source and destination formats also use ((13x+31y+7xy+47c) mod 1024 / 128−3) × 2^((x+3y) mod 13−6), seed 659. The first four texels are (−0,0,−0,0), (.25,1,−4,−.5), (1,−2,.125,.875) and (−1,4,−.25,.75). For the 24 matte cases, use the 64² output with U8, F16 and F32 sources, both filters, offset (4,−4), no tiling and matte (.5,.123456,−.312345,1.144445). Once a mip level has been generated, the transform reads its border at destination precision.

For the composite transform and the second renderer's LOD, the original four renders provide 114,688 pixels per root and the previous four-format and independent inputs add 17,408. For nine further cases, define channel c as ((97x+43y+7xy+143c+9xc) mod 1024)/1023 in an original RGBA32F image. Use 16² F32/seed 43, 64² in all four formats/73, 128² F32/197, 32×16 input at 32² F32/239, signed 32² F32/311 with 3H−1.25, and a repeat of 64² F32 at seed 401. The LOD graph has no exposed numeric control. For the composite filter use defaults at 16², and otherwise matrix (1.25,.375,−.125,.875), offset (.1875,−.4375), bilinear. At 128² use (−1,0,0,.5), offset (−.0625,.25), nearest. For signed input use (2.5,−.75,.25,1.125), offset (.25,−.125), bilinear. Each graph matches all 39,168 additional pixels.

### Large transform offsets and additive blends

| Root | Measured cases | Exact pixels (maximum error zero) |
|---|---:|---:|
| 3D viewer panoramic LOD | 20 | 219,392/219,392 |
| Channel mixer | 18 | 171,264/171,264 |
| Horizon occlusion subsample | 19 | 171,520/171,520 |
| Second PBR render panoramic LOD | 18 | 186,624/186,624 |
| Orthographic vector shadow pass | 28 | 211,456/211,456 |
| Premultiplied to straight | 19 | 172,288/172,288 |
| Irradiance panoramic LOD | 18 | 186,624/186,624 |
| Shape splatter data extract (seven outputs) | 19 | 1,206,016/1,206,016 |

**Panoramic LOD.** At 128², the image feeding the first incorrect transform already matched every texel. A scale of 256 with offset (127.5,−127.5) had been evaluated with offset (33,−33). This moved one strip of the image from row 127 to row 80. Affine offsets and matrices now retain their finite authored values. The sampler bounds coordinates before it indexes the image, and mip reduction stops at the existing limit. The viewer total includes the ten [original panorama cases](#panoramic-lod), the independent 32² F32 case and nine additional inputs. The four 64² archived format comparisons repeat cases in the original ten and count once.

The nine additional comparisons bind the [original color fields](#signed-levels-driven-grayscale-weights-and-grayscale-gradients) to the single image input. Use small/16² F32/seed 31, full/64² U8, U16, F16 and F32/71, large/128² F32/177, rectangular/32² F32/283, signed/128² F32/309 and full/64² F32/509. The other two exact panorama roots each add four archived renders and five previous precision and independent-input cases.

The first PBR render panorama remains open. The four original F32 renders match 182/16,384, 734/65,536, 182/16,384 and 340/16,384, maximum 1.78813934e−7. The five previous precision and independent cases and nine original color fields give 38,653/71,936. U8 and F16 match every pixel, U16 matches 8,163/8,192 with maximum one level, and F32 matches 14,106/47,360 with maximum 2.38418579e−7. At 128² the first minifying transform matches 12,935/16,384 for the large field and 10,444/16,384 for the signed field. Its input matches, and float mip filtering remains under test.

**Transform controls.** Use a flat 128² RGBA8 input (23,139,211,255), black opaque matte and seed 337 in all four output formats. Repeat with the signed-field hash from the [3D gradient probe](#3d-linear-gradient), taking the low byte of each hash as the input, at seed 571 in U8 and F32. For each row below, matrix entries follow SBS column order. Independent horizontal and vertical wrapping are given as X or Y.

| Matrix | Offset | Wrap | Filter |
|---|---|---|---|
| Identity | (36.125,−65.375) | XY | Bilinear |
| Identity | (−127.25,64.125) | XY | Nearest |
| diag(128,128) | (63.5,−63.5) | X | Bilinear |
| diag(256,256) | (127.5,−127.5) | X | Bilinear |
| diag(512,512) | (255.5,−252.5) | X | Bilinear |
| diag(1024,1024) | (−492.5,511.5) | Y | Bilinear |
| (2,512,0,1) | (38.125,.125) | Neither | Nearest |

The 42 transforms match 687,744/688,128 pixels. The only differences are 128 F32 field pixels each with scales 128, 256 and 1024, maximum 5.96046448e−8. The flat 256-scale case stores (11,69,105,255) on row 127 and opaque black on row 80. The 1024-scale case gives the same half-intensity color on column 125.

**Channel mixer and blend.** All 18 [channel mixer cases](#integer-levels-endpoints-and-channel-mixing) now match. Add and subtract bound the global opacity before applying the mask. In source-alpha interpretation, take the complement of transparency for the mask and foreground alpha. Calculate the background weight as a fused subtraction from one. Multiply the combined foreground color by its weight, then add the weighted background with one fused operation. For foreground 9,830/65,535, background 2,753/65,535 and opacity .55, the F32 output is 3dfefcff. At opacity 1.3 it is 3e449cc5.

At 64² use grayscale U16 foreground (9053x+3581y+43xy+9830) mod 65536 and background (4519x+9721y+19xy+2753) mod 65536. For signed F32 input, convert each stored level to float, divide by 16384 and subtract 1.5. Run add and subtract with opacity −.3, 0, .2, .55, 1 and 1.3 in each of the four output formats, seed 443. All 393,216 grayscale pixels match. The 120 RGBA cases on the [varying-alpha fields](#color-burn-and-color-dodge) add 491,520 exact pixels. These include varying inputs, opaque inputs, white masks, constants, black foreground, transparent background, signed RGB and byte, word and half storage. Add uses all six opacities, and subtract uses −.3, .55 and 1.3. Repeated RGBA captures at the same settings are included in the 216 direct comparisons. Maximum error is zero. The subnormal Levels range probe in the [integer endpoint controls](#integer-levels-endpoints-and-channel-mixing) now matches 32,768/32,768. The texture evaluator flushes a subnormal reciprocal range to signed zero before multiplying.

**Horizon occlusion subsample.** Let H=(13x+31y+7xy+47j) mod 1024 divided by 1023, with j=0 for input and 1 for mip input. For normalized pixel-center coordinates u,v, set B=max(0,1−3sqrt((u−.35−.15j)²+(v−.625+.15j)²)). Each F32 input is .375H+.625B. The additional cases below match 39,424/39,424 grayscale pixels. The 64² F32 case contains 172 distinct values.

| Output, seed | Source sides (input/mip) | Angle, mip, jitter, XY multiplier |
|---|---|---|
| 16² F32, 41 | 16/16 | Defaults |
| 64² U8/U16/F16/F32, 73, repeat F32 at 137 | 64/32 | .1875, 3, .3125, (.75,1.25) |
| 128² F32, 197 | 128/64 | −.375, 1, 1, (1.375,.625) |
| 32² F32, 211 | 32×16 / 16×8 | .8125, 2, 0, (.625,1.125) |
| 32² F32, 317 | 32/32, constant .25/.75 | .25, 4, .75, (1.25,.75) |
| 16² F32, 419 | 16/32, both 3H−1.5 | .125, 2, .5, (1,.75) |

**Premultiplied color and orthographic shadow.** For the ten original F32 input sets use the same H with j=0 through 3. The outputs are 16² F32/31, 64² in four formats/73, 128² F32/163, rectangular 32×16 sources to 32² F32/229, 32² F32 black-alpha/317, 32² F32 signed/431, and a repeat of 64² F32/137. For premultiplied color, choose A from (0,1/65535,.03125,.125,.375,.625,.875,1) by (x+3y) mod 8. RGB is A times the first three H channels. Set pixel (0,0) in the small field to (.75,.25,.125,0). Use all-zero A for the black-alpha control. For signed input replace A by 3A−1, RGB by (3H−1.25)A, and pixel (1,0) by (.25,−.125,.75,0).

For the shadow pass use H as RGBA noise. Use (u,v,H₂,1) for the light coordinates. Normal XY before encoding are (.7cos(2πu),.7sin(2πv)), with positive unit-length Z. Environment RGB is (.35+.28j)(.25+1.5Hⱼ), A=1. The sequence/sample settings for small, the four formats, large, rectangular, flat, signed and repeat are defaults, 3/8, 6/4, 0/3, 2/2, 5/6 and 3/8. The flat case uses zero light coordinates and environment (.5,.75,1.25,1). The signed case uses 3H−1 for the environment. Eight of these ten reference images are opaque black. Both the rectangular case (88 distinct colors) and flat-environment case (26 colors) exercise nonzero output.

The nine independent lighting controls match 39,168/39,168, maximum 0. Use sequence zero with the previous sources. At 16²/53, 128²/149 and signed 32²/211 supply zero light coordinates and 3, 4 and 2 samples respectively. At 64²/73 in four formats, repeated in F32/389, supply (.015625u,.015625v,0,0) and four samples. At 32²/251 use rectangular inputs, light (.125+.25u,.5+.0625v,0,1) and three samples. The 128² reference has 68 distinct colors and its blue channel reaches 9.03062534.

**Shape splatter data extract.** Read `position`, `uv_texture_space`, `bounding_box_size`, `uv_shape_space`, `pattern_id`, `shape_angle`, `shape_index` from the two data textures. Each of the seven ports matches 172,288/172,288 pixels, maximum zero. For slot s=0 or 1 and normalized pixel-center coordinates u,v, define Hⱼ=(19x+37y+11xy+53(j+4s)) mod 1024 divided by 1023. Write (H₀,u,H₂,H₃) for slot 0 and (H₀,v,H₂,H₃) for slot 1. Output cases are 16² F32/seed 17, 64² in four formats/97, 128² F32/137, 32² F32/197 with a 32×16 first input, flat 32² F32/281, signed 32² F32/337 and repeat 64² F32/419. The flat inputs are (.25,.5,.75,1) and (.875,.625,.375,.125). The signed case uses 3 times each channel minus one. Four archived cases and five previous format and independent-source cases complete the 19 configurations.

### Integer Levels endpoints and channel mixing

**Integer Levels.** The 60 signed controls and the 24 mixed, narrow and signed-range controls from the [Levels endpoint table](#levels-endpoints-camera-tangents-and-uniform-exponentials) now match **344,064/344,064 RGBA pixels**, maximum zero. For an integer destination, bound each input and output endpoint to [0,1] before computing the curve. The intermediate-clamp switch still controls the position on the curve. For F16 and F32, retain finite endpoints outside the slider range.

In 32 additional comparisons, replace one endpoint at a time by the literal RGBA tuple (1024, −1024, 10¹², −10¹²). Use the other default endpoints and midpoint, the 64² signed field, seed 131, both intermediate-clamp settings and all four formats. All **131,072/131,072 pixels** match, maximum zero. At the uniform input .125, the signed-endpoint row in the previous table stores (64,88,81,158) in RGBA8 and (16323,22684,20789,40620) in RGBA16.

**Driven controls.** Attach the scalar function numerator/denominator to input high, input low or output high. Use numerators zero and one and exposed denominators 1, +0, −0, 1/1024, −1/1024, 10⁻¹², −10⁻¹² and −1 in that order. Apply the otherwise default Levels filter to the [64² signed field](#signed-levels-driven-grayscale-weights-and-grayscale-gradients) with seeds 607 through 614. A scalar becomes (value,value,value,1). At the integer boundary positive infinity becomes one, and negative infinity and NaN become zero. The 96 integer cases match **393,216/393,216 pixels**. The 72 finite-result float cases match **294,912/294,912**, maximum zero. The 24 float cases with a zero denominator still report a nonfinite parameter error.

A separate upper-endpoint tuple (2¹²⁵,2¹²⁷,−F32_MAX,F32_MAX), with both clamp settings, matches 8,192/8,192 in each integer format. F16 matches 6,411/8,192 with maximum numerical error zero and different zero signs. F32 matches 3,883/8,192 with maximum 3.91219466e−38. Subnormal range scaling remains open.

**Channel mixer.** The default 128² and 256² renders and seed 42 now cook and match all **98,304/98,304 RGBA16 pixels**. The changed 128² case matches 16,380/16,384, maximum four levels. The 64² changed monochrome case in all four formats and the independent 32² F32 case match **17,408/17,408**, maximum zero. The historical changed case uses red (50,25,10,5), green (−10,100,10,0), blue (20,0,50,25) and monochrome enabled. The input fields are original.

Nine further channel mixes use the [original small, full, large, rectangular and signed fields](#signed-levels-driven-grayscale-weights-and-grayscale-gradients). The four numbers in each channel control are percentages. All comparisons count RGBA pixels with all four stored channels equal.

| Input, output, seed | Red / green / blue | Exact pixels | Maximum error |
|---|---|---:|---:|
| Small, 16² F32, 41 | (100,0,0,0) / (0,100,0,0) / (0,0,100,0) | 256/256 | 0 |
| Full, 64² U8 / U16 / F16 / F32, 73 | (60,30,10,0) / (−10,130,−20,0) / (20,15,55,10) | 561 / 513 / 515 / 513 of 4,096 | 58 / 14,864 / .22705078125 / .226810098 |
| Large, 128² F32, 99 | (.1,0,99.9,0) / (0,99,.1,1) / (0,.125,100,0) | 16,383/16,384 | .00006103515625 |
| Rectangular, 32² F32, 211 | (29.9,58.7,11.4,0) / (0,100,0,0) / (0,0,100,0), monochrome on | 1,024/1,024 | 0 |
| Signed, 32² F32, 317 | (−15,75,40,−5) / (70,−6,23,13) / (110,−15,−20,25) | 534/1,024 | .307743947 |
| Full, 64² F32, 137 | Repeat the seed 73 controls | 513/4,096 | .226810098 |

Reading the intermediate images identifies an additive grayscale blend with opacity 1.3 as the first large difference in the 64² F32 mix. Its two LUM16 inputs match. The first texel has background 0 and foreground 13,281. The reference stores 13,281 and MaterialCraft stores 17,265. A second additive blend with opacity .55 differs at only two of 4,096 texels, maximum one level. The root remains in progress.

### Surface position and signed pixel products

The surface position root matches **92,928/92,928 RGBA pixels across 39 renders**, maximum error zero. These include seven archived renders, four output formats, the independent 32² field, nine camera and distance-field combinations, and 18 separate tilt controls. The totals are 30,720 U8, 5,120 U16, 5,120 F16 and 51,968 F32. All 39,168 pixels in the tilt controls now match. The earlier count was 33,449. The atlas formulas, camera positions, lenses, seeds and ground-plane settings are in the [camera control table](#levels-endpoints-camera-tangents-and-uniform-exponentials).

The camera position, screen aspect and ray direction already matched. The first difference occurred at the near and far intersections with the bounding box. At 64², camera UV (.1875,.6875), distance 2.75 and field of view 37, separately rounding the negative origin product gave 2,963/4,096 exact near intersections and 3,082/4,096 far intersections. Fusing the negative origin times the inverse direction with the signed half-extent term gave 4,096/4,096 in both. In a pixel expression a sign change can be applied to the factor inside a fused multiply-add. A shared product that another output reads directly must retain its rounded value.

| Original scalar or vector controls | Renders | Exact pixels / compared | Maximum error |
|---|---:|---:|---:|
| Camera, aspect, ray, bounds, entry point and pixel-scope camera direction | 80 | 155,648/155,648 | 0 |
| Origin, inverse direction, center, half-extent, near and far terms by axis | 90 | 175,104/175,104 | 0 |
| Seven signed product expressions across two fields | 14 | 229,376/229,376 | 0 |
| Nine uses of a shared product or its negation across two fields | 18 | 294,912/294,912 | 0 |
| Two independently reused products feeding four components | 36 | 544,952/589,824 | 0.0000019073486328125 |

For each camera component, use five F32 renders of a centered unit cube. The (side, seed, camera UV, distance, field of view) cases are (64,197,(.1875,.6875),2.75,37), (16,131,(.375,.65625),1.75,21), (32,83,(.35,.6),3,65), (16,829,(.5,.5),4,50) and (64,607,(.296875,.84375),−2.25,83). Store each component in its own output to avoid changing the rounding of a shared intermediate.

For the arithmetic controls use 128² signed RGBA input from the [original field hash](#3d-linear-gradient) and the [mixed-exponent field](#stored-precision-in-transform-mip-levels). Form P=A×B and Q=C×D. Compare −P+C, −P−C, C−(−P), C+(−P), −P+Q and −P−Q. With input component bits c0742acf, bf116093, 405370ba and 3f3eb229, −P+C gives 3f9190bd on its own. When the same graph also returns −P, the sum gives 3f9190bc. The last row includes cases with both P and Q feeding other sums. For example, emitting (P+C,P+Q,Q+A,Q−A) still differs on the uses of Q. This difference remains open.

### Levels endpoints, camera tangents and uniform exponentials

**Color to mask** matches **168,192/168,192 pixels in 18 cases**, maximum error zero. Keeping all upstream images fixed on the 64² float color field isolated the last 21 differing pixels at the final Levels filter. Multiply the centered input by the reciprocal input span and perform the output scaling and addition as a single fused operation. The 32×16 source, mixed alpha, signed input, altered keying modes and seeds are included in the [mask case definitions](#signed-levels-driven-grayscale-weights-and-grayscale-gradients). The grayscale HDR range viewer also retains its 18 exact cases.

The 60 original signed Levels controls now give **245,760/245,760 exact RGBA pixels**, 61,440 per output format, maximum zero. When the input endpoints coincide the measured slope is 10,000. For a nonzero span the slope is its reciprocal, including spans below 0.000001. The four channel ranges are computed once per image. The following tests use the same 64² signed field and seed 131 as the [grayscale conversion probe](#signed-levels-driven-grayscale-weights-and-grayscale-gradients). Run each group with and without clamping in all four output formats.

| Channels in RGBA order | Input low | Midpoint | Input high | Output low | Output high |
|---|---|---|---|---|---|
| Mixed | (−.0625,.125,.75,.34375) | (.3125,.5625,.375,.6875) | (.8125,.875,.125,.78125) | (.1875,.375,.8125,.125) | (.875,.625,.125,.9375) |
| Narrow and coincident | .5 | .5 | (.5,.5000003576,.4999996424,.5000047684) | (0,.125,.875,.25) | (1,.875,.125,.75) |
| Signed endpoints | (−1.75,−.625,−.125,−.5) | (.469,.44,.51,.25) | (2.375,2.75,1.5,2.875) | (−.5,−1,.25,.125) | (1.875,2.25,1.375,.875) |

The 24 further cases give **24,576/24,576 F16** and **24,576/24,576 F32**, maximum zero. U8 matches 19,631/24,576 (maximum 214); U16 matches 19,447/24,576 (maximum 54,976). Clamping the input and output endpoints to [0,1] before evaluating the signed-endpoint probe reproduced all 4,096 reference pixels separately for both integer formats and both clamp settings in an independent calculation. This integer-output behavior remains open in the cook.

**Surface position** matches 25,600/25,600 pixels in seven original 16², 64² and 128² comparisons, 16,384/16,384 in the four 64² changed-camera formats, and 1,024/1,024 in the independent 32² float case. The 64² F32 comparison formerly matched 87/4,096. Nine additional signed-distance cases give **10,181/10,752**, including 1,024/1,024 for each 32² format with a sphere. A tilted field still differs at 64² (3,541/4,096, maximum 9.15527344e−5) and 16² (240/256, maximum 2.28881836e−5).

Use an original single-channel F32 atlas of side D=128 or 256 with s=D/16. At pixel (x,y), set X=((x mod s)+.5)/s−.5, Y=((y mod s)+.5)/s−.5, Z=(floor(x/s)+16floor(y/s)+.5)/256−.5. The sphere is length(X,Y,Z)−.3, the tilted field .55X+.3Y−.7Z+.06, and the shifted sphere length(X−.125,Y+.1,Z−.05)−.34. Ground-plane size is (2.75,3.25).

| Output / seed | Input and D | Camera UV / distance / field of view | Ground plane / infinite |
|---|---|---|---|
| 16² F32 / 17 | Constant one, 128 | (.5,.5) / 4 / 50 | off / on |
| 32², four formats / 83, repeat F32 / 43 | Sphere, 256 | (.35,.6) / 3 / 65 | on / on |
| 64² F32 / 197 | Tilted, 256 | (.1875,.6875) / 2.75 / 37 | on / off |
| 32² F32 / 259 | Shifted sphere, 128 | (.75,.3125) / 3.25 / 103 | on / on |
| 16² F32 / 131 | Tilted, 128 | (.375,.65625) / 1.75 / 21 | off / off |

Before contraction of signed products, 18 camera controls used the 64² and 16² settings above and varied only the field. In the order one, zero, sphere, X, Y, Z, tilted, tilted with ground toggled, tilted with infinity toggled, the 64² exact counts were 4,096, 2,991, 3,801, 3,170, 3,298, 3,357, 3,541, 3,538 and 3,541. The 16² counts were 256, 164, 256, 256, 256, 208, 240, 240 and 240. Together these gave 33,449/39,168, maximum 0.000106811523 on the 64² Z plane. A constant-zero distance field reproduced the discrepancy at both settings. All 18 renders now match. [Surface position and signed pixel products](#surface-position-and-signed-pixel-products)

For a runtime **tangent**, calculate the scoped sine and cosine and divide using that scope’s arithmetic. A pixel processor emitting (tan(x),sin(x)/cos(x),sin(x),cos(x)) matches **81,920/81,920 RGBA32F pixels** over five independently generated 128² fields at seed 331. All finite errors are zero and the ten nonfinite component bit patterns match. For row-major index i, use (10+150i/16,383)×π/360, ((i mod 4)+.5)π+(floor(i/4)−2,048)×2⁻²², and (i−8,192)×2⁻¹⁸. For the other two fields take consecutive uniform draws from NumPy PCG64 seed 71077 in [−6.5,6.5) and then [−8,192,8,192), converting each to F32. Uniform tangents match 131,072/131,072 F32 pixels in eight 128² controls, using x=start+column×step with F32 operations. The (start,step) pairs are (.05,1/128), (−.6875,1/64), (1.5,1/1024), (−2,1/64), (2,1/128), (90,1/16), (−.0025,1/32768), (700,1/32). Literal tangents retain ordinary float folding.

The **uniform natural exponential** uses the same [seventh-degree polynomial](#natural-exponential-and-ridged-noise) with separately rounded products and additions. Dynamic pixel expressions use fused operations. The 62 independently generated 128² parameter cases match **1,015,808/1,015,808 F32 pixels**, maximum finite error zero, including 6,784 identical nonfinite components. Three additional literal and eleven exposed controls in each execution scope match **10,752/10,752 image pixels and 14/14 typed floats**. For example, the runtime pixel exponential at input bits 3d07db5a gives 3f845115. The exposed uniform expression and the literal give 3f845114.

To reproduce the parameter cases, broadcast exp(start+column×step) down each column, with arguments, multiplication and addition in F32. The first 24 (start,step) pairs are (0,1/128), (−1,1/128), (−8,1/8), (−.5,1/32768), (.625,1/16384), (−4,1/37), (−3,1/19), (32,1/64), (−31,1/64), (.0625,1/128), (.125,1/128), (.28125,1/1024), (.5,1/512), (.8125,1/2048), (.03125,1/128), (.3,1/128), (.4,1/256), (.9,1/4096), (.2,1/32768), (1,1/64), (3,1/16), (.6,1/2048), (.25,1/256), (.5625,1/1024). The next 16 use (j×ln(2)/16,ln(2)/2048) for j=0…15. The ten wide pairs are (−99,1/32), (−90,1/64), (−88.5,1/128), (−87.75,1/256), (−24,1/31), (−3,1/63), (.2,1/59), (17,1/32), (81,1/32), (87.5,1/64). The twelve independent pairs are (.500032187,.0000311367), (.5316711,.0013927), (.5062,.0029721), (.904311,.00049123), (.7009667,.0011769), (.222237,.0003721), (1.103987,.0139583), (3.14159,.053537), (13.175,.0002953), (.43291,.027716), (−89.112,.02023), (84.67,.032698).

Before the uniform power correction, surface irradiance matched 2,304/5,376 pixels in the 21 parameter and environment comparisons, maximum 2.98023224e−8. The current run matches all 5,376. [Surface irradiance and uniform power](#surface-irradiance-and-uniform-power)

### Signed Levels, driven grayscale weights and grayscale gradients

The **grayscale HDR range viewer** matches **167,424/167,424 pixels across 18 cases**, maximum error zero. This includes all 35,328 pixels of the nine signed range inputs described under [gradient interpolation](#gradient-interpolation-color-blend-and-chrominance). At 64² the input to the inverted Levels filter was already exact. An independent pixel expression that takes the absolute value, evaluates 2 raised to its base-two logarithm, restores the sign and subtracts from one matched its entire 4,096-pixel intermediate. Direct subtraction matched 2,503/4,096, maximum 9.53674316e−7. Levels now honors the intermediate clamp on load, cook and save and uses the signed power curve.

The **color to mask** root matches all 132,096 pixels of its original four cases, four 64² format cases and independent 32² case. In nine further cases **36,075/36,096** match. The only differences are 21 pixels in the 64² F32 output, maximum 1.19209290e−7. Capturing every connected primitive locates all 21 at the final Levels filter. Its incoming images and every earlier filter match, including the grayscale weights driven by a function.

For the nine mask cases, form each original float channel c at integer (x,y) as ((x×(13+4c)+y×(29+2c)+xy×(3+c)+17c) mod 257)/256. In the left half replace RGB by .5×RGB−.25+target. Set column zero to the target RGB and columns one and two to alpha zero and one. Each 64² case uses seed 91 and the same field. The signed case additionally maps RGB to 3×RGB−1. All other rows use F32.

| Input and output dimensions | Seed | Target RGB | Keying type | Flatten alpha | Range / softness |
|---|---:|---|---:|---|---|
| 16² | 17 | (.25,.5,.75) | 0 | off | .25 / .125 |
| 64², U8, U16, F16 and F32 | 91 | (.375,.625,.8125) | 1 | on | .375 / .1875 |
| 128² | 73 | (.3125,.6875,.25) | 1 | off | .5 / .25 |
| 32×16 input, 32² output | 39 | (.125,.75,.375) | 0 | on | .5625 / .25 |
| 32² | 137 | (.375,.625,.8125) | 0 | off | .0625 / .03125 |
| 32², signed | 159 | (.3125,.4375,.6875) | 1 | on | .5 / .375 |

The isolated **grayscale conversion** matches **122,880/122,880 pixels across 30 cases**, including the sign of zero and all four output formats. At 64² let channel c be (((13x+31y+7xy+47c) mod 1024)−384)/96. Replace the first 16 pixels of row zero by −2, −1, −.5, −.25, −.125, −.0625, −0, +0, .125, .25, .5, .75, 1, 1.5, 2 and 3 in all lanes. Use weights (.125,.25,.5,.125) as a literal four-vector, a constant-returning function and a function reading an exposed four-vector. Run all three with flattening off and on, matte .3125, seed 131 and each output format. In F32 repeat the exposed vector at seeds 199–201 with (1,0,0,0), (0,.625,.125,.25), and (.299,.587,.114,0). The four fused multiply-adds start from positive zero. Flattening is a fused interpolation from the weighted pixel toward the matte by one minus alpha.

For **Levels**, use the same signed four-channel field, size and seed. Test the omitted clamp, explicit false and explicit true. With the input range [0,1], neutral, inverted output endpoints and per-channel midpoints (.25,.375,.625,.75) give **147,456/147,456** pixels across 36 cases, maximum zero. A second pair of probes maps [.125,.75] to [.1875,.8125] and reverses both ranges, with neutral midpoints. These 24 cases match 24,576/24,576 U8 and 24,576/24,576 U16, 23,925/24,576 F16 (maximum .00390625) and 19,527/24,576 F32 (maximum 1.43051147e−6).

Grayscale gradient output reads the same four-channel stops as color output. Saving a scalar-key document writes the equivalent native four-channel array. Grayscale interpolation first averages RGB at each stop. For a smooth grayscale segment, multiply the color difference by u and multiply the result by u×(3−2u) before adding the left stop. A repeated position uses its first stop on both sides. The following measurements use the three indexed input ramps and seed 103 from [gradient interpolation](#gradient-interpolation-color-blend-and-chrominance). Grayscale covers the unit ramp, the three-stop ramp with distinct RGB and the same three-stop ramp with R replicated into G and B. Each uses linear and smooth interpolation. The duplicate-position color ramp uses positions (.125,.5,.5,.75), signed biases (.5,.25,−.25,0) and colors (.375,.75,.875,.6875), (.875,.0625,.3125,.25), (.1875,.75,.25,.8125), (.25,.5625,.75,.375).

| Controls | U8 exact / total | U16 exact / total | F16 exact / total | F32 exact / total |
|---|---:|---:|---:|---:|
| Six grayscale ramps | 405,239/405,504 | 336,699/405,504 | 405,504/405,504 | 405,504/405,504 |
| Two duplicate-position color ramps | 135,168/135,168 | 125,227/135,168 | 135,168/135,168 | 135,168/135,168 |

Every floating-point maximum in the gradient table is zero. Each unsigned mismatch is one stored level. The four-stop irregular-position F32 control from the previous batch remains open.

### Gradient interpolation, color blend and chrominance

The **color blend** and **chrominance extract** roots each match **171,264/171,264 pixels in 18 cases**, maximum error zero. The original four cases account for 114,688 pixels per root. The four 64² output formats at seed 42 and the independent 32² F32 case account for 17,408. Repeat the nine [original color input cases](#color-burn-and-color-dodge) for a further 39,168. Color blend uses the same foreground, background and mask with opacity .375 in the 16², 128² and signed cases and .8125 elsewhere. Chrominance takes only the original foreground. Before the fix the independent color blend matched 1,021/1,024 pixels, maximum .00676953793. Seven of its grayscale-to-color gradient pixels differed by up to 23 byte levels. The incoming scalar image matched in full.

Gradient Map now reads and writes the interpolation setting and signed midpoint bias. Linear and smooth gradients sample a 1,024-entry float table in F16 and F32 and a 256-entry integer table in U8 and U16. In a smooth segment the fractional position advances by one rounded add per entry. For fractional position t and signed bias b on the left stop, evaluate u=t×(1−b×(1−t)), then u²×(3−2u). Float texture filtering and final storage use the existing precision paths.

| Gradient table controls | Cases | Exact pixels / compared | Maximum stored-component error |
|---|---:|---:|---:|
| U8 | 36 | 1,325,806/1,327,104 | 1 |
| U16 | 36 | 1,238,928/1,327,104 | 1 |
| F16 | 36 | 1,327,104/1,327,104 | 0 |
| F32 | 48 | 1,376,256/1,376,256 | 0 |

For the unit ramp, use positions 0 and 1, black and white RGB, and alpha .25 and .75. For a short ramp use positions 0 and .125, colors (0,0,0,1) and (.875,.75,.5,1). For a three-stop ramp use positions (.0625,.34375,.890625) with RGBA (.1875,.25,.4375,.8125), (.59375,.9375,.3125,.5625), (.8125,.125,.75,.375). All three use zero midpoint bias. The fourth ramp uses black and white with alpha one and signed biases .25 and .75. For omitted, linear and smooth interpolation, feed x/255 across a 256² F32 image at seed 17 in each output format. Also feed ((13x+71y+5xy) mod 4096)/4095 at 64², seed 83, F32 output. With explicit linear and smooth interpolation, use a linear pixel index i and inputs (i mod (d+1))/d. Use d=255 and 1,023 at 32², d=65,535 at 256², seed 103 and all four output formats.

Additional controls still differ. A four-stop signed/HDR field with positions (.093,.41327,.77777,.812) gives 127,009/135,168 exact F32 pixels, maximum 7.15255737e−7 across linear and smooth modes. Use biases (−.4375,.1875,−.375,0) and colors (−.25,.75,1.25,.25), (2,−.875,.375,1), (.875,.25,−.0625,.625), (.375,1.5,.625,.375). Repeating the middle position in a separate (.125,.5,.5,.75) ramp gives 101,217/135,168 exact F32 pixels, maximum .687494829. The reference uses the first stop at the repeated position for the following segment. An isolated grayscale fixture using the `gradientluminance` array produced an identity ramp in the reference app and the authored ramp in MaterialCraft. The accepted grayscale key representation still needs validation.

For **color to mask**, the four 64² changed formats each match 4,074/4,096 pixels. Maxima are 255, 65,535, 1 and 1 in format order. The 32² F32 case matches 1,018/1,024, maximum 1. On the changed F32 case the first differing primitive is the grayscale conversion. The imported graph uses default thirds for its function-valued (.299,.587,.114,0) weights.

The **grayscale HDR range viewer** matches all 114,688 original pixels and all 17,408 four-format and independent pixels. For a signed input use (((31x+83y+7xy) mod 1024)−389)/93. Replace the first column with successive repetitions of (low,high,0,1). At 64²/seed 73 with range (−.75,2.25), each of the four formats matches 2,541/4,096, with maxima 255, 65,535, 1 and 1. Additional F32 cases at 16²/19 with (−1.5,.75), 128²/199 with (−3,6), 32×16 input at 32²/181 with (−.1875,.375), 32²/47 with (1.25,4.25) and 16²/311 with (.125,.1875) bring the signed total to 21,956/35,328. The first differing primitive is an inverted Levels node with intermediate clamping disabled. At 64² its LUM32F result matches 262/4,096, maximum 5.81720495.

### Color temperature, range reductions, sky parameters and polygon paths

The four roots below match every recorded case. Both the first-pass defaults and the [four-format and independent comparisons](#four-format-inventory-comparison) were replayed with their original procedural inputs. Typed results include a float32 bit comparison for every component.

| Root | Cases | Exact / compared | Maximum absolute error |
|---|---:|---:|---:|
| Color temperature adjustment | 18 | 171,264/171,264 RGBA pixels | 0 |
| Min/max | 25 per output | 50/50 float outputs | 0 |
| Sky `Impl/GetOneParam` | 25 | 25/25 float3 outputs | 0 |
| Polygon paths | 19 | 5,120/5,120 RGBA buffer pixels | 0 |

For **color temperature**, repeat the nine input sizes, seeds and output formats under [color burn and color dodge](#color-burn-and-color-dodge) using only the foreground input. The signed input uses the same RGB and alpha scaling. Use temperature −.5625, magenta/green .3125 and color space 1 in the 16², 128² and signed cases. Use .375, −.1875 and color space 0 in the others. The 32² byte endpoint field sets blue to 0 where `(x+3y) mod 4=0` and to 255 elsewhere.

For **min/max and the sky parameter**, generate a byte field `V=(17x+37y+3xy+19) mod 256`. The scalar input takes V. For the sky parameter, set RGBA to `(V,(11x−13y+17) mod 256,(43x+5y+61) mod 256,42×((7x+11y+17) mod 7))`. Cook 16²/seed 59, four formats at 64²/73, 128²/211, 32×16 input at 32²/113, and a signed float input at 32²/237. Use F32 outside the four-format sweep. For the last field, divide byte channels by 79 and subtract .6875. Use three equal RGB channels and alpha one for the scalar float input. Sky controls use albedo .3125, elevation .1875 at 16² and .8125 elsewhere, turbidity 3.25 at 128² and 5.5 elsewhere, and parameter index 1 at 16² and 6 elsewhere.

Eight additional byte ranges use `V=((7x+13y+3xy) mod (13+5j))+19j+11`, with `j=0..7`. Use sizes (16,32,64,64,64,128,32,16), seeds (97,127,211,317,41,53,181,149), and cycle the four output formats. For j=3 use the constant byte 83. For j=4 use 113 throughout with 49 in the upper-left corner and 171 in the lower-right corner. The constant's two reported values are .32549017667770386 and .32549020648002625 in both apps. For the sky lookup use RGBA `(V,(3x+11y+17j) mod 256,(27x+5y+19j) mod 256,255)`, with elevation `(j+1)/9`, turbidity `1.125+.875j`, albedo `j/8` and parameter index j.

For **polygon paths**, vary the side count, requested parent size and seed as (3,16,73), (4,32,53), (5,128,317), (8,64,7), (11,128,211), (17,64,127). Use RGBA32F and compare the whole buffer. Seventeen sides produces a 32×16 buffer and the other five produce 16×16. Seven sides at 64²/seed 199 also matches at U8, U16, F16 and F32.

The **nonuniform color rotation** and **panorama rotation** roots still differ on a new 128² input. Each passed the four original cases and five inventory comparisons. In the eight additional size/format cases above, color rotation matches 35,067/35,072 pixels, maximum .00257879496. Panorama rotation matches 35,070/35,072, maximum .00306707621. All seven smaller cases are exact. For color rotation use the same byte field and channel formulas with t=1,2,3 in place of the channel offsets' multipliers (19t,17t,61t,17t) for the rotation map, pivot map and input, respectively. Set angle multiplier to −.6875 at 16² and .78125 elsewhere, angle offset to .09375 at 128² and −.1875 elsewhere, and pivot to (.8125,.21875). For panorama use the sky color field, rotation .3125 at 16² and −.1875 elsewhere, direction −.34375 at 128² and .5625 elsewhere, and pre/post pole rotations .28125 and −.40625.

### Color burn and color dodge

Color burn and color dodge each match **171,264/171,264 RGBA pixels in 18 cases**, maximum error zero. Each root passed the four original 128²/256² cases (114,688 pixels), the [four-format and independent comparisons](#four-format-inventory-comparison) (17,408), and nine additional cases (39,168). In the independent 32² color-burn field, four pixels had blue foreground 0 and blue background 255. The divide stage differed by 255 levels at precisely those pixels. In color dodge the final switch previously matched 524/1,024 on that field with a maximum of 5.96046448e−8.

For the nine cases, use byte channels modulo 256 with foreground index `t=1`, background 2 and mask 3. Set `V=11x+17y+3xy+31t`, and set RGB to `(V,41x−19y+11t,3x+79y+t)`. Alpha is `42×((13x+7y+31t) mod 7)`. Use V alone for the mask. The 16²/seed 17 and 128²/137 cases use opacity .375 with alpha blending enabled. The 64²/71 cases use opacity .8125 with alpha blending disabled and cover U8, U16, F16 and F32. Three more cases use a signed 32²/211 F32 field with opacity .375 (RGBA bytes divided by 100 minus .625, mask divided by 255), a 32²/43 U8 field with alternating blue endpoints, and a 64×32 input at 64²/53 F32. For alternating endpoints set foreground blue to zero where `(x+3y) mod 4=0` and to 255 elsewhere, then reverse the two values for background blue. The last two cases use opacity .8125 with alpha blending disabled.

Independent atomic switch and divide probes each match **1,122,304/1,122,304 pixels in 34 cases** with byte inputs. Sweep all 65,536 byte pairs in both input orders at opacity 1 and .6137, comparing all four stored formats. The other cases use 64² varying RGBA, mask and alpha fields, including opaque, constant-mask and transparent-background controls. For switch, the mask is read through its transparency and the background's weighted contribution is fused into the rounded foreground product for all four channels. For divide, either signed zero divisor produces 16,777,216 before opacity, including zero divided by zero. Small nonzero divisors retain their sign and use the runtime pixel reciprocal.

The separate straight-alpha divide probe still differs, including 443/4,096 exact RGBA32F pixels on its varying field, maximum .5. On equal-channel float colors bound to grayscale inputs, some outputs differ in the sign of zero. Divide matches 63,488/65,536 pixels at F16 and F32 for both opacities. Switch at full opacity matches 63,744/65,536 at F16 and 63,232/65,536 at F32. At .6137 opacity it matches 65,280/65,536 in both formats. All four signed endpoint fields have zero maximum absolute error.

### Runtime pixel square roots

For each pixel with linear index i in eight 1024² RGBA32F fields numbered j=0..7, set t=(j×2^20+i)/2^23. Read a nearest float input at the pixel center, then output `sqrt(1+t)`, `sqrt(2×(1+t))`, `exp(t)` and `exp(−t)`. The two square-root components improved from 13,947,413/16,777,216 exact (maximum 1.19209290e−7) to **16,777,216/16,777,216 exact**. Every one of the 8,388,608 four-component pixels matches bit for bit, maximum error zero.

The two input intervals cover all 23 mantissa bits for even and odd binary exponents. Within each of 64 equal subdivisions of each interval, fit a quadratic to the measured float outputs. Use the upper 15 partial-product columns of the 16-bit fractional square and round the slope to 23 fraction bits for the first interval or 22 for the second. Round curvature to 24 bits. With these terms fixed, take the midpoint of the minimum and maximum residual and round it to 25 fraction bits for the constant. Evaluate the sum in fixed point, round halfway down and restore the power of two. The implementation uses independently fitted coefficients from procedural fields.

Seven independent 256² fields spanning signed values, bin boundaries, broad exponents, fractions, powers of two, irregular values and tiny magnitudes match **458,752/458,752 pixels**. Two additional 256² RGBA32F probes return the root, equality to zero, equality to itself and its reciprocal. They match **131,072/131,072 pixels**. Negative inputs, signed zeros, subnormals and NaNs produce positive zero in the runtime pixel expression. Positive infinity propagates. Literal square roots keep their separately tested path.

The [surface irradiance](#3d-surface-and-volume-render-passes) matrix still matches 2,304/2,816 pixels. Its two changed 16² RGBA32F outputs differ by at most 2.98023224e−8.

### Natural exponential and ridged noise

Ridged noise with the [original shared lookup](#basic-and-ridged-3d-perlin) matches **253,184/253,184 pixels across 29 cases**, maximum error zero. The seven original cases contribute 163,840 pixels, the [thirteen octave controls](#ridged-noise-octave-isolation) contribute 53,248, and the nine cases below contribute 36,096. The octave with roughness .7 and levels 1–4 now matches all 4,096 LUM32F pixels. The runtime power of .7 to the fourth has bits `3e75dccb` in both apps.

| Position dimensions | Seed | Levels | Roughness | Stored outputs | Exact / total |
|---|---:|---|---:|---|---:|
| 16² | 41 | 1–4 | .3125 | F32 | 256/256 |
| 64² | 71 | 1–4 | .7 | U8, U16, F16, F32 | 4,096/4,096 each |
| 32² | 137 | 2–6 | .61 | F32 | 1,024/1,024 |
| 32² | 211 | 3 only | .825 | F32 | 1,024/1,024 |
| 128² | 311 | 1–7 | .375 | F32 | 16,384/16,384 |
| 32², signed position | 42 | 1–5 | .82 | F32 | 1,024/1,024 |

For the first five rows, use opaque RGBA8 positions with RGB equal to `(7x+13y+3xy, 29x−5y+39, 11y+17x+61)` modulo 256. The signed field uses opaque RGBA32F. Set RGB to `(((17x+3y) mod 67)−9, ((11y−3x) mod 73)−11, ((5xy+19) mod 79)−13)/63`. With each app's own shipped lookup, the seed-zero position ramp at 128² and 256² gives **176/16,384 (maximum 139 levels), 768/65,536 (maximum 154 levels)** for ridged noise. Basic Perlin gives 173/16,384 (maximum 203 levels), 663/65,536 (maximum 203 levels) on the same two ramps. Both roots remain open.

The runtime natural exponential reduces the argument by an integral multiple of ln(2) using two fused subtractions, then evaluates a seventh-degree polynomial on the reduced interval. At pixel index `i` in eight consecutive 1024² fields numbered `j=0..7`, set `t=(j×2^20+i)/2^23`. Both exp(t) and exp(−t) match at every index, **16,777,216/16,777,216 raw float components**, maximum error zero. The two separate square-root controls in those fields match 13,947,413/16,777,216, maximum 1.19209290e−7.

Ten 256² single-channel exponential fields match **655,360/655,360 LUM32F pixels**. Seven use the signed, exceptional, wide-exponent, near-one, fractional, power-of-two and roughness inputs under [runtime pixel logarithms and powers of two](#runtime-pixel-logarithms-and-powers-of-two). For the last three, use a nearest-sampled linear float input at seed 163. Draw 65,536 float32 values uniformly on [−90,90] or [0,ln(2)] with NumPy PCG64 seeds 6841 and 6842, respectively, casting to float32 after drawing. For the boundary field, round `(i mod 257−128)×ln(2)` to float32 and add `floor(i/257) mod 25−12` to its unsigned bit pattern. General power matches **458,752/458,752** pixels on the seven paired-input fields. All finite maximum errors are zero. Counts include bitwise tests for signed zero and nonfinite results.

### General pixel power and ridged noise

Using the original 1024×16 lookup in both apps, all seven [ridged noise comparisons](#basic-and-ridged-3d-perlin) now match **163,840/163,840 pixels**, maximum error zero. This includes changed roughness .7 at 128² in LUM8, LUM16, LUM16F and LUM32F. The [thirteen octave controls](#ridged-noise-octave-isolation) improve from 29,604/53,248 to 51,030/53,248. Twelve match every pixel. With roughness .7 and levels 1–4 the result is 1,878/4,096, maximum 1.19209290e−7. The earlier stock-lookup difference remains independently reproducible.

The general power of a positive pixel argument uses exp(exponent×ln(base)). A negative base with an integral exponent uses its absolute magnitude and negates the result for odd exponents. Fractional powers of negative bases and powers of either signed zero return positive zero, including exponent zero or −1. The exceptional-input field also gives negative zero for a NaN base and an odd integer exponent. Starting with the *reference's* absolute-base logarithm and natural exponential, applying these sign and domain rules reproduces 65,536/65,536 reference powers in each of the signed and exceptional fields, maximum finite error zero.

| Direct MaterialCraft power on the [original fields](#runtime-pixel-logarithms-and-powers-of-two) | Exact / compared | Maximum finite absolute error |
|---|---:|---:|
| Exceptional pairs | 65,536/65,536 | 0 |
| Near one, small signed exponents | 65,536/65,536 | 0 |
| Signed base, exponent −6 to 6 | 61,933/65,536 | 1,073,741,824 |
| Base .5 to 2, exponent −1 to 1 | 58,754/65,536 | 1.19209290e−7 |
| Roughness and octave exponent pairs | 62,788/65,536 | 5.96046448e−8 |

The large absolute error in the signed field occurs on a large-magnitude output. On the octave field .7 raised to 1, 2, 3 and 1.375 gives float bits `3f333334`, `3efae14a`, `3eaf9db5` and `3f1cc3e8` in both apps. At exponent four the reference gives `3e75dccb` and MaterialCraft gives `3e75dcca`. On the same field the standalone natural exponential matches 59,569/65,536, maximum 4.76837158e−7.

### Ridged noise octave isolation

Use the position ramp and original 1024×16 lookup under [basic and ridged 3D Perlin](#basic-and-ridged-3d-perlin). Retain the 128² position input and cook 64² LUM32F at seed 42. Each cell gives exact pixels out of 4,096 and maximum absolute error. Roughness .5 with one or two active levels matches every pixel. Roughness .7 already differs with a single active level.

| Minimum–maximum level | Roughness .5 | Roughness .7 |
|---|---:|---:|
| 1–1 | 4,096 · 0 | 1,847 · 2.38418579e−7 |
| 1–2 | 4,096 · 0 | 1,210 · 1.19209290e−7 |
| 1–3 | 3,704 · 5.96046448e−8 | 1,100 · 1.19209290e−7 |
| 1–4 | 3,331 · 1.19209290e−7 | 144 · 2.38418579e−7 |
| 1–7 | 3,004 · 1.19209290e−7 | 1,320 · 1.78813934e−7 |

Selecting only level 2, 3 or 7 at roughness .7 gives 2,040, 1,871 and 1,841 exact pixels out of 4,096, each with maximum 2.38418579e−7. With the same lookup at 128² and roughness .7, U16, F16 and F32 still match 16,337, 16,380 and 5,160 of 16,384, respectively.

### Uniform angles and planar projection

The grayscale and color 3D planar projection roots each match **168,960/168,960 pixels in 15 cases**, maximum error zero. Their eight [position-ramp and rotated-texture cases](#grayscale-and-color-3d-planar-projection) match 131,072/131,072 per root including the .125-turn 128² case. An additional seven cases match 37,888/37,888 per root. Replaying both first-pass sizes and the separate [four-format and 32² comparisons](#four-format-inventory-comparison) also matches all 81,920 and 17,408 pixels per root, respectively.

For the seven cases generate new linear byte fields at each output size N. Position is (floor(255x/(N−1)),floor(255y/(N−1)),128+floor(63sin(2πx/N)cos(2πy/N)),255). World normal is (128+round(24sin(2πx/N)),128+round(24cos(2πy/N)),250,255). Let z=(29x+7y+3xy) mod 256. Grayscale texture is z and color texture is (z,43x−17y,11x+3y,23x+13y+7), reduced modulo 256. Set scale .53 and size (1,.75). Rotate −.1875 turns at 32² F32 seed 41 and 64² U8/U16/F16/F32 seed 71. Rotate .3125 turns at 128² F32 seed 211. At 64² F32 seed 313 set projection input 1, rotation .0675, target position (.375,.625,.5), target normal (.5,.5,1), and normal threshold .1.

The rotation discrepancy came from computing trigonometry in the processor's output-size parameter. Node parameters, value processors, and pattern parameters use a common runtime approximation. Scale radians by float32(1/(2π)) and add .25 for sine or .5 for cosine. Subtract the nearest even integer, take the absolute value, multiply by four and subtract one to obtain x. Evaluate the odd-power series through x⁷, rounding each product and each sum separately. For k=π/2 the first three coefficients are k, −k³/6 and k⁵/120. The seventh-order coefficient targets 1−2⁻²³ at x=1, using the preceding coefficients before conversion to float32. A literal sine or cosine still uses the ordinary floating-point operation.

| Original function probe | Measurements | Exact / compared | Maximum error |
|---|---:|---:|---:|
| Output-size parameter computing 128 angles per graph | 94 images at 128², 12,032 angle evaluations | 1,540,096/1,540,096 LUM32F pixels | 0 |
| Single parameter, Cartesian components, and literal expressions | 58 images at 16² | 14,848/14,848 LUM32F pixels | 0 |
| Value processor with the same expressions | 58 typed floats | 58/58 | 0 |
| Pattern opacity, scalar or Cartesian component, with matching gray or color declarations | 52 images at 16² | 13,312/13,312 LUM32F or RGBA32F pixels | 0 |
| Pixel function reading an exposed angle directly | 30 images at 16² | 7,680/7,680 LUM32F pixels | 0 |

For the 128-column probe compute angle=float32(start+float32(i×step)), with i=0..127. Assign each result to its own variable in the output-size function. In the pixel function return the variable indexed by the output column. Use the nine (start,step) pairs (0,1/80), (0,1/8192), (.75,1/4096), (−16,.25), (float32(π/2)−64/2²⁰,2⁻²⁰), (−64/2²⁴,2⁻²⁴), (−987654,16037.125), (−1.5625,1/80) and (−2π,π/32), seed 41. For each operation initialize NumPy default_rng(431) and draw 32 more starts uniformly on [−20,20), each followed by a step uniformly on [−1,1) for the first 16 or [−.0001,.0001) for the rest. Use seed 257 for the graphs. The remaining six pairs at seed 73 are (2³⁰,65536), (−2³³,2¹⁸), (10²⁰,2×10¹⁷), (0,−2⁻³¹), (.75,2⁻²⁶) and (1.5,2⁻²⁵). Inputs and step constants are float32.

The single-angle parameter and typed probes use .625, .125, −.125, .38, 0, 1, −.0078125, π/4, −π/4, 2π, −974634.875, 10⁻⁷ and 10⁹ with alternating seeds 42 and 17. Cartesian radius is −1.25. Literal expressions use .625, π/4 or 2π at seed 73. For the pattern, give a single square size (1,1), seed 17 and F32 precision. Set opacity to sine, cosine, or the corresponding Cartesian component with radius .875. Use .125, .625, π/4, 1, 10⁻⁷ and .38, plus literal-only .625. The independent pixel-parameter probes add .7 and π at seed 71 and agree with the [runtime pixel quarter wave](#runtime-pixel-sine-cosine-and-atlas-rotation). A grayscale pattern producer with a color output declaration had incompatible channel counts in a separate control. Its reference output was RGBA32F and MaterialCraft returned one channel.

### Runtime pixel sine, cosine and atlas rotation

Sine and cosine match **1,443,840/1,443,840 LUM32F pixels** across 30 original Pixel Processor cases. Exactness includes signed zero and all 84 nonfinite output components. Maximum finite error is zero. Direct Cartesian coordinates and separately multiplying the radius by the cosine and sine match another **197,632/197,632 component pixels**. With radius −1.25 and angle float32(π/4), the two coordinate components have bits `bf62462e bf624630`.

The runtime pixel operations use a 25-bit phase. The low 25 bits of the angle reduction determine a quadrant and position in a 64-interval quarter wave. Each quadratic is derived from the sine function. Its fractional square retains the same partial products as runtime division. Reflection across a quadrant edge complements the phase bits, and the magnitude truncates to float32. At angle .625 the runtime sine and cosine have bits `3f15c8eb` and `3f4f9b48`. The literal-only results are `3f15c8ef` and `3f4f9b47`.

Use separate grayscale processors to nearest-sample the red channel of each original linear RGBA32F input and return its sine or cosine. Let i be the row-major pixel index. For the hashed fields start with h=(747796405i+2891336453) modulo 2³², replace h by ((h >> ((h >> 28)+4)) XOR h)×277803737 modulo 2³², then XOR it with h >> 22. Round each input to float32 before sampling.

| Inputs | Size and seed | Exact pixels, sine and cosine together |
|---|---|---:|
| R=(q+.625)π/2²⁴, with q=64i, i, 1966080+i, 4194304+i−32768, or a sample without replacement from 0..4194303 using NumPy default_rng(117) | Five 256² fields, seed 113 | 655,360/655,360 |
| R=float32(i/65535)×float32(2π), or R with bits `0x3f490fdb-32768+i` | Two 256² fields, seed 42 | 262,144/262,144 |
| R=(26h/2³²−13)π, R=(2h/2³²−1)×1037287, or float bits with sign i mod 2, exponent floor(i/2) mod 255 and mantissa h mod 2²³ | Three 256² fields, seed 137 | 393,216/393,216 |
| Float32((i mod 32−16)π/2), offset its unsigned bit pattern by floor(i/32) mod 17−8 and replace nonfinite inputs by zero | 256², seed 137 | 131,072/131,072 |
| Cycle through ±0, ±.1, .625, π/4, −π/4, π/2, π, ±2π, 589.9901123047, −974634.875, ±2³⁴, ±∞ and NaN in that order | 16², seed 17 | 512/512 |
| Literal-only .625, −π/4 and 2π | Three 16² cases per operation, seed 17 | 1,536/1,536 |

For Cartesian coordinates, read radius from red and angle from green. In the two 128² signed fields use radius 9i/16384−4.5 and angle (2h/2³²−1) multiplied by 13π or 1037287. A third field cycles radius through (0,−0,1,−1,.375,−1.25) and changes angle every seven pixels through (0,−0,π/4,π/2,π,2π,−2π,−974634.875). The 16² literal control uses (.875,.625). Read each component separately at seed 313. Before this correction, the direct large-angle cosine coordinate matched 1/16,384 pixels with maximum error .184980124.

The atlas crop root matches **378,880/378,880 pixels in 37 cases**, maximum error zero. In the [wired atlas wrapper](#atlas-crop-with-wired-value-inputs), default and .125-turn rotation each match 114,688/114,688 RGBA32F pixels across seven parent-size, seed and precision settings. Grid view, zero count and count two each match 16,384/16,384. Adjusting the final processor itself to U8, U16, F16 and F32 gives eight further matching 64² outputs, 32,768/32,768 pixels. This includes the 128 differing edge pixels previously measured at .125 turns and 256².

For twelve additional wrapper cases, keep the single shape table and white index field. At 128² bind linear RGBA bytes (3xy+19y+7x, 5x−31y, 17x+11y, 23x+13y), modulo 256. At 16² use v=(17x+53y) mod 256 and background (v,255−v,3xy mod 256,(7x+3y) mod 256). Opacity is zero where (floor(x/3)+floor(y/2)) mod 3 is zero and (37x+11y) mod 256 elsewhere. Test −.1875 turns without the grid and .3125 turns with the grid, grid opacity .375 and selection opacity .625. Each rotation matches 33,792/33,792 at 32² seed 73, 64² seed 137 with four parent precision requests and 128² seed 313. The instance stores RGBA32F in all twelve cases.

With the [planar projection inputs](#grayscale-and-color-3d-planar-projection), .125 turns still matches 16,335/16,384 LUM16 pixels and 16,378/16,384 RGBA8 pixels. Maxima are 226 and one stored level. A separate graph computing sine or cosine in the node's output-size function gives `3f350503` for both operations at float32(π/4). This is a distinct arithmetic path from the per-pixel function.

### Runtime angles and circular gradient

The Circular Gradient root matches **163,840/163,840 pixels** in thirteen cases, maximum error zero. Its default outputs at 128² and 256² with seed zero and 128² with seed 42 match all 98,304 pixels. Set the two points to (.3,.6) and (.8,.7), angle .2 and non-square expansion off. At 128² seed zero the authored output matches 16,384/16,384. At 64² seed 42, U8, U16, F16 and F32 each match 4,096/4,096. For an independent rotation, use (.71875,.21875), (.0625,.8125), angle −.15625 and enable non-square expansion. The 128² F32 output at seed 71 and all four formats at 64² seed 137 match another 32,768/32,768 pixels. This root takes no image input.

Nearest-sample R and G from an RGBA32F input and emit the angle of the vector (R,G) in radians. The six runtime fields below match **230,400/230,400 raw LUM32F samples**. Maximum finite error is zero. All 186 nonfinite outputs in the axes field agree bit for bit. The signed field previously matched 5/16,384 with maximum error .000235080719.

| Field | Definition | Exact pixels / total |
|---|---|---:|
| Signed channels, 128² | R and G from the original field under [3D linear gradient](#3d-linear-gradient), seed zero | 16,384/16,384 |
| First octant, 128² | R=1, G=i/16,383, seed zero | 16,384/16,384 |
| One turn, 256² | R=float32(i/65,535) times float32(2π), G=1−R, seed 42 | 65,536/65,536 |
| Near π/4, 256² | R has bits `0x3f490fdb-32768+i`. G=float32(8h/2³²−4), h=(747796405i+2891336453) modulo 2³², seed 42 | 65,536/65,536 |
| Axes and exceptional values, 32² | At x and y independently cycle through +0, −0, 1, −1, +∞, −∞, quiet NaN, 2⁻¹⁴⁰, 2⁻¹²⁰ and 2¹²⁰, seed zero | 1,024/1,024 |
| Exponent range, 256² | R=2^(x−128), G=2^(y−128). Negate R when (x+y) mod 3 is zero and G when (x+2y) mod 5 is zero, seed 91 | 65,536/65,536 |

Here i is the row-major pixel index. The first-octant fit is quartic in the smaller-to-larger magnitude ratio. It uses the measured runtime reciprocal and the same fused accumulation order as a four-channel dot product. In the other quadrants, reflection across the vertical and horizontal axes requires different additions of π/2 or π. Four separate literal-only controls at 32², seed zero, with vectors (1,1), (−1,−1), (.2,−.6) and (0,0) match another 4,096/4,096 LUM32F pixels. The runtime (1,1) angle has bits `0x3f490fe1` and the literal result has bits `0x3f490fdb`.

### Pixel Processor random draws

Emit four independent calls to `rand(1)` as RGBA32F. In the reference app the four values recover a unique initial 24-bit state at each of **27,968/27,968 pixels**. Advance the state with `s=(3626811*s+51137) mod 2^24` and return `floor(s/2)/2^23`. The four outputs then match at every pixel, maximum error zero. Use square canvases of sides 8, 16, 32, 64 and 128 at seed zero, also 32² at seeds 1, 2, 42 and 137, and 32×16 and 16×32 at seeds zero and 42. The first 32² pixel starts from 9,091,073 at seed zero and 3,342,178 at seed 42.

Four additional original graphs replicate a single draw to RGBA, reuse two draws in alternating channels, double a reused draw, or apply ranges 1, 1, −2 and .375 to successive draws. At 16², 32² and 64² with seed zero and 32² with seed 42, all **25,600/25,600 reference pixels** agree with the corresponding channel expressions, maximum error zero. The MaterialCraft evaluator now matches the progression given the recovered initial state. Before the [per-pixel initialization fix](#per-pixel-random-initialization), the native and CPU four-draw images at 32² matched 0/1,024 entire pixels at seeds zero and 42, with maximum component errors .995849967 and .989675045.

### Color conversion, checker and clone

The ACEScg-to-linear-sRGB root matches **131,072/131,072 RGBA pixels across eight cases**, maximum component error zero. Bind the following original linear RGBA8 fields directly to its image input. At integer texel `(x,y)` on an `N×N` field, let `U=floor(255*x/(N-1))` and `V=floor(255*y/(N-1))`. Arithmetic in the four stored byte channels wraps modulo 256.

- Foreground RGBA is `(17*x+31*y, 3*x+7*y, 11*x+5*y, 5*x+11*y)`.
- Background RGBA is `(29*x+13*y, 255-U, V, 13*x+3*y)`.
- The scalar mask is `7*x+9*y`.

For the first four cases, use the foreground field at 128² and 256² with seed zero, repeat 128² with seed 42, then substitute the background at 128² with seed zero. The RGBA8 outputs match 16,384, 65,536, 16,384 and 16,384 pixels respectively. For the four remaining cases, keep the 128² background input, cook at 64² with seed 42, and select U8, U16, F16 and F32 on the final processor. Each format matches 4,096/4,096 pixels. The float32 output contains 4,096 distinct RGBA values and spans −0.53148961 to 1.51964462.

Two other roots matched the original four eight-bit cases at 128²/256², including seed 42 and the changed settings below. Each matched 114,688/114,688 eight-bit pixels, maximum error zero. The first 64² seed-42 format sweep exposed the differences below.

| Root and changed setting | U8 exact / max | U16 exact / max | F16 exact / max | F32 exact / max |
|---|---:|---:|---:|---:|
| `checker_1.sbs/checker_1`, tiling 3, 45-degree rotation enabled | 4,096 / 0 | 4,096 / 0 | 3,740 / 4.8828125e-4 | 4,096 / 0 |
| `clone.sbs/clone`, offset (.17, −.25), mask blur 0, original foreground and mask | 4,096 / 0 | 4,073 / 1 | 4,096 / 0 | 994 / 1.192093e-7 |

Each format column compares all channels of 4,096 pixels. Clone uses 128² input fields in the 64² cases.

### Clone mask and alpha rounding

In the 64² Clone graph the transformed foreground, source image and blurred mask each matched all 4,096 source pixels. The final normal-copy Blend produced the entire U16 and F32 discrepancy. After correcting that operation, all eight cases listed above match **131,072/131,072 pixels**, maximum error zero. This includes 4,096/4,096 each in RGBA8, RGBA16, RGBA16F and RGBA32F at 64².

For an independent 64² Blend control, form byte-valued foreground `(17*x+31*y, 7*x+3*y, 11*x+5*y, 5*x+11*y)`, background `(29*x+13*y, 128+7*x, 5+5*y, 13*x+3*y)` and mask `7*x+9*y`, each modulo 256. Use default normal-copy blending and seed zero. Test each as linear U8 and as float32 normalized by 255. The grayscale float fields may be encoded as equal RGB channels with an opaque alpha and connected to a grayscale input.

For each input format, compare the varying fields, an opaque foreground, a white mask, white and black foreground RGB, white and black background RGB, zero background alpha, and constant foreground alpha 127 with mask 159. Repeat the first three controls with grayscale source and background. Each of these 24 float-output cases matches 4,096/4,096 pixels, maximum error zero.

With float inputs, repeat the varying field, black background RGB and zero background alpha at global opacity .3 and .625. Repeat at .3 with alpha blending disabled. For signed fields replace each normalized input component `v` with `3*v-1` and run at opacity .3 and 1. Finally test opacity .07 with and without the mask, and opacity 1e-8 with the mask. All 24 controls match 4,096/4,096 pixels each, maximum error zero.

The measured operation first takes the complement of mask transparency in float32, then multiplies by the global opacity. Color weights take the complement of foreground transparency. Output coverage uses the original foreground alpha. Fusing the background contribution into the weighted foreground reproduces the measured RGB rounding, including negative inputs and non-opaque coverage.

### Height to normal in world units

Both roots in `height_to_normal_world_units.sbs` match **131,072/131,072 RGBA pixels each** across eight cases. For a square grayscale height input, store `(17*x+31*y) mod 256` at each integer coordinate. Use default settings at 128² and 256², repeat 128² with seed 42, then set `surface_size=100`, `height_depth=8`, `normal_format=0` and `sampling=1` at 128² with seed zero. Each root matches 16,384, 65,536, 16,384 and 16,384 RGBA8 pixels respectively. Keep those changed settings and the 128² input, then cook at 64² with seed 42. U8, U16, F16 and F32 each match 4,096/4,096 pixels for both roots. Every maximum error is zero. Each 64² output contains 49 distinct normals.

### Height distance, texture mapping, skew and weave

Twelve roots match **163,840/163,840 pixels each**, maximum component error zero, across thirteen cases per root. The total is **1,966,080/1,966,080 pixels**. Both versions of the render helpers were cooked separately. Read the final output bridge at its native dimensions and storage precision. The F32 controls below contain 36 distinct outputs for the height-distance root, 42 for Weave 3 and at least 3,682 for each mapping, shift or skew root.

| Package and graph | First changed case at 128² | Independent field and changed settings |
|---|---|---|
| `normal_to_height_hq.sbs/base_algorithm_height_distance` | Intensity .6, light angle .3, light offset .4, detail balance .6 | Height field, respectively .875, .125, .375 and .5 |
| `pbr_render.sbs` and `pbr_render2.sbs`, `PBR_render_mapping_color` | Substitute foreground for background texture | Color and UV fields, background color (.25,.5,.75,.375) |
| Both render packages, `PBR_render_mapping_grayscale` | Substitute full-range gray for limited-range gray | Height and UV fields, same background color |
| Both render packages, `chroma_shift` | Radius 2, angle .125 turns | Color field, radius 3.5, angle .3125 turns |
| `skew.sbs/skew` and `skew_grayscale` | Axis 1, amount .35, align 0 | Color or height field, axis 0, amount −.3125, align 2 |
| `uv_mapper.sbs/uv_mapper_color_sub` and `uv_mapper_grayscale_sub` | Filtering mode 0 | Color or height and UV fields, filtering mode 1. Color background (.25,.5,.75,.375) |
| `weave_3.sbs/weave_3` | Tiling 2, rotation enabled | Tiling 3, rotation disabled |

For each root the first three cases use defaults at 128² and 256² with seed zero, then 128² with seed 42. The fourth uses the first changed setting in the table at 128² and seed zero. Each root matches 16,384, 65,536, 16,384 and 16,384 pixels respectively. Height distance and Weave 3 use LUM16 here. The other roots use eight-bit output. At 64² use seed 42, the same changed control and U8, U16, F16 and F32 output. Each matches 4,096/4,096. Inputs keep their 128² source dimensions, except the limited-range gray fixture, which has a 64² counterpart.

Use the foreground and background byte fields specified under [color conversion, checker and clone](#color-conversion-checker-and-clone). The full-range gray is `(17*x+31*y) mod 256` and the limited-range gray is `32+((17*x+31*y) mod 160)`. At side N the original UV input is `(floor(255*x/(N−1)),floor(255*y/(N−1)),128,255)`. Height distance and grayscale skew take the full-range gray, chroma shift and color skew take the foreground. UV mapping takes the background or limited-range gray. Weave 3 has no image input.

For the five independent cases, use F32 at 128² with seed 71 and all four formats at 64² with seed 137. Inputs match the canvas size. Generate byte channels modulo 256 at integer (x,y). Color is `(11*x+37*y+29,41*x−19*y,3*x+79*y+1,13*x+7*y)`, height is its red channel and UV is `(5*x+3*y,9*y−7*x,round(255*y/(N−1)),A)`, where A is zero on `(x+y) mod 7 = 0` and 255 elsewhere. Each root matches all 16,384 float pixels and 4,096 pixels in each 64² format.

The same independent checks exposed remaining differences in five additional roots. All four 64² formats at seed 137 are exact. At 128² and seed 71 their F32 outputs differ as follows.

| Root | Changed control | Exact pixels / total | Maximum component error |
|---|---|---:|---:|
| `pattern_stripes.sbs/stripes` | Stripes 9, width .28125, softness .125, shift 3, align 2, filtering disabled | 16,341/16,384 | 1 |
| `quad_transform.sbs/quad_transform` | Color field, tiling enabled, four corners (.1,.125), (.2,.9), (.9,.2), (.8,.85) in parameter order p00, p01, p10, p11 | 16,378/16,384 | .00387579203 |
| `quad_transform.sbs/quad_transform_grayscale` | Height field, same corners | 16,378/16,384 | .000564604998 |
| `quantize_color.sbs/modify_color_palette_sub` | Color field, 16-bit ID equal to 4,369 times `(7*x+13*y) mod 16` | 254/256 | 1.19209290e-7 |
| `shape_splatter.sbs/shape_splatter_blend_color` | Two active patterns, color input opacity .625, HSL adjustment (.125,−.25,.0625) | 16,381/16,384 | 8.94069672e-8 |

For the palette control use selection mode 1, index 6, spread .125, contrast .25, distance color space 0, mode 1, opacity .375, override (.8,.125,.3) and HSL (.125,−.25,.0625). For the splatter control, bind Color to the background and second data input, UV to the color input and first data input. For pattern number j from 1 through 8, its RGBA bytes are `(13*x+11*y+23*j,5*x+31*j,17*y+13*j,19*x+7*y+29*j)` modulo 256.

For `spline_tools.sbs/internal_tools/compute_bridge_distance`, wire a Float2 value processor returning (2,1) into parent size and an Integer value processor into spline amount. Enable non-square correction. Bind the original 16² coordinate circle described under [spline preprocessing with wired counts](#spline-preprocessing-with-wired-counts). With count 1, the LUM32F outputs at 16² seed zero, 32² seed 42 and 64² seed 137 match 256/256, 1,024/1,024 and 4,096/4,096, maximum zero. With count 2 the counts are 252/256, 992/1,024 and 4,032/4,096. The respective maxima are 5.96046448e-8, .228374884 and .0570636913.

### Alveolus quadrant placement and falloff

In an original blank 64² pattern graph, follow the lower-left quadrant one, two or three times and draw a square of size (.25, .25) at the leaf. With zero offsets its centers are (16, 48), (8, 56) and (4, 60) in pixel-edge coordinates. Applying (.25, .125) to either the branch offset or the pattern offset moves the center by (16, 8) texels at every depth, with tiling at the image bounds. The size still shrinks at each depth. A separate unit Gaussian stamp shows a circular cutoff and a falloff of approximately `max(0, exp(-11.1*r²) - exp(-11.1))` in local coordinates. At 64², unit opacity and F32, the maximum difference between that analytic profile and the observed red channel is 3.5e-7.

The corrected offsets and Gaussian profile bring the default Alveolus pattern stage from **0/8,192** LUM16 pixels (maximum error 55,100) to **8,096/8,192** (maximum error 1). The following table compares the final output bridge. This generator uses no input image.

| Case | Size | Seed | Output format | Exact pixels / total | Maximum error |
|---|---:|---:|---|---:|---:|
| Defaults | 128² | 0 and 42 | LUM16 | 16,192/16,384 each | 2 |
| Defaults | 256² | 0 | LUM16 | 64,896/65,536 | 2 |
| `Gradient_Filled_Cells=true`, `Interstice_Width=.1` | 128² | 0 | LUM16 | 16,264/16,384 | 1,000 |
| Same changed parameters | 64² | 42 | LUM8, LUM16, LUM16F, LUM32F | 4,096/4,096 each | 0 |

### Anisotropic blur

For the grayscale root bind `(17*x+31*y) mod 256`. For the color root use the same value in R, `U` in G, `255-V` in B and 255 in A. The definitions of `U` and `V` are under [color conversion, checker and clone](#color-conversion-checker-and-clone). At 128² the color input bridge matches all 16,384 pixels. The first directional pass already differs.

| Case | Color exact / total | Gray exact / total | Maximum error, color / gray |
|---|---:|---:|---:|
| Defaults, 128², eight-bit | 11,526/16,384 | 11,526/16,384 | 1 / 1 |
| Angle .125 turns, anisotropy .8, 128², eight-bit | 3,228/16,384 | 3,310/16,384 | 16 / 16 |
| Quality 1, intensity 4, 256², eight-bit | 65,536/65,536 | 65,536/65,536 | 0 / 0 |
| Same 256² input and changed settings at 64², seed 42, U8, U16, F16, F32 | 4,096/4,096 each | 4,096/4,096 each | 0 / 0 |

At the first directional pass the default color case matches 11,944/16,384 pixels with maximum error 1. With angle .125 it matches 2,997/16,384 with maximum error 21. A separate F32 impulse at (64,64) on a 128² field, intensity 10 and angle .125, has nonzero samples on both sides of the diagonal.

### Pixel Processor canvas and mip maxima

The `mip_max` helpers in `rt_irradiance.sbs` and `rt_shadows.sbs` each match **167,168/167,168 pixels in 17 cases**, maximum error zero. Each exposes one grayscale input and uses fixed internal parameters. The four first-pass cases account for 114,688 pixels, the four precision cases and independent 32² case for 17,408, and the eight cases below for 35,072. The earlier four-format table records the original size discrepancy.

At 64², compare the input bridge, nine reduced-resolution processors, the final processor and its output bridge on the 64² levels field from the four-format table. Each root matches 13,656/13,656 pixels across all twelve stages. The reductions produce LUM8 at 32², 16², 8², 4², 2² and four times at 1². The last processor and output bridge produce LUM32F at 64². Previously the final processor inherited its first sampled texture's 32² dimensions.

| Original grayscale input | Output and seed | Exact pixels per root | Maximum error |
|---|---|---:|---:|
| U8 at 16² and 128², (17x+53y+7xy) mod 251 | Same size, F32, seeds 41 and 137 respectively | 16,640/16,640 | 0 |
| U16 at 64², (711x+1337y+17xy) mod 65521 | 64² U8, U16, F16 and F32, seed 71 | 16,384/16,384 | 0 |
| 64×16 F32, (((17x−31y+3xy) mod 237)−63)/127 in RGB, alpha one | 32² F32, seed 211 | 1,024/1,024 | 0 |
| 16×32 U8, constant 93 | 32² U16, seed 313 | 1,024/1,024 | 0 |

For the isolated size control, supply a 16×8 linear RGBA8 texture with (11x+31y+3xy) mod 256 in RGB and opaque alpha. Sample red at `$pos` with nearest filtering in a grayscale F32 processor. Use graph sides 32, 64 and 128 with seeds 0, 42 and 137 respectively. Missing output size, parent-relative (0,0) and input-relative (0,0) each match 1,024, 4,096 and 16,384 pixels at the parent sizes. Absolute size exponents (5,4) match all 512 pixels at each parent size. The 16×8 input is exact throughout.

With the same input, a zero-intensity blur uses the input dimensions for missing or input-relative size and matches 128/128 pixels. Absolute 32×16 matches 512/512 at every parent size. Explicit parent-relative blur still differs when resizing to 32², 64² and 128². The exact counts are 631/1,024, 1,476/4,096 and 4,738/16,384, with maximum byte errors 17, 38 and 49 respectively.

### Fixed graph and instance output sizes

Both `pattern_tile_sampler.sbs/arrow` and `tile_sampler.sbs/arrow` specify an absolute 512×512 graph output. MaterialCraft now imports and saves that size separately from the default parent canvas. Each output of a graph instance also retains the dimensions and storage precision of its source. Downstream connections and cache hits use the selected output port.

For a size control, draw a centered white square half the canvas width and height on black. Give its graph an absolute output size of (6,5) in base-two exponents. With parent sizes 64²/U8, 128²/U16, 256²/F16 and 128²/F32, the stored output is 64×32. All four cases match 2,048/2,048 pixels, maximum error zero. Use seed zero for the first two cases and 42 for the last two.

A second control fixes only the final processor to 64×32 F32. Place the child in an original wrapper with a 128² or 64² parent. Exercise inherited instance size, parent-relative (0,0), and absolute (5,4). For each setting also request U8, U16, F16 and F32 on the instance. Each of these 18 cases produces a 64×32 LUM32F image and matches 2,048/2,048 pixels, maximum error zero. A two-output evaluator regression separately exercises a 64×32 F32 port, a 4×8 U8 port, their downstream consumers and a cached cook.

Neither Arrow root has input pins or exposed parameters. Restoring the fixed size brought each to 261,338/262,144 LUM8 pixels at its default 512² size, including seed 42, with maximum error 131. With a 64² parent and seed 42 the stored dimensions remained 512². The following counts precede the transform filtering correction.

| Stored format | Exact pixels / total | Maximum error |
|---|---:|---:|
| LUM8 | 261,338/262,144 | 131 |
| LUM16 | 261,338/262,144 | 33,667 |
| LUM16F | 260,826/262,144 | 0.513916015625 |
| LUM32F | 260,826/262,144 | 0.5137255192 |

### Transform edges and floating-point filtering

Both Arrow roots now match **1,835,008/1,835,008 pixels each**, maximum error zero. Repeat the 512²/U8 default twice at seed zero and once at seed 42. Then use a 64² parent and seed 42 with LUM8, LUM16, LUM16F and LUM32F on the final Blend. Every run returns a 512² image and matches 262,144/262,144 pixels. Checker 1 also matches **131,072/131,072 pixels**, maximum error zero, across the eight cases under [color conversion, checker and clone](#color-conversion-checker-and-clone). Its rotated 64² half-float result went from 3,740/4,096 to 4,096/4,096.

The first two mismatching Arrow stages transform a uniform white image with tiling off. A smaller control uses an original white 128² RGBA8 input with a transparent matte and bilinear filtering. Translate by (.25,.375) texels. The last column, last row and bottom-right corner of its RGBA32F output contain 49,151/65,535, 40,959/65,535 and 30,720/65,535 respectively. Each outside sample contributes the matte according to its own weight. Two further controls use matrix (1.23654997,0,0,4) with UV offset (.118276998,0), or matrix (1.01874995,−.982617021,1.01874995,.982617021) with offset (0,.367500007). These three transforms match 16,384/16,384 pixels in each of four output formats, maximum error zero.

To isolate matte precision, move a 128² image two UV units along both axes with tiling off. Set the matte to (.5,.1,−.312345,1.144444). Use U8, U16, F16 and F32 input storage and repeat with each of the four output formats. All 16 cases match 16,384/16,384 pixels each. In the F32 outputs the red channel retains the source-format levels 127/255, 32,767/65,535, .5 and .5 respectively.

For floating-point filtering, use the original signed 128² RGBA field specified under [3D linear gradient](#3d-linear-gradient). Store it first as F16 and then as F32. Use a tiled identity transform, F32 output, seed zero and offsets in texels of (.37,.18), (−.129,.678), (.601,0), (0,0), (.0039,.496) and (.501953125,.376953125). Each of these 12 cases matches 16,384/16,384 pixels, maximum error zero. The measured filter quantizes the four weights to a sum of 256. Per component it aligns contributing texels to their largest exponent with four extra significand bits, truncates, accumulates the weighted sum, and rounds to the source precision with halfway cases away from zero.

The same sampler was measured independently inside a Pixel Processor. Bind a 128² RGBA field to input zero, add a texel offset divided by 128 to the pixel-center UV, then read the input with bilinear filtering into an RGBA32F output. Run offsets (.25,.375), (.37,.18) and (−.129,.678) for each of U16, F16 and F32 input storage. All nine runs match 16,384/16,384 pixels each, **147,456/147,456** overall, maximum component error zero. The F16 and F32 cases use the same signed input field and produce different filtered values.

### Linear gradient 2 and multiangle albedo

Each root matches **131,072/131,072 pixels in eight cases**, maximum component error zero. Use defaults at 128² and 256² with seed zero, repeat 128² with seed 42, then run the changed settings at 128² with seed zero. Those four runs match 16,384, 65,536, 16,384 and 16,384 pixels respectively. Keep the changed settings and render at 64² with seed 42. Each of U8, U16, F16 and F32 matches 4,096/4,096 pixels for both roots.

- `gradient_linear_2` generates its own input. The first four outputs are LUM16. For the changed cases set `Tiling=3` and `rotation=1`.
- `multiangle_to_albedo` uses the foreground field on input 1 and the background field on inputs 2 through 8, as specified under [color conversion, checker and clone](#color-conversion-checker-and-clone). The first four outputs are RGBA8. For the changed cases set `samples_amount=4` and keep the 128² inputs for the 64² format sweep.

Six additional albedo cases use a different original image on each of the eight pins. For input number `j=1..8` the RGBA bytes are `((17+j)*x+(11+2*j)*y+29*j, (7*j+1)*x+5*y+13*j, 3*x+(13*j+1)*y+41*j, (5+j)*x+(11-j)*y+23*j)` modulo 256. Match the input and output dimensions. The default at 64² with seed zero matches 4,096/4,096 RGBA8 pixels. Four samples at 128² and seed 137 match 16,384/16,384 RGBA8 pixels. At 32² and seed 42 the same setting matches 1,024/1,024 pixels in each of U8, U16, F16 and F32. Maximum error is zero throughout.

### Atlas crop with wired value inputs

For `atlas_spliter.sbs/crop_and_transform_sub`, connect constant value processors to `shape_count` and `shape_data_table_size`. Use count 1, table size (1,1), and a single RGBA32F table texel (0,0,1,1). Bind a transparent 16² background and solid white 16² shape-index and opacity images. For the 128² color image, use linear RGBA8 texels `((17*x+31*y)%256, floor(255*x/127), 255-floor(255*y/127), (7*x+13*y)%256)`. Give the instance a parent-relative (0,0) size. The reference app returns RGBA32F at its authored precision.

| Rotation | 128² seed 0 or 42, exact / max | 256² seed 0, exact / max | 64² seed 42, exact / max |
|---|---:|---:|---:|
| Default | 16,384/16,384 · 0 | 49,152/65,536 · 0.250003815 | 4,096/4,096 · 0 |
| .125 turns | 16,320/16,384 · 1 | 49,091/65,536 · 1 | 4,064/4,096 · 0.996078431 |

The 16,384 differences at default rotation and 256² occupy exactly the lower-right 128² quadrant. At 128², three further controls each match 16,384/16,384, maximum error zero. Enable grid view, set count and table dimensions to zero, or set count to 2 and table dimensions to (1,2), respectively. All three use the same single table texel.

Set the final Pixel Processor's storage explicitly, keep the 128² input, and cook at 64² with seed 42. At default rotation, RGBA8, RGBA16, RGBA16F and RGBA32F each match 4,096/4,096 pixels, maximum error zero. At .125 turns, each matches 4,064/4,096. The respective maximum errors are 254, 65,278, 0.995605469 and 0.996078431.

### Spline preprocessing with wired counts

For `spline_tools.sbs/internal_tools/scatter_splines_on_splines_preprocess`, wire integer value processors to `input_spline_amount` and `custom_spline_amount`. Use input counts 1 and 2, custom count 1, `side=3`, `amount_mode=0` and `spline_amount_per_spline=1`. The 16² grayscale mask stores `(11*x+7*y)%256`. An original 16² circle coordinate buffer and its tangent buffer supply the other two images.

At 16² with seed zero or 42 and at 32² with seed zero, the two input counts return the exact integers 1 and 2 in all six comparisons, maximum error zero. With count 1 the image has two distinct RGBA8 values. All three images match 32,768/32,768 pixels after the instance retains the 2048×16 source dimensions. Before the fix MaterialCraft returned a 16² or 32² image through the wrapper.

To reproduce with a fresh pair of circle buffers, initialize two 16×16 RGBA32F images to zero. For `x=0..15`, let `a=2π(x+.5)/16`. In row zero, store `(.375+.1875*cos(a), .625+.1875*sin(a), 1, -1)` and `(-sin(a), cos(a), 0, 1)`. A separate run with these fields also matches all six typed integers and all 98,304 RGBA8 pixels through the wrapper, maximum error zero.

With the 16² parent, seed zero and count 1, set the output FX-Map storage explicitly to RGBA8, RGBA16, RGBA16F and RGBA32F. Each matches 32,768/32,768 pixels, maximum error zero. The reference app twice failed to return an image for the disconnected zero-count default within six minutes. That case has no pixel count or error measurement.

### Facet masks and stripe boundaries

**Mask to facets** matches **52,736/52,736 pixels in 17 cases**, maximum component error zero. Four original 16² RGBA32F cases use the striped mask or mask gradient described below, seeds zero and 42, and thresholds .5 and .3. Four more at 64² seed 42 use the original gradient at threshold .3 and cover RGBA8, RGBA16, RGBA16F and RGBA32F. A 32² seed 113 control uses the independent gray1 field at .3.

For the eight further cases, test all-black and all-white 16² masks at seeds 37 and 57 with threshold .5. For the spatial mask use (9x+7y+3xy) mod 256 inside the union of a circle with center (.35N,.4N) and radius .29N and the region x>.65N, y>.5N. Set all other pixels to zero. Use 32² F32 seed 71 at threshold .5, then 64² in four formats and 128² F32 at threshold .625, with seeds 137 and 193 respectively. Those eight cases match 34,304/34,304 pixels, including 689 distinct RGBA32F values at 64².

For **both Stripes roots**, use 13 stripes, width .71875, softness .3125, shift 4, align 0 and filtering enabled. Each matches 17,408/17,408 pixels across 32² F32 seed 37 and 64² seed 137 in four formats. With nine stripes, width .28125, softness .125, shift 3, align 2 and filtering disabled at 128² seed 71, the pattern-package root matches 16,341/16,384 pixels in each format. The standalone-package root matches 16,329/16,384 in each format. The maxima for both are 255 U8 levels, 65,535 U16 levels or 1 in F16/F32.

### Four-format inventory comparison

This sweep covers **375 roots and 406 output producers** that were unresolved when the sweep began. Each row reports a 64² parent with seed 42. Set the producer of the named output to U8, U16, F16 and F32 in turn. Retain the graph’s declared size rules and the dimensions of each bound input. A separate 32² F32 run at seed 113 covers **383 producers in 352 roots**. The other roots have the dedicated controls linked from the inventory. The first-pass records below supply defaults, alternate resolutions and seed comparisons. The per-root sections above retain the more demanding controls, including measured failures when the sweep happens to match.

Each numeric cell gives **exact pixels / total pixels · maximum absolute component error**. A pixel counts only when every channel matches. For a typed output the unit is one complete value. Unsigned errors use stored levels, and F16/F32 comparisons use the raw bit patterns and finite component differences. ¹ indicates a local storage grid or format discrepancy. ² indicates nonfinite components. A dash for the maximum means no finite pair. A size pair gives local dimensions followed by reference dimensions. Cook errors and incompatible dimensions have no comparable samples. The four requested formats can yield the same typed output or a fixed image size.

The 64² checkpoint column identifies the code used for that row. In build order, `checked` includes pixel product fusion, `sampler` includes the first float texture filter, `filtered` includes transform matte behavior, `bridge` includes source-precision input resizing, `angle` adds the runtime arc tangent, `gray` adds grayscale reads of color, `scope` adds value coordinates, `value` also averages color in value reads, and `loop` adds typed zero for skipped loops. The 32² comparison uses the `scope` checkpoint. The reference app is version 16.0.7 with the Direct3D 11 backend. Later work has its own dated numerical records.

#### Procedural inputs

The 64² input name includes the source side when applicable. All pixel indices start at zero. Unless another format is specified, color fields are linear RGBA bytes. Reduce byte expressions modulo 256. For source side N use U=floor(255x/(N−1)), V=floor(255y/(N−1)) and T=17x+31y.

| Input name | Generation |
|---|---|
| `position`, `normal`, `texture`, `texture-color` | (U,V,128,255), (128,128,255,255), gray T and (T,U,255−V,255), respectively |
| `offset-gray`, `offset-alpha`, `levels-range` | Gray T, (T,U,255−V,7x+13y), and 32+(T mod 160) |
| `blend-fg-alpha`, `blend-bg-alpha`, `blend-mask` | (T,3x+7y,11x+5y,5x+11y), (29x+13y,255−U,V,13x+3y), and gray 7x+9y |
| `material-input-j` | (13jx+7y+11j,3x+19jy+53j,floor(x/5) XOR (j×(23+9×floor(y/3))),7x+11y+17j) |
| `material-id`, `curved-normal` | Eight equal vertical strips of opaque red, green, blue, white, cyan, magenta, yellow and black. For the normal use X=.55sin(2πx/N), Y=.5cos(2πy/N), Z=√(1−X²−Y²), and round 127.5×(component+1) |
| `crop-rectangle-64`, `flood-bbox-128` | Opaque white on x=12…51, y=20…43, transparent elsewhere. For the float bounding boxes set X=floor(x/32)/4 and Y=floor(y/32)/4, then store (X,Y,X+.25,Y+.25) |
| `hdr-field-128` | With u=(x+.5)/128 and v=(y+.5)/128, store RGBA32F (4u,3v−1,4+4sin(2πu)cos(2πv),((7x+11y) mod 256)/255) |
| `original-mask-16`, `original-mask-gradient-16`, `original-rgba-16` | Gray 210 for even floor(x/3)+floor(y/4), 40 otherwise. The gradient is 11x+7y. RGBA uses (17x,17y,11x+7y,A), with A=64 when x+y is divisible by three and 255 otherwise |
| `original-hald-identity-512`, `original-palette-128`, `original-id16-128` | Use the 64-level identity lookup above. The palette is sixteen vertical HSV stripes with hue j/16, saturation .7, value .9 and alpha 1, rounded to bytes. The ID field is 17×(floor(x/32)+4×floor(y/32)) |
| Irradiance and volume inputs | At 16², the position and environment use floor(255×(index+.5)/16) for XY, with a constant blue byte of 128 or 192. The normal is (128,128,255,255). Sample tables are (64,64,255,255) or (64,192,255,255). The gray half-distance is byte 128. Float distance fields are 128 and −.1, and float thin density is .001 in RGB with alpha one |
| Paths and splines | The closed seven-point path uses a circle of radius .4 about (.5,.5) in row 1 of a 16² buffer. The tagged endpoints repeat the first position. The offset spline places 16 records in row zero around (.4,.6). Sparse facet and spline controls retain the recorded original buffers described below |

For the independent 32² inputs, number the bound pins in alphabetical order starting at t=1. The table lists the actual field on each pin. For `grayt` use G=((11+2t)x+(37+4t)y+3xy+29t) mod 256. For `colort` use (G,41x−19y+11t,3x+79y+t,13x+7y+31t). A position field uses (U,V,floor((3U+V)/4),255). A normal uses X=.9(x/31−.5), Y=.7(y/31−.5), Z=√(1−X²−Y²) with the same byte encoding. The rectangle is white on x=3…23 and y=8…26, black elsewhere. When a structured original remains bound, its basename appears in the last column. The table records numerical evidence for those original bytes. Exact file fingerprints for the archived sparse and noise inputs follow the table.

| Root and output | 64² checkpoint | Bound 64² inputs and settings | 64² U8 | 64² U16 | 64² F16 | 64² F32 | 32² F32 and inputs |
|---|---|---|---:|---:|---:|---:|---|
| `3d_perlin_noise.sbs/3D_perlin_noise` · output (LUM) | checked | position=position-128<br>scale=6 | 17/4,096 · 175 | 0/4,096 · 45078 | 0/4,096 · 0.687866211 | 0/4,096 · 0.687842071 | 0/1,024 · 0.659080476<br>position=position |
| `3d_perlin_noise.sbs/3D_ridged_noise_fractal` · output (LUM) | checked | position=position-128<br>roughness=0.7 | 57/4,096 · 92 | 0/4,096 · 23627 | 3/4,096 · 0.360351562 | 0/4,096 · 0.360533118 | 0/1,024 · 0.376043171<br>position=position |
| `3d_planar_projection.sbs/3d_planar_projection` · output (LUM) | value | position=position-256, texture=texture-256, wsnormal=normal-256<br>defaults | 4,096/4,096 · 0 | 4,096/4,096 · 0 | 4,096/4,096 · 0 | 4,096/4,096 · 0 | 1,024/1,024 · 0<br>position=position, texture=gray2, wsnormal=normal |
| `3d_planar_projection.sbs/3d_planar_projection_color` · output (RGBA) | value | position=position-256, texture=texture-color-256, wsnormal=normal-256<br>defaults | 4,096/4,096 · 0 | 4,096/4,096 · 0 | 4,096/4,096 · 0 | 4,096/4,096 · 0 | 1,024/1,024 · 0<br>position=position, texture=color2, wsnormal=normal |
| `3d_texture_render.sbs/3d_texture_surface_render_irradiance` · output (RGBA) | value | see input group 1<br>i=128 | 4,096/4,096 · 0 | 4,096/4,096 · 0 | 4,096/4,096 · 0 | 4,096/4,096 · 0 | 1,024/1,024 · 0<br>same pin order |
| `3d_texture_render.sbs/3d_texture_surface_render_position` · output (RGBA) | checked | input=sdf-half-16<br>camera_uv_position=(0.35,0.6), camera_distance=3 | 4,096/4,096 · 0 | 4,096/4,096 · 0 | 3,906/4,096 · 0.00048828125 | 0/4,096 · 1.26957893e-05 | 0/1,024 · 1.26957893e-05<br>input=sdf-half-16 |
| `3d_texture_render.sbs/3d_texture_volume_render_ibl_pass` · output (RGBA) | checked | see input group 2<br>absorption=2 | 912/4,096 · 4 | 1/4,096 · 885 | 110/4,096 · 0.013671875 | 0/4,096 · 0.0135003328 | 0/1,024 · 0.0134982467<br>same pin order |
| `3d_texture_render.sbs/3d_texture_volume_render_light_pass` · output (RGBA) | checked | density=volume-density-thin, noise=irradiance-blue_noise, position=irradiance-position, sdf=sdf-negative-16<br>albedo=(0.3,0.6,0.9) | 4,096/4,096 · 0 | 4,045/4,096 · 2 | 4,069/4,096 · 6.10351562e-05 | 362/4,096 · 3.02419066e-05 | 141/1,024 · 7.41332769e-06<br>density=volume-density-thin, noise=irradiance-blue_noise, position=irradiance-position, sdf=sdf-negative-16 |
| `3d_viewer.sbs/ibl_specular_contrib_anisotropic/panoramic_lod` · Diffuse (RGBA) | value | input=offset-alpha-128<br>defaults | 4,096/4,096 · 0 | 4,096/4,096 · 0 | 4,096/4,096 · 0 | 4,096/4,096 · 0 | 1,024/1,024 · 0<br>input=color1 |
| `3d_volume_mask.sbs/3d_volume_mask` · output (LUM) | checked | position=position-128<br>shape=1, rotation=(0.1,0.2,0.3), size=(0.8,0.6,1), feather_width=0.1 | 4,096/4,096 · 0 | 4,088/4,096 · 1 | 4,083/4,096 · 0.015625 | 302/4,096 · 0.000900268555 | 123/1,024 · 1.14440918e-05<br>position=position |
| `3d_worley_noise.sbs/3D_worley_noise` · output (LUM) | checked | position=position-128<br>style=1, border_width=0.2, roundness=0.1 | 49/4,096 · 129 | 0/4,096 · 33338 | 5/4,096 · 0.508789062 | 0/4,096 · 0.508701503 | 0/1,024 · 0.523628771<br>position=position |
| `alveolus.sbs/alveolus` · Alveolus (LUM) | checked | none<br>Gradient_Filled_Cells=true, Interstice_Width=0.1 | 3,352/4,096 · 255 | 3,352/4,096 · 65535 | 3,352/4,096 · 1 | 3,352/4,096 · 1 | 1,008/1,024 · 0.0152590219 |
| `anisotropic_blur.sbs/anisotropic_blur` · Anisotropic_Blur (RGBA) | value | Source=texture-color-256<br>Quality=1, Intensity=4 | 4,096/4,096 · 0 | 4,096/4,096 · 0 | 4,096/4,096 · 0 | 4,096/4,096 · 0 | 1,024/1,024 · 0<br>Source=color1 |
| `anisotropic_blur.sbs/anisotropic_blur_grayscale` · Anisotropic_Blur (LUM) | value | Source=offset-gray-256<br>Quality=1, Intensity=4 | 4,096/4,096 · 0 | 4,096/4,096 · 0 | 4,096/4,096 · 0 | 4,096/4,096 · 0 | 1,024/1,024 · 0<br>Source=gray1 |
| `bevel_smooth.sbs/blur_gaussian_grayscale` · Blur_HQ (LUM) | value | Source=offset-gray-128<br>intensity=5 | 2,652/4,096 · 2 | 503/4,096 · 372 | 873/4,096 · 0.005859375 | 439/4,096 · 0.00568625331 | 1,024/1,024 · 0<br>Source=gray1 |
| `bevel_smooth.sbs/debug/bevel_smooth_debug_position` · output (RGBA) | value | input=position-128<br>mode=true, pixel_position=(0.25,0.75) | 3,109/4,096 · 204 | 3,109/4,096 · 52428 | 3,109/4,096 · 1.05004883 | 3,109/4,096 · 1.05 | 1,024/1,024 · 0<br>input=position |
| `bevel_smooth.sbs/jump_flood_segment_iteration` · output (RGBA) | value | input=texture-color-128<br>distance_mode=1, scale=(1,0.5) | 4,096/4,096 · 0 | 4,096/4,096 · 0 | 4,096/4,096 · 0 | 4,096/4,096 · 0 | 1,024/1,024 · 0<br>input=color1 |
| `blackbody.sbs/blackbody` · output (RGBA) | checked | none<br>temperature=12000 | 4,096/4,096 · 0 | 4,096/4,096 · 0 | 4,096/4,096 · 0 | 0/4,096 · 5.96046448e-08 | 0/1,024 · 5.96046448e-08 |
| `blend.sbs/color` · Color (RGBA) | checked | Background=blend-bg-alpha-256, Foreground=blend-fg-alpha-256, Mask=blend-mask-256<br>Opacity=0.67 | 4,094/4,096 · 1 | 4,082/4,096 · 110 | 4,094/4,096 · 0.00170898438 | 1,345/4,096 · 0.00166922808 | 1,021/1,024 · 0.00676953793<br>Background=color1, Foreground=color2, Mask=gray3 |
| `blend.sbs/color_burn` · Color_Burn (RGBA) | checked | Background=blend-bg-alpha-256, Foreground=blend-fg-alpha-256, Mask=blend-mask-256<br>Opacity=0.67 | 4,096/4,096 · 0 | 4,090/4,096 · 1 | 4,096/4,096 · 0 | 1,006/4,096 · 1.1920929e-07 | 1,020/1,024 · 0.135751605<br>Background=color1, Foreground=color2, Mask=gray3 |
| `blend.sbs/color_dodge` · Color_Dodge (RGBA) | checked | Background=blend-bg-alpha-256, Foreground=blend-fg-alpha-256, Mask=blend-mask-256<br>Opacity=0.67 | 4,096/4,096 · 0 | 4,088/4,096 · 1 | 4,096/4,096 · 0 | 2,202/4,096 · 5.96046448e-08 | 524/1,024 · 5.96046448e-08<br>Background=color1, Foreground=color2, Mask=gray3 |
| `blend.sbs/dissolve_2` · output (RGBA) | checked | background=blend-bg-alpha-256, foreground=blend-fg-alpha-256, mask=blend-mask-256<br>opacity=0.67 | 2,491/4,096 · 196 | 2,491/4,096 · 50344 | 2,491/4,096 · 0.768066406 | 2,109/4,096 · 0.768196866 | 669/1,024 · 0.960784314<br>background=color1, foreground=color2, mask=gray3 |
| `blend.sbs/luminosity` · Exclusion (RGBA) | value | Background=blend-bg-alpha-256, Foreground=blend-fg-alpha-256, Mask=blend-mask-256<br>Opacity=0.67 | 4,096/4,096 · 0 | 4,096/4,096 · 0 | 4,096/4,096 · 0 | 4,096/4,096 · 0 | 1,024/1,024 · 0<br>Background=color1, Foreground=color2, Mask=gray3 |
| `blur_hq.sbs/blur_hq` · Blur_HQ (RGBA) | value | Source=offset-alpha-128<br>Intensity=1 | 4,096/4,096 · 0 | 4,096/4,096 · 0 | 4,096/4,096 · 0 | 4,096/4,096 · 0 | 1,024/1,024 · 0<br>Source=color1 |
| `blur_hq.sbs/blur_hq_grayscale` · Blur_HQ (LUM) | value | Source=offset-gray-128<br>Intensity=1 | 4,096/4,096 · 0 | 4,096/4,096 · 0 | 4,096/4,096 · 0 | 4,096/4,096 · 0 | 1,024/1,024 · 0<br>Source=gray1 |
| `bnw_spots_1.sbs/bnw_spots_1` · BnW_Spots (LUM) | checked | none<br>Disorder=0.35 | 22/4,096 · 217 | 0/4,096 · 55788 | 1/4,096 · 0.851074219 | 0/4,096 · 0.851272523 | 1/1,024 · 0.544572145 |
| `bnw_spots_3.sbs/bnw_spots_3` · BnW_Spots_3 (LUM) | checked | none<br>Disorder=0.35 | 37/4,096 · 196 | 0/4,096 · 50175 | 1/4,096 · 0.765441895 | 0/4,096 · 0.765622027 | 0/1,024 · 0.427085683 |
| `brick_1.sbs/brick_1` · Brick (LUM) | checked | none<br>Tiling=2, Edge_Smoothness=0.2, Interstice_Width=0.2 | 2,464/4,096 · 255 | 2,464/4,096 · 65535 | 2,464/4,096 · 1 | 2,464/4,096 · 1 | 928/1,024 · 0.00692760944 |
| `brick_2.sbs/brick_2` · Brick_2 (LUM) | checked | none<br>Tiling=2, Edge_Smoothness=0.2, Interstice_Width=0.2 | 2,368/4,096 · 255 | 2,368/4,096 · 65535 | 2,368/4,096 · 1 | 2,368/4,096 · 1 | 448/1,024 · 0.225268938 |
| `brick_3.sbs/brick_3` · Bricks (LUM) | checked | none<br>Roundness=0.3, Lum_var=0.7, Disorder=0.3, Tiling=2 | 1,064/4,096 · 148 | 1,064/4,096 · 37925 | 1,064/4,096 · 0.578857422 | 1,064/4,096 · 0.578698397 | 168/1,024 · 0.58255893 |
| `cartesian_polar_transformation.sbs/cartesian_to_polar` · output (RGBA) | value | input=blend-bg-alpha-128<br>defaults | 4,096/4,096 · 0 | 4,095/4,096 · 26 | 4,095/4,096 · 0.00048828125 | 4,095/4,096 · 0.000396728516 | 1,024/1,024 · 0<br>input=color1 |
| `cartesian_polar_transformation.sbs/cartesian_to_polar_grayscale` · output (LUM) | value | input=levels-range-64<br>defaults | 4,096/4,096 · 0 | 4,095/4,096 · 17 | 4,095/4,096 · 0.000244140625 | 4,095/4,096 · 0.000259429216 | 1,024/1,024 · 0<br>input=gray1 |
| `cartesian_polar_transformation.sbs/polar_to_cartesian` · output (RGBA) | angle | input=blend-bg-alpha-128<br>defaults | 4,095/4,096 · 1 | 4,094/4,096 · 58 | 4,094/4,096 · 0.000732421875 | 4,094/4,096 · 0.000885009766 | 1,023/1,024 · 0.00361639261<br>input=color1 |
| `cartesian_polar_transformation.sbs/polar_to_cartesian_grayscale` · output (LUM) | angle | input=levels-range-64<br>defaults | 4,095/4,096 · 1 | 4,094/4,096 · 130 | 4,094/4,096 · 0.001953125 | 4,094/4,096 · 0.00198367238 | 1,023/1,024 · 0.00144961476<br>input=gray1 |
| `cells_1.sbs/cells_1` · Cells (LUM) | checked | none<br>Distance=7, Disorder=25 | 51/4,096 · 161 | 0/4,096 · 41413 | 5/4,096 · 0.631591797 | 0/4,096 · 0.631928951 | 0/1,024 · 0.696351647 |
| `channel_mixer.sbs/channel_mixer` · Output (RGBA) | checked | Input=blend-fg-alpha-128<br>red_channel=(50,25,10,5), green_channel=(-10,100,10,0), blue_channel=(20,0,50,25), monochrome=true | 0/4,096 · 228 | 0/4,096 · 58651 | 0/4,096 · 0.894958496 | 0/4,096 · 0.894956894 | 0/1,024 · 0.909117267<br>Input=color1 |
| `checker_1.sbs/checker_1` · Checker (LUM) | filtered | none<br>Tiling=3, Rotate_45_Degrees=true | 4,096/4,096 · 0 | 4,096/4,096 · 0 | 4,096/4,096 · 0 | 4,096/4,096 · 0 | separate root controls |
| `chrominance_extract.sbs/chrominance_extract` · Chrominance_Extract (RGBA) | checked | Source=blend-bg-alpha-128<br>defaults | 2,551/4,096 · 1 | 2,551/4,096 · 258 | 2,551/4,096 · 0.00439453125 | 2,551/4,096 · 0.00392174721 | 659/1,024 · 0.00392174721<br>Source=color1 |
| `clone.sbs/clone` · Clone (RGBA) | filtered | Mask=blend-mask-128, Source=blend-fg-alpha-128<br>Offset=(0.17,-0.25), Blur_Mask=0 | 4,096/4,096 · 0 | 4,096/4,096 · 0 | 4,096/4,096 · 0 | 4,096/4,096 · 0 | separate root controls |
| `clouds_1.sbs/clouds_1` · Clouds (LUM) | checked | none<br>Disorder=0.4 | 44/4,096 · 140 | 1/4,096 · 35949 | 2/4,096 · 0.548339844 | 0/4,096 · 0.54854019 | 0/1,024 · 0.583147138 |
| `clouds_2.sbs/clouds_2` · Clouds_2 (LUM) | checked | none<br>defaults | 55/4,096 · 139 | 0/4,096 · 35594 | 6/4,096 · 0.54296875 | 0/4,096 · 0.54312177 | 0/1,024 · 0.435591429 |
| `color_space_conversion.sbs/acescg_to_linear_srgb` · output (RGBA) | filtered | input=blend-bg-alpha-128<br>defaults | 4,096/4,096 · 0 | 4,096/4,096 · 0 | 4,096/4,096 · 0 | 4,096/4,096 · 0 | separate root controls |
| `color_space_conversion.sbs/srgb_to_acescg` · output (RGBA) | checked | input=blend-bg-alpha-128<br>defaults | 4,096/4,096 · 0 | 4,067/4,096 · 1 | 4,093/4,096 · 0.000122070312 | 46/4,096 · 2.38418579e-07 | 11/1,024 · 2.38418579e-07<br>input=color1 |
| `color_temperature_adjustment.sbs/color_temperature_adjustment` · output (RGBA) | value | input=blend-fg-alpha-128<br>temp=0.25, magreen=-0.2, color_space=1 | 4,095/4,096 · 1 | 4,095/4,096 · 257 | 4,095/4,096 · 0.00390625 | 4,095/4,096 · 0.00392156839 | 1,024/1,024 · 0<br>input=color1 |
| `color_to_mask.sbs/color_to_mask` · output (LUM) | checked | Input=blend-fg-alpha-128<br>color=(0.8,0.2,0.4), mask_range=0.3, mask_softness=0.1, keying_type=1, flatten_alpha=true | 4,074/4,096 · 255 | 4,074/4,096 · 65535 | 4,074/4,096 · 1 | 4,074/4,096 · 1 | 1,018/1,024 · 1<br>Input=color1 |
| `color_to_mask_v2.sbs/color_to_mask` · output (LUM) | checked | color_input=blend-bg-alpha-128, input=blend-fg-alpha-128<br>use_color_input=true, threshold=0.3, selection_fade=0.2, distance_color_space=1, distance_weights=(1,0.5,1.5) | 4,096/4,096 · 0 | 4,078/4,096 · 1 | 4,083/4,096 · 0.0009765625 | 443/4,096 · 3.33786011e-06 | 148/1,024 · 2.38418579e-06<br>color_input=color1, input=color2 |
| `composite.sbs/composite/blends/blend_content` · output (RGBA) | checked | background=blend-bg-alpha-128, foreground=blend-fg-alpha-128, mask=blend-mask-128<br>Opacity=0.37, Blending_Mode=3 | 4,096/4,096 · 0 | 4,093/4,096 · 1 | 4,096/4,096 · 0 | 2,114/4,096 · 5.96046448e-08 | 1,016/1,024 · 1<br>background=color1, foreground=color2, mask=gray3 |
| `composite.sbs/composite/blends/blend_content_pre_post_composite` · output (RGBA) | checked | background=blend-bg-alpha-128, foreground=blend-fg-alpha-128<br>Opacity=0.37, Blending_Mode=3 | 4,096/4,096 · 0 | 4,089/4,096 · 1 | 4,096/4,096 · 0 | 2,018/4,096 · 5.96046448e-08 | 543/1,024 · 1<br>background=color1, foreground=color2 |
| `composite.sbs/composite/blends/blend_matte` · output (LUM) | checked | background=offset-gray-128, foreground=levels-range-64<br>defaults | 0/4,096 · 145 | 0/4,096 · 37250 | 0/4,096 · 0.567993164 | 0/4,096 · 0.568396799 | 2/1,024 · 0.834894292<br>background=gray1, foreground=gray2 |
| `composite.sbs/composite/filters/filter_transformation` · Output (RGBA) | checked | Input=blend-fg-alpha-128<br>matrix=(1,0.25,-0.5,1), offset=(0.125,-0.25), filtering=1 | 4,096/4,096 · 0 | 4,096/4,096 · 0 | 4,096/4,096 · 0 | 4,096/4,096 · 0 | 1,020/1,024 · 1<br>Input=color1 |
| `constant_values.sbs/integer/constant_integer4` · output (int4) | gray | none<br>integer4_value=(-31,0,1,2147483647) | 0/1 · 4.2949673e+09 | 0/1 · 4.2949673e+09 | 0/1 · 4.2949673e+09 | 0/1 · 4.2949673e+09 | 0/1 · 4.2949673e+09 |
| `creased.sbs/creased` · Creased (LUM) | checked | none<br>Warp_Intensity=20, Disorder=0.3 | 43/4,096 · 146 | 0/4,096 · 37532 | 3/4,096 · 0.572509766 | 0/4,096 · 0.572693318 | 0/1,024 · 0.34687297 |
| `crop.sbs/crop` · output (RGBA) | value | input=blend-fg-alpha-128<br>show_crop_area=false, input_size=(128,128), matrix=(1.2,0.25,-0.125,0.8), offset=(0.1,-0.15), filtering=1 | 4,096/4,096 · 0 | 4,096/4,096 · 0 | 4,096/4,096 · 0 | 4,096/4,096 · 0 | 1,024/1,024 · 0<br>input=color1 |
| `crop.sbs/crop` · area (RGBA) | value | input=blend-fg-alpha-128<br>show_crop_area=false, input_size=(128,128), matrix=(1.2,0.25,-0.125,0.8), offset=(0.1,-0.15), filtering=1 | 4,096/4,096 · 0 | 4,096/4,096 · 0 | 4,096/4,096 · 0 | 4,096/4,096 · 0 | 1,024/1,024 · 0<br>input=color1 |
| `crop.sbs/crop_grayscale` · output (LUM) | value | input=offset-gray-128<br>show_crop_area=false, input_size=(128,128), matrix=(1.2,0.25,-0.125,0.8), offset=(0.1,-0.15), filtering=1 | 4,096/4,096 · 0 | 4,096/4,096 · 0 | 4,096/4,096 · 0 | 4,096/4,096 · 0 | 1,024/1,024 · 0<br>input=gray1 |
| `crop.sbs/crop_grayscale` · area (LUM) | value | input=offset-gray-128<br>show_crop_area=false, input_size=(128,128), matrix=(1.2,0.25,-0.125,0.8), offset=(0.1,-0.15), filtering=1 | 4,096/4,096 · 0 | 4,096/4,096 · 0 | 4,096/4,096 · 0 | 4,096/4,096 · 0 | 1,024/1,024 · 0<br>input=gray1 |
| `crystal_1.sbs/crystal_1` · Crystal (LUM) | checked | none<br>Warp_Intensity=10, Disorder=0.35 | 78/4,096 · 168 | 0/4,096 · 43208 | 0/4,096 · 0.658927917 | 0/4,096 · 0.659312377 | 0/1,024 · 0.766939715 |
| `crystal_2.sbs/crystal_2` · Crystal_2 (LUM) | checked | none<br>Warp_Intensity=10 | 47/4,096 · 182 | 0/4,096 · 46586 | 0/4,096 · 0.710449219 | 0/4,096 · 0.710852653 | 0/1,024 · 0.600114677 |
| `curvature.sbs/curvature` · Output (LUM) | checked | Input=curved-normal-128<br>intensity=2, invert_green=true, normal_format=1 | 81/4,096 · 22 | 1/4,096 · 5756 | 6/4,096 · 0.087890625 | 1/4,096 · 0.0878310204 | 0/1,024 · 0.713176146<br>Input=normal |
| `curvature_smooth.sbs/curvature_smooth` · height (LUM) | checked | input=curved-normal-128<br>normal_format=1 | 4,083/4,096 · 1 | 1,445/4,096 · 3 | 3,754/4,096 · 0.00048828125 | 1,445/4,096 · 4.58061695e-05 | 592/1,024 · 1.53183937e-05<br>input=normal |
| `diffusion.sbs/impl/pass_RGBA` · output (RGBA) | checked | direction=curved-normal-128, input=blend-fg-alpha-128, mask=blend-mask-128, reference=blend-bg-alpha-128<br>iter=3, nbIter=16, dither=true | 35/4,096 · 124 | 9/4,096 · 31735 | 11/4,096 · 0.483886719 | 5/4,096 · 0.484244436 | 12/1,024 · 0.474744245<br>direction=normal, input=color2, mask=gray3, reference=color4 |
| `diffusion.sbs/impl/pass_UV` · output (RGBA) | checked | input=position-128<br>iter=3, nbIter=16 | 2,554/4,096 · 192 | 1,905/4,096 · 49216 | 1,905/4,096 · 0.750976562 | 1,833/4,096 · 0.750980422 | 267/1,024 · 0.758823559<br>input=position |
| `diffusion.sbs/impl/pass_gray` · output (LUM) | checked | input=offset-gray-128, intensity=levels-range-64, reference=blend-bg-alpha-128<br>iter=3, nbIter=16, dither=true | 275/4,096 · 75 | 75/4,096 · 19121 | 91/4,096 · 0.291748047 | 64/4,096 · 0.2917656 | 7/1,024 · 0.306589752<br>input=gray1, intensity=gray2, reference=color3 |
| `diffusion.sbs/impl/pass_normal` · output (RGBA) | checked | input=curved-normal-128, reference=curved-normal-128<br>iter=3, nbIter=16, dither=true | 332/4,096 · 91 | 4/4,096 · 23349 | 20/4,096 · 0.356201172 | 0/4,096 · 0.356286973 | 16/1,024 · 0.125420004<br>input=normal, reference=normal |
| `directional_distance.sbs/directional_distance` · output (LUM) | value | angle_map=levels-range-64, distance_map=levels-range-64, input=crop-rectangle-64<br>output=1, max_distance=0.2, angle=0.125, angle_map_multiplier=0.25 | 4,096/4,096 · 0 | 4,096/4,096 · 0 | 4,096/4,096 · 0 | 4,096/4,096 · 0 | 1,024/1,024 · 0<br>angle_map=gray1, distance_map=gray2, input=rectangle |
| `directional_distance.sbs/directional_distance` · uv (RGBA) | value | angle_map=levels-range-64, distance_map=levels-range-64, input=crop-rectangle-64<br>output=1, max_distance=0.2, angle=0.125, angle_map_multiplier=0.25 | 4,096/4,096 · 0 | 4,096/4,096 · 0 | 4,096/4,096 · 0 | 4,096/4,096 · 0 | 1,024/1,024 · 0<br>angle_map=gray1, distance_map=gray2, input=rectangle |
| `directionnal_noise.sbs/directionnal_noise` · Directionnal_Noise (LUM) | checked | none<br>defaults | 6/4,096 · 221 | 0/4,096 · 56887 | 0/4,096 · 0.867797852 | 0/4,096 · 0.868038811 | 0/1,024 · 0.783939391 |
| `dirt_1.sbs/dirt_1` · Dirt (LUM) | checked | none<br>Disorder=0.4 | 35/4,096 · 237 | 6/4,096 · 60905 | 8/4,096 · 0.929046631 | 6/4,096 · 0.929355118 | 0/1,024 · 0.616165075 |
| `dirt_3.sbs/dirt_3` · Film_Dust (LUM) | checked | none<br>Disorder=0.4 | 93/4,096 · 224 | 3/4,096 · 57618 | 4/4,096 · 1.09405518 | 3/4,096 · 1.09495693 | 0/1,024 · 0.756450712 |
| `dirt_5.sbs/dirt_5` · White_Spots (LUM) | checked | none<br>defaults | 672/4,096 · 243 | 635/4,096 · 62456 | 635/4,096 · 0.952599525 | 635/4,096 · 0.953009048 | 123/1,024 · 0.669190332 |
| `dirt_6.sbs/dirt_6` · White_Stains (LUM) | checked | none<br>defaults | 118/4,096 · 255 | 90/4,096 · 65535 | 0/4,096 · 1.19921875 | 0/4,096 · 1.19999991 | 0/1,024 · 0.726540804 |
| `dirt_gradient.sbs/dirt_gradient` · Dirt_Gradient (LUM) | checked | none<br>defaults | 12/4,096 · 255 | 0/4,096 · 65425 | 0/4,096 · 0.998325348 | 0/4,096 · 0.998324568 | 0/1,024 · 0.731058508 |
| `fibers_1.sbs/fibers_1` · Rope (LUM) | checked | none<br>Tiling=2 | 16/4,096 · 188 | 0/4,096 · 48235 | 4/4,096 · 0.735839844 | 0/4,096 · 0.736018926 | 0/1,024 · 0.0354467034 |
| `flood_fill.sbs/flood_fill_to_random_color` · output (RGBA) | checked | input=flood-bbox-128<br>defaults | 0/4,096 · 251 | 0/4,096 · 64288 | 0/4,096 · 0.98085022 | 0/4,096 · 0.980972027 | 0/1,024 · 0.951476281<br>input=flood-bbox-128 |
| `flood_fill.sbs/flood_fill_to_random_grayscale` · output (LUM) | checked | input=flood-bbox-128<br>defaults | 28/4,096 · 246 | 0/4,096 · 63364 | 0/4,096 · 0.966690063 | 0/4,096 · 0.966872638 | 0/1,024 · 0.853513394<br>input=flood-bbox-128 |
| `flood_fill_2.sbs/flood_fill_to_color` · output (RGBA) | checked | color_input=blend-bg-alpha-128, input=flood-bbox-128<br>hsl_adjustement=(0.1,0.2,-0.1), hsl_random=(0.2,0.1,0.1), filtering_mode=1 | 0/4,096 · 180 | 0/4,096 · 46114 | 0/4,096 · 0.703369141 | 0/4,096 · 0.703650177 | 0/1,024 · 0.644458398<br>color_input=color1, input=flood-bbox-128 |
| `flood_fill_2.sbs/flood_fill_to_grayscale` · output (LUM) | checked | grayscale_input=levels-range-64, input=flood-bbox-128<br>luminance_adjustement=0.2, luminance_random=0.1, filtering_mode=1 | 336/4,096 · 20 | 1/4,096 · 5186 | 60/4,096 · 0.0791015625 | 0/4,096 · 0.0791372955 | 3/1,024 · 0.0657403171<br>grayscale_input=gray1, input=flood-bbox-128 |
| `flood_fill_2.sbs/flood_fill_to_random_color` · output (RGBA) | checked | input=flood-bbox-128<br>defaults | 0/4,096 · 251 | 0/4,096 · 64288 | 0/4,096 · 0.98085022 | 0/4,096 · 0.980972027 | 0/1,024 · 0.951476281<br>input=flood-bbox-128 |
| `flood_fill_2.sbs/flood_fill_to_random_grayscale` · output (LUM) | checked | input=flood-bbox-128<br>defaults | 19/4,096 · 249 | 0/4,096 · 64050 | 0/4,096 · 0.977127075 | 0/4,096 · 0.977340349 | 0/1,024 · 0.951964607<br>input=flood-bbox-128 |
| `fluid.sbs/fluid` · Fluid (LUM) | checked | none<br>Disorder=0.3, Warp_Intensity=10, Flow=0.3, Pattern_Lenght=0.75, Pattern_Width=0.2 | 18/4,096 · 210 | 0/4,096 · 53947 | 2/4,096 · 0.822875977 | 0/4,096 · 0.823176099 | 0/1,024 · 0.646288291 |
| `fractal_sum_base.sbs/fractal_sum_base` · Fractal_Sum_Base (LUM) | checked | none<br>Roughness=0.7, GlobalOpacity=0.75, MinLevel=2, MaxLevel=8 | 127/4,096 · 36 | 0/4,096 · 9013 | 8/4,096 · 0.137573242 | 0/4,096 · 0.137529567 | 0/1,024 · 0.115114063 |
| `fur_1.sbs/fur_1` · Fur (LUM) | checked | none<br>Disorder=0.4 | 18/4,096 · 255 | 9/4,096 · 65478 | 0/4,096 · 1.55157471 | 0/4,096 · 1.5521629 | 0/1,024 · 0.950347155 |
| `fur_2.sbs/fur_2` · Fur_2 (LUM) | checked | none<br>Disorder=0.3, Waves_Scale=0.5, Waves_Rotation=0.125 | 1/4,096 · 249 | 0/4,096 · 63919 | 0/4,096 · 0.975170135 | 0/4,096 · 0.97533028 | 0/1,024 · 0.509616658 |
| `fur_3.sbs/fur_3` · Fur_3 (LUM) | checked | none<br>Density=0.4, Disorder=0.2 | 19/4,096 · 249 | 0/4,096 · 63797 | 2/4,096 · 0.973403931 | 0/4,096 · 0.973479849 | 0/1,024 · 0.794445702 |
| `fur_4.sbs/fur_4` · Fur_4 (LUM) | checked | none<br>Disorder=0.3 | 42/4,096 · 255 | 30/4,096 · 65535 | 32/4,096 · 1 | 30/4,096 · 1 | 6/1,024 · 0.996810864 |
| `gaussian_1.sbs/gaussian_1` · Gaussian_Spots (LUM) | value | none<br>Tiling=2 | 4,096/4,096 · 0 | 4,064/4,096 · 1 | 4,096/4,096 · 0 | 4,064/4,096 · 1.52587891e-05 | 1,024/1,024 · 0 |
| `gaussian_2.sbs/gaussian_2` · Gaussian_Spots_2 (LUM) | value | none<br>Tiling=2 | 4,096/4,096 · 0 | 4,032/4,096 · 1 | 4,096/4,096 · 0 | 4,032/4,096 · 1.52587891e-05 | 1,024/1,024 · 0 |
| `gaussian_spots_1.sbs/gaussian_spots_1` · Few_Gauss (LUM) | checked | none<br>Disorder=0.3 | 37/4,096 · 209 | 1/4,096 · 53631 | 2/4,096 · 0.863525391 | 0/4,096 · 0.864231735 | 10/1,024 · 0.997272894 |
| `gaussian_spots_2.sbs/gaussian_spots_2` · Gauss_Spots (LUM) | checked | none<br>Disorder=0.3 | 40/4,096 · 255 | 2/4,096 · 65535 | 2/4,096 · 1.18993378 | 2/4,096 · 1.19044852 | 0/1,024 · 0.624857013 |
| `gradient_axial.sbs/gradient_axial` · output (LUM) | value | none<br>point_1=(0.15,0.3), point_2=(0.7,0.85), non_square_expansion=false | 4,096/4,096 · 0 | 4,096/4,096 · 0 | 4,096/4,096 · 0 | 4,096/4,096 · 0 | 1,024/1,024 · 0 |
| `gradient_axial.sbs/gradient_axial_reflected` · output (LUM) | value | none<br>point_1=(0.15,0.3), point_2=(0.7,0.85), non_square_expansion=false | 4,096/4,096 · 0 | 4,096/4,096 · 0 | 4,096/4,096 · 0 | 4,096/4,096 · 0 | 1,024/1,024 · 0 |
| `gradient_circular.sbs/gradient_circular` · output (LUM) | angle | none<br>point_1=(0.3,0.6), point_2=(0.8,0.7), angle=0.2, non_square_expansion=false | 4,096/4,096 · 0 | 4,096/4,096 · 0 | 4,096/4,096 · 0 | 4,096/4,096 · 0 | separate root controls |
| `gradient_linear_1.sbs/gradient_linear_1` · Simple_Gradient (LUM) | checked | none<br>Tiling=3, rotation=1 | 4,032/4,096 · 1 | 4,032/4,096 · 1 | 3,872/4,096 · 0.00048828125 | 3,936/4,096 · 1.33514404e-05 | 384/1,024 · 0.34570837 |
| `gradient_linear_2.sbs/gradient_linear_2` · Simple_Gradient_2 (LUM) | filtered | none<br>Tiling=3, rotation=1 | 4,096/4,096 · 0 | 4,096/4,096 · 0 | 4,096/4,096 · 0 | 4,096/4,096 · 0 | separate root controls |
| `gradient_linear_3.sbs/gradient_linear_3` · output (LUM) | checked | none<br>Tiling=3, position=0.3, rotation=1 | 4,096/4,096 · 0 | 3,712/4,096 · 2 | 640/4,096 · 0.0009765625 | 2,432/4,096 · 2.00867653e-05 | 864/1,024 · 5.96046448e-08 |
| `gradient_radial.sbs/gradient_radial` · output (LUM) | checked | none<br>shape=1, point_1=(0.3,0.6), point_2=(0.8,0.7), non_square_expansion=false | 4,096/4,096 · 0 | 4,090/4,096 · 1 | nonfinite cook | nonfinite cook | 507/1,024 · 7.74860382e-07² |
| `hald_clut.sbs/hald_clut` · output (RGBA) | value | input=blend-fg-alpha-128, lut=original-hald-identity-512<br>lut_intensity_by_input_alpha=false | 4,096/4,096 · 0 | 4,096/4,096 · 0 | 4,096/4,096 · 0 | 4,096/4,096 · 0 | 1,024/1,024 · 0<br>input=color1, lut=original-hald-identity-512 |
| `hbao.sbs/hbao_mip` · output (LUM) | checked | input=offset-gray-128<br>mip=3 | 3,200/4,096 · 1 | 256/4,096 · 66 | 2,560/4,096 · 0.0009765625 | 0/4,096 · 0.00100710988 | 8/1,024 · 0.00863063335<br>input=gray1 |
| `hbao_2.sbs/hbao_mip` · output (LUM) | checked | input=offset-gray-128<br>mip=3 | 3,200/4,096 · 1 | 256/4,096 · 66 | 2,560/4,096 · 0.0009765625 | 0/4,096 · 0.00100710988 | 8/1,024 · 0.00863063335<br>input=gray1 |
| `hbao_2.sbs/hbao_subsample` · output (LUM) | filtered | input=levels-range-64, mip_input=offset-gray-128<br>angle=0.125, mip=3, jiterring=0.5, offset_multiplier=(0.75,1.25) | 4,096/4,096 · 0 | 4,096/4,096 · 0 | 4,096/4,096 · 0 | 4,035/4,096 · 5.96046448e-08 | 1,004/1,024 · 5.96046448e-08<br>input=gray1, mip_input=gray2 |
| `hdr_merge.sbs/average_luminance_merge` · output (RGBA) | value | input=hdr-field-128<br>exposure_delta=2 | 4,096/4,096 · 0 | 4,096/4,096 · 0 | 4,096/4,096 · 0 | 4,096/4,096 · 0 | 1,024/1,024 · 0<br>input=hdr-field-128 |
| `hdr_range_viewer.sbs/hdr_range_viewer_grayscale` · output (RGBA) | checked | input=hdr-field-128<br>range_min=0, range_max=4 | 2,906/4,096 · 1 | 2,906/4,096 · 257 | 2,906/4,096 · 0.00439453125 | 2,906/4,096 · 0.0039215982 | 717/1,024 · 0.00392156933<br>input=hdr-field-128 |
| `height_blend.sbs/height_blend` · blended_height (LUM) | checked | height_bottom=offset-gray-128, height_top=levels-range-64, mask=blend-mask-128<br>height_position=0.35, contrast=0.6, mode=1, opacity=0.7 | 1,899/4,096 · 29 | 1,780/4,096 · 7540 | 1,795/4,096 · 0.114990234 | 1,780/4,096 · 0.115042463 | 502/1,024 · 0.103863969<br>height_bottom=gray1, height_top=gray2, mask=gray3 |
| `height_blend.sbs/height_blend` · height_mask (LUM) | checked | height_bottom=offset-gray-128, height_top=levels-range-64, mask=blend-mask-128<br>height_position=0.35, contrast=0.6, mode=1, opacity=0.7 | 1,928/4,096 · 85 | 1,928/4,096 · 22025 | 1,928/4,096 · 0.336181641 | 1,928/4,096 · 0.336087614 | 660/1,024 · 0.196429372<br>height_bottom=gray1, height_top=gray2, mask=gray3 |
| `height_to_normal_world_units.sbs/height_to_normal_world_units` · output (RGBA) | filtered | input=offset-gray-128<br>surface_size=100, height_depth=8, normal_format=0, sampling=1 | 4,096/4,096 · 0 | 4,096/4,096 · 0 | 4,096/4,096 · 0 | 4,096/4,096 · 0 | separate root controls |
| `height_to_normal_world_units.sbs/height_to_normal_world_units_2` · output (RGBA) | filtered | input=offset-gray-128<br>surface_size=100, height_depth=8, normal_format=0, sampling=1 | 4,096/4,096 · 0 | 4,096/4,096 · 0 | 4,096/4,096 · 0 | 4,096/4,096 · 0 | separate root controls |
| `highpass.sbs/highpass` · Highpass (RGBA) | checked | Source=blend-fg-alpha-128<br>Radius=2 | 4,096/4,096 · 0 | 4,081/4,096 · 1 | 4,096/4,096 · 0 | 1,002/4,096 · 5.96046448e-08 | 287/1,024 · 1.1920929e-07<br>Source=color1 |
| `highpass.sbs/highpass_grayscale` · Highpass (LUM) | value | Source=offset-gray-128<br>Radius=2 | 4,096/4,096 · 0 | 4,096/4,096 · 0 | 4,096/4,096 · 0 | 4,096/4,096 · 0 | 1,024/1,024 · 0<br>Source=gray1 |
| `histogram_scan.sbs/histogram_scan` · Output (LUM) | checked | Input_1=offset-gray-128<br>Position=0.4, Contrast=0.6, Invert_Position=true | 4,096/4,096 · 0 | 4,096/4,096 · 0 | 4,096/4,096 · 0 | 3,358/4,096 · 5.96046448e-08 | 896/1,024 · 5.96046448e-08<br>Input_1=gray1 |
| `histogram_shift.sbs/histogram_shift` · output (LUM) | checked | input=offset-gray-128<br>position=0.3 | 4,096/4,096 · 0 | 4,002/4,096 · 1 | 4,096/4,096 · 0 | 4,002/4,096 · 1.52587891e-05 | 1,004/1,024 · 1.52587891e-05<br>input=gray1 |
| `histogram_tools.sbs/histogram_compute` · cdf (LUM) | value | input=offset-gray-128<br>histogram_resolution=7 | 116/128 · 1 | 44/128 · 272 | 72/128 · 0.00415039062 | 44/128 · 0.00415039062 | 128/128 · 0<br>input=gray1 |
| `histogram_tools.sbs/histogram_compute` · histogram (LUM) | value | input=offset-gray-128<br>histogram_resolution=7 | 124/128 · 255 | 124/128 · 65535 | 24/128 · 13.25 | 24/128 · 13.25 | 128/128 · 0<br>input=gray1 |
| `make_it_tile_patch.sbs/make_it_tile_patch` · Make_It_Tile_Patch (RGBA) | checked | Source=blend-fg-alpha-128<br>Mask_Size=0.8, Mask_Precision=0.7, Mask_Warping=0.1, Pattern_size_width=120, Pattern_size_height=160, Disorder=0.25, Size_Variation=0.15, Octave=2, Rotation=0.125 | 12/4,096 · 255 | 9/4,096 · 65535 | 9/4,096 · 1 | 9/4,096 · 1 | 158/1,024 · 1<br>Source=color1 |
| `make_it_tile_patch_grayscale.sbs/make_it_tile_patch_grayscale` · Make_It_Tile_Patch (LUM) | checked | Source=offset-gray-128<br>Mask_Size=0.8, Mask_Precision=0.7, Mask_Warping=0.1, Pattern_size_width=120, Pattern_size_height=160, Disorder=0.25, Size_Variation=0.15, Octave=2, Rotation=0.125 | 34/4,096 · 216 | 9/4,096 · 55426 | 10/4,096 · 0.845703125 | 9/4,096 · 0.845746636 | 158/1,024 · 0.873212839<br>Source=gray1 |
| `material_transforms.sbs/material_transforms` · Diffuse (RGBA) | checked | Diffuse_In=material-input-1-128, Emissive_In=material-input-5-128, Height_In=material-input-3-128, Opacity_In=material-input-4-128, Specular_In=material-input-2-128<br>ScaleUV=(75,125), Rotation=0.125, Offset=(0.125,-0.25) | 4,009/4,096 · 1 | 3,928/4,096 · 253 | 3,928/4,096 · 0.00390625 | 3,928/4,096 · 0.00386054814 | 1,010/1,024 · 0.00379949808<br>Diffuse_In=color1, Emissive_In=color2, Height_In=color3, Opacity_In=color4, Specular_In=color5 |
| `material_transforms.sbs/material_transforms` · Specular (RGBA) | checked | Diffuse_In=material-input-1-128, Emissive_In=material-input-5-128, Height_In=material-input-3-128, Opacity_In=material-input-4-128, Specular_In=material-input-2-128<br>ScaleUV=(75,125), Rotation=0.125, Offset=(0.125,-0.25) | 4,056/4,096 · 1 | 3,935/4,096 · 205 | 3,939/4,096 · 0.00317382812 | 3,935/4,096 · 0.00312809646 | 1,010/1,024 · 0.00329595804<br>Diffuse_In=color1, Emissive_In=color2, Height_In=color3, Opacity_In=color4, Specular_In=color5 |
| `material_transforms.sbs/material_transforms` · Height (LUM) | checked | Diffuse_In=material-input-1-128, Emissive_In=material-input-5-128, Height_In=material-input-3-128, Opacity_In=material-input-4-128, Specular_In=material-input-2-128<br>ScaleUV=(75,125), Rotation=0.125, Offset=(0.125,-0.25) | 4,095/4,096 · 1 | 4,057/4,096 · 58 | 4,060/4,096 · 0.0009765625 | 4,057/4,096 · 0.000885039568 | 1,010/1,024 · 0.00204470754<br>Diffuse_In=color1, Emissive_In=color2, Height_In=color3, Opacity_In=color4, Specular_In=color5 |
| `material_transforms.sbs/material_transforms` · Opacity (LUM) | checked | Diffuse_In=material-input-1-128, Emissive_In=material-input-5-128, Height_In=material-input-3-128, Opacity_In=material-input-4-128, Specular_In=material-input-2-128<br>ScaleUV=(75,125), Rotation=0.125, Offset=(0.125,-0.25) | 4,093/4,096 · 1 | 4,057/4,096 · 49 | 4,066/4,096 · 0.0009765625 | 4,057/4,096 · 0.000747680664 | 1,010/1,024 · 0.00202944875<br>Diffuse_In=color1, Emissive_In=color2, Height_In=color3, Opacity_In=color4, Specular_In=color5 |
| `material_transforms.sbs/material_transforms` · Emissive (RGBA) | checked | Diffuse_In=material-input-1-128, Emissive_In=material-input-5-128, Height_In=material-input-3-128, Opacity_In=material-input-4-128, Specular_In=material-input-2-128<br>ScaleUV=(75,125), Rotation=0.125, Offset=(0.125,-0.25) | 4,081/4,096 · 1 | 4,057/4,096 · 179 | 4,061/4,096 · 0.0029296875 | 4,057/4,096 · 0.00273138285 | 1,010/1,024 · 0.00372320414<br>Diffuse_In=color1, Emissive_In=color2, Height_In=color3, Opacity_In=color4, Specular_In=color5 |
| `me_water_level.sbs/scratches_2` · Thorns (RGBA) | checked | none<br>Pattern_size_width=60, Pattern_size_height=140, Disorder=0.3 | 0/4,096 · 87 | 0/4,096 · 22444 | 0/4,096 · 0.342529297 | 0/4,096 · 0.342473418 | 0/1,024 · 0.300794378 |
| `mean_of_least_variance.sbs/mlv_color_iteration` · output (RGBA) | checked | input=blend-fg-alpha-128<br>intensity=3, smoothness=0.3, criterion=1, gaussian=true, affect_alpha=true | 4,096/4,096 · 0 | 4,093/4,096 · 1 | 4,096/4,096 · 0 | 990/4,096 · 1.1920929e-07 | 194/1,024 · 1.1920929e-07<br>input=color1 |
| `mean_of_least_variance.sbs/mlv_grayscale_iteration` · output (LUM) | checked | input=offset-gray-128<br>intensity=3, smoothness=0.3, criterion=1, gaussian=true | 4,096/4,096 · 0 | 4,096/4,096 · 0 | 4,096/4,096 · 0 | 2,627/4,096 · 5.96046448e-08 | 667/1,024 · 1.1920929e-07<br>input=gray1 |
| `median_filter.sbs/median_filter_color` · output (RGBA) | filtered | input=blend-fg-alpha-128<br>kernel_size=1, filter_type=1, affect_alpha=true | 3,517/4,096 · 194 | 3,517/4,096 · 49858 | 3,517/4,096 · 0.760437012 | 3,517/4,096 · 0.760784321 | 1,023/1,024 · 0.223529428<br>input=color1 |
| `mesh_1.sbs/mesh_1` · Mesh (LUM) | checked | none<br>Tiling=2, Rotate_45_Degrees=true | 8/4,096 · 255 | 0/4,096 · 65535 | 0/4,096 · 1 | 0/4,096 · 1 | 0/1,024 · 0.856000617 |
| `mesh_2.sbs/mesh_2` · Mesh_2 (LUM) | checked | none<br>Tiling=2, Rotation=true | 96/4,096 · 255 | 80/4,096 · 65535 | 80/4,096 · 1 | 80/4,096 · 1 | 48/1,024 · 0.984206913 |
| `mesh_adaptive_material_blender.sbs/mesh_material_blender_2_inputs` · Composited_Diffuse (RGBA) | checked | see input group 3<br>Material_Transfo_1=(0.75,0.125,0.25,0.5), Height_Range_1=(0.2,0.8), Specular_Range_1=(0.1,0.7) | 4,064/4,096 · 1 | 4,036/4,096 · 257 | 4,042/4,096 · 0.00390625 | 1,437/4,096 · 0.00392156839 | 1,019/1,024 · 0.0039061904<br>same pin order |
| `mesh_adaptive_material_blender.sbs/mesh_material_blender_2_inputs` · Composited_Height (LUM) | checked | see input group 4<br>Material_Transfo_1=(0.75,0.125,0.25,0.5), Height_Range_1=(0.2,0.8), Specular_Range_1=(0.1,0.7) | 4,092/4,096 · 1 | 4,091/4,096 · 257 | 4,091/4,096 · 0.00415039062 | 3,092/4,096 · 0.00392156839 | 1,023/1,024 · 0.00261437893<br>same pin order |
| `mesh_adaptive_material_blender.sbs/mesh_material_blender_2_inputs` · Composited_Specular (RGBA) | checked | see input group 5<br>Material_Transfo_1=(0.75,0.125,0.25,0.5), Height_Range_1=(0.2,0.8), Specular_Range_1=(0.1,0.7) | 0/4,096 · 192 | 0/4,096 · 49344 | 0/4,096 · 0.753051758 | 0/4,096 · 0.752941176 | 0/1,024 · 0.811764702<br>same pin order |
| `mesh_adaptive_material_blender.sbs/mesh_material_blender_4_inputs` · Composited_Diffuse (RGBA) | checked | see input group 6<br>Material_Transfo_1=(0.75,0.125,0.25,0.5), Height_Range_1=(0.2,0.8), Specular_Range_1=(0.1,0.7) | 3,977/4,096 · 1 | 3,853/4,096 · 257 | 3,862/4,096 · 0.00390625 | 2,767/4,096 · 0.00392156839 | 1,023/1,024 · 0.00181466341<br>same pin order |
| `mesh_adaptive_material_blender.sbs/mesh_material_blender_4_inputs` · Composited_Height (LUM) | checked | see input group 7<br>Material_Transfo_1=(0.75,0.125,0.25,0.5), Height_Range_1=(0.2,0.8), Specular_Range_1=(0.1,0.7) | 4,094/4,096 · 1 | 4,094/4,096 · 257 | 4,094/4,096 · 0.00415039062 | 4,094/4,096 · 0.00392156839 | 1,024/1,024 · 0<br>same pin order |
| `mesh_adaptive_material_blender.sbs/mesh_material_blender_4_inputs` · Composited_Specular (RGBA) | checked | see input group 8<br>Material_Transfo_1=(0.75,0.125,0.25,0.5), Height_Range_1=(0.2,0.8), Specular_Range_1=(0.1,0.7) | 0/4,096 · 190 | 0/4,096 · 48830 | 0/4,096 · 0.745117188 | 0/4,096 · 0.745098025 | 4/1,024 · 0.772549018<br>same pin order |
| `mesh_adaptive_material_blender.sbs/mesh_material_blender_8_inputs` · Composited_Diffuse (RGBA) | checked | see input group 9<br>Material_Transfo_1=(0.75,0.125,0.25,0.5), Height_Range_1=(0.2,0.8), Specular_Range_1=(0.1,0.7) | 4,032/4,096 · 1 | 4,030/4,096 · 257 | 4,030/4,096 · 0.00439453125 | 3,730/4,096 · 0.00392156839 | 1,024/1,024 · 0<br>same pin order |
| `mesh_adaptive_material_blender.sbs/mesh_material_blender_8_inputs` · Composited_Height (LUM) | checked | see input group 10<br>Material_Transfo_1=(0.75,0.125,0.25,0.5), Height_Range_1=(0.2,0.8), Specular_Range_1=(0.1,0.7) | 4,092/4,096 · 1 | 4,092/4,096 · 257 | 4,092/4,096 · 0.00415039062 | 4,092/4,096 · 0.00392156839 | 1,024/1,024 · 0<br>same pin order |
| `mesh_adaptive_material_blender.sbs/mesh_material_blender_8_inputs` · Composited_Specular (RGBA) | checked | see input group 11<br>Material_Transfo_1=(0.75,0.125,0.25,0.5), Height_Range_1=(0.2,0.8), Specular_Range_1=(0.1,0.7) | 0/4,096 · 189 | 0/4,096 · 48573 | 0/4,096 · 0.741210938 | 0/4,096 · 0.741176456 | 4/1,024 · 0.921568631<br>same pin order |
| `messy_fibers_1.sbs/messy_fibers_1` · Messy_Fibers (LUM) | checked | none<br>Rotation_Var=90, Links_Number=10, Disorder=0.3, Emboss_Intensity=0.5 | 11/4,096 · 250 | 0/4,096 · 64213 | 0/4,096 · 0.979415894 | 0/4,096 · 0.979830824 | 0/1,024 · 0.981659172 |
| `messy_fibers_2.sbs/messy_fibers_2` · Lines (LUM) | checked | none<br>Lines_Number=25, Disorder=0.3 | 14/4,096 · 255 | 6/4,096 · 65535 | 0/4,096 · 2.16217041 | 0/4,096 · 2.1632987 | 0/1,024 · 2.01119667 |
| `messy_fibers_3.sbs/messy_fibers_3` · Spike_Fibers (LUM) | checked | none<br>Rotation_Var=90, Disorder=0.3, Rotation=0.25 | 1,503/4,096 · 255 | 1,497/4,096 · 65535 | 212/4,096 · 1.16601562 | 109/4,096 · 1.1666851 | 1/1,024 · 1.16668423 |
| `messy_fibers_4.sbs/messy_fibers_4` · Sponge_Stains (LUM) | checked | none<br>defaults | 9/4,096 · 241 | 0/4,096 · 61957 | 1/4,096 · 0.945007324 | 0/4,096 · 0.945403206 | 0/1,024 · 0.921232916 |
| `microscope_view.sbs/microscope_view` · Microscope_View (LUM) | checked | none<br>Warp_intensity=6, Disorder=0.3 | 1/4,096 · 255 | 0/4,096 · 65474 | 0/4,096 · 0.999069214 | 0/4,096 · 0.9990692 | 0/1,024 · 0.842359186 |
| `min_max.sbs/min_max` · min (float) | value | Input=levels-range-64<br>defaults | 0/1 · 5.96046448e-08 | 0/1 · 5.96046448e-08 | 0/1 · 5.96046448e-08 | 0/1 · 5.96046448e-08 | 1/1 · 0<br>Input=gray1 |
| `min_max.sbs/min_max` · max (float) | value | Input=levels-range-64<br>defaults | 1/1 · 0 | 1/1 · 0 | 1/1 · 0 | 1/1 · 0 | 1/1 · 0<br>Input=gray1 |
| `moisture_noise.sbs/moisture_noise` · Moisture_Noise (LUM) | checked | none<br>Pattern_size_width=60, Pattern_size_height=140, Disorder=0.3 | 56/4,096 · 140 | 6/4,096 · 35895 | 9/4,096 · 0.547485352 | 0/4,096 · 0.547717758 | 0/1,024 · 0.405112803 |
| `mosaic.sbs/mosaic` · Mosaic (RGBA) | checked | Effect=levels-range-64, Source=blend-fg-alpha-128<br>Samples=4, Intensity=0.25 | 0/4,096 · 153 | 0/4,096 · 39321 | 0/4,096 · 0.599731445 | 0/4,096 · 0.600000009 | 0/1,024 · 0.694117658<br>Effect=gray1, Source=color2 |
| `mosaic.sbs/mosaic_grayscale` · Mosaic (LUM) | checked | Effect=levels-range-64, Source=offset-gray-128<br>Samples=4, Intensity=0.25 | 48/4,096 · 102 | 48/4,096 · 26214 | 48/4,096 · 0.399902344 | 48/4,096 · 0.400000006 | 11/1,024 · 0.505882375<br>Effect=gray1, Source=gray2 |
| `multi_directional_warp.sbs/multi_directional_warp_color` · output (RGBA) | checked | input=blend-bg-alpha-128, intensity_input=offset-gray-128<br>intensity=4, warp_angle=0.125, mode=1, directions=2 | 152/4,096 · 112 | 152/4,096 · 28784 | 152/4,096 · 0.438964844 | 152/4,096 · 0.43921569 | 56/1,024 · 0.200000018<br>input=color1, intensity_input=gray2 |
| `multi_directional_warp.sbs/multi_directional_warp_grayscale` · output (LUM) | checked | input=offset-gray-128, intensity_input=levels-range-64<br>intensity=4, warp_angle=0.125, mode=1, directions=2 | 2,167/4,096 · 46 | 2,167/4,096 · 11822 | 2,167/4,096 · 0.180664062 | 2,167/4,096 · 0.180392146 | 611/1,024 · 0.105882347<br>input=gray1, intensity_input=gray2 |
| `multiangle_to_albedo.sbs/multiangle_to_albedo` · albedo (RGBA) | filtered | input_1=blend-fg-alpha-128, input_2=blend-bg-alpha-128, input_3=blend-bg-alpha-128, input_4=blend-bg-alpha-128, input_5=blend-bg-alpha-128, input_6=blend-bg-alpha-128, input_7=blend-bg-alpha-128, input_8=blend-bg-alpha-128<br>samples_amount=4 | 4,096/4,096 · 0 | 4,096/4,096 · 0 | 4,096/4,096 · 0 | 4,096/4,096 · 0 | separate root controls |
| `noise_anisotropic_noise.sbs/anisotropic_noise` · Anisotropic_Noise (LUM) | checked | none<br>X_Amount=8, Y_Amount=64, rotate=true, smoothness=0.5, smoothness_interpolation=0.25 | 3,404/4,096 · 255 | 1,349/4,096 · 65471 | 1,681/4,096 · 0.999023438 | 1,349/4,096 · 0.999023438 | 438/1,024 · 0.0019531846 |
| `noise_anisotropic_noise_v2.sbs/anisotropic_noise_v2` · output (LUM) | value | none<br>X_Amount=8, Y_Amount=64, rotate=true, smoothness=0.5, smoothness_interpolation=0.25, disorder=0.3, disorder_speed=0.5 | 4,096/4,096 · 0 | 4,096/4,096 · 0 | 4,096/4,096 · 0 | 4,096/4,096 · 0 | 1,024/1,024 · 0 |
| `noise_bnw_spots_1.sbs/bnw_spots_1` · output (LUM) | checked | none<br>scale=2, disorder=0.37, roughness=0.6 | execution budget | execution budget | execution budget | execution budget | execution budget |
| `noise_bnw_spots_1_v2.sbs/bnw_spots_1` · output (LUM) | checked | none<br>scale=2, disorder=0.37, disorder_speed=0.6, disorder_anisotropy=0.3, disorder_anisotropy_angle=0.125, roughness=0.6, tile_offset=(0.17,0.29) | execution budget | execution budget | execution budget | execution budget | execution budget |
| `noise_bnw_spots_3.sbs/bnw_spots_3` · output (LUM) | checked | none<br>scale=2, disorder=0.37 | execution budget | execution budget | execution budget | execution budget | execution budget |
| `noise_bnw_spots_3_v2.sbs/bnw_spots_3` · output (LUM) | checked | none<br>scale=2, disorder=0.37, disorder_speed=0.6, disorder_anisotropy=0.3, disorder_anisotropy_angle=0.125, tile_offset=(0.17,0.29) | execution budget | execution budget | execution budget | execution budget | execution budget |
| `noise_cells_1.sbs/cells_1` · output (LUM) | checked | none<br>scale=2, disorder=0.37 | 64/4,096 · 85 | 0/4,096 · 21913 | 8/4,096 · 0.334472656 | 0/4,096 · 0.334366918 | 0/1,024 · 0.312387526 |
| `noise_cells_1_v2.sbs/cells_1` · output (LUM) | checked | none<br>scale=2, disorder=0.37, disorder_speed=0.6, disorder_anisotropy=0.3, disorder_anisotropy_angle=0.125, angle=0.125, angle_random=0.2, pattern=3, pattern_size=(0.8,1.2), pattern_scale=0.8, luminance_random=0.25, tile_offset=(0.17,0.29) | 0/4,096 · 59 | 0/4,096 · 14988 | 0/4,096 · 0.228515625 | 0/4,096 · 0.228701174 | 0/1,024 · 0.0729952455 |
| `noise_cells_4.sbs/cells_4` · output (LUM) | checked | input=offset-gray-128<br>scale=2, disorder=0.37, color_source=1, pseudo_random_seed=11 | 687/4,096 · 166 | 687/4,096 · 42559 | 687/4,096 · 0.648925781 | 687/4,096 · 0.649408698 | 90/1,024 · 0.823483601<br>input=gray1 |
| `noise_cells_4_v2.sbs/cells_4` · output (LUM) | checked | input=offset-gray-128<br>scale=2, disorder=0.37, disorder_speed=0.6, color_source=1, pseudo_random_seed=11 | 1,400/4,096 · 111 | 1,400/4,096 · 28400 | 1,400/4,096 · 0.433105469 | 1,400/4,096 · 0.433356225 | 353/1,024 · 0.433356225<br>input=gray1 |
| `noise_clouds_1.sbs/clouds_1` · output (LUM) | checked | none<br>scale=2, disorder=0.37 | 65/4,096 · 129 | 0/4,096 · 33005 | 9/4,096 · 0.503417969 | 0/4,096 · 0.503617346 | 0/1,024 · 0.366951793 |
| `noise_clouds_1_v2.sbs/clouds_1` · output (LUM) | checked | none<br>scale=2, disorder=0.37, disorder_speed=0.6, disorder_anisotropy=0.3, disorder_anisotropy_angle=0.125, tile_offset=(0.17,0.29) | 59/4,096 · 100 | 0/4,096 · 25779 | 8/4,096 · 0.393066406 | 0/4,096 · 0.393369287 | 0/1,024 · 0.363682389 |
| `noise_clouds_2.sbs/clouds_2` · output (LUM) | checked | none<br>scale=2, disorder=0.37 | 64/4,096 · 92 | 1/4,096 · 23488 | 6/4,096 · 0.358398438 | 0/4,096 · 0.358411312 | 0/1,024 · 0.271892846 |
| `noise_clouds_2_v2.sbs/clouds_2` · output (LUM) | checked | none<br>scale=2, disorder=0.37, disorder_speed=0.6, disorder_anisotropy=0.3, disorder_anisotropy_angle=0.125, tile_offset=(0.17,0.29) | 83/4,096 · 71 | 1/4,096 · 18337 | 10/4,096 · 0.279785156 | 0/4,096 · 0.279799819 | 0/1,024 · 0.27019763 |
| `noise_creased.sbs/creased` · output (LUM) | checked | none<br>scale=2, disorder=0.37, warp_intensity=0.4 | 82/4,096 · 104 | 2/4,096 · 26838 | 4/4,096 · 0.409423828 | 2/4,096 · 0.40952161 | 0/1,024 · 0.366201252 |
| `noise_crystal_1.sbs/crystal_1` · output (LUM) | checked | none<br>scale=2, disorder=0.37 | 71/4,096 · 102 | 0/4,096 · 26232 | 1/4,096 · 0.400058746 | 0/4,096 · 0.400280991 | 0/1,024 · 0.333630634 |
| `noise_crystal_2.sbs/crystal_2` · output (LUM) | checked | none<br>scale=2, disorder=0.37 | 48/4,096 · 189 | 0/4,096 · 48510 | 6/4,096 · 0.739868164 | 0/4,096 · 0.740217775 | 0/1,024 · 0.561198354 |
| `noise_directional_noise_1.sbs/directional_noise_1` · output (LUM) | checked | none<br>scale=2, disorder=0.37, angle=0.125, angle_random=0.2 | 1/4,096 · 250 | 0/4,096 · 64279 | 0/4,096 · 1.3684082 | 0/4,096 · 1.36927214 | 0/1,024 · 1.44740984 |
| `noise_directional_noise_1_v2.sbs/directional_noise_1` · output (LUM) | checked | none<br>scale=2, disorder=0.37, disorder_speed=0.6, disorder_anisotropy=0.3, disorder_anisotropy_angle=0.125, angle=0.125, angle_random=0.2, tile_offset=(0.17,0.29) | 0/4,096 · 213 | 0/4,096 · 54794 | 0/4,096 · 1.91894531 | 0/4,096 · 1.9202345 | 0/1,024 · 1.42954385 |
| `noise_directional_noise_2.sbs/directional_noise_2` · output (LUM) | checked | none<br>scale=2, disorder=0.37, angle=0.125, angle_random=0.2 | execution budget | execution budget | execution budget | execution budget | execution budget |
| `noise_directional_noise_2_v2.sbs/directional_noise_2` · output (LUM) | checked | none<br>scale=2, disorder=0.37, disorder_speed=0.6, disorder_anisotropy=0.3, disorder_anisotropy_angle=0.125, angle=0.125, angle_random=0.2, tile_offset=(0.17,0.29) | execution budget | execution budget | execution budget | execution budget | execution budget |
| `noise_directional_noise_3.sbs/directional_noise_3` · output (LUM) | checked | none<br>scale=2, disorder=0.37, angle=0.125, angle_random=0.2 | execution budget | execution budget | execution budget | execution budget | execution budget |
| `noise_directional_noise_3_v2.sbs/directional_noise_3` · output (LUM) | checked | none<br>scale=2, disorder=0.37, disorder_speed=0.6, disorder_anisotropy=0.3, disorder_anisotropy_angle=0.125, angle=0.125, angle_random=0.2, tile_offset=(0.17,0.29) | execution budget | execution budget | execution budget | execution budget | execution budget |
| `noise_directional_noise_4.sbs/directional_noise_4` · output (LUM) | checked | none<br>scale=2, disorder=0.37, angle=0.125, angle_random=0.2 | 33/4,096 · 171 | 0/4,096 · 44073 | 4/4,096 · 0.672271729 | 0/4,096 · 0.67251296 | 0/1,024 · 0.624325883 |
| `noise_directional_noise_4_v2.sbs/directional_noise_4` · output (LUM) | checked | none<br>scale=2, disorder=0.37, disorder_speed=0.6, disorder_anisotropy=0.3, disorder_anisotropy_angle=0.125, angle=0.125, angle_random=0.2, tile_offset=(0.17,0.29) | 44/4,096 · 168 | 0/4,096 · 43073 | 5/4,096 · 0.68711853 | 0/4,096 · 0.687143391 | 0/1,024 · 0.599172756 |
| `noise_directional_scratches.sbs/directional_scratches` · output (LUM) | checked | none<br>scale=2, disorder=0.37, angle=0.125, angle_random=0.2, pattern_size=(0.8,1.2), pattern_amount=0.6, pattern_size_random=(0.1,0.2) | 1,079/4,096 · 250 | 1,065/4,096 · 64250 | 1,065/4,096 · 0.979980469 | 1,065/4,096 · 0.980386198 | 483/1,024 · 0.851179719 |
| `noise_directional_scratches_v2.sbs/directional_scratches` · output (LUM) | checked | none<br>scale=2, disorder=0.37, disorder_speed=0.6, disorder_anisotropy=0.3, disorder_anisotropy_angle=0.125, angle=0.125, angle_random=0.2, pattern_size=(0.8,1.2), tile_offset=(0.17,0.29), pattern_amount=0.6, pattern_size_random=(0.1,0.2) | 1,558/4,096 · 206 | 1,430/4,096 · 52891 | 1,430/4,096 · 0.806640625 | 1,430/4,096 · 0.807068884 | 253/1,024 · 0.825664742 |
| `noise_dirt_1.sbs/dirt_1` · output (LUM) | checked | none<br>scale=2, disorder=0.37 | execution budget | execution budget | execution budget | execution budget | execution budget |
| `noise_dirt_1_v2.sbs/dirt_1` · output (LUM) | checked | none<br>scale=2, disorder=0.37, disorder_speed=0.6, disorder_anisotropy=0.3, disorder_anisotropy_angle=0.125, tile_offset=(0.17,0.29) | execution budget | execution budget | execution budget | execution budget | execution budget |
| `noise_dirt_2.sbs/dirt_2` · output (LUM) | checked | none<br>scale=2, disorder=0.37 | execution budget | execution budget | execution budget | execution budget | execution budget |
| `noise_dirt_3.sbs/dirt_3` · output (LUM) | checked | none<br>scale=2, disorder=0.37 | 123/4,096 · 177 | 83/4,096 · 45560 | 83/4,096 · 0.694824219 | 83/4,096 · 0.695203125 | 0/1,024 · 0.458531976 |
| `noise_dirt_3_v2.sbs/dirt_3` · output (LUM) | checked | none<br>scale=2, disorder=0.37, disorder_speed=0.6, disorder_anisotropy=0.3, disorder_anisotropy_angle=0.125, tile_offset=(0.17,0.29) | 47/4,096 · 126 | 1/4,096 · 32344 | 3/4,096 · 0.493225098 | 0/4,096 · 0.493531004 | 0/1,024 · 0.518233344 |
| `noise_dirt_4.sbs/dirt_4` · output (LUM) | angle | none<br>scale=2, disorder=0.37 | execution budget | execution budget | execution budget | execution budget | execution budget |
| `noise_dirt_4_v2.sbs/dirt_4` · output (LUM) | angle | none<br>scale=2, disorder=0.37, disorder_speed=0.6, disorder_anisotropy=0.3, disorder_anisotropy_angle=0.125, tile_offset=(0.17,0.29) | execution budget | execution budget | execution budget | execution budget | execution budget |
| `noise_dirt_5.sbs/dirt_5` · output (LUM) | checked | none<br>scale=2, disorder=0.37 | execution budget | execution budget | execution budget | execution budget | execution budget |
| `noise_dirt_5_v2.sbs/dirt_5` · output (LUM) | checked | none<br>scale=2, disorder=0.37, disorder_speed=0.6, disorder_anisotropy=0.3, disorder_anisotropy_angle=0.125, tile_offset=(0.17,0.29) | execution budget | execution budget | execution budget | execution budget | execution budget |
| `noise_dirt_gradient.sbs/dirt_gradient` · output (LUM) | checked | none<br>disorder=0.37 | 2/4,096 · 239 | 0/4,096 · 61336 | 0/4,096 · 1.77966309 | 0/4,096 · 1.78030248 | 0/1,024 · 0.360767424 |
| `noise_dirt_gradient_v2.sbs/dirt_gradient` · output (LUM) | checked | none<br>disorder=0.37, disorder_speed=0.6, disorder_anisotropy=0.3, disorder_anisotropy_angle=0.125 | execution budget | execution budget | execution budget | execution budget | execution budget |
| `noise_fluid.sbs/fluid` · output (LUM) | checked | none<br>scale=2, disorder=0.37, pattern_size=(0.8,1.2), warp_intensity=0.4 | 68/4,096 · 126 | 0/4,096 · 32244 | 4/4,096 · 0.491943359 | 0/4,096 · 0.492016941 | 0/1,024 · 0.347934932 |
| `noise_fractal_sum_base.sbs/fractal_sum_base` · output (LUM) | checked | none<br>Roughness=0.6, GlobalOpacity=0.8, MinLevel=2, MaxLevel=5 | 53/4,096 · 105 | 0/4,096 · 27008 | 5/4,096 · 0.412109375 | 0/4,096 · 0.412115663 | 0/1,024 · 0.319676518 |
| `noise_fractal_sum_base.sbs/fractal_sum_base_2` · output (LUM) | checked | none<br>Roughness=0.6, GlobalOpacity=0.8, MinLevel=2, MaxLevel=5 | 65/4,096 · 97 | 0/4,096 · 24926 | 5/4,096 · 0.380249023 | 0/4,096 · 0.380346403 | 0/1,024 · 0.295078933 |
| `noise_fractal_sum_base_v2.sbs/noise_blend` · output (LUM) | checked | input=offset-gray-128, previous=levels-range-64<br>roughness=0.6, global_opacity=0.8, min_level=1, max_level=3, level=2, contrast=0.8 | 4/4,096 · 80 | 4/4,096 · 20560 | 4/4,096 · 0.313476562 | 4/4,096 · 0.313725471 | 82/1,024 · 0.392156869<br>input=gray1, previous=gray2 |
| `noise_fractal_sum_base_v2.sbs/noise_upscale` · output (LUM) | checked | input=levels-range-64<br>defaults | 64/4,096 · 15 | 0/4,096 · 3791 | 0/4,096 · 0.228057861 | 0/4,096 · 0.228064887 | 0/1,024 · 0.228064887<br>input=gray1 |
| `noise_fur_1.sbs/fur_1` · output (LUM) | checked | none<br>scale=2, disorder=0.37 | 8/4,096 · 255 | 7/4,096 · 65535 | 0/4,096 · 2.29673576 | 0/4,096 · 2.29806608 | 0/1,024 · 1.16170315 |
| `noise_fur_2.sbs/fur_2` · output (LUM) | checked | none<br>scale=2, waves_scale=0.25, waves_rotation=0.2 | 14/4,096 · 191 | 0/4,096 · 49242 | 2/4,096 · 0.751342773 | 0/4,096 · 0.751376905 | 0/1,024 · 0.445814531 |
| `noise_fur_3.sbs/fur_3` · output (LUM) | checked | none<br>scale=2, disorder=0.37, waves_amount=3 | 11/4,096 · 191 | 0/4,096 · 48925 | 0/4,096 · 0.74621582 | 0/4,096 · 0.746538982 | 0/1,024 · 0.513960555 |
| `noise_gaussian_noise.sbs/gaussian_noise` · output (LUM) | checked | none<br>scale=2, disorder=0.37 | 0/4,096 · 159 | 0/4,096 · 40736 | 0/4,096 · 0.621582031 | 0/4,096 · 0.621597558 | 0/1,024 · 0.218729451 |
| `noise_gaussian_noise_v2.sbs/gaussian_noise` · output (LUM) | checked | none<br>scale=2, disorder=0.37, disorder_speed=0.6, disorder_anisotropy=0.3, disorder_anisotropy_angle=0.125, tile_offset=(0.17,0.29) | 0/4,096 · 195 | 0/4,096 · 50135 | 0/4,096 · 0.897216797 | 0/4,096 · 0.897545502 | 0/1,024 · 0.339412361 |
| `noise_gaussian_spots_1.sbs/gaussian_spots_1` · output (LUM) | checked | none<br>scale=2, disorder=0.37 | 28/4,096 · 177 | 0/4,096 · 45368 | 0/4,096 · 0.691833496 | 0/4,096 · 0.69227396 | 2/1,024 · 1.04430412 |
| `noise_gaussian_spots_1_v2.sbs/gaussian_spots_1` · output (LUM) | checked | none<br>scale=2, disorder=0.37, disorder_speed=0.6, disorder_anisotropy=0.3, disorder_anisotropy_angle=0.125, tile_offset=(0.17,0.29) | 23/4,096 · 215 | 0/4,096 · 55101 | 0/4,096 · 0.840454102 | 0/4,096 · 0.84077388 | 0/1,024 · 1.1514894 |
| `noise_gaussian_spots_2.sbs/gaussian_spots_2` · output (LUM) | checked | none<br>scale=2, disorder=0.37 | 37/4,096 · 149 | 0/4,096 · 38197 | 0/4,096 · 0.582519531 | 0/4,096 · 0.582842484 | 0/1,024 · 0.877645788 |
| `noise_gaussian_spots_2_v2.sbs/gaussian_spots_2` · output (LUM) | checked | none<br>scale=2, disorder=0.37, disorder_speed=0.6, disorder_anisotropy=0.3, disorder_anisotropy_angle=0.125, tile_offset=(0.17,0.29) | 20/4,096 · 253 | 0/4,096 · 65028 | 1/4,096 · 1.05671692 | 0/4,096 · 1.05743184 | 0/1,024 · 0.613122775 |
| `noise_liquid.sbs/liquid` · output (LUM) | checked | none<br>scale=2, disorder=0.37, warp_intensity=0.4 | 0/4,096 · 154 | 0/4,096 · 39615 | 0/4,096 · 0.604248047 | 0/4,096 · 0.604488969 | 0/1,024 · 0.2390953 |
| `noise_messy_fibers_1.sbs/messy_fibers_1` · output (LUM) | checked | none<br>scale=2, disorder=0.37, angle=0.125, angle_random=0.2, lines_number=8 | 7/4,096 · 252 | 0/4,096 · 64724 | 1/4,096 · 0.987136841 | 0/4,096 · 0.987619292 | 0/1,024 · 0.988889564 |
| `noise_messy_fibers_1_v2.sbs/messy_fibers_1` · output (LUM) | checked | none<br>scale=2, disorder=0.37, disorder_speed=0.6, disorder_anisotropy=0.3, disorder_anisotropy_angle=0.125, angle=0.125, angle_random=0.2, tile_offset=(0.17,0.29), lines_number=8 | 7/4,096 · 232 | 0/4,096 · 59482 | 0/4,096 · 0.907287598 | 0/4,096 · 0.907628112 | 0/1,024 · 0.806026489 |
| `noise_messy_fibers_2.sbs/messy_fibers_2` · output (LUM) | checked | none<br>scale=2, disorder=0.37, angle=0.125, angle_random=0.2, links_number=8 | 20/4,096 · 255 | 10/4,096 · 65535 | 2/4,096 · 2.58886719 | 0/4,096 · 2.58883643 | 0/1,024 · 2.50310458 |
| `noise_messy_fibers_2_v2.sbs/messy_fibers_2` · output (LUM) | checked | none<br>scale=2, disorder=0.37, disorder_speed=0.6, disorder_anisotropy=0.3, disorder_anisotropy_angle=0.125, angle=0.125, angle_random=0.2, tile_offset=(0.17,0.29), lines_number=8 | 40/4,096 · 255 | 33/4,096 · 65535 | 0/4,096 · 2.7121582 | 0/4,096 · 2.71319878 | 0/1,024 · 2.26003554 |
| `noise_messy_fibers_3.sbs/messy_fibers_3` · output (LUM) | checked | none<br>scale=2, disorder=0.37, angle=0.125, angle_random=0.2, color_random=0.25 | 29/4,096 · 255 | 21/4,096 · 65535 | 21/4,096 · 1.1640625 | 21/4,096 · 1.1643126 | 2/1,024 · 1.16184852 |
| `noise_messy_fibers_3_v2.sbs/messy_fibers_3` · output (LUM) | checked | none<br>scale=2, disorder=0.37, disorder_speed=0.6, disorder_anisotropy=0.3, disorder_anisotropy_angle=0.125, angle=0.125, angle_random=0.2, luminance_random=0.25, tile_offset=(0.17,0.29) | 0/4,096 · 255 | 0/4,096 · 65535 | 0/4,096 · 1.16308594 | 0/4,096 · 1.1639756 | 0/1,024 · 1.11929643 |
| `noise_microscope_view.sbs/microscope_view` · output (LUM) | checked | none<br>scale=2, disorder=0.37, warp_intensity=0.4 | 0/4,096 · 197 | 0/4,096 · 50531 | 0/4,096 · 0.770599365 | 0/4,096 · 0.771053366 | 0/1,024 · 0.152535662 |
| `noise_moisture_noise.sbs/moisture_noise` · output (LUM) | checked | none<br>scale=2, disorder=0.37, pattern_size=(0.8,1.2), pattern_angle=0.125, pattern_angle_random=0.2, global_opacity=0.8 | execution budget | execution budget | execution budget | execution budget | execution budget |
| `noise_moisture_noise_1_v2.sbs/moisture_noise_1` · output (LUM) | checked | none<br>scale=2, disorder=0.37, disorder_speed=0.6, disorder_anisotropy=0.3, disorder_anisotropy_angle=0.125, pattern_size=(0.8,1.2), tile_offset=(0.17,0.29), pattern_angle=0.125, pattern_angle_random=0.2, global_opacity=0.8 | execution budget | execution budget | execution budget | execution budget | execution budget |
| `noise_moisture_noise_2.sbs/moisture_noise_2` · output (LUM) | checked | none<br>scale=2, disorder=0.37, disorder_speed=0.6, disorder_anisotropy=0.3, disorder_anisotropy_angle=0.125, pattern_size=(0.8,1.2), tile_offset=(0.17,0.29), pattern_angle=0.125, pattern_angle_random=0.2, global_opacity=0.8 | execution budget | execution budget | execution budget | execution budget | execution budget |
| `noise_perlin_noise.sbs/perlin_noise` · output (LUM) | checked | none<br>scale=2, disorder=0.37 | 176/4,096 · 15 | 2/4,096 · 3818 | 11/4,096 · 0.0581054688 | 1/4,096 · 0.0582590103 | 0/1,024 · 0.266117394 |
| `noise_perlin_noise_v2.sbs/perlin_noise` · output (LUM) | checked | none<br>scale=2, disorder=0.37, disorder_speed=0.6, tile_offset=(0.17,0.29) | 11/4,096 · 76 | 0/4,096 · 19520 | 0/4,096 · 0.297607422 | 0/4,096 · 0.29785502 | 0/1,024 · 0.236935616 |
| `noise_plasma.sbs/plasma` · output (LUM) | checked | none<br>scale=2, disorder=0.37 | 164/4,096 · 44 | 0/4,096 · 11163 | 0/4,096 · 0.170257568 | 0/4,096 · 0.170336485 | 0/1,024 · 0.22627604 |
| `noise_upscale_1.sbs/noise_upscale_1` · Noise_ResolutionX2 (LUM) | checked | Source=offset-gray-128<br>Offset1X=0.17, Offset1Y=0.23, Offset2X=-0.125, Offset2Y=0.375 | 12,334/16,384 · 20 | 10,336/16,384 · 5184 | 10,658/16,384 · 0.0791015625 | 10,145/16,384 · 0.0791027546 | 2,759/4,096 · 0.0647692829<br>Source=gray1 |
| `noise_upscale_2.sbs/noise_upscale_2` · Noise_ResolutionX2_2 (LUM) | checked | Source=offset-gray-128<br>Offset1X=0.17, Offset1Y=0.23, Offset2X=-0.125, Offset2Y=0.375 | 13,079/16,384 · 12 | 12,706/16,384 · 2879 | 12,715/16,384 · 0.0439453125 | 10,841/16,384 · 0.04392156 | 2,468/4,096 · 0.0373333395<br>Source=gray1 |
| `noise_upscale_3.sbs/noise_upscale_3` · Noise_ResolutionX2_3 (LUM) | checked | Mask=blend-mask-128, Source=levels-range-64<br>defaults | 16,384/16,384 · 0 | 16,375/16,384 · 1 | 15,176/16,384 · 0.00048828125 | 12,398/16,384 · 5.96046448e-08 | 156/4,096 · 0.00392246246<br>Mask=gray1, Source=gray2 |
| `noise_voronoi.sbs/3d_voronoi` · output (LUM) | checked | position=blend-fg-alpha-128<br>scale=2, disorder=0.37 | 4,096/4,096 · 0 | 4,095/4,096 · 1 | 4,096/4,096 · 0 | 3,462/4,096 · 5.96046448e-08 | 850/1,024 · 5.96046448e-08<br>position=color1 |
| `noise_voronoi.sbs/3d_voronoi_fractal` · output (LUM) | checked | position=blend-fg-alpha-128<br>scale=2, disorder=0.37, roughness=0.6, global_opacity=0.8, min_level=1, max_level=3 | 4,096/4,096 · 0 | 4,072/4,096 · 1 | 4,096/4,096 · 0 | 14/4,096 · 1.78813934e-07 | 4/1,024 · 1.78813934e-07<br>position=color1 |
| `noise_voronoi.sbs/voronoi` · output (LUM) | checked | none<br>scale=2, disorder=(0.37,0.259,0.111) | 4,096/4,096 · 0 | 4,093/4,096 · 1 | 4,096/4,096 · 0 | 3,584/4,096 · 5.96046448e-08 | 887/1,024 · 5.96046448e-08 |
| `noise_voronoi.sbs/voronoi_fractal` · output (LUM) | checked | none<br>scale=2, disorder=(0.37,0.259,0.111), roughness=0.6, global_opacity=0.8, min_level=1, max_level=3 | 4,096/4,096 · 0 | 4,067/4,096 · 1 | 4,092/4,096 · 0.00048828125 | 11/4,096 · 2.38418579e-07 | 4/1,024 · 1.78813934e-07 |
| `non_square_transform.sbs/non_square_transform` · output (RGBA) | gray | input=blend-fg-alpha-128<br>tile_mode=1, tile=(2,3), offset=(0.125,-0.25), rotation=0.125, tile_safe_rotation=false | 4,038/4,096 · 1 | 3,960/4,096 · 235 | 3,960/4,096 · 0.00360107422 | 3,960/4,096 · 0.00358587503 | 1,004/1,024 · 0.00372320414<br>input=color1 |
| `non_square_transform.sbs/non_square_transform_greyscale` · output (LUM) | gray | input=offset-gray-128<br>tile_mode=1, tile=(2,3), offset=(0.125,-0.25), rotation=0.125, tile_safe_rotation=false | 4,063/4,096 · 1 | 3,960/4,096 · 195 | 3,964/4,096 · 0.0029296875 | 3,960/4,096 · 0.00297552347 | 1,004/1,024 · 0.00329594314<br>input=gray1 |
| `non_uniform_blur.sbs/non_uniform_blur_grayscale_sample` · Output (LUM) | angle | Effect=levels-range-64, Source=offset-gray-128<br>Intensity=0.7, Samples=8, Angle=0.125, Asymmetry=0.25, Anisotropy=0.5, InnerRotation=0.2 | 2,353/4,096 · 3 | 2,353/4,096 · 771 | 2,353/4,096 · 0.01171875 | 2,353/4,096 · 0.0117647052 | 669/1,024 · 0.00784315169<br>Effect=gray1, Source=gray2 |
| `non_uniform_blur.sbs/non_uniform_blur_sample` · Output (RGBA) | angle | Effect=levels-range-64, Source=blend-fg-alpha-128<br>Intensity=0.7, Samples=8, Angle=0.125, Asymmetry=0.25, Anisotropy=0.5, InnerRotation=0.2 | 1,024/4,096 · 3 | 1,024/4,096 · 771 | 1,024/4,096 · 0.0119628906 | 1,024/4,096 · 0.011764735 | 247/1,024 · 0.00784315169<br>Effect=gray1, Source=color2 |
| `non_uniform_rotation.sbs/non_uniform_rotation_color` · output (RGBA) | value | input=blend-bg-alpha-128, rotation_map=offset-gray-128, rotation_pivot_map=blend-bg-alpha-128<br>rotation_angle_multiplier=0.5, rotation_angle_offset=0.125, pivot_position=(0.3,0.7) | 4,095/4,096 · 1 | 4,092/4,096 · 103 | 4,092/4,096 · 0.00170898438 | 4,092/4,096 · 0.00157165527 | 1,024/1,024 · 0<br>input=color1, rotation_map=gray2, rotation_pivot_map=color3 |
| `non_uniform_rotation.sbs/non_uniform_rotation_grayscale` · output (LUM) | value | input=offset-gray-128, rotation_map=levels-range-64, rotation_pivot_map=blend-bg-alpha-128<br>rotation_angle_multiplier=0.5, rotation_angle_offset=0.125, pivot_position=(0.3,0.7) | 4,096/4,096 · 0 | 4,092/4,096 · 130 | 4,093/4,096 · 0.00244140625 | 4,092/4,096 · 0.00198370218 | 1,024/1,024 · 0<br>input=gray1, rotation_map=gray2, rotation_pivot_map=color3 |
| `normal_normalise.sbs/normal_normalise` · Normalise (RGBA) | checked | Normal=curved-normal-128<br>defaults | 4,032/4,096 · 1 | 4,024/4,096 · 193 | 4,032/4,096 · 0.00305175781 | 3,216/4,096 · 0.00293892622 | 862/1,024 · 1.1920929e-07<br>Normal=normal |
| `normal_sobel.sbs/normal_sobel` · output (RGBA) | checked | input=offset-gray-128<br>intensity=0.6, normal_format=1 | 4,096/4,096 · 0 | 4,096/4,096 · 0 | 4,096/4,096 · 0 | 2,975/4,096 · 1.1920929e-07 | 1,015/1,024 · 1.1920929e-07<br>input=gray1 |
| `normal_sobel.sbs/normal_sobel_2` · output (RGBA) | checked | input=offset-gray-128<br>intensity=0.6, normal_format=1 | 4,096/4,096 · 0 | 4,096/4,096 · 0 | 4,096/4,096 · 0 | 2,975/4,096 · 1.1920929e-07 | 1,014/1,024 · 1.1920929e-07<br>input=gray1 |
| `normal_to_height_2.sbs/normal_to_height` · height (LUM) | checked | Input=curved-normal-128<br>normal_format=1, Relief_Balance=(0.2,0.6,0.8), global_opacity=0.6 | 2,823/4,096 · 69 | 2,752/4,096 · 17727 | 2,753/4,096 · 0.270507812 | 2,752/4,096 · 0.270496666 | 270/1,024 · 0.305134654<br>Input=normal |
| `normal_to_height_hq.sbs/base_algorithm_height_distance` · output (LUM) | bridge | input=offset-gray-128<br>intensity=0.6, light_angle=0.3, light_offset=0.4, detail_balance=0.6 | 4,096/4,096 · 0 | 4,096/4,096 · 0 | 4,096/4,096 · 0 | 4,096/4,096 · 0 | separate root controls |
| `normal_transform.sbs/normal_transform` · output (RGBA) | angle | input=curved-normal-128<br>offset=(0.125,-0.25), normal_format=1, matrix22=(0,-1,1,0), handle_scale_and_skew_transformation=true | 4,096/4,096 · 0 | 4,095/4,096 · 1 | 4,096/4,096 · 0 | 3,626/4,096 · 1.1920929e-07 | 868/1,024 · 1.1920929e-07<br>input=normal |
| `panorama_gradients.sbs/panorama_gradient_2_points` · output (RGBA) | checked | none<br>point_1_position=(0.2,0.3), point_2_position=(0.8,0.6), point_1_contrast=0.3 | 4,096/4,096 · 0 | 4,045/4,096 · 1 | 4,088/4,096 · 0.00048828125 | 49/4,096 · 4.17232513e-07 | 19/1,024 · 4.76837158e-07 |
| `panorama_gradients.sbs/panorama_gradient_linear` · output (RGBA) | checked | none<br>point_position=(0.3,0.4), top_color=(0.2,0.7,0.4), bottom_color=(0.8,0.1,0.3), contrast=0.25 | 4,096/4,096 · 0 | 4,070/4,096 · 1 | 4,091/4,096 · 0.000122070312 | 0/4,096 · 1.78813934e-07 | 0/1,024 · 1.93715096e-07 |
| `panorama_nadir_extract.sbs/nadir_extract` · output (RGBA) | value | input=blend-fg-alpha-128<br>rotation=0.125, scale=3, output_gamma=1 | 4,096/4,096 · 0 | 4,094/4,096 · 34 | 4,094/4,096 · 0.00048828125 | 4,094/4,096 · 0.000518798828 | 1,024/1,024 · 0<br>input=color1 |
| `panorama_nadir_patch.sbs/panorama_nadir_patch` · output (RGBA) | angle | input=blend-fg-alpha-128, mask_input=levels-range-64<br>show_frames_helper=false, frame_thickness=0.1, patch_scale=0.5, patch_size=(0.7,1.2), patch_rotation=0.125, patch_hardness=0.8, source_position=(0.6,0.7), patch_position=(0.4,0.8) | 4,096/4,096 · 0 | 4,085/4,096 · 30 | 4,095/4,096 · 0.00048828125 | 4,022/4,096 · 0.000457763672 | 1,004/1,024 · 4.85777855e-06<br>input=color1, mask_input=gray2 |
| `panorama_physical_sun_sky.sbs/Impl/GetOneParam` · output (float3) | checked | input=blend-fg-alpha-128<br>elevationK=0.5, turbidity=2, albedo=0.5, paramIndex=2 | 0/1 · 1.1920929e-07 | 0/1 · 1.1920929e-07 | 0/1 · 1.1920929e-07 | 0/1 · 1.1920929e-07 | 0/1 · 7.51018524e-06<br>input=color1 |
| `panorama_position.sbs/panorama_3d_position` · output (RGBA) | checked | none<br>up_vector=1 | 4,096/4,096 · 0 | 4,086/4,096 · 1 | 4,096/4,096 · 0 | 292/4,096 · 2.38418579e-07 | 43/1,024 · 2.38418579e-07 |
| `panorama_rotation.sbs/panorama_rotation` · output (RGBA) | angle | input=blend-fg-alpha-128<br>rotation=0.125, direction_angle=0.125, pre_rotation_around_pole=0.125, post_rotation_around_pole=-0.125 | 4,090/4,096 · 1 | 4,083/4,096 · 63 | 4,083/4,096 · 0.0009765625 | 4,083/4,096 · 0.000961303711 | 1,023/1,024 · 0.0012512207<br>input=color1 |
| `panorama_shape.sbs/panorama_line_light` · output (RGBA) | checked | background_image_input=blend-fg-alpha-128, pattern_image_input=levels-range-64, shape_image_input=blend-bg-alpha-128<br>point_2_position=(0.8,0.6,1), up_vector=1, shape_color=(0.2,0.6,0.8), shape_exposure=2, line_thickness=0.2 | 4,093/4,096 · 1 | 4,081/4,096 · 4 | 4,093/4,096 · 0.00048828125 | 3,608/4,096 · 6.66379929e-05 | 904/1,024 · 6.27338886e-06<br>background_image_input=color1, pattern_image_input=gray2, shape_image_input=color3 |
| `panorama_shape.sbs/panorama_plane_light` · output (RGBA) | value | background_image_input=blend-fg-alpha-128, pattern_image_input=levels-range-64, shape_image_input=blend-bg-alpha-128<br>up_vector=1, shape_color=(0.2,0.6,0.8), shape_exposure=2, plane_scale=0.8, plane_size=(0.7,1.2), plane_rotation=(0.125,0.25,0) | 4,096/4,096 · 0 | 4,096/4,096 · 0 | 4,096/4,096 · 0 | 4,096/4,096 · 0 | 1,024/1,024 · 0<br>background_image_input=color1, pattern_image_input=gray2, shape_image_input=color3 |
| `panorama_shape.sbs/panorama_shape` · output (RGBA) | angle | input=blend-fg-alpha-128<br>offset=(0.125,-0.25), matrix22=(0,-1,1,0), shape=1, shape_color=(0.2,0.6,0.8), shape_intensity=2, hotspot_intensity=3, hotspot_size=0.3, hotspot_falloff=0.7 | 4,096/4,096 · 0 | 4,083/4,096 · 1 | 4,092/4,096 · 0.00048828125 | 3,804/4,096 · 1.07288361e-05 | 941/1,024 · 6.31809235e-06<br>input=color1 |
| `panorama_shape.sbs/panorama_shape_2` · output (RGBA) | angle | background_image_input=blend-bg-alpha-128, shape_image_input=blend-fg-alpha-128<br>offset=(0.125,-0.25), matrix22=(0,-1,1,0), shape=1, shape_color=(0.2,0.6,0.8), hotspot_size=0.3, hotspot_falloff=0.7, shape_exposure=2 | 4,096/4,096 · 0 | 4,081/4,096 · 1 | 4,094/4,096 · 7.62939453e-06 | 2,734/4,096 · 4.9829483e-05 | 680/1,024 · 3.71932983e-05<br>background_image_input=color1, shape_image_input=color2 |
| `panorama_shape.sbs/panorama_sphere_light` · output (RGBA) | checked | background_image_input=blend-fg-alpha-128, sphere_image_input=blend-bg-alpha-128<br>up_vector=1, sphere_world_position=(1,2,3), sphere_uv_position=(0.3,0.6), sphere_radius=1, sphere_exposure=2 | 4,096/4,096 · 0 | 4,096/4,096 · 0 | 4,096/4,096 · 0 | 4,069/4,096 · 1.90734863e-06 | 1,018/1,024 · 1.90734863e-06<br>background_image_input=color1, sphere_image_input=color2 |
| `panorama_shape.sbs/panorama_spot` · output (RGBA) | checked | input=blend-fg-alpha-128<br>scale=3, shape_color=(0.2,0.6,0.8), shape_intensity=2, hotspot_intensity=3, hotspot_size=0.3, hotspot_falloff=0.7, position=(0.3,0.6) | 4,095/4,096 · 1 | 4,056/4,096 · 1 | 4,089/4,096 · 0.00048828125 | 2,826/4,096 · 2.76565552e-05 | 705/1,024 · 3.71932983e-05<br>input=color1 |
| `panorama_straighten_horizon.sbs/panorama_straighten_horizon` · output (RGBA) | angle | input=blend-fg-alpha-128<br>enable_horizon_correction=true, horizon_p1=(0.2,0.4), horizon_p2=(0.7,0.6), horizontal_rotation=0.125 | 0/4,096 · 225 | 0/4,096 · 57829 | 0/4,096 · 0.882263184 | 0/4,096 · 0.882413983 | 0/1,024 · 0.937407508<br>input=color1 |
| `paper_path.sbs/paper_path` · Paper_Path (LUM) | checked | none<br>Pattern_size_width=130, Pattern_size_height=80, Disorder=0.2 | 2,334/4,096 · 255 | 2,297/4,096 · 65535 | 45/4,096 · 1.16601562 | 18/4,096 · 1.1666851 | 7/1,024 · 0.0057874918 |
| `parquet_1.sbs/parquet_1` · Parquet (LUM) | checked | none<br>Tiling=2, Luminance_Variation=0.3 | 4,016/4,096 · 106 | 4,016/4,096 · 27242 | 4,016/4,096 · 0.415527344 | 4,016/4,096 · 0.41568628 | 1,008/1,024 · 0.43921569 |
| `paths_tools.sbs/Impl/compute_paths_lengths` · lengths (LUM) | loop | paths=original-paths-seven<br>passthrough=true, interm_lengths=true, scale=(0.7,1.2) | 1/1 · 0 | 1/1 · 0 | 1/1 · 0 | 1/1 · 0 | loop result type<br>paths=original-paths-seven |
| `paths_tools.sbs/Impl/facets_to_paths_impl_link_template` · output (RGBA) | value | input=original-facets-gradient<br>radius=2, init=true, foreach=2, finally=(0.2,0.4,0.6,0.8) | 4,096/4,096 · 0 | 4,096/4,096 · 0 | 4,096/4,096 · 0 | 4,096/4,096 · 0 | 1,024/1,024 · 0<br>input=original-facets-gradient |
| `paths_tools.sbs/Impl/facets_to_paths_impl_stitch_template` · output (RGBA) | checked | input=original-facets-gradient<br>radius=2, init=true, foreach=2, finally=(0.2,0.4,0.6,0.8) | 0/4,096 · 204 | 0/4,096 · 52428 | 0/4,096 · 0.799804688 | 0/4,096 · 0.800000012 | 0/1,024 · 0.800000012<br>input=original-facets-gradient |
| `paths_tools.sbs/Impl/mask_to_facets` · output (RGBA) | value | mask=original-mask-gradient-16<br>threshold=0.3 | 4,096/4,096 · 0 | 4,096/4,096 · 0 | 4,096/4,096 · 0 | 4,096/4,096 · 0 | 1,024/1,024 · 0<br>mask=gray1 |
| `paths_tools.sbs/Impl/sort_by_blue` · output (RGBA) | loop | input=original-rgba-16<br>passthrough=true, count=32 | 4,096/4,096 · 0 | 4,096/4,096 · 0 | 4,096/4,096 · 0 | 4,096/4,096 · 0 | loop result type<br>input=color1 |
| `paths_tools.sbs/paths_polygon` · paths (RGBA) | checked | none<br>nbSides=7 | 256/256 · 0 | 254/256 · 1 | 256/256 · 0 | 250/256 · 5.60283661e-06 | 250/256 · 5.60283661e-06 |
| `paths_tools.sbs/paths_trace` · tracing (RGBA) | checked | background=original-rgba-16, paths=original-paths-seven<br>trace_corners=true, trace_vertices=true, thickness=1.5 | 3,686/4,096 · 239 | 2,192/4,096 · 61469 | 3,411/4,096 · 1.37158203 | 0/4,096 · 1.37210912 | 890/1,024 · 1.75791217<br>background=color1, paths=original-paths-seven |
| `paths_tools.sbs/vertex_processor` · paths (RGBA) | checked | input_1=original-rgba-16, input_2=original-mask-16, input_3=original-rgba-16, input_4=original-mask-16, input_5=original-rgba-16, input_6=original-mask-16, input_7=original-rgba-16, input_8=original-mask-16, paths=original-paths-seven<br>fetchPrevCount=1, fetchNextCount=1, imageInputCount=2, perVertex=(0.25,0.75) | 3,869/4,096 · 191 | 3,869/4,096 · 49151 | 3,857/4,096 · 0.75 | 3,791/4,096 · 0.75 | 968/1,024 · 0.75<br>input_1=color1, input_2=gray2, input_3=color3, input_4=gray4, input_5=color5, input_6=gray6, input_7=color7, input_8=gray8, paths=original-paths-seven |
| `paths_tools.sbs/vertex_processor_simple` · paths (RGBA) | checked | input_1=original-rgba-16, input_2=original-mask-16, input_3=original-rgba-16, input_4=original-mask-16, paths=original-paths-seven<br>imageInputCount=2, perVertex=(0.25,0.75) | 3,869/4,096 · 191 | 3,869/4,096 · 49151 | 3,857/4,096 · 0.75 | 3,791/4,096 · 0.75 | 968/1,024 · 0.75<br>input_1=color1, input_2=gray2, input_3=color3, input_4=gray4, paths=original-paths-seven |
| `pattern_alveolus.sbs/alveolus` · output (LUM) | value | none<br>Tiling=2, Gradient_Filled_Cells=true, Interstice_Width=0.15 | 4,096/4,096 · 0 | 4,096/4,096 · 0 | 4,096/4,096 · 0 | 4,096/4,096 · 0 | 1,024/1,024 · 0 |
| `pattern_arc_pavement.sbs/arc_pavement` · output (LUM) | angle | pattern_input=offset-gray-128<br>scale=2, pattern_amount=7, pattern_amount_random=0.2, arcs_amount=8, pattern=3, pattern_variation=0.3, pattern_scale=0.8, pattern_width=0.6, pattern_height=0.7, pattern_width_random=0.2, color_random=0.3 | 3,218/4,096 · 254 | 3,207/4,096 · 65269 | 3,210/4,096 · 0.995605469 | 3,207/4,096 · 0.99593389 | 849/1,024 · 0.999720931<br>pattern_input=gray1 |
| `pattern_brick_1.sbs/brick_1` · output (LUM) | sampler | none<br>scale=2, edge_smoothness=0.3, interstice_width=0.2 | 2,688/4,096 · 65 | 2,688/4,096 · 16641 | 2,688/4,096 · 0.25390625 | 2,688/4,096 · 0.253936917 | 672/1,024 · 0.12161243 |
| `pattern_brick_2.sbs/brick_2` · output (LUM) | sampler | none<br>scale=2, edge_smoothness=0.3, interstice_width=0.2 | 2,560/4,096 · 30 | 2,560/4,096 · 7601 | 2,560/4,096 · 0.116210938 | 2,560/4,096 · 0.115979761 | 384/1,024 · 0.187185353 |
| `pattern_cube_3d.sbs/cube_3d` · output (LUM) | sampler | none<br>scale=0.35, orientation_offset=(0.17,0.32), size=(0.7,1,1.3) | 4,096/4,096 · 0 | 4,087/4,096 · 1 | 4,096/4,096 · 0 | 1,653/4,096 · 2.98023224e-07 | 432/1,024 · 2.98023224e-07 |
| `pattern_cube_3d.sbs/cube_3d_gbuffers` · position (RGBA) | sampler | none<br>scale=0.35, orientation_offset=(0.17,0.32), size=(0.7,1,1.3) | 4,096/4,096 · 0 | 4,086/4,096 · 1 | 4,064/4,096 · 0.00048828125 | 1,342/4,096 · 2.98023224e-07 | 330/1,024 · 2.38418579e-07 |
| `pattern_cube_3d.sbs/cube_3d_gbuffers` · normal (RGBA) | sampler | none<br>scale=0.35, orientation_offset=(0.17,0.32), size=(0.7,1,1.3) | 4,096/4,096 · 0 | 4,096/4,096 · 0 | 4,096/4,096 · 0 | 4,096/4,096 · 0 | 1,024/1,024 · 0 |
| `pattern_cube_3d.sbs/cube_3d_gbuffers` · depth (LUM) | sampler | none<br>scale=0.35, orientation_offset=(0.17,0.32), size=(0.7,1,1.3) | 4,096/4,096 · 0 | 4,087/4,096 · 1 | 4,096/4,096 · 0 | 1,653/4,096 · 2.98023224e-07 | 432/1,024 · 2.98023224e-07 |
| `pattern_cube_3d.sbs/cube_3d_gbuffers` · uv (RGBA) | sampler | none<br>scale=0.35, orientation_offset=(0.17,0.32), size=(0.7,1,1.3) | 4,096/4,096 · 0 | 4,090/4,096 · 1 | 4,093/4,096 · 0.000244140625 | 1,277/4,096 · 2.98023224e-07 | 324/1,024 · 2.38418579e-07 |
| `pattern_fibers_1.sbs/fibers_1` · output (LUM) | sampler | none<br>Tiling=2 | 1,872/4,096 · 8 | 56/4,096 · 2010 | 300/4,096 · 0.0302734375 | 56/4,096 · 0.0306706131 | 0/1,024 · 0.0354467034 |
| `pattern_fibers_2.sbs/fibers_2` · output (LUM) | sampler | none<br>Tiling=2 | 1,424/4,096 · 11 | 160/4,096 · 2879 | 464/4,096 · 0.0439453125 | 160/4,096 · 0.0439307094 | 0/1,024 · 0.0511024594 |
| `pattern_gaussian_1.sbs/gaussian_1` · output (LUM) | value | none<br>Tiling=2 | 4,096/4,096 · 0 | 4,064/4,096 · 1 | 4,096/4,096 · 0 | 4,064/4,096 · 1.52587891e-05 | 1,024/1,024 · 0 |
| `pattern_gaussian_2.sbs/gaussian_2` · output (LUM) | value | none<br>Tiling=2 | 4,096/4,096 · 0 | 4,032/4,096 · 1 | 4,096/4,096 · 0 | 4,032/4,096 · 1.52587891e-05 | 1,024/1,024 · 0 |
| `pattern_mesh_1.sbs/mesh_1` · output (LUM) | sampler | none<br>Tiling=2, Rotate_45_Degrees=true | 16/4,096 · 225 | 0/4,096 · 57804 | 0/4,096 · 0.882080078 | 0/4,096 · 0.882032499 | 0/1,024 · 0.856000617 |
| `pattern_mesh_2.sbs/mesh_2` · output (LUM) | sampler | none<br>Tiling=2, Rotation=true | 96/4,096 · 255 | 80/4,096 · 65535 | 80/4,096 · 1 | 80/4,096 · 1 | 48/1,024 · 0.984206913 |
| `pattern_polygon_2.sbs/polygon_2` · output (LUM) | sampler | none<br>Tiling=2, Scale=0.8, Rotation=0.125, Sides=5, Curve=0.2, Gradient=0.6, InvertGradient=true | 4,080/4,096 · 1 | 3,160/4,096 · 3 | 3,992/4,096 · 0.00048828125 | 3,160/4,096 · 4.57912683e-05 | 768/1,024 · 3.05175781e-05 |
| `pattern_shape.sbs/hemisphere` · output (LUM) | sampler | none<br>scale=0.7, size_xy=(0.7,1.2), angle=0.125 | 4,096/4,096 · 0 | 4,091/4,096 · 1 | nonfinite cook | nonfinite cook | 201/1,024 · 4.38094139e-06² |
| `pattern_shape.sbs/shape_base` · output (LUM) | sampler | none<br>scale=0.7, pattern=3, pattern_specific=0.3, size_xy=(0.7,1.2), angle=0.125 | 4,096/4,096 · 0 | 4,072/4,096 · 1 | 4,088/4,096 · 0.00048828125 | 2,910/4,096 · 1.43051147e-06 | 717/1,024 · 1.37090683e-06 |
| `pattern_splatter_circular.sbs/splatter_circular` · Splatter_Circular (LUM) | angle | background=levels-range-64, pattern_input_1=offset-gray-128, pattern_input_2=levels-range-64, pattern_input_3=levels-range-64, pattern_input_4=levels-range-64, pattern_input_5=levels-range-64, pattern_input_6=levels-range-64<br>pattern_amount=7, pattern_amount_random=0.2, pattern=3, pattern_scale=0.8, ring_amount=2, radius=0.35, radius_random=0.15, angle_random=0.2, spiral_factor=0.1, pattern_rotation_random=0.3, pattern_size=(0.7,1.2) | 3,986/4,096 · 223 | 3,986/4,096 · 57311 | 3,986/4,096 · 0.874511719 | 3,986/4,096 · 0.874509797 | 976/1,024 · 0.976470588<br>background=gray1, pattern_input_1=gray2, pattern_input_2=gray3, pattern_input_3=gray4, pattern_input_4=gray5, pattern_input_5=gray6, pattern_input_6=gray7 |
| `pattern_splatter_circular.sbs/splatter_circular_color` · Splatter_Circular (RGBA) | angle | background=blend-bg-alpha-128, pattern_input_1=blend-fg-alpha-128, pattern_input_2=blend-bg-alpha-128, pattern_input_3=blend-bg-alpha-128, pattern_input_4=blend-bg-alpha-128, pattern_input_5=blend-bg-alpha-128, pattern_input_6=blend-bg-alpha-128<br>pattern_amount=7, pattern_amount_random=0.2, pattern=3, pattern_scale=0.8, ring_amount=2, radius=0.35, radius_random=0.15, angle_random=0.2, spiral_factor=0.1, pattern_rotation_random=0.3, pattern_size=(0.7,1.2) | 3,986/4,096 · 254 | 3,986/4,096 · 65278 | 3,986/4,096 · 0.996078491 | 3,986/4,096 · 0.996078431 | 976/1,024 · 0.996078431<br>background=color1, pattern_input_1=color2, pattern_input_2=color3, pattern_input_3=color4, pattern_input_4=color5, pattern_input_5=color6, pattern_input_6=color7 |
| `pattern_stripes.sbs/stripes` · Stripes (LUM) | value | none<br>Stripes=7, Width=0.35, Softness=0.2, Shift=5, Align=1 | 4,096/4,096 · 0 | 4,096/4,096 · 0 | 4,096/4,096 · 0 | 4,096/4,096 · 0 | 1,024/1,024 · 0 |
| `pattern_tile_generator.sbs/tile_generator` · output (LUM) | value | background=offset-gray-128, pattern_input_1=levels-range-64, pattern_input_2=levels-range-64, pattern_input_3=levels-range-64, pattern_input_4=levels-range-64, pattern_input_5=levels-range-64, pattern_input_6=levels-range-64<br>scale=2, pattern=3, pattern_specific=(0.3,0), x_amount=7, y_amount=3, pattern_rotation_random=0.3, pattern_size=(0.7,1.2), rotation=0.125 | 4,096/4,096 · 0 | 4,096/4,096 · 0 | 4,096/4,096 · 0 | 4,096/4,096 · 0 | 1,024/1,024 · 0<br>background=gray1, pattern_input_1=gray2, pattern_input_2=gray3, pattern_input_3=gray4, pattern_input_4=gray5, pattern_input_5=gray6, pattern_input_6=gray7 |
| `pattern_tile_generator.sbs/tile_generator_color` · output (RGBA) | sampler | background=blend-bg-alpha-128, pattern_input_1=blend-fg-alpha-128, pattern_input_2=blend-bg-alpha-128, pattern_input_3=blend-bg-alpha-128, pattern_input_4=blend-bg-alpha-128, pattern_input_5=blend-bg-alpha-128, pattern_input_6=blend-bg-alpha-128<br>scale=2, pattern=3, pattern_specific=(0.3,0), x_amount=7, y_amount=3, pattern_rotation_random=0.3, pattern_size=(0.7,1.2), rotation=0.125 | 4,096/4,096 · 0 | 4,096/4,096 · 0 | 0/4,096 · 4.953125 | 0/4,096 · 4.95686245 | 0/1,024 · 4.99215698<br>background=color1, pattern_input_1=color2, pattern_input_2=color3, pattern_input_3=color4, pattern_input_4=color5, pattern_input_5=color6, pattern_input_6=color7 |
| `pattern_tile_random.sbs/tile_random` · output (LUM) | sampler | background_input=levels-range-64, pattern_input=offset-gray-128<br>pattern=3, pattern_specific=0.3, x_amount=7, y_amount=3, pattern_rotation_random=0.3, shape_rotation=0.125 | 3,030/4,096 · 223 | 3,030/4,096 · 57311 | 3,030/4,096 · 1 | 3,030/4,096 · 1.00000006 | 746/1,024 · 1.00000006<br>background_input=gray1, pattern_input=gray2 |
| `pattern_tile_random.sbs/tile_random_color` · output (RGBA) | sampler | background_input=blend-fg-alpha-128, pattern_input=blend-bg-alpha-128<br>pattern=3, pattern_specific=0.3, x_amount=7, y_amount=3, pattern_rotation_random=0.3, shape_rotation=0.125 | 3,030/4,096 · 250 | 3,030/4,096 · 64250 | 2,440/4,096 · 1 | 2,440/4,096 · 1.00000006 | 598/1,024 · 1.00000006<br>background_input=color1, pattern_input=color2 |
| `pattern_tile_random_2.sbs/tile_random_2` · output (LUM) | sampler | bevel_distance_map=levels-range-64, mask_map=levels-range-64, random_size_map=offset-gray-128, random_slant_map=levels-range-64, round_corners_radius_map=levels-range-64<br>amount_x=7, amount_y=3, random_size_multiplier=0.7, round_corners_radius=0.1, shape_rotation=0.125, random_position_multiplier=0.5 | 1,751/4,096 · 255 | 1,746/4,096 · 65535 | 1,746/4,096 · 1 | 1,746/4,096 · 1 | 363/1,024 · 1<br>bevel_distance_map=gray1, mask_map=gray2, random_size_map=gray3, random_slant_map=gray4, round_corners_radius_map=gray5 |
| `pattern_tile_random_2.sbs/tile_random_2` · flood_fill_data1 (RGBA) | sampler | bevel_distance_map=levels-range-64, mask_map=levels-range-64, random_size_map=offset-gray-128, random_slant_map=levels-range-64, round_corners_radius_map=levels-range-64<br>amount_x=7, amount_y=3, random_size_multiplier=0.7, round_corners_radius=0.1, shape_rotation=0.125, random_position_multiplier=0.5 | 1,194/4,096 · 253 | 1,194/4,096 · 64954 | 1,194/4,096 · 0.990722656 | 1,194/4,096 · 0.991127551 | 258/1,024 · 0.996064007<br>bevel_distance_map=gray1, mask_map=gray2, random_size_map=gray3, random_slant_map=gray4, round_corners_radius_map=gray5 |
| `pattern_triangle_grid.sbs/triangle_grid_base` · output (RGBA) | sampler | color_input=blend-bg-alpha-128, height=offset-gray-128, vector_map=blend-bg-alpha-128<br>x_amount=7, y_amount=3, random_position_multiplier=0.5, height_random=0.5, rotation=0.125, rotation_disorder=0.3 | 4,095/4,096 · 1 | 4,088/4,096 · 1 | 4,040/4,096 · 0.00048828125 | 2,649/4,096 · 5.96046448e-07 | 641/1,024 · 4.47034836e-07<br>color_input=color1, height=gray2, vector_map=color3 |
| `pattern_weave_1.sbs/weave_1` · output (LUM) | sampler | none<br>Tiling=2, rotate_45_degrees=true | 0/4,096 · 255 | 0/4,096 · 65535 | 0/4,096 · 1 | 0/4,096 · 1 | 0/1,024 · 1 |
| `pattern_weave_2.sbs/weave_2` · output (LUM) | sampler | none<br>scale=2, rotate_45_degrees=true, Disorder=0.25 | 166/4,096 · 255 | 108/4,096 · 65535 | 109/4,096 · 1 | 108/4,096 · 1 | 0/1,024 · 0.860593565 |
| `pavement_1.sbs/pavement_1` · Pavement (LUM) | sampler | none<br>Roudness=0.75 | 368/4,096 · 255 | 368/4,096 · 65535 | 368/4,096 · 1 | 368/4,096 · 1 | 0/1,024 · 0.718337655 |
| `pbr_render.sbs/PBR_render_mapping_color` · output (RGBA) | bridge | texture=blend-fg-alpha-128, uv=position-128<br>defaults | 4,096/4,096 · 0 | 4,096/4,096 · 0 | 4,096/4,096 · 0 | 4,096/4,096 · 0 | separate root controls |
| `pbr_render.sbs/PBR_render_mapping_grayscale` · output (RGBA) | bridge | texture=offset-gray-128, uv=position-128<br>defaults | 4,096/4,096 · 0 | 4,096/4,096 · 0 | 4,096/4,096 · 0 | 4,096/4,096 · 0 | separate root controls |
| `pbr_render.sbs/chroma_shift` · output (RGBA) | bridge | input=blend-fg-alpha-128<br>radius=2, angle=0.125 | 4,096/4,096 · 0 | 4,096/4,096 · 0 | 4,096/4,096 · 0 | 4,096/4,096 · 0 | separate root controls |
| `pbr_render.sbs/environment_map_rescale` · output (RGBA) | sampler | input=blend-bg-alpha-128<br>defaults | 512/4,096 · 155 | 0/4,096 · 39853 | 0/4,096 · 0.705078125 | 0/4,096 · 0.705963194 | 0/1,024 · 1.06516372<br>input=color1 |
| `pbr_render.sbs/get_max_coords/get_max_coord` · Maximum (float2) | scope | input=levels-range-64<br>defaults | 1/1 · 0 | 1/1 · 0 | 1/1 · 0 | 1/1 · 0 | separate root controls |
| `pbr_render.sbs/ibl_specular_contrib_anisotropic/panoramic_lod` · Diffuse (RGBA) | sampler | input=blend-bg-alpha-128<br>defaults | 4,092/4,096 · 1 | 1,127/4,096 · 1 | 3,587/4,096 · 0.00048828125 | 0/4,096 · 7.65919685e-06 | 7/1,024 · 1.1920929e-07<br>input=color1 |
| `pbr_render.sbs/position_to_normal` · output (RGBA) | sampler | input=blend-bg-alpha-128, position=position-128<br>shape=1, shape_scale=0.7, plane_size=(0.7,1.3), uv_tiling=2, sphere_uv_scale=(2,3), plane_uv_scale=(2,1), cylinder_uv_scale=(2,1), cylinder_length=1.5, uv_offset=(0.125,0.25), rotation=0.125, rotation_direction=0.2, rotation_around_direction=0.3, normal_format=1, recompute_normals=true | 3,226/4,096 · 9 | 1,062/4,096 · 2220 | 1,221/4,096 · 0.0338134766 | 0/4,096 · 0.0338821262 | 0/1,024 · 0.0451358557<br>input=color1, position=position |
| `pbr_render.sbs/raytrace_position` · output (RGBA) | sampler | blue_noise=levels-range-64, height=offset-gray-128, opacity=levels-range-64<br>shape=1, shape_scale=0.7, shape_position=(0.1,-0.2,0.3), plane_size=(0.7,1.3), displacement_intensity=0.2, uv_tiling=2, sphere_uv_scale=(2,3), plane_uv_scale=(2,1), cylinder_uv_scale=(2,1), cylinder_length=1.5, cylinder_radius=0.7, uv_offset=(0.125,0.25), rotation=0.125, rotation_direction=0.2, rotation_around_direction=0.3 | 3,696/4,096 · 253 | 3,607/4,096 · 64898 | 3,665/4,096 · 1.49023438 | 0/4,096 · 1.49027765 | 0/1,024 · 1.47207022<br>blue_noise=gray1, height=gray2, opacity=gray3 |
| `pbr_render.sbs/raytraced_irradiance_pass_8` · output (RGBA) | sampler | blue_noise=blend-bg-alpha-128, emissive=blend-bg-alpha-128, environment=hdr-field-128, height=offset-gray-128, normal=curved-normal-128, position=position-128, previous=blend-bg-alpha-128<br>defaults | 2,665/4,096 · 147 | 2,661/4,096 · 37823 | 0/4,096 · 6.45703125 | 0/4,096 · 6.45703125 | 219/1,024 · 7.4375<br>blue_noise=color1, emissive=color2, environment=hdr-field-128, height=gray4, normal=normal, position=position, previous=color7 |
| `pbr_render.sbs/raytraced_irradiance_pass_8` · next_pass_index (float) | sampler | blue_noise=blend-bg-alpha-128, emissive=blend-bg-alpha-128, environment=hdr-field-128, height=offset-gray-128, normal=curved-normal-128, position=position-128, previous=blend-bg-alpha-128<br>defaults | 1/1 · 0 | 1/1 · 0 | 1/1 · 0 | 1/1 · 0 | 1/1 · 0<br>blue_noise=color1, emissive=color2, environment=hdr-field-128, height=gray4, normal=normal, position=position, previous=color7 |
| `pbr_render.sbs/raytraced_specular_pass_8` · output (RGBA) | sampler | aniso_angle_level=blend-bg-alpha-128, blue_noise=levels-range-64, emissive=blend-bg-alpha-128, environment=hdr-field-128, height=offset-gray-128, normal=curved-normal-128, position=position-128, specular_color_roughness=blend-bg-alpha-128<br>shape=1, shape_scale=0.7, plane_size=(0.7,1.3), displacement_intensity=0.2, uv_tiling=2, cylinder_length=1.5, cylinder_radius=0.7, uv_offset=(0.125,0.25), rotation=0.125, rotation_direction=0.2, rotation_around_direction=0.3 | 3,808/4,096 · 1 | 1,698/4,096 · 128 | 112/4,096 · 0.00390625 | 112/4,096 · 0.00390625 | 1,023/1,024 · 1.1920929e-07<br>aniso_angle_level=color1, blue_noise=gray2, emissive=color3, environment=hdr-field-128, height=gray5, normal=normal, position=position, specular_color_roughness=color8 |
| `pbr_render2.sbs/PBR_render_mapping_color` · output (RGBA) | bridge | texture=blend-fg-alpha-128, uv=position-128<br>defaults | 4,096/4,096 · 0 | 4,096/4,096 · 0 | 4,096/4,096 · 0 | 4,096/4,096 · 0 | separate root controls |
| `pbr_render2.sbs/PBR_render_mapping_grayscale` · output (RGBA) | bridge | texture=offset-gray-128, uv=position-128<br>defaults | 4,096/4,096 · 0 | 4,096/4,096 · 0 | 4,096/4,096 · 0 | 4,096/4,096 · 0 | separate root controls |
| `pbr_render2.sbs/chroma_shift` · output (RGBA) | bridge | input=blend-fg-alpha-128<br>radius=2, angle=0.125 | 4,096/4,096 · 0 | 4,096/4,096 · 0 | 4,096/4,096 · 0 | 4,096/4,096 · 0 | separate root controls |
| `pbr_render2.sbs/environment_map_rescale` · output (RGBA) | sampler | input=blend-bg-alpha-128<br>defaults | 512/4,096 · 155 | 0/4,096 · 39853 | 0/4,096 · 0.705078125 | 0/4,096 · 0.705963194 | 0/1,024 · 1.06516372<br>input=color1 |
| `pbr_render2.sbs/get_max_coords/get_max_coord` · Maximum (float2) | scope | input=levels-range-64<br>defaults | 1/1 · 0 | 1/1 · 0 | 1/1 · 0 | 1/1 · 0 | separate root controls |
| `pbr_render2.sbs/ibl_specular_contrib_anisotropic/panoramic_lod` · Diffuse (RGBA) | value | input=blend-bg-alpha-128<br>defaults | 4,096/4,096 · 0 | 4,096/4,096 · 0 | 4,096/4,096 · 0 | 4,096/4,096 · 0 | 1,024/1,024 · 0<br>input=color1 |
| `pbr_render2.sbs/lod` · Diffuse (RGBA) | value | input=blend-bg-alpha-128<br>defaults | 4,096/4,096 · 0 | 4,096/4,096 · 0 | 4,096/4,096 · 0 | 4,096/4,096 · 0 | 1,024/1,024 · 0<br>input=color1 |
| `pbr_render2.sbs/position_to_normal` · output (RGBA) | sampler | normal=curved-normal-128, position=position-128<br>shape=1, shape_scale=0.7, plane_size=(0.7,1.3), uv_tiling=2, sphere_uv_scale=(2,3), plane_uv_scale=(2,1), cylinder_uv_scale=(2,1), cylinder_length=1.5, uv_offset=(0.125,0.25), rotation=0.125, rotation_direction=0.2, rotation_around_direction=0.3, normal_format=1, recompute_normals=true | 3,895/4,096 · 2 | 403/4,096 · 399 | 1,874/4,096 · 0.00610351562 | 0/4,096 · 0.00607806444 | 0/1,024 · 0.00274163485<br>normal=normal, position=position |
| `pbr_render2.sbs/raytrace_position` · output (RGBA) | sampler | blue_noise=levels-range-64, height=offset-gray-128, opacity=levels-range-64<br>shape=1, shape_scale=0.7, shape_position=(0.1,-0.2,0.3), plane_size=(0.7,1.3), displacement_intensity=0.2, uv_tiling=2, sphere_uv_scale=(2,3), plane_uv_scale=(2,1), cylinder_uv_scale=(2,1), cylinder_length=1.5, cylinder_radius=0.7, uv_offset=(0.125,0.25), rotation=0.125, rotation_direction=0.2, rotation_around_direction=0.3 | 3,722/4,096 · 253 | 3,621/4,096 · 64898 | 3,683/4,096 · 1.49023438 | 0/4,096 · 1.49027765 | 0/1,024 · 1.47399312<br>blue_noise=gray1, height=gray2, opacity=gray3 |
| `pbr_render2.sbs/raytraced_irradiance_is_pass_8` · output (RGBA) | value | emissive=blend-bg-alpha-128, environment=hdr-field-128, height=offset-gray-128, noise=levels-range-64, normal=curved-normal-128, position=position-128, previous=blend-bg-alpha-128, sample_table=blend-bg-alpha-128<br>shape=1, shape_scale=0.7, plane_size=(0.7,1.3), displacement_intensity=0.2, uv_tiling=2, cylinder_length=1.5, cylinder_radius=0.7, uv_offset=(0.125,0.25), rotation=0.125, rotation_direction=0.2, rotation_around_direction=0.3, pass_index_1=2 | 4,096/4,096 · 0 | 4,096/4,096 · 0 | 4,096/4,096 · 0 | 4,096/4,096 · 0 | 1,024/1,024 · 0<br>emissive=color1, environment=hdr-field-128, height=gray3, noise=gray4, normal=normal, position=position, previous=color7, sample_table=color8 |
| `pbr_render2.sbs/raytraced_specular_pass_8` · output (RGBA) | sampler | aniso_angle_level=blend-bg-alpha-128, blue_noise=levels-range-64, emissive=blend-bg-alpha-128, environment=hdr-field-128, height=offset-gray-128, normal=curved-normal-128, position=position-128, specular_color_roughness=blend-bg-alpha-128<br>shape=1, shape_scale=0.7, plane_size=(0.7,1.3), displacement_intensity=0.2, uv_tiling=2, cylinder_length=1.5, cylinder_radius=0.7, uv_offset=(0.125,0.25), rotation=0.125, rotation_direction=0.2, rotation_around_direction=0.3 | 4,053/4,096 · 1 | 3,211/4,096 · 96 | 2,839/4,096 · 0.001953125 | 2,839/4,096 · 0.001953125 | 1,023/1,024 · 3.81469727e-06<br>aniso_angle_level=color1, blue_noise=gray2, emissive=color3, environment=hdr-field-128, height=gray5, normal=normal, position=position, specular_color_roughness=color8 |
| `pbr_render_ortho.sbs/blue_noise` · output (RGBA) | sampler | none<br>dithering=0.5 | 9/4,096 · 255 | 0/4,096 · 65535 | 0/4,096 · 1.99951172 | 0/4,096 · 1.99996763 | 0/1,024 · 1.99178356 |
| `pbr_render_ortho.sbs/environment_map_rescale` · output (RGBA) | sampler | input=blend-bg-alpha-128<br>defaults | 512/4,096 · 155 | 0/4,096 · 39853 | 0/4,096 · 0.705078125 | 0/4,096 · 0.705963194 | 0/1,024 · 1.06516372<br>input=color1 |
| `pbr_render_ortho.sbs/ibl_shadow_pass` · output (RGBA) | sampler | blue_noise=blend-fg-alpha-128, environment=hdr-field-128, normal=curved-normal-128<br>seq=3, total_samples=8, horizontal_rotation=0.125 | 0/4,096 · 255 | 0/4,096 · 65535 | 0/4,096 · 31.453125 | 0/4,096 · 31.4588242 | 0/1,024 · 28.3294125<br>blue_noise=color1, environment=hdr-field-128, normal=normal |
| `pbr_render_ortho.sbs/mipmap_generator` · Diffuse (RGBA) | sampler | input=blend-bg-alpha-128<br>defaults | 3,872/4,096 · 66 | 3,008/4,096 · 16919 | 2,766/4,096 · 0.258041382 | 0/4,096 · 0.25817919 | 17/1,024 · 0.017053945<br>input=color1 |
| `pbr_render_ortho.sbs/rotate_environment` · output (RGBA) | angle | input=blend-fg-alpha-128<br>amount=0.125, directionAngle=0.2, aroundPoleBefore=0.125, aroundPoleAfter=0.25, exposure=1 | 4,027/4,096 · 2 | 3,958/4,096 · 470 | 3,959/4,096 · 0.0068359375 | 3,958/4,096 · 0.00741589069 | 1,007/1,024 · 0.00659191608<br>input=color1 |
| `pbr_render_ortho.sbs/sphere_displace` · output (RGBA) | sampler | input=offset-gray-128<br>displacement_intensity=0.2, uv_offset=(0.125,0.25), samples=8, size=0.7 | 4,096/4,096 · 0 | 4,090/4,096 · 2 | 4,096/4,096 · 0 | 4,090/4,096 · 3.05175781e-05 | 1,022/1,024 · 1.52587891e-05<br>input=gray1 |
| `pbr_render_ortho.sbs/vector_shadow_pass` · output (RGBA) | sampler | environment=hdr-field-128, light_uv_coord_image_input=blend-bg-alpha-128, normal=curved-normal-128, white_noise=blend-fg-alpha-128<br>seq=3, samples=8 | 4,082/4,096 · 126 | 3,901/4,096 · 32393 | 3,969/4,096 · 0.995117188 | 3,816/4,096 · 0.994867861 | 1,024/1,024 · 0<br>environment=hdr-field-128, light_uv_coord_image_input=color2, normal=normal, white_noise=color4 |
| `perlin_noise_zoom.sbs/perlin_noise_zoom` · Noise_Zoom (LUM) | sampler | none<br>Distance=8, Disorder=0.3 | 44/4,096 · 149 | 2/4,096 · 38164 | 4/4,096 · 0.58190918 | 0/4,096 · 0.582338475 | 0/1,024 · 0.511653885 |
| `pre_multiply_straight.sbs/premult_to_straight` · output (RGBA) | bridge | input=blend-bg-alpha-128<br>defaults | 4,095/4,096 · 1 | 4,095/4,096 · 257 | 4,095/4,096 · 0.00390625 | 4,095/4,096 · 0.00392156839 | 1,024/1,024 · 0<br>input=color1 |
| `quad_transform.sbs/quad_transform` · output (RGBA) | bridge | input=blend-fg-alpha-128<br>p00=(0.1,0.125), p01=(0.2,0.9), p10=(0.9,0.2), p11=(0.8,0.85), enable_tiling=true | 4,096/4,096 · 0 | 4,096/4,096 · 0 | 4,096/4,096 · 0 | 4,096/4,096 · 0 | 1,024/1,024 · 0<br>input=color1 |
| `quad_transform.sbs/quad_transform_grayscale` · output (LUM) | bridge | input=offset-gray-128<br>p00=(0.1,0.125), p01=(0.2,0.9), p10=(0.9,0.2), p11=(0.8,0.85), enable_tiling=true | 4,096/4,096 · 0 | 4,096/4,096 · 0 | 4,096/4,096 · 0 | 4,096/4,096 · 0 | 1,024/1,024 · 0<br>input=gray1 |
| `quantize_color.sbs/color_conversions/convert_Lab_to_RGB` · output (RGBA) | sampler | input=blend-bg-alpha-128<br>defaults | 4,096/4,096 · 0 | 4,064/4,096 · 1 | 4,089/4,096 · 0.000244140625 | 566/4,096 · 4.59887087e-06 | 159/1,024 · 1.43051147e-06<br>input=color1 |
| `quantize_color.sbs/color_conversions/convert_RGB_to_Lab` · output (RGBA) | sampler | input=blend-fg-alpha-128<br>keep_alpha=true | 4,096/4,096 · 0 | 4,053/4,096 · 1 | 4,093/4,096 · 0.00048828125 | 480/4,096 · 3.57627869e-07 | 2/1,024 · 4.17232513e-07<br>input=color1 |
| `quantize_color.sbs/get_min_max/GetMaxFromAlpha_v2` · Maximum (RGBA) | sampler | input=blend-bg-alpha-128<br>defaults | 1/1 · 0 | 1/1 · 0 | 1/1 · 0 | 1/1 · 0 | 1/1 · 0<br>input=color1 |
| `quantize_color.sbs/kmeans_iteration` · output (RGBA) | sampler | input=blend-fg-alpha-128, seeds=original-palette-128<br>k=4, ignore_alpha=true, color_space=1 | 3,600/4,096 · 255 | 3,600/4,096 · 65535 | 3,584/4,096 · 1977.125 | 3,584/4,096 · 1978.05885 | 3,584/4,096 · 2014.94087<br>input=color1, seeds=original-palette-128 |
| `quantize_color.sbs/modify_color_palette_sub` · palette (RGBA) | bridge | id=original-id16-128, palette=original-palette-128<br>color_selection_mode=1, color_index=3, color_selection_spread=0.2, color_selection_contrast=0.4, distance_color_space=1, mode=1, opacity=0.6, override_color=(0.2,0.6,0.8), hsl=(0.1,0.2,-0.1) | 256/256 · 0 | 256/256 · 0 | 256/256 · 0 | 256/256 · 0 | 256/256 · 0<br>id=original-id16-128, palette=original-palette-128 |
| `radial_blur.sbs/radial_blur_grayscale` · output (LUM) | sampler | input=offset-gray-128<br>samples=16, angle=0.2, center_position=(0.5,0.5) | 4,083/4,096 · 1 | 2,771/4,096 · 23 | 3,937/4,096 · 0.00048828125 | 2,771/4,096 · 0.000350952148 | 315/1,024 · 0.00073248148<br>input=gray1 |
| `replace_color.sbs/replace_color` · ToTargetColor (RGBA) | sampler | Input=blend-fg-alpha-128<br>SourceColor=(0.2,0.3,0.4), TargetColor=(0.8,0.7,0.1) | 0/4,096 · 61 | 0/4,096 · 15809 | 0/4,096 · 0.241210938 | 0/4,096 · 0.241241872 | 0/1,024 · 0.244705796<br>Input=color1 |
| `rt_caustics.sbs/rt_caustics_base` · output (LUM) | sampler | height=offset-gray-128, noise=levels-range-64, normal=curved-normal-128<br>photon_grid_size=1, surface_height_scale=0.2, surface_height_position=0.7, surface_ior=1.5, photon_size=6, jittering=0.25, light_position=(0.3,0.7) | 3,822/4,096 · 83 | 729/4,096 · 21227 | 495/4,096 · 0.323730469 | 281/4,096 · 0.323898762 | 199/1,024 · 0.0106643774<br>height=gray1, noise=gray2, normal=normal |
| `rt_irradiance.sbs/compute_irradiance` · output (RGBA) | sampler | emissive=blend-bg-alpha-128, hit_uv=blend-fg-alpha-128, normal=curved-normal-128<br>defaults | 3,868/4,096 · 1 | 2/4,096 · 98 | 1,416/4,096 · 0.00146484375 | 0/4,096 · 0.00150150061 | 55/1,024 · 0.000606775284<br>emissive=color1, hit_uv=color2, normal=normal |
| `rt_irradiance.sbs/compute_ray_direction` · output (RGBA) | sampler | noise=levels-range-64, normal=curved-normal-128<br>defaults | 4,092/4,096 · 1 | 3,914/4,096 · 23 | 4,053/4,096 · 0.00048828125 | 0/4,096 · 0.000345274806 | 0/1,024 · 2.98023224e-07<br>noise=gray1, normal=normal |
| `rt_irradiance.sbs/debug_hit_uv` · output (RGBA) | angle | input=blend-fg-alpha-128<br>defaults | 37/4,096 · 135 | 37/4,096 · 34695 | 37/4,096 · 0.529296875 | 37/4,096 · 0.529411778 | 7/1,024 · 0.607843159<br>input=color1 |
| `rt_irradiance.sbs/mip_max` · output (LUM) | sampler | input=levels-range-64<br>defaults | size 32×32 / 64×64 | size 32×32 / 64×64 | size 32×32 / 64×64 | size 32×32 / 64×64 | size 16×16 / 32×32<br>input=gray1 |
| `rt_irradiance.sbs/panoramic_lod` · Diffuse (RGBA) | sampler | input=blend-bg-alpha-128<br>defaults | 4,096/4,096 · 0 | 4,096/4,096 · 0 | 4,096/4,096 · 0 | 4,096/4,096 · 0 | 1,024/1,024 · 0<br>input=color1 |
| `rt_shadows.sbs/mip_max` · output (LUM) | sampler | input=levels-range-64<br>defaults | size 32×32 / 64×64 | size 32×32 / 64×64 | size 32×32 / 64×64 | size 32×32 / 64×64 | size 16×16 / 32×32<br>input=gray1 |
| `safe_transform.sbs/safe_transform` · output (RGBA) | sampler | input=blend-fg-alpha-128<br>tile=2, offset_mode=1, offset=(0.125,-0.25), rotation=0.125, tile_safe_rotation=false, symmetry=1 | 4,096/4,096 · 0 | 4,096/4,096 · 0 | 4,096/4,096 · 0 | 4,096/4,096 · 0 | 1,024/1,024 · 0<br>input=color1 |
| `safe_transform.sbs/safe_transform_grayscale` · output (LUM) | sampler | input=offset-gray-128<br>tile=2, offset_mode=1, offset=(0.125,-0.25), rotation=0.125, tile_safe_rotation=false, symmetry=1 | 4,096/4,096 · 0 | 4,096/4,096 · 0 | 4,096/4,096 · 0 | 4,096/4,096 · 0 | 1,024/1,024 · 0<br>input=gray1 |
| `scratches_1.sbs/scratches_1` · Scratches (LUM) | sampler | none<br>Pattern_size_width=130, Pattern_size_height=80, Disorder=0.25, Rotation_Var=15, Rotation=30 | 1,951/4,096 · 198 | 978/4,096 · 50874 | 879/4,096 · 0.775802612 | 878/4,096 · 0.776283483 | 248/1,024 · 0.901990709 |
| `scratches_2.sbs/scratches_2` · Thorns (LUM) | sampler | none<br>Pattern_size_width=130, Pattern_size_height=80, Disorder=0.25 | 28/4,096 · 254 | 13/4,096 · 65365 | 4/4,096 · 1.49499512 | 0/4,096 · 1.49516 | 0/1,024 · 1.17333403 |
| `scratches_4.sbs/scratches_4` · Thorns_3 (LUM) | sampler | none<br>defaults | 12/4,096 · 255 | 5/4,096 · 65535 | 5/4,096 · 1 | 5/4,096 · 1 | 0/1,024 · 1 |
| `shape.sbs/shape_base` · output (LUM) | sampler | none<br>pattern=3, pattern_specific=0.3, scale=0.7, size_xy=(0.7,1.2), angle=0.125 | 4,096/4,096 · 0 | 4,072/4,096 · 1 | 4,088/4,096 · 0.00048828125 | 2,910/4,096 · 1.43051147e-06 | 717/1,024 · 1.37090683e-06 |
| `shape_mapper.sbs/shape_mapper` · output (LUM) | sampler | input=offset-gray-128<br>rotation=0.125, shape=1, pattern_amount=6, segments=5, radius=0.3, width=0.7, flip_1on_2=true | 3,533/4,096 · 34 | 1,686/4,096 · 8738 | 1,735/4,096 · 0.133300781 | 1,686/4,096 · 0.13333334 | 1,018/1,024 · 0.0025177598<br>input=gray1 |
| `shape_splatter.sbs/shape_splatter_blend` · output (LUM) | sampler | see input group 12<br>pattern_number=3, luminance_adjustement=0.2, luminance_random=0.3, color_input_opacity=0.4 | 4,096/4,096 · 0 | 4,096/4,096 · 0 | 4,096/4,096 · 0 | 4,096/4,096 · 0 | 1,024/1,024 · 0<br>same pin order |
| `shape_splatter.sbs/shape_splatter_blend_color` · output (RGBA) | bridge | see input group 13<br>pattern_number=3, color_input_opacity=0.4 | 4,096/4,096 · 0 | 4,096/4,096 · 0 | 4,096/4,096 · 0 | 4,096/4,096 · 0 | 1,024/1,024 · 0<br>same pin order |
| `shape_splatter.sbs/shape_splatter_data_extract` · position (RGBA) | sampler | splatter_data_1=blend-bg-alpha-128, splatter_data_2=blend-fg-alpha-128<br>defaults | 4,096/4,096 · 0 | 4,096/4,096 · 0 | 4,096/4,096 · 0 | 4,003/4,096 · 1.1920929e-07 | 1,024/1,024 · 0<br>splatter_data_1=color1, splatter_data_2=color2 |
| `shape_splatter.sbs/shape_splatter_data_extract` · uv_texture_space (RGBA) | sampler | splatter_data_1=blend-bg-alpha-128, splatter_data_2=blend-fg-alpha-128<br>defaults | 4,096/4,096 · 0 | 4,096/4,096 · 0 | 4,096/4,096 · 0 | 1,181/4,096 · 1.1920929e-07 | 300/1,024 · 1.1920929e-07<br>splatter_data_1=color1, splatter_data_2=color2 |
| `shape_splatter.sbs/shape_splatter_data_extract` · bounding_box_size (RGBA) | sampler | splatter_data_1=blend-bg-alpha-128, splatter_data_2=blend-fg-alpha-128<br>defaults | 4,096/4,096 · 0 | 4,096/4,096 · 0 | 4,096/4,096 · 0 | 3,902/4,096 · 2.98023224e-08 | 1,024/1,024 · 0<br>splatter_data_1=color1, splatter_data_2=color2 |
| `shape_splatter.sbs/shape_splatter_data_extract` · uv_shape_space (RGBA) | sampler | splatter_data_1=blend-bg-alpha-128, splatter_data_2=blend-fg-alpha-128<br>defaults | 4,096/4,096 · 0 | 4,032/4,096 · 1 | 4,032/4,096 · 7.62939453e-06 | 3,782/4,096 · 7.62939453e-06 | 1,024/1,024 · 0<br>splatter_data_1=color1, splatter_data_2=color2 |
| `shape_splatter.sbs/shape_splatter_data_extract` · pattern_id (LUM) | sampler | splatter_data_1=blend-bg-alpha-128, splatter_data_2=blend-fg-alpha-128<br>defaults | 4,096/4,096 · 0 | 4,096/4,096 · 0 | 4,096/4,096 · 0 | 4,096/4,096 · 0 | 1,024/1,024 · 0<br>splatter_data_1=color1, splatter_data_2=color2 |
| `shape_splatter.sbs/shape_splatter_data_extract` · shape_angle (LUM) | sampler | splatter_data_1=blend-bg-alpha-128, splatter_data_2=blend-fg-alpha-128<br>defaults | 4,096/4,096 · 0 | 4,032/4,096 · 1 | 4,096/4,096 · 0 | 4,032/4,096 · 7.62939453e-06 | 1,024/1,024 · 0<br>splatter_data_1=color1, splatter_data_2=color2 |
| `shape_splatter.sbs/shape_splatter_data_extract` · shape_index (RGBA) | sampler | splatter_data_1=blend-bg-alpha-128, splatter_data_2=blend-fg-alpha-128<br>defaults | 4,096/4,096 · 0 | 4,096/4,096 · 0 | 4,096/4,096 · 0 | 4,096/4,096 · 0 | 1,024/1,024 · 0<br>splatter_data_1=color1, splatter_data_2=color2 |
| `shape_splatter_v2.sbs/mipmap_tools/Mipmap_N_from_O_color` · output (RGBA) | sampler | input=blend-fg-alpha-128<br>N=2, O=1 | 3,968/4,096 · 255 | 3,968/4,096 · 65535 | 3,968/4,096 · 1 | 3,968/4,096 · 1 | 992/1,024 · 1<br>input=color1 |
| `shape_splatter_v2.sbs/shape_splatter_compute_normal` · output (RGBA) | sampler | input_0=blend-fg-alpha-128, input_1=blend-bg-alpha-128, input_2=blend-bg-alpha-128, input_3=blend-bg-alpha-128, input_4=blend-bg-alpha-128, input_5=blend-bg-alpha-128, input_6=levels-range-64, input_7=levels-range-64<br>x_amount=7, y_amount=3, position_distribution_mode=1, amount=7 | 0/4,096 · 255 | 0/4,096 · 65462 | 0/4,096 · 0.998477936 | 0/4,096 · 0.99889338 | 0/1,024 · 0.979619235<br>input_0=color1, input_1=color2, input_2=color3, input_3=color4, input_4=color5, input_5=color6, input_6=gray7, input_7=gray8 |
| `skew.sbs/skew` · Skew (RGBA) | bridge | Source=blend-fg-alpha-128<br>Axis=1, Amount=0.35, Align=0 | 4,096/4,096 · 0 | 4,096/4,096 · 0 | 4,096/4,096 · 0 | 4,096/4,096 · 0 | separate root controls |
| `skew.sbs/skew_grayscale` · Skew (LUM) | bridge | Source=offset-gray-128<br>Axis=1, Amount=0.35, Align=0 | 4,096/4,096 · 0 | 4,096/4,096 · 0 | 4,096/4,096 · 0 | 4,096/4,096 · 0 | separate root controls |
| `slope_blur.sbs/slope_blur` · Slope_Blur (RGBA) | sampler | Effect=levels-range-64, Source=blend-fg-alpha-128<br>Samples=4, Intensity=3, mode=0 | 0/4,096 · 158 | 0/4,096 · 40606 | 0/4,096 · 0.619384766 | 0/4,096 · 0.619607851 | 0/1,024 · 0.76862745<br>Effect=gray1, Source=color2 |
| `slope_blur.sbs/slope_blur_grayscale` · Slope_Blur (LUM) | sampler | Effect=levels-range-64, Source=offset-gray-128<br>Samples=4, Intensity=3, mode=0 | 63/4,096 · 58 | 63/4,096 · 14906 | 63/4,096 · 0.227539062 | 63/4,096 · 0.227450997 | 12/1,024 · 0.32941179<br>Effect=gray1, Source=gray2 |
| `slope_blur.sbs/slope_blur_grayscale_2` · Slope_Blur (LUM) | sampler | Effect=levels-range-64, Source=offset-gray-128<br>Samples=4, Intensity=3, mode=0 | 63/4,096 · 58 | 63/4,096 · 14906 | 63/4,096 · 0.227539062 | 63/4,096 · 0.227450997 | 12/1,024 · 0.32941179<br>Effect=gray1, Source=gray2 |
| `smart_auto_tile.sbs/pattern_find_edges_step` · mask_output (RGBA) | sampler | baseColor=blend-fg-alpha-128, height=levels-range-64, mask_input=blend-bg-alpha-128, normal=curved-normal-128<br>grid_resolution=5, edge_use_threshold_per_channel=true, edge_threshold=0.3 | 1,024/1,024 · 0 | 1,024/1,024 · 0 | 1,024/1,024 · 0 | 1,024/1,024 · 0 | 1,024/1,024 · 0<br>baseColor=color1, height=gray2, mask_input=color3, normal=normal |
| `splatter.sbs/splatter` · Splatter_Grayscale (LUM) | sampler | Source=offset-gray-128<br>Pattern_size_width=130, Pattern_size_height=80, Disorder=0.25, Rotation_Var=15, Rotation=30, Grid_Number=3, PatternSize=1, Pan_X=0.15, Pan_Y=-0.2, Gain=0.7 | 244/4,096 · 147 | 235/4,096 · 37779 | 235/4,096 · 0.576171875 | 235/4,096 · 0.576470613 | 87/1,024 · 0.664316773<br>Source=gray1 |
| `splatter_circular.sbs/splatter_circular` · Splatter_Circular (LUM) | sampler | Source=offset-gray-128, background=levels-range-64<br>Rotation=30, Number=7, Radius=0.35, Radius_Random=0.2, Pattern=3, PatternSize=(0.15,0.2), PatternSize_Random=0.2 | 3,968/4,096 · 221 | 3,968/4,096 · 56797 | 3,968/4,096 · 1 | 3,968/4,096 · 1.00000006 | 972/1,024 · 1.00000006<br>Source=gray1, background=gray2 |
| `splatter_circular.sbs/splatter_circular_color` · Splatter_Circular (RGBA) | sampler | Source=blend-fg-alpha-128, background=blend-bg-alpha-128<br>rotation=0.125, pattern=3, pattern_specific=0.3, radius=0.3, random_mask=0.25, number=7, radius_random=0.2, pattern_size=(0.15,0.2), pattern_size_Random=0.2 | 3,517/4,096 · 247 | 3,517/4,096 · 63479 | 3,517/4,096 · 0.96862793 | 3,517/4,096 · 0.968627449 | 872/1,024 · 1<br>Source=color1, background=color2 |
| `splatter_color.sbs/splatter_color` · Splatter (RGBA) | sampler | Source=blend-fg-alpha-128, Source_alpha=levels-range-64<br>Pattern_size_width=130, Pattern_size_height=80, Disorder=0.25, Rotation_Var=15, Rotation=30, Grid_Number=3, PatternSize=1, Pan_X=0.15, Pan_Y=-0.2, Gain=0.7 | 238/4,096 · 109 | 238/4,096 · 28089 | 238/4,096 · 0.42855835 | 238/4,096 · 0.428610656 | 97/1,024 · 0.508613691<br>Source=color1, Source_alpha=gray2 |
| `spline_tools.sbs/internal_tools/2dtransform_spline_sub` · spline_coords (RGBA) | sampler | spline_coords=original-offset-circle-spline_coords, spline_data=original-offset-circle-spline_data<br>flip_direction=true, matrix22=(0,-1,1,0), offset=(0.1,-0.2) | 4,096/4,096 · 0 | 4,096/4,096 · 0 | 4,096/4,096 · 0 | 3,988/4,096 · 5.96046448e-08 | 1,024/1,024 · 0<br>spline_coords=original-offset-circle-spline_coords, spline_data=original-offset-circle-spline_data |
| `spline_tools.sbs/internal_tools/2dtransform_spline_sub` · spline_data (RGBA) | sampler | spline_coords=original-offset-circle-spline_coords, spline_data=original-offset-circle-spline_data<br>flip_direction=true, matrix22=(0,-1,1,0), offset=(0.1,-0.2) | 4,096/4,096 · 0 | 4,096/4,096 · 0 | 4,096/4,096 · 0 | 3,967/4,096 · 5.96046448e-08 | 1,024/1,024 · 0<br>spline_coords=original-offset-circle-spline_coords, spline_data=original-offset-circle-spline_data |
| `spline_tools.sbs/internal_tools/2dtransform_spline_sub` · spline_amount (int) | sampler | spline_coords=original-offset-circle-spline_coords, spline_data=original-offset-circle-spline_data<br>flip_direction=true, matrix22=(0,-1,1,0), offset=(0.1,-0.2) | 1/1 · 0 | 1/1 · 0 | 1/1 · 0 | 1/1 · 0 | 1/1 · 0<br>spline_coords=original-offset-circle-spline_coords, spline_data=original-offset-circle-spline_data |
| `spline_tools.sbs/internal_tools/compute_bridge_distance` · output (LUM) | bridge | input=original-offset-circle-spline_coords<br>non_square_correction=false | 4,096/4,096 · 0 | 4,096/4,096 · 0 | 4,096/4,096 · 0 | 4,096/4,096 · 0 | 1,024/1,024 · 0<br>input=original-offset-circle-spline_coords |
| `spline_tools.sbs/internal_tools/compute_spline_length_init` · output (LUM) | sampler | input=original-offset-circle-spline_coords<br>non_square_correction=false | 4,096/4,096 · 0 | 4,096/4,096 · 0 | 4,096/4,096 · 0 | 3,873/4,096 · 8.56816769e-08 | 1,011/1,024 · 7.4505806e-09<br>input=original-offset-circle-spline_coords |
| `spline_tools.sbs/internal_tools/compute_spline_tangents` · spline_data (RGBA) | sampler | spline_coords=original-offset-circle-spline_coords, spline_data=original-offset-circle-spline_data<br>mode=1 | 4,096/4,096 · 0 | 4,096/4,096 · 0 | 4,096/4,096 · 0 | 3,873/4,096 · 1.78813934e-07 | 1,014/1,024 · 1.1920929e-07<br>spline_coords=original-offset-circle-spline_coords, spline_data=original-offset-circle-spline_data |
| `spline_tools.sbs/internal_tools/modify_spline_property_sub` · spline_coords (RGBA) | sampler | map=original-mask-gradient-16, spline_coords=original-offset-circle-spline_coords, spline_data=original-offset-circle-spline_data<br>sampling_mode=1, property=1, opacity=0.6, blending_mode=1 | 4,096/4,096 · 0 | 4,096/4,096 · 0 | 3,898/4,096 · 0.177734375 | 3,810/4,096 · 0.177648306 | 969/1,024 · 0.261176467<br>map=gray1, spline_coords=original-offset-circle-spline_coords, spline_data=original-offset-circle-spline_data |
| `spline_tools.sbs/internal_tools/normalize_spline_distance` · output (LUM) | sampler | input=original-mask-gradient-16<br>defaults | 4,093/4,096 · 1 | 3,126/4,096 · 3 | 3,965/4,096 · 0.0625 | 66/4,096 · 0.0169067383 | 1,024/1,024 · 0<br>input=gray1 |
| `spline_tools.sbs/internal_tools/points_preview` · output (LUM) | sampler | point_list=original-offset-circle-spline_coords<br>show_labels=false, show_points=true, points_size=0.02 | external font | external font | external font | external font | external font<br>point_list=original-offset-circle-spline_coords |
| `spline_tools.sbs/internal_tools/resample_spline_data` · spline_data (RGBA) | sampler | spline_coords=original-offset-circle-spline_coords, spline_data=original-offset-circle-spline_data, uv=original-rgba-16<br>defaults | 4,092/4,096 · 1 | 3,813/4,096 · 13 | 3,856/4,096 · 0.000244140625 | 3,446/4,096 · 0.000185694546 | 1,024/1,024 · 0<br>spline_coords=original-offset-circle-spline_coords, spline_data=original-offset-circle-spline_data, uv=color3 |
| `spline_tools.sbs/internal_tools/resample_spline_data` · spline_coords (RGBA) | sampler | spline_coords=original-offset-circle-spline_coords, spline_data=original-offset-circle-spline_data, uv=original-rgba-16<br>defaults | 3,813/4,096 · 128 | 3,707/4,096 · 32768 | 3,772/4,096 · 0.5 | 3,396/4,096 · 0.5 | 986/1,024 · 0.5<br>spline_coords=original-offset-circle-spline_coords, spline_data=original-offset-circle-spline_data, uv=color3 |
| `spline_tools.sbs/internal_tools/select_spline_sub` · output (RGBA) | sampler | data=original-offset-circle-spline_data<br>spline_index=0, output=1, selection_mode=1, range_start=1, range_end=1, start=0.2, end=0.8 | 30,720/32,768 · 128 | 30,720/32,768 · 32768 | 30,720/32,768 · 0.5 | 30,720/32,768 · 0.5 | 30,720/32,768 · 0.5<br>data=original-offset-circle-spline_data |
| `spline_tools.sbs/internal_tools/spline_warp_sub` · spline_coords (RGBA) | sampler | attenuation_curve=original-mask-gradient-16, intensity_map=original-mask-gradient-16, spline_coords=original-offset-circle-spline_coords, spline_data=original-offset-circle-spline_data, vector_map=original-rgba-16<br>warp_intensity=0.2, warp_center=0.4, sampling_mode=1, use_vector_map=true, use_attenuation_curve=true, start_attenuation=0.2, end_attenuation=0.3 | 4,032/4,096 · 33 | 4,032/4,096 · 8481 | 4,032/4,096 · 0.129150391 | 4,032/4,096 · 0.129411757 | 992/1,024 · 0.181807004<br>attenuation_curve=gray1, intensity_map=gray2, spline_coords=original-offset-circle-spline_coords, spline_data=original-offset-circle-spline_data, vector_map=color5 |
| `spot_emboss.sbs/spot_emboss` · Spot_Emboss (RGBA) | sampler | Height=levels-range-64, Source=blend-fg-alpha-128<br>Light_Position_X=0.3, Light_Position_Y=0.7, Intensity=2, Highlight_Color=(0.8,0.6,0.3,1), Shadow_Color=(0.1,0.2,0.4,1) | 20/4,096 · 183 | 20/4,096 · 47134 | 20/4,096 · 0.863037109 | 15/4,096 · 0.863529354 | 8/1,024 · 0.545882344<br>Height=gray1, Source=color2 |
| `starburst.sbs/starburst` · Starburst (LUM) | sampler | none<br>Branches=12, BlurAmount=1, Randomness=0.3, SizeMin=0.1, SizeMax=0.8, SizeVariation=0.2, Tiling=2 | 3,272/4,096 · 236 | 3,184/4,096 · 60659 | 3,184/4,096 · 0.925292969 | 3,184/4,096 · 0.92559699 | 784/1,024 · 0.776928335 |
| `stripes.sbs/stripes` · Stripes (LUM) | angle | none<br>Stripes=7, Width=0.35, Softness=0.2, Shift=6, Align=1 | 4,096/4,096 · 0 | 4,096/4,096 · 0 | 4,096/4,096 · 0 | 4,096/4,096 · 0 | 1,024/1,024 · 0 |
| `swirl.sbs/swirl` · output (RGBA) | sampler | input=blend-fg-alpha-128<br>matrix22=(1.2,0.15,-0.2,0.9), offset=(0.125,-0.25), amount=3 | 4,094/4,096 · 1 | 4,094/4,096 · 63 | 4,094/4,096 · 0.0009765625 | 4,094/4,096 · 0.000961303711 | 1,024/1,024 · 0<br>input=color1 |
| `swirl.sbs/swirl_grayscale` · output (LUM) | sampler | input=offset-gray-128<br>matrix22=(1.2,0.15,-0.2,0.9), offset=(0.125,-0.25), amount=3 | 4,094/4,096 · 1 | 4,094/4,096 · 63 | 4,094/4,096 · 0.0009765625 | 4,094/4,096 · 0.000961303711 | 1,024/1,024 · 0<br>input=gray1 |
| `symmetry_slice.sbs/symmetry_slice` · Symmetry_Slice (RGBA) | sampler | Source=blend-fg-alpha-128<br>Symmetry_Mode=2, Transfer_Mode=1, Blend=0.3, Flip_Side=true, Flip_Side2=true | 2,916/4,096 · 7 | 2,859/4,096 · 1632 | 2,833/4,096 · 0.0249023438 | 1,485/4,096 · 0.0248995423 | 442/1,024 · 1.1920929e-07<br>Source=color1 |
| `tile_generator.sbs/tile_generator` · TileGenerator (LUM) | sampler | Source=offset-gray-128, background=levels-range-64<br>Nb_X=7, Nb_Y=3, Scale=0.8, ScaleVariation=0.2, Offset=0.5 | 3,586/4,096 · 69 | 3,585/4,096 · 17827 | 3,302/4,096 · 0.271850586 | 3,302/4,096 · 0.272031903 | 831/1,024 · 0.276015699<br>Source=gray1, background=gray2 |
| `tile_generator_color.sbs/tile_generator_color` · TileGenerator_Color (RGBA) | sampler | Source=blend-fg-alpha-128, background=blend-bg-alpha-128<br>Nb_X=7, Nb_Y=3, Scale=0.8, ScaleVariation=0.2, Offset=0.5 | 3,344/4,096 · 80 | 3,302/4,096 · 20446 | 1,934/4,096 · 0.963867188 | 1,934/4,096 · 0.964705944 | 494/1,024 · 1<br>Source=color1, background=color2 |
| `tile_random.sbs/tile_random` · output (LUM) | sampler | background_input=levels-range-64, pattern_input=offset-gray-128<br>x_amount=7, y_amount=3, pattern=3, pattern_rotation_random=0.3 | 2,725/4,096 · 223 | 2,725/4,096 · 57311 | 2,725/4,096 · 1 | 2,725/4,096 · 1.00000006 | 723/1,024 · 1.00000006<br>background_input=gray1, pattern_input=gray2 |
| `tile_random.sbs/tile_random_color` · output (RGBA) | sampler | background_input_1=blend-fg-alpha-128, pattern_input=blend-bg-alpha-128<br>x_amount=7, y_amount=3, pattern=3, pattern_rotation_random=0.3 | 2,725/4,096 · 250 | 2,725/4,096 · 64250 | 511/4,096 · 1 | 511/4,096 · 1.00000006 | 150/1,024 · 1.00000006<br>background_input_1=color1, pattern_input=color2 |
| `tile_sampler.sbs/arrow` · output (LUM) | filtered | none<br>defaults | 262,144/262,144 · 0 | 262,144/262,144 · 0 | 262,144/262,144 · 0 | 262,144/262,144 · 0 | separate root controls |
| `tileable_material_to_mesh_material.sbs/tileable_material_to_mesh_material` · Specular (RGBA) | sampler | AO=levels-range-64, Curvature=levels-range-64, Diffuse=blend-bg-alpha-128, Normal_WS=curved-normal-128, Specular=blend-bg-alpha-128<br>Pre_Lighting_Intensity=0.3, AO_Intensity=0.5, Edges_Sharpness_Intensity=0.2, AO_Intensity_Specular=0.4 | 4,096/4,096 · 0 | 4,090/4,096 · 1 | 4,096/4,096 · 0 | 1,331/4,096 · 5.96046448e-08 | 363/1,024 · 5.96046448e-08<br>AO=gray1, Curvature=gray2, Diffuse=color3, Normal_WS=normal, Specular=color5 |
| `tileable_material_to_mesh_material.sbs/tileable_material_to_mesh_material` · Diffuse (RGBA) | sampler | AO=levels-range-64, Curvature=levels-range-64, Diffuse=blend-bg-alpha-128, Normal_WS=curved-normal-128, Specular=blend-bg-alpha-128<br>Pre_Lighting_Intensity=0.3, AO_Intensity=0.5, Edges_Sharpness_Intensity=0.2, AO_Intensity_Specular=0.4 | 3,843/4,096 · 1 | 2,849/4,096 · 103 | 2,861/4,096 · 0.00170898438 | 1,262/4,096 · 0.00156864524 | 322/1,024 · 0.00156867504<br>AO=gray1, Curvature=gray2, Diffuse=color3, Normal_WS=normal, Specular=color5 |
| `trapezoid_transform.sbs/trapezoid_transform` · output (RGBA) | sampler | input=blend-fg-alpha-128<br>top_stretch=0.3, bottom_stretch=-0.2 | 4,096/4,096 · 0 | 4,096/4,096 · 0 | 4,096/4,096 · 0 | 4,096/4,096 · 0 | 1,024/1,024 · 0<br>input=color1 |
| `trapezoid_transform.sbs/trapezoid_transform_grayscale` · output (LUM) | sampler | input=offset-gray-128<br>top_stretch=0.3, bottom_stretch=-0.2 | 4,096/4,096 · 0 | 4,096/4,096 · 0 | 4,096/4,096 · 0 | 4,096/4,096 · 0 | 1,024/1,024 · 0<br>input=gray1 |
| `uv_mapper.sbs/uv_mapper_color_sub` · output (RGBA) | bridge | input=blend-bg-alpha-128, uv=position-128<br>filtering_mode=0 | 4,096/4,096 · 0 | 4,096/4,096 · 0 | 4,096/4,096 · 0 | 4,096/4,096 · 0 | separate root controls |
| `uv_mapper.sbs/uv_mapper_grayscale_sub` · output (LUM) | bridge | input=levels-range-64, uv=position-128<br>filtering_mode=0 | 4,096/4,096 · 0 | 4,096/4,096 · 0 | 4,096/4,096 · 0 | 4,096/4,096 · 0 | separate root controls |
| `vector_morph.sbs/vector_morph` · output (RGBA) | sampler | input=blend-fg-alpha-128, vector_field=blend-bg-alpha-128<br>amount=3 | 3,835/4,096 · 221 | 3,801/4,096 · 56909 | 3,801/4,096 · 0.868286133 | 3,801/4,096 · 0.868375674 | 1,024/1,024 · 0<br>input=color1, vector_field=color2 |
| `vector_morph.sbs/vector_morph_grayscale` · output (LUM) | sampler | input=levels-range-64, vector_field=blend-fg-alpha-128<br>amount=3 | 4,091/4,096 · 127 | 4,091/4,096 · 32611 | 4,091/4,096 · 0.497558594 | 4,091/4,096 · 0.497611985 | 1,024/1,024 · 0<br>input=gray1, vector_field=color2 |
| `vertical_noise.sbs/vertical_noise` · Vertical_Noise (LUM) | sampler | none<br>Disorder=0.3 | 70/4,096 · 158 | 1/4,096 · 40470 | 3/4,096 · 0.617553711 | 0/4,096 · 0.617534697 | 0/1,024 · 0.488726646 |
| `waveform_01_v2.sbs/waveform_01_v2` · Waveform (LUM) | sampler | none<br>pattern=3, samples=128, function=1, roughness=0.7, scale=16, amplitude_min=0.1, amplitude_max=0.8, noise=0.25, disorder=0.3, disorder_speed=0.6 | 4,096/4,096 · 0 | 4,096/4,096 · 0 | 4,096/4,096 · 0 | 4,096/4,096 · 0 | 1,024/1,024 · 0 |
| `waveform_1.sbs/waveform_1` · Waveform (LUM) | sampler | none<br>SizeMin=0.1, SizeMax=0.8, Samples=128, WaveNumber=3, Noise=0.2, Pattern=3 | 4,096/4,096 · 0 | 4,096/4,096 · 0 | 4,096/4,096 · 0 | 4,096/4,096 · 0 | 1,024/1,024 · 0 |
| `weave_1.sbs/weave_1` · Weave (LUM) | sampler | none<br>Tiling=2, Rotate_45_Degrees=true | 0/4,096 · 255 | 0/4,096 · 65535 | 0/4,096 · 1 | 0/4,096 · 1 | 0/1,024 · 1 |
| `weave_2.sbs/weave_2` · Weave_2 (LUM) | sampler | none<br>Tiling=2, Disorder=0.3, Rotate_45_Degrees=true | 0/4,096 · 160 | 0/4,096 · 41142 | 0/4,096 · 0.627441406 | 0/4,096 · 0.627786696 | 10/1,024 · 0.243762881 |
| `weave_3.sbs/weave_3` · Weave_3 (LUM) | bridge | none<br>Tiling=2, Rotation=true | 4,096/4,096 · 0 | 4,096/4,096 · 0 | 4,096/4,096 · 0 | 4,096/4,096 · 0 | separate root controls |
| `weave_4.sbs/weave_4` · Weave_4 (LUM) | sampler | none<br>Tiling=2, Rotation=true | 4,096/4,096 · 0 | 4,096/4,096 · 0 | 4,096/4,096 · 0 | 4,096/4,096 · 0 | 1,024/1,024 · 0 |
| `white_noise.sbs/white_noise_fast` · output (LUM) | sampler | none<br>defaults | 14/4,096 · 253 | 0/4,096 · 65120 | 1/4,096 · 0.993654251 | 0/4,096 · 0.99366051 | 0/1,024 · 0.952913761 |
| `white_noise_v2.sbs/white_noise_v2` · output (LUM) | sampler | none<br>noise_distribution=1, disorder=0.3, disorder_speed=0.6 | 37/4,096 · 169 | 1/4,096 · 43567 | 7/4,096 · 0.671630859 | 0/4,096 · 0.671823263 | 0/1,024 · 0.592708588 |
| `wood_fibers_1.sbs/wood_fibers_1` · Wood_Fibers (LUM) | sampler | none<br>Disorder=0.3 | 1/4,096 · 251 | 1/4,096 · 64474 | 1/4,096 · 0.983825684 | 1/4,096 · 0.98381361 | 0/1,024 · 0.980358532 |
| `wood_fibers_2.sbs/wood_fibers_2` · Wood_Noise (LUM) | sampler | none<br>defaults | execution budget | execution budget | execution budget | execution budget | execution budget |

#### Wide input groups

| Group | Bound inputs at 64² | Bound inputs at 32² |
|---:|---|---|
| 1 | blue_noise=irradiance-blue_noise, environment=irradiance-environment, normal=irradiance-normal, position=irradiance-position, sample_table=irradiance-samples-polar192, sdf=irradiance-sdf-hdr | blue_noise=irradiance-blue_noise, environment=irradiance-environment, normal=irradiance-normal, position=irradiance-position, sample_table=irradiance-samples-polar192, sdf=irradiance-sdf-hdr |
| 2 | density=volume-density-thin, importance_sampling=irradiance-samples-polar64, input=irradiance-environment, noise=irradiance-blue_noise, position=irradiance-position, sdf=sdf-negative-16 | density=volume-density-thin, importance_sampling=irradiance-samples-polar64, input=irradiance-environment, noise=irradiance-blue_noise, position=irradiance-position, sdf=sdf-negative-16 |
| 3 | Material_1_Diffuse_Black=material-input-1-128, Material_1_Height_Black=material-input-5-128, Material_1_Specular_Black=material-input-3-128, Material_2_Diffuse_White=material-input-2-128, Material_2_Height_White=material-input-6-128, Material_2_Specular_White=material-input-4-128, SVG_Mask=material-id-128 | Material_1_Diffuse_Black=color1, Material_1_Height_Black=color2, Material_1_Specular_Black=color3, Material_2_Diffuse_White=color4, Material_2_Height_White=color5, Material_2_Specular_White=color6, SVG_Mask=material-id-128 |
| 4 | Material_1_Diffuse_Black=material-input-1-128, Material_1_Height_Black=material-input-5-128, Material_1_Specular_Black=material-input-3-128, Material_2_Diffuse_White=material-input-2-128, Material_2_Height_White=material-input-6-128, Material_2_Specular_White=material-input-4-128, SVG_Mask=material-id-128 | Material_1_Diffuse_Black=color1, Material_1_Height_Black=color2, Material_1_Specular_Black=color3, Material_2_Diffuse_White=color4, Material_2_Height_White=color5, Material_2_Specular_White=color6, SVG_Mask=material-id-128 |
| 5 | Material_1_Diffuse_Black=material-input-1-128, Material_1_Height_Black=material-input-5-128, Material_1_Specular_Black=material-input-3-128, Material_2_Diffuse_White=material-input-2-128, Material_2_Height_White=material-input-6-128, Material_2_Specular_White=material-input-4-128, SVG_Mask=material-id-128 | Material_1_Diffuse_Black=color1, Material_1_Height_Black=color2, Material_1_Specular_Black=color3, Material_2_Diffuse_White=color4, Material_2_Height_White=color5, Material_2_Specular_White=color6, SVG_Mask=material-id-128 |
| 6 | Hue_Shift_SVG_Mask=material-id-128, Material_1_Diffuse_Black=material-input-1-128, Material_1_Height_Black=material-input-9-128, Material_1_Specular_Black=material-input-6-128, Material_2_Diffuse_Red=material-input-2-128, Material_2_Height_Red=material-input-10-128, Material_2_Specular_Red=material-input-8-128, Material_3_Diffuse_Green=material-input-3-128, Material_3_Height_Green=material-input-11-128, Material_3_Specular_Green=material-input-5-128, Material_4_Diffuse_Blue=material-input-4-128, Material_4_Height_Blue=material-input-12-128, Material_4_Specular_Blue=material-input-7-128 | Hue_Shift_SVG_Mask=material-id-128, Material_1_Diffuse_Black=color2, Material_1_Height_Black=color3, Material_1_Specular_Black=color4, Material_2_Diffuse_Red=color5, Material_2_Height_Red=color6, Material_2_Specular_Red=color7, Material_3_Diffuse_Green=color8, Material_3_Height_Green=color9, Material_3_Specular_Green=color10, Material_4_Diffuse_Blue=color11, Material_4_Height_Blue=color12, Material_4_Specular_Blue=color13 |
| 7 | Hue_Shift_SVG_Mask=material-id-128, Material_1_Diffuse_Black=material-input-1-128, Material_1_Height_Black=material-input-9-128, Material_1_Specular_Black=material-input-6-128, Material_2_Diffuse_Red=material-input-2-128, Material_2_Height_Red=material-input-10-128, Material_2_Specular_Red=material-input-8-128, Material_3_Diffuse_Green=material-input-3-128, Material_3_Height_Green=material-input-11-128, Material_3_Specular_Green=material-input-5-128, Material_4_Diffuse_Blue=material-input-4-128, Material_4_Height_Blue=material-input-12-128, Material_4_Specular_Blue=material-input-7-128 | Hue_Shift_SVG_Mask=material-id-128, Material_1_Diffuse_Black=color2, Material_1_Height_Black=color3, Material_1_Specular_Black=color4, Material_2_Diffuse_Red=color5, Material_2_Height_Red=color6, Material_2_Specular_Red=color7, Material_3_Diffuse_Green=color8, Material_3_Height_Green=color9, Material_3_Specular_Green=color10, Material_4_Diffuse_Blue=color11, Material_4_Height_Blue=color12, Material_4_Specular_Blue=color13 |
| 8 | Hue_Shift_SVG_Mask=material-id-128, Material_1_Diffuse_Black=material-input-1-128, Material_1_Height_Black=material-input-9-128, Material_1_Specular_Black=material-input-6-128, Material_2_Diffuse_Red=material-input-2-128, Material_2_Height_Red=material-input-10-128, Material_2_Specular_Red=material-input-8-128, Material_3_Diffuse_Green=material-input-3-128, Material_3_Height_Green=material-input-11-128, Material_3_Specular_Green=material-input-5-128, Material_4_Diffuse_Blue=material-input-4-128, Material_4_Height_Blue=material-input-12-128, Material_4_Specular_Blue=material-input-7-128 | Hue_Shift_SVG_Mask=material-id-128, Material_1_Diffuse_Black=color2, Material_1_Height_Black=color3, Material_1_Specular_Black=color4, Material_2_Diffuse_Red=color5, Material_2_Height_Red=color6, Material_2_Specular_Red=color7, Material_3_Diffuse_Green=color8, Material_3_Height_Green=color9, Material_3_Specular_Green=color10, Material_4_Diffuse_Blue=color11, Material_4_Height_Blue=color12, Material_4_Specular_Blue=color13 |
| 9 | Hue_Shift_SVG_Mask=material-id-128, Material_1_Diffuse_Black=material-input-1-128, Material_1_Height_Black=material-input-15-128, Material_1_Specular_Black=material-input-11-128, Material_2_Diffuse_Red=material-input-2-128, Material_2_Height_Red=material-input-16-128, Material_2_Specular_Red=material-input-13-128, Material_3_Diffuse_Green=material-input-3-128, Material_3_Height_Green=material-input-17-128, Material_3_Specular_Green=material-input-8-128, Material_4_Diffuse_Blue=material-input-4-128, Material_4_Height_Blue=material-input-18-128, Material_4_Specular_Blue=material-input-12-128, Material_5_Diffuse_Yellow=material-input-5-128, Material_5_Height_Yellow=material-input-19-128, Material_5_Specular_Yellow=material-input-9-128, Material_6_Diffuse_Magenta=material-input-23-128, Material_6_Height_Magenta=material-input-24-128, Material_6_Specular_Magenta=material-input-25-128, Material_7_Diffuse_Cyan=material-input-6-128, Material_7_Height_Cyan=material-input-20-128, Material_7_Specular_Cyan=material-input-14-128, Material_8_Diffuse_White=material-input-7-128, Material_8_Height_White=material-input-21-128, Material_8_Specular_White=material-input-10-128 | Hue_Shift_SVG_Mask=material-id-128, Material_1_Diffuse_Black=color2, Material_1_Height_Black=color3, Material_1_Specular_Black=color4, Material_2_Diffuse_Red=color5, Material_2_Height_Red=color6, Material_2_Specular_Red=color7, Material_3_Diffuse_Green=color8, Material_3_Height_Green=color9, Material_3_Specular_Green=color10, Material_4_Diffuse_Blue=color11, Material_4_Height_Blue=color12, Material_4_Specular_Blue=color13, Material_5_Diffuse_Yellow=color14, Material_5_Height_Yellow=color15, Material_5_Specular_Yellow=color16, Material_6_Diffuse_Magenta=color17, Material_6_Height_Magenta=color18, Material_6_Specular_Magenta=color19, Material_7_Diffuse_Cyan=color20, Material_7_Height_Cyan=color21, Material_7_Specular_Cyan=color22, Material_8_Diffuse_White=color23, Material_8_Height_White=color24, Material_8_Specular_White=color25 |
| 10 | Hue_Shift_SVG_Mask=material-id-128, Material_1_Diffuse_Black=material-input-1-128, Material_1_Height_Black=material-input-15-128, Material_1_Specular_Black=material-input-11-128, Material_2_Diffuse_Red=material-input-2-128, Material_2_Height_Red=material-input-16-128, Material_2_Specular_Red=material-input-13-128, Material_3_Diffuse_Green=material-input-3-128, Material_3_Height_Green=material-input-17-128, Material_3_Specular_Green=material-input-8-128, Material_4_Diffuse_Blue=material-input-4-128, Material_4_Height_Blue=material-input-18-128, Material_4_Specular_Blue=material-input-12-128, Material_5_Diffuse_Yellow=material-input-5-128, Material_5_Height_Yellow=material-input-19-128, Material_5_Specular_Yellow=material-input-9-128, Material_6_Diffuse_Magenta=material-input-23-128, Material_6_Height_Magenta=material-input-24-128, Material_6_Specular_Magenta=material-input-25-128, Material_7_Diffuse_Cyan=material-input-6-128, Material_7_Height_Cyan=material-input-20-128, Material_7_Specular_Cyan=material-input-14-128, Material_8_Diffuse_White=material-input-7-128, Material_8_Height_White=material-input-21-128, Material_8_Specular_White=material-input-10-128 | Hue_Shift_SVG_Mask=material-id-128, Material_1_Diffuse_Black=color2, Material_1_Height_Black=color3, Material_1_Specular_Black=color4, Material_2_Diffuse_Red=color5, Material_2_Height_Red=color6, Material_2_Specular_Red=color7, Material_3_Diffuse_Green=color8, Material_3_Height_Green=color9, Material_3_Specular_Green=color10, Material_4_Diffuse_Blue=color11, Material_4_Height_Blue=color12, Material_4_Specular_Blue=color13, Material_5_Diffuse_Yellow=color14, Material_5_Height_Yellow=color15, Material_5_Specular_Yellow=color16, Material_6_Diffuse_Magenta=color17, Material_6_Height_Magenta=color18, Material_6_Specular_Magenta=color19, Material_7_Diffuse_Cyan=color20, Material_7_Height_Cyan=color21, Material_7_Specular_Cyan=color22, Material_8_Diffuse_White=color23, Material_8_Height_White=color24, Material_8_Specular_White=color25 |
| 11 | Hue_Shift_SVG_Mask=material-id-128, Material_1_Diffuse_Black=material-input-1-128, Material_1_Height_Black=material-input-15-128, Material_1_Specular_Black=material-input-11-128, Material_2_Diffuse_Red=material-input-2-128, Material_2_Height_Red=material-input-16-128, Material_2_Specular_Red=material-input-13-128, Material_3_Diffuse_Green=material-input-3-128, Material_3_Height_Green=material-input-17-128, Material_3_Specular_Green=material-input-8-128, Material_4_Diffuse_Blue=material-input-4-128, Material_4_Height_Blue=material-input-18-128, Material_4_Specular_Blue=material-input-12-128, Material_5_Diffuse_Yellow=material-input-5-128, Material_5_Height_Yellow=material-input-19-128, Material_5_Specular_Yellow=material-input-9-128, Material_6_Diffuse_Magenta=material-input-23-128, Material_6_Height_Magenta=material-input-24-128, Material_6_Specular_Magenta=material-input-25-128, Material_7_Diffuse_Cyan=material-input-6-128, Material_7_Height_Cyan=material-input-20-128, Material_7_Specular_Cyan=material-input-14-128, Material_8_Diffuse_White=material-input-7-128, Material_8_Height_White=material-input-21-128, Material_8_Specular_White=material-input-10-128 | Hue_Shift_SVG_Mask=material-id-128, Material_1_Diffuse_Black=color2, Material_1_Height_Black=color3, Material_1_Specular_Black=color4, Material_2_Diffuse_Red=color5, Material_2_Height_Red=color6, Material_2_Specular_Red=color7, Material_3_Diffuse_Green=color8, Material_3_Height_Green=color9, Material_3_Specular_Green=color10, Material_4_Diffuse_Blue=color11, Material_4_Height_Blue=color12, Material_4_Specular_Blue=color13, Material_5_Diffuse_Yellow=color14, Material_5_Height_Yellow=color15, Material_5_Specular_Yellow=color16, Material_6_Diffuse_Magenta=color17, Material_6_Height_Magenta=color18, Material_6_Specular_Magenta=color19, Material_7_Diffuse_Cyan=color20, Material_7_Height_Cyan=color21, Material_7_Specular_Cyan=color22, Material_8_Diffuse_White=color23, Material_8_Height_White=color24, Material_8_Specular_White=color25 |
| 12 | background_color=levels-range-64, color_input=levels-range-64, pattern_1=levels-range-64, pattern_2=levels-range-64, pattern_3=levels-range-64, pattern_4=levels-range-64, pattern_5=levels-range-64, pattern_6=levels-range-64, pattern_7=levels-range-64, pattern_8=levels-range-64, splatter_data_1=blend-fg-alpha-128, splatter_data_2=blend-bg-alpha-128 | background_color=gray1, color_input=gray2, pattern_1=gray3, pattern_2=gray4, pattern_3=gray5, pattern_4=gray6, pattern_5=gray7, pattern_6=gray8, pattern_7=gray9, pattern_8=gray10, splatter_data_1=color11, splatter_data_2=color12 |
| 13 | background_color=blend-bg-alpha-128, color_input=blend-bg-alpha-128, pattern_1=blend-bg-alpha-128, pattern_2=blend-bg-alpha-128, pattern_3=blend-bg-alpha-128, pattern_4=blend-bg-alpha-128, pattern_5=blend-bg-alpha-128, pattern_6=blend-bg-alpha-128, pattern_7=blend-bg-alpha-128, pattern_8=blend-bg-alpha-128, splatter_data_1=blend-fg-alpha-128, splatter_data_2=blend-bg-alpha-128 | background_color=color1, color_input=color2, pattern_1=color3, pattern_2=color4, pattern_3=color5, pattern_4=color6, pattern_5=color7, pattern_6=color8, pattern_7=color9, pattern_8=color10, splatter_data_1=color11, splatter_data_2=color12 |

| Original procedural file | SHA-256 of recorded input |
|---|---|
| `irradiance-blue_noise.png` | `a343e22b62b454354fbdbb8a42a533ed58de615730c13a08b35b200699974742` |
| `original-paths-seven.exr` | `9eca99f28823ccd6a9d2008d348bac6bacbe074f6074629779dca8ff31bae6a2` |
| `original-facets-gradient.exr` | `4b7f8cb83b6fee5e95e0adea357b8fb75b0c0c4a99b7133db75b37005e9d4c47` |
| `original-offset-circle-spline_coords.exr` | `a0a299005e150a5d0b9a0880fcd71fb70b12f10a52a793a56d239b829d657457` |
| `original-offset-circle-spline_data.exr` | `6b246ba71fb60fe87fee053da0faffb6a5aa5db99338f7fd9d6cd8751b4381de` |

## 2026-11-17 root comparisons

The clean-room comparisons below use the 16.0.7 reference app with the Direct3D 11 backend and a release MaterialCraft CPU cook. They compare the raw output bridge at its stored precision. A pixel counts only when every channel agrees. Integer errors are in stored levels. Floating errors are absolute. Each case starts from the graph defaults. Where a row selects another storage format, set the final processor's format to absolute in both apps before cooking.

### 3D linear gradient

Bind a linear RGBA8 position map at the output size `N`. At zero-based texel `(x,y)`, set R and G to the nearest integers to `255*x/(N-1)` and `255*y/(N-1)`, with ties to even. Set B to 128 and A to 255. Use the default UV points, or set `points_position_mode=1` for world mode. The oblique case sets the world points to `(.2,.1,.3)` and `(.8,.9,.6)` and contrast to .35. The float output includes values outside the unit interval.

| Case | Size | Seed | Format | Exact pixels / total | Maximum error |
|---|---:|---:|---|---:|---:|
| Default UV | 256² / 512² | 0 | LUM32F | 65,536/65,536 and 262,144/262,144 | 0 |
| Default world points | 256² / 512² | 0 | LUM32F | 65,536/65,536 and 262,144/262,144 | 0 |
| Default UV | 128² | 42 | LUM32F | 16,384/16,384 | 0 |
| UV, contrast .6 | 256² / 512² | 0 | LUM32F | 26,043/65,536 and 104,172/262,144 | 2.384186e-7 |
| Oblique | 256² / 512² | 0 | LUM32F | 38,634/65,536 and 154,536/262,144 | 1.192093e-7 |
| Oblique | 128² | 42 | LUM8 | 16,384/16,384 | 0 |
| Oblique | 128² | 42 | LUM16 | 16,379/16,384 | 1 |
| Oblique | 128² | 42 | LUM16F | 16,367/16,384 | 2.44140625e-4 |

At 256² the two default cases previously matched 46,763 and 32,755 float pixels. A separate pair of 128² probes for each of float2, float3 and float4 isolated the dot product. Use a varying signed RGBA32F input and dot its leading lanes with themselves or with the corresponding prefix of `(.2126,.7152,.0722,-.39)`. The measured accumulation starts with the rounded Y product and fuses X, then Z and W when present. All 98,304 sampled dot and squared-length results now agree exactly. Literal-only dot, division and interpolation expressions retain their separately measured rounding.

For the signed test image, number RGBA components in row-major order starting at zero. Compute `v = i*747796405 + 2891336453`, then `v = ((v >> ((v >> 28)+4)) ^ v)*277803737` with unsigned 32-bit wrap at both steps. Store `8*((v ^ (v >> 22))/2^32)-4` as float32. Interpolating R to G with B as the factor and dividing R by the float32 value 1.7320508 each match 16,384/16,384 pixels with zero error. With G as the divisor, 14,372/16,384 match and the maximum error is 4.8828125e-4. Multiplying R by a separately captured reference reciprocal of G reproduces all 16,384 reference quotients. A standalone `R*G-B` probe matches 11,721/16,384 with a maximum error of 1.907349e-6. Fusing that product and subtraction reproduces every reference pixel. At 256² the contrast-.6 reference output is exactly a fused `gradient*2.5-.75` on the zero-contrast reference image. Separate graph operations and reciprocal approximation remain open. The unconnected Position case from the first pass also remains open.

### Basic and ridged 3D Perlin

Use the same position ramp. For a controlled comparison, bind an original 1024×16 grayscale lookup to the internal bitmap in both apps. With one Python `random.Random(104729)` instance, shuffle a fresh list of integers 0 through 1023 for each successive row. Write each entry as the nearest integer to `65535*entry/1023` in a linear 16-bit PNG. Both internal lookup outputs matched all 16,384 levels with zero error. The changed parameter is `scale=6` for basic Perlin and `roughness=.7` for ridged Perlin. Use an 8-bit parent format.

| Case | Size | Seed | Format | Basic exact / total | Basic max | Ridged exact / total | Ridged max |
|---|---:|---:|---|---:|---:|---:|---:|
| Default | 128² | 0 | LUM8 | 16,383/16,384 | 1 | 16,384/16,384 | 0 |
| Default | 256² | 0 | LUM8 | 65,534/65,536 | 1 | 65,536/65,536 | 0 |
| Default | 128² | 42 | LUM8 | 16,384/16,384 | 0 | 16,384/16,384 | 0 |
| Changed | 128² | 0 | LUM8 | 16,384/16,384 | 0 | 16,384/16,384 | 0 |
| Changed | 128² | 42 | LUM16 | 16,360/16,384 | 1 | 16,304/16,384 | 1 |
| Changed | 128² | 42 | LUM16F | 16,384/16,384 | 0 | 16,378/16,384 | 4.8828125e-4 |
| Changed | 128² | 42 | LUM32F | 12,105/16,384 | 5.066395e-7 | 3,119/16,384 | 4.172326e-7 |

The changed float case previously matched 10,399 pixels for basic Perlin and 3,926 for ridged Perlin. Whole-graph rounding remains visible in both. A separate seed-zero comparison used each app's shipped lookup, with MaterialCraft's tracked CC0 image read from Git. Basic Perlin matched 173/16,384 at 128² and 662/65,536 at 256², with maximum errors of 203 levels at both sizes. Ridged Perlin matched 176/16,384 and 768/65,536, with maximum errors of 139 and 154 levels. The shared lookup isolates an asset-dependent difference from the arithmetic.

### Grayscale and color 3D planar projection

Bind the position ramp above and a constant world-space normal of RGBA8 `(128,128,255,255)`. The grayscale texture is `T=(17*x+31*y) mod 256`. The color texture is RGBA8 `(T,R,255-G,(251+3*x+5*y) mod 256)`, where R and G are the position ramp bytes. Keep all input maps linear and the same size as the graph. The rotated case sets `rotation=.125` and `scale=.38`. Each format pair below gives the grayscale and color output formats respectively.

| Case | Size | Seed | Formats | Grayscale exact / total | Gray max | Color exact / total | Color max |
|---|---:|---:|---|---:|---:|---:|---:|
| Default | 128² | 0 | LUM16, RGBA8 | 16,384/16,384 | 0 | 16,384/16,384 | 0 |
| Default | 256² | 0 | LUM16, RGBA8 | 65,536/65,536 | 0 | 65,536/65,536 | 0 |
| Default | 128² | 42 | LUM16, RGBA8 | 16,384/16,384 | 0 | 16,384/16,384 | 0 |
| Rotated | 128² | 0 | LUM16, RGBA8 | 16,334/16,384 | 226 | 16,378/16,384 | 1 |
| Rotated | 64² | 42 | LUM8, RGBA8 | 4,094/4,096 | 1 | 4,094/4,096 | 1 |
| Rotated | 64² | 42 | LUM16, RGBA16 | 4,093/4,096 | 32 | 4,093/4,096 | 32 |
| Rotated | 64² | 42 | LUM16F, RGBA16F | 4,093/4,096 | 4.8828125e-4 | 4,093/4,096 | 4.8828125e-4 |
| Rotated | 64² | 42 | LUM32F, RGBA32F | 4,093/4,096 | 4.882888e-4 | 4,093/4,096 | 4.882888e-4 |

Before the sampler change the default 256² cases matched 65,048 grayscale pixels and 65,492 RGBA pixels. An independent one-processor graph isolates the integer texture read. In a 256² source, use the 2×2 block starting at (12,15). Light one corner at a time, then repeat with corner levels `(.37,.13,.91,.68)` rounded to the input precision, in row-major order. For output pixel (x,y), sample at `((12.5+x/256)/256,(15.5+y/256)/256)`. Run all five patterns with RGBA8 and RGBA16 input and read the red channel into LUM32F. All 655,360 pixels match exactly.

In units of 1/256 texel, the rounded horizontal and vertical fractions are `dx` and `dy`. The bottom-right weight is `q=floor((dx*dy+128)/256)`. The other three weights in row-major order are `256-dx-dy+q`, `dx-q` and `dy-q`. Accumulate the four normalized 16-bit levels with these integer weights, divide by 256 and round half up. A separate 128² high-frequency texture sweep at seven fixed offsets matched all 114,688 pixels, including wrapped edges. The rotated graph cases still differ with this sampler. Floating-source filtering has separate rounding behavior and remains open.

### 3D surface and volume render passes

The four rows of the inventory preceding `position_debug` share one package. The small output side `N` is 16 for irradiance, volume IBL and volume light, and 32 for surface position. The large case doubles both dimensions. The surface position parent is 8-bit. The other parents are float32. At the small and large sizes, generate new input ramps with byte values `U=floor(255*(x+.5)/N)` and `V=floor(255*(y+.5)/N)`. Bind all images as linear.

- Surface irradiance uses a constant float SDF of 128, position `(U,V,128,255)`, normal `(128,128,255,255)`, sample table `(64,64,255,255)`, environment `(U,V,192,255)` and grayscale noise `(17*x+31*y) mod 256`. The changed case sets `i=16` and albedo to `(.3,.6,.9)`.
- Surface position uses a constant white SDF. The changed case enables the ground plane and sets camera UV to `(.35,.6)` and field of view to 65 degrees.
- The volume passes use constant float SDF −.1 and density .001, the same position ramp, and the noise formula evaluated at the noise input's native 256². IBL also uses the irradiance sample table and environment. Its changed case sets `i=1` and absorption to 2. Volume light sets light intensity to 2 and light UV to `(.3,.6)` for its changed case.

Each cell gives **exact pixels / total (maximum error)**. The native output formats in column order are RGBA32F, RGBA8, RGBA16F and RGBA16F.

| Case | Surface irradiance | Surface position | Volume IBL | Volume light |
|---|---:|---:|---:|---:|
| Small, defaults, seed 0 | 0/256 (8.940697e-8) | 1,024/1,024 (0) | 9/256 (.00634765625) | 250/256 (6.103516e-5) |
| Large, defaults, seed 0 | 0/1,024 (7.450581e-8) | 4,096/4,096 (0) | 44/1,024 (.00634765625) | 994/1,024 (.0001220703125) |
| Small, defaults, seed 42 | 0/256 (8.940697e-8) | 1,024/1,024 (0) | 8/256 (.00537109375) | 250/256 (6.103516e-5) |
| Small, changed, seed 0 | 0/256 (8.940697e-8) | 1,023/1,024 (1) | 9/256 (.009765625) | 250/256 (6.103516e-5) |
| Small, changed, seed 42, RGBA8 | 256/256 (0) | 1,023/1,024 (1) | 69/256 (3) | 255/256 (1) |
| Small, changed, seed 42, RGBA16 | 256/256 (0) | 1,023/1,024 (246) | 0/256 (701) | 250/256 (2) |
| Small, changed, seed 42, RGBA16F | 256/256 (0) | 410/1,024 (4) | 3/256 (.0107421875) | 250/256 (6.103516e-5) |
| Small, changed, seed 42, RGBA32F | 0/256 (8.940697e-8) | 0/1,024 (.67333984375) | 0/256 (.010691524) | 0/256 (4.068762e-5) |

**Irradiance.** Replace noise by zero, environment by `(64,128,192,255)` and position by `(128,128,128,255)`. Both renders become spatially constant and all six input bridges match. RGB still differs by at most 8.940697e-8 and alpha is exact. Changing only the sample table from byte `(64,64,255,255)` to float `(.25,.25,1,1)` makes all 256 RGBA32F pixels exact. Replacing only the normal by `(.5,.5,1,1)` gives 0/256 and maximum error 1.192093e-7. The remaining difference depends on the sampled direction even with flat lighting.

**Surface position.** A black SDF with defaults at 16² matched 256/256 RGBA8 pixels. The changed camera at 64² still matches 4,092/4,096 with maximum error 2. At 32² and RGBA32F, changing only field of view gives 0/1,024 with maximum error 1.490116e-7. Changing only camera UV gives 0/1,024 with maximum error 1.269579e-5. With both changes the largest error occurs at (0,2), where X is −7683.423 in MaterialCraft and −7684.096 in the reference. Alpha agrees at every pixel. Integer storage hides much of this coordinate difference.

**Volume integration.** At 16² and RGBA32F, replace noise by zero, position by `(128,128,128,255)` and the IBL environment by `(64,128,192,255)`. All six IBL inputs and all four light inputs match exactly. IBL still gives 0/256 with maximum error .00578630. Disabling dithering reduces this to 3.039837e-6 with 16/256 exact at both seeds 0 and 42. With nonconstant inputs and dithering disabled the maximum is .0002391934 and 12/256 match. Light gives 0/256 on the flat inputs with maximum error .0001328364, or .0001164377 with dithering disabled. Setting density to zero makes both flat, undithered renders exact at all 256 pixels. Nonzero-density integration remains open.

### Position debug

All **622,592/622,592 RGBA pixels** match across 14 cases, with maximum component error zero. The two previously failing 256² sphere cases match the original reference captures, including all 17 colored texels in each. Changing `debug_pos` from `(.5,.5)` to `(.25,.75)` changes four pixels between reference images. Both internal 4096² processors cook at full size.

Use a float32 parent, the RGBA8 position ramp above and an original signed-distance atlas. For side `N=128` or 256, let `s=N/16`, `X=((x mod s)+.5)/s-.5`, `Y=((y mod s)+.5)/s-.5`, and `Z=(floor(x/s)+16*floor(y/s)+.5)/256-.5`. Compute `sqrt(X²+Y²+Z²)-.3` and store the result in each RGB channel of a float32 EXR with alpha 1.

| Probe | Size | Seed | Output formats | Exact pixels / total | Maximum error |
|---|---:|---:|---|---:|---:|
| Default debug point | 128² | 0 and 42 | RGBA32F | 32,768/32,768 | 0 |
| Default and changed debug point | 256² | 0 | RGBA32F | 131,072/131,072 | 0 |
| Changed debug point | 128² | 42 | RGBA8, RGBA16, RGBA16F, RGBA32F | 65,536/65,536 | 0 |
| Changed debug point | 256² | 42 | Same four formats | 262,144/262,144 | 0 |
| Changed point, plane and shifted sphere | 256² | 42 | RGBA32F | 131,072/131,072 | 0 |

The plane uses `.55*X+.3*Y-.7*Z+.06`. The shifted sphere uses `sqrt((X-.125)²+(Y+.1)²+(Z-.125)²)-.36`. Both produced the same sparse marker as the centered sphere for the changed debug point. The 128² cases had zero colored texels. The parity mark covers these output images and settings.

### Panoramic LOD

The panorama helper exposes one image input. For its first-pass fixtures, let `U=floor(255*x/(N-1))`, `V=floor(255*y/(N-1))` and `T=(17*x+31*y) mod 256`. Bind linear RGBA8 `(T,U,255-V,255)`. The alpha case uses `(7*x+13*y) mod 256` for A. The new reference outputs for all three first-pass cases match their archived raw pixels exactly. The fractional-alpha input and opaque input produce identical output here. The four 64² cases use the varying-alpha color texture from the planar projection comparison.

| Input | Size | Seed | Format | Exact pixels / total | Maximum error |
|---|---:|---:|---|---:|---:|
| Opaque | 128² | 0 and 42 | RGBA8 | 15,883/16,384 each | 125 |
| Opaque | 256² | 0 | RGBA8 | 63,309/65,536 | 126 |
| Varying alpha | 128² | 0 | RGBA8 | 15,883/16,384 | 125 |
| Flat (23,139,211), alpha 75 or 255 | 128² | 0 | RGBA8 | 16,128/16,384 each | 116 |
| Varying alpha | 64² | 42 | RGBA8 / RGBA16 | 3,964/4,096 each | 1 / 257 |
| Same input | 64² | 42 | RGBA16F / RGBA32F | 3,964/4,096 each | .00439453125 / .0039215982 |

MaterialCraft previously sampled the full-resolution input for every bilinear transform. The 128² and 256² opaque cases then matched 719 and 5,015 pixels. An isolated transform uses a 2048×1024 RGBA8 input `(T,x mod 256,(255-y) mod 256,255)`, a 128² output, matrix `diag(-1,1.99309003)`, offset `(0,.5)`, horizontal-only wrap and opaque black matte. Its input bridge matches all 2,097,152 texels. The transform improved from 8,216/16,384 (maximum 117) to 16,384/16,384 (0). Changing the vertical scale to 2 improved from 8,192/16,384 (103) to 16,384/16,384 (0). When the first-pass 128² input is resized to the same 2048×1024 bridge, 1,923,823/2,097,152 input texels match (maximum 2). The two transforms give 16,287 and 16,284 exact output pixels, both with maximum 1. The full graph still differs along rows 0 and 47 with a flat input.

For an independent mip-level check, generate a 128² RGBA grid using the row-major unsigned hash from the gradient probe. Use the low 8 or 16 bits of the final hash for each integer channel. For a signed float input store `4*hash/2^32-2` in float32. Run an identity transform at 128², 64², 32² and 16². With automatic mip selection, both RGBA8-to-RGBA8 and RGBA16-to-RGBA16 match 21,760/21,760 pixels with zero error. For RGBA8 input and output, setting the manual mip level to 0, 1 or 2 also gives 21,760/21,760 at each level. An automatic uniform scale of 1.5 gives the same exact count. For the signed RGBA32F input and output, the four sizes match 16,384, 2,781, 308 and 29 pixels respectively, with maxima 0, 1.192093e-7, 5.960464e-8 and 2.980232e-8. Float filtering remains open.

### 3D volume mask and Worley noise

Use the first-pass linear position texture. For side `N`, store RGBA8 `(floor(255*x/(N-1)),floor(255*y/(N-1)),128,255)`. The mask's rotated case uses cylinder shape 1, rotation `(.1,.2,.3)`, size `(.8,.6,1)` and feather width .1. The Worley border case sets style 1, border width .2 and roundness .1. For its 64² output-format sweep, round the two position channels to nearest instead of flooring. Each cell gives exact pixels / total (maximum error).

| Case | Volume mask | Worley noise |
|---|---:|---:|
| 128², defaults, seed 0, LUM16 | 16,384/16,384 (0) | 1/16,384 (44,494) |
| 256², defaults, seed 0, LUM16 | 65,536/65,536 (0) | 3/65,536 (44,494) |
| 128², defaults, seed 42, LUM16 | 16,384/16,384 (0) | 1/16,384 (43,865) |
| 128², scale .4 for mask or 6 for Worley, seed 0, LUM16 | 16,384/16,384 (0) | 2/16,384 (40,476) |
| 128², rotated mask or Worley border, seed 0, LUM16 | 16,326/16,384 (1) | 1/16,384 (32,729) |
| Same settings, seed 42, LUM8 (mask 128², Worley 64²) | 16,383/16,384 (1) | 45/4,096 (138) |
| Same sizes and seed, LUM16 | 16,326/16,384 (1) | 0/4,096 (35,289) |
| Same sizes and seed, LUM16F | 16,349/16,384 (.0078125) | 4/4,096 (.53833008) |
| Same sizes and seed, LUM32F | 664/16,384 (.000015258789) | 0/4,096 (.53847277) |

**Mask.** The 128² cylinder with default rotation, size and feathering gives 16,379/16,384 exact LUM16 pixels, with maximum error 1. The RGBA16 position bridge matches all 16,384 pixels. In LUM32F, changing only cylinder rotation gives 564/16,384 (1.907349e-6), only size gives 14,088/16,384 (9.536743e-7), and only feather width gives 13,716/16,384 (3.814697e-6). A cube with `position_input=1` and `position_worldspace=(.375,.375,.5)` matches 16,384/16,384 float pixels exactly.

**Worley.** With a constant input position of `(0,0,0,255)`, no LUM32F pixel matches in the 32² default case at seed 0 (maximum .02120566) or seed 42 (.07068205). The graph's internal random table uses four draws. A standalone Pixel Processor writing four successive `rand(1)` values to RGBA16 reproduces a discrepancy before any cell-distance calculation. It gives 0/1,024 exact pixels at 32², seed 0 (maximum 65,122), 0/16,384 at 128², seed 0 (65,136), and 0/16,384 at 128², seed 42 (65,348). At seed 0 the first reference pixel changes with resolution, from `(47238,35719,5497,27913)` to `(37730,56747,64327,4184)`. MaterialCraft returns `(42015,40251,56435,61318)` at both sizes. The subsequent [per-pixel initialization comparison](#per-pixel-random-initialization) covers these settings and the complete 34-render Worley matrix.

### November 17 validation

All six new regression tests failed with their corresponding fixes disabled or absent. The release suites for nodes, evaluation and formats passed 246 tests. Two opt-in format interoperability tests were skipped. Workspace formatting, strict all-target Clippy, dependency layers and WebAssembly checks passed. Raw captures and generated input images were kept outside the repository.

## Remaining differences

The following entries record the first pass. The dated comparisons above give the newer results for revisited roots.

- **`3d_gradient.sbs/3d_linear_gradient` - Too hard for this pass:** Corrected the wrong channel mapping and normalized-integer bilinear sampling offset. Compared generated position maps in UV and world modes, with zero and nonzero contrast, at 256 and 512 pixels (eight cases). Remaining 32-bit differences reach 7.153e-7; some values straddle a 16-bit rounding boundary by one level. The default world-mode case matches every pixel after 16-bit conversion at both sizes, but full float parity is not established. Further work needs matching the reference shader arithmetic rounding. An unconnected Position input yields NaNs in the reference app; MaterialCraft retains its actionable non-finite arithmetic error. Moving to the next node.

- **`3d_perlin_noise.sbs/3D_perlin_noise` - Too hard for this pass:** Exact at 128x128 for defaults, seed 42, and scale 6. At 256x256, 3 of 65,536 eight-bit pixels remain one level different: (x=246,y=155), (25,179), and (154,227). Input maps are generated linear XY ramps with Z=128/255 and alpha=1. The output-format error is fixed; residual arithmetic/conversion rounding still prevents a parity mark.

- **`3d_perlin_noise.sbs/3D_ridged_noise_fractal` - Too hard for this pass:** Default and roughness-0.7 cases at 128x128 are exact. The default 256x256 case has two differing eight-bit pixels; seed 42 at 128x128 has one. Maximum difference is one level. Full parity is not claimed.

- **`3d_planar_projection.sbs/3d_planar_projection` - Too hard for this pass:** With generated position, normal, and high-frequency grayscale texture inputs, 16,240/16,384 sixteen-bit pixels match at 128x128 and 65,048/65,536 at 256x256. Maximum differences are 125 and 128 levels respectively, concentrated at texture sampling boundaries. Independent sampler half-step and nearest-read probes agree; remaining projection arithmetic is a likely contributor.

- **`3d_planar_projection.sbs/3d_planar_projection_color` - Too hard for this pass:** With generated position, normal, and RGBA texture inputs, 16,370/16,384 pixels match at 128x128 and 65,498/65,536 at 256x256. All four eight-bit channels, including alpha, were compared. Remaining differences are at most one level.

- **`3d_texture_render.sbs/3d_texture_surface_render_irradiance` - Too hard for this pass:** Eight 16x16 RGBA32F cases use original position/normal maps, constant positive HDR distance fields, directional sample tables, and a colored environment ramp. Four nonzero lighting cases (iteration 0, 1, 16, and 128) differ by at most 8.941e-8 in RGB; alpha is exact. Four opposite-hemisphere cases match exactly but produce black lighting. All cooks succeed. Full float parity needs matching shader arithmetic rounding; black-image agreement alone is not used for a parity mark.

- **`3d_texture_render.sbs/3d_texture_volume_render_ibl_pass` - Too hard for this pass:** Six 16x16 RGBA16F cases use original position/noise/environment/sample maps and constant negative distance fields, with dense and thin media, iteration changes, and absorption changes. Thin-media cases produce visible nonzero lighting but only 9/256, 10/256, and 4/256 pixels match exactly; maximum component differences are 0.006348, 0.005372, and 0.011719. Three additional thin-media cases bind an original 256x256 noise map at the required input size; they still differ, reaching 0.012208. All cooks succeed. The volume-integration discrepancy needs further isolation; half-float storage is separately verified.

- **`3d_texture_render.sbs/3d_texture_volume_render_light_pass` - Too hard for this pass:** Fixed the function-expansion rejection. All three 16x16 thin-media cases now cook successfully, including changed light position/intensity and changed albedo. Exact RGBA16F pixels: 252/256, 254/256, and 254/256. Maximum component differences are 6.104e-5, 3.052e-5, and 1.526e-5. Eight pixels in total prevent a parity mark.

- **`3d_texture_render.sbs/3d_texture_surface_render_position` - Too hard for this pass:** All four initial 16x16 RGBA8 cases match exactly (solid/positive fields, ground plane, changed camera). Solid-field 64x64 and ground-plane 128x128 cases also match exactly. A 64x64 case with a changed camera direction and 65-degree field of view has four differing pixels, at most two levels apart. All cooks succeed. Full camera/geometry parity is not claimed.

- **`3d_texture_render.sbs/position_debug` - Too hard for this pass:** Initial zero/positive/midpoint fields and checker fields produce constant images and are insufficient for a parity mark. A generated signed sphere-distance atlas and original position map produce nonconstant 256x256 RGBA32F outputs. Center and changed debug-selection cases each match 65,519/65,536 pixels; 17 pixels per case differ by up to 1.0. Both internal 4096x4096 processors cook at full native size.

- **`3d_volume_mask.sbs/3d_volume_mask` - Too hard for this pass:** Corrected omitted nearest sampling and finite clamping after division by zero. Default 128x128, default 256x256, and changed-scale 128x128 outputs are exact (98,304 sixteen-bit pixels). Cylinder mode has four one-level differences; rotated, resized, narrow-feather cylinder mode has 58 one-level differences. All five cases cook successfully.

- **`3d_worley_noise.sbs/3D_worley_noise` - Too hard for this pass:** All cooks succeed, but the noise field differs substantially. Defaults at 128x128/256x256, seed 42, scale 6, and border style have only 1, 3, 1, 2, and 1 matching sixteen-bit pixels respectively; maximum difference is 44,494 levels. Changing omitted sampler defaults does not resolve this.

- **Pixel Processor random stream - Unresolved:** Three original scalar-random probes (16x16 seeds 0/42 and 32x32 seed 0) disagree at all 1,536 float pixels. The reference stream also depends on output resolution. No guessed replacement sequence was introduced. A causal link to the remaining noise or volume-lighting differences is not established.

- **`alveolus.sbs/alveolus` - Too hard for this pass:** Defaults at 128x128/256x256 and seed 42 have no exactly matching sixteen-bit pixels. Gradient-filled cells with interstice width 0.1 match 13,880/16,384 pixels, but differ by up to 65,535 levels. All four cooks succeed.

- **`anisotropic_blur.sbs/anisotropic_blur` and `anisotropic_blur_grayscale` - Too hard for this pass:** Compared default 128x128, rotated high-anisotropy 128x128, and quality/intensity changes at 256x256. All cooks succeed. Color matches 224/16,384, 109/16,384, and 977/65,536 RGBA8 pixels; grayscale matches 252/16,384, 125/16,384, and 1,024/65,536 pixels. Maximum difference reaches 87 eight-bit levels. Blur sampling/kernel parity remains open.

- **`atlas_spliter.sbs/crop_and_transform_sub` - Too hard for this pass:** Its two typed value pins are outside the reference graph's exposed-parameter API. The direct comparison harness cannot set shape count/table size through that API. A future comparison needs an original wrapper that supplies these typed value inputs; the regular image-pin bindings are available.

- **`3d_viewer.sbs/ibl_specular_contrib_anisotropic/panoramic_lod` - Too hard for this pass:** Corrected one-axis wrapping. For 128x128 and 256x256 opaque color inputs, exact RGBA8 pixels improve from 576/16,384 and 4,391/65,536 to 719/16,384 and 5,015/65,536. Maximum component differences drop from 252/254 to 128/127. A varying-alpha 128x128 case still has no fully matching pixels and a maximum difference of 239. All cooks succeed. Input resizing and transform minification/filtering remain open.

- **`bevel_smooth.sbs/blur_gaussian_grayscale` - Too hard for this pass:** Defaults at 128x128 and 256x256 are exact but perform no visible blur at these settings. Intensity 5 at 128x128 matches only 158/16,384 eight-bit pixels and differs by up to 23 levels. All cooks succeed; nonzero-kernel parity remains open.

- **`bevel_smooth.sbs/jump_flood_segment_iteration` - Too hard for this pass:** All cooks succeed. Default 128x128, iteration 1, changed distance mode/scale, and default 256x256 match 16,366/16,384, 16,376/16,384, 16,379/16,384, and 65,466/65,536 RGBA8 pixels. A total of 101 segment selections differ; maximum component difference is 192. Input segment endpoints are original synthetic color fields.

- **`bevel_smooth.sbs/debug/bevel_smooth_debug_position` - Too hard for this pass:** Corrected undefined function intermediates, ordered NaN comparisons, omitted color-mode imports, and pattern alpha. All four cases now cook successfully in RGBA32F. Default bounded-position maps at 128x128/256x256 match 16,359/16,384 and 65,415/65,536 pixels. Offset and pixel-selection cases match 14,326/16,384 and 15,362/16,384. Remaining stamp placement/coverage differences reach 19.165; a parity mark is withheld.

- **`blackbody.sbs/blackbody` - Too hard for this pass:** Compared the default 3,000 K at 128x128 and 1,000/6,500/12,000 K at 16x16, retaining RGBA32F HDR output. Alpha is exact, but no complete RGBA pixel matches; RGB differences reach 4.769e-7. All cooks succeed.

- **Remaining blend cases:** All four RGBA8 cases cook for every graph. `blend.sbs/color` matches 114,111/114,688 pixels, with differences up to nine levels. `color_burn` and `color_dodge` each have seven differing pixels, reaching 16 and 144 levels; these include degenerate black/white endpoints and one-level arithmetic differences. `dissolve_2` matches 75,035/114,688 pixels, with stochastic differences up to 254 levels. After the grayscale default fix, `luminosity` has two one-level differences; the fused-dot recheck retains those two differences. These graphs are not marked parity.

- **`blur_hq.sbs/blur_hq` and `blur_hq_grayscale` - Too hard for this pass:** All four cases cook. Intensity 1 at 128x128 is exact for both output types but has no visible blur at that scale. Default 128x128/256x256 and high-quality intensity-5 color cases match 2/16,384, 0/65,536, and 12/16,384 RGBA8 pixels; grayscale matches 125/16,384, 15,168/65,536, and 144/16,384 pixels. Differences reach 52 color levels and 30 grayscale levels. Directional blur kernel/sampling remains open.

- **`bnw_spots_1.sbs/bnw_spots_1` - Too hard for this pass:** All four cases cook. Exact pixels for defaults at 128x128/256x256, seed 42, and changed parameters: 0/16384, 5/65536, 0/16384, 0/16384. Maximum component difference is 62677 integer levels. Pattern/noise behavior remains open.

- **`bnw_spots_3.sbs/bnw_spots_3` - Too hard for this pass:** All four cases cook. Exact pixels for defaults at 128x128/256x256, seed 42, and changed parameters: 0/16384, 0/65536, 0/16384, 0/16384. Maximum component difference is 63995 integer levels. Pattern/noise behavior remains open.

- **`brick_1.sbs/brick_1` - Too hard for this pass:** All four cases cook. Exact pixels for defaults at 128x128/256x256, seed 42, and changed parameters: 14992/16384, 59968/65536, 14992/16384, 8352/16384. Maximum component difference is 65535 integer levels. Pattern/noise behavior remains open.

- **`brick_2.sbs/brick_2` - Too hard for this pass:** All four cases cook. Exact pixels for defaults at 128x128/256x256, seed 42, and changed parameters: 15264/16384, 61360/65536, 15264/16384, 8896/16384. Maximum component difference is 65535 integer levels. Pattern/noise behavior remains open.

- **`brick_3.sbs/brick_3` - Too hard for this pass:** All four cases cook. Exact pixels for defaults at 128x128/256x256, seed 42, and changed parameters: 4616/16384, 24922/65536, 5408/16384, 2880/16384. Maximum component difference is 65535 integer levels. Pattern/noise behavior remains open.

- **`cartesian_polar_transformation.sbs/cartesian_to_polar` - Too hard for this pass:** All four cases cook. Exact pixels for defaults at 128x128/256x256, seed 42, and a changed input: 14130/16384, 52081/65536, 14130/16384, 14081/16384. Maximum component difference is 4 integer levels. Polar sampling/coordinate precision remains open.

- **`cartesian_polar_transformation.sbs/polar_to_cartesian` - Too hard for this pass:** All four cases cook. Exact pixels for defaults at 128x128/256x256, seed 42, and a changed input: 15330/16384, 61564/65536, 15330/16384, 15842/16384. Maximum component difference is 1 integer levels. Polar sampling/coordinate precision remains open.

- **`cartesian_polar_transformation.sbs/cartesian_to_polar_grayscale` - Too hard for this pass:** All four cases cook. Exact pixels for defaults at 128x128/256x256, seed 42, and a changed input: 15216/16384, 58474/65536, 15216/16384, 15281/16384. Maximum component difference is 4 integer levels. Polar sampling/coordinate precision remains open.

- **`cartesian_polar_transformation.sbs/polar_to_cartesian_grayscale` - Too hard for this pass:** All four cases cook. Exact pixels for defaults at 128x128/256x256, seed 42, and a changed input: 15804/16384, 63310/65536, 15804/16384, 15794/16384. Maximum component difference is 1 integer levels. Polar sampling/coordinate precision remains open.

- **`cells_1.sbs/cells_1` - Too hard for this pass:** All four cases cook. Exact pixels for defaults at 128x128/256x256, seed 42, and changed parameters: 5/16384, 12/65536, 1/16384, 2/16384. Maximum component difference is 64385 integer levels. Pattern/noise behavior remains open.

- **`channel_mixer.sbs/channel_mixer` - Too hard for this pass:** Default 128x128/256x256 and seed-42 cases fail final validation of the dynamic `in_high` parameter. The changed-channel, monochrome case cooks but matches 0/16384 RGBA16 pixels, with differences up to 62259 levels. Parameter endpoint semantics and channel mixing remain open.

- **`chrominance_extract.sbs/chrominance_extract` - Too hard for this pass:** All cases cook. Exact pixels in case order: 10557/16384, 42010/65536, 10557/16384, 10515/16384; maximum component difference 1 integer levels.

- **`clone.sbs/clone` - Too hard for this pass:** All cases cook. Exact pixels in case order: 16384/16384, 65536/65536, 16384/16384, 16155/16384; maximum component difference 1 integer levels.

- **`clouds_1.sbs/clouds_1` - Too hard for this pass:** All cases cook. Exact pixels in case order: 1/16384, 12/65536, 4/16384, 3/16384; maximum component difference 51526 integer levels.

- **`clouds_2.sbs/clouds_2` - Too hard for this pass:** All cases cook. Exact pixels in case order: 1/16384, 2/65536, 1/16384; maximum component difference 42679 integer levels.

- **`color_space_conversion.sbs/acescg_to_linear_srgb` - Too hard for this pass:** All cases cook. Exact pixels in case order: 16384/16384, 65536/65536, 16384/16384, 16383/16384; maximum component difference 1 integer levels.

- **`color_space_conversion.sbs/srgb_to_acescg` - Too hard for this pass:** All cases cook. Exact pixels in case order: 16383/16384, 65534/65536, 16383/16384, 16382/16384; maximum component difference 1 integer levels.

- **`color_to_mask.sbs/color_to_mask` - Too hard for this pass:** All cases cook. Exact pixels in case order: 16384/16384, 65536/65536, 16384/16384, 16293/16384; maximum component difference 255 integer levels.

- **`color_to_mask_v2.sbs/color_to_mask` - Too hard for this pass:** Corrected integer storage after undefined final pixel arithmetic. All four cases now cook. Defaults at 128x128/256x256 and seed 42 match exactly. Changed input-color selection has one one-level difference (16,383/16,384 pixels match).

- **`checker_1.sbs/checker_1` - Too hard for this pass:** Corrected the transposed transform matrix. Defaults at 128x128/256x256, seed 42, and rotated tiling 1/2 now match exactly. Tiling 3 still has minification differences: 14,400/16,384 pixels without rotation and 13,476/16,384 with rotation; maxima 56 and 119 eight-bit levels. A parity mark is withheld.

- **`constant_values.sbs/integer/constant_integer4` - Too hard for this pass:** Zero and mixed signed-vector cases match. At the positive 32-bit integer limit, the reference returns -2,147,483,648 for a component set to 2,147,483,647; MaterialCraft retains 2,147,483,647. No overflow behavior was introduced to imitate this boundary result.

- **`color_temperature_adjustment.sbs/color_temperature_adjustment` - Too hard for this pass:** Fixed the integer option read that previously rejected every cook. Default 128x128/256x256 and seed-42 cases are exact. Temperature 0.25, magenta/green -0.2, and alternate color space at 128x128 have two one-level RGBA8 differences. All four cases cook.

- **`composite.sbs/composite/filters/filter_transformation` - Too hard for this pass:** All four cases cook. Exact pixels by case/output: 16320/16384, 65280/65536, 16320/16384, 16318/16384; maximum component difference 255 integer levels.

- **`composite.sbs/composite/blends/blend_content` - Too hard for this pass:** All four cases cook. Exact pixels by case/output: 16288/16384, 65152/65536, 16288/16384, 16288/16384; maximum component difference 255 integer levels.

- **`composite.sbs/composite/blends/blend_content_pre_post_composite` - Too hard for this pass:** All four cases cook. Exact pixels by case/output: 16288/16384, 65152/65536, 16288/16384, 16287/16384; maximum component difference 255 integer levels.

- **`creased.sbs/creased` - Too hard for this pass:** All four cases cook. Exact pixels by case/output: 0/16384, 1/65536, 0/16384, 0/16384; maximum component difference 55711 integer levels.

- **`crop.sbs/crop` - Too hard for this pass:** All four cases cook. Exact pixels by case/output: 16340/16384, 16384/16384, 65350/65536, 65536/65536, 16340/16384, 16384/16384, 16384/16384, 16384/16384; maximum component difference 1 integer levels.

- **`crop.sbs/crop_grayscale` - Too hard for this pass:** All four cases cook. Exact pixels by case/output: 16340/16384, 16384/16384, 65350/65536, 65536/65536, 16340/16384, 16384/16384, 16384/16384, 16384/16384; maximum component difference 1 integer levels.

- **`crystal_1.sbs/crystal_1` - Too hard for this pass:** All 4 cases cook. Exact pixels by case/output: 2/16384, 5/65536, 2/16384, 3/16384; maximum component difference 64433 integer levels.

- **`crystal_2.sbs/crystal_2` - Too hard for this pass:** All 4 cases cook. Exact pixels by case/output: 1/16384, 2/65536, 0/16384, 1/16384; maximum component difference 55976 integer levels.

- **`curvature.sbs/curvature` - Too hard for this pass:** All 4 cases cook. Exact pixels by case/output: 31/16384, 660/65536, 31/16384, 20/16384; maximum component difference 962 integer levels. Inputs include an original curved, normalized normal field.

- **`curvature_smooth.sbs/curvature_smooth` - Too hard for this pass:** All 4 cases cook. Exact pixels by case/output: 23/16384, 167/65536, 23/16384, 25/16384; maximum component difference 1602 integer levels. Inputs include an original curved, normalized normal field.

- **`diffusion.sbs/impl/pass_normal` - Too hard for this pass:** All 4 cases cook. Exact pixels by case/output: 16383/16384, 65527/65536, 16383/16384, 1304/16384; maximum component difference 94 integer levels.

- **`diffusion.sbs/impl/pass_RGBA` - Too hard for this pass:** All 4 cases cook. Exact pixels by case/output: 15919/16384, 63936/65536, 15919/16384, 458/16384; maximum component difference 154 integer levels.

- **`diffusion.sbs/impl/pass_gray` - Too hard for this pass:** All 4 cases cook. Exact pixels by case/output: 16178/16384, 64736/65536, 16178/16384, 2192/16384; maximum component difference 104 integer levels.

- **`diffusion.sbs/impl/pass_UV` - Too hard for this pass:** All 4 cases cook. Exact pixels by case/output: 9704/16384, 40270/65536, 9720/16384, 9704/16384; maximum component difference 192 integer levels.

- **`directional_distance.sbs/directional_distance` - Too hard for this pass:** All 4 cases cook. Exact pixels by case/output: 16378/16384, 16384/16384, 65510/65536, 65536/65536, 16378/16384, 16384/16384, 16384/16384, 16384/16384; maximum component difference 1 integer levels. All output bridges were compared.

- **`directionnal_noise.sbs/directionnal_noise` - Too hard for this pass:** All 3 cases cook. Exact pixels by case/output: 0/16384, 0/65536, 0/16384; maximum component difference 60592 integer levels.

- **`dirt_1.sbs/dirt_1` - Too hard for this pass:** All 4 cases cook. Exact pixels by case/output: 79/16384, 334/65536, 55/16384, 100/16384; maximum component difference 65535 integer levels.

- **`dirt_3.sbs/dirt_3` - Too hard for this pass:** All 4 cases cook. Exact pixels by case/output: 6/16384, 546/65536, 2/16384, 5/16384; maximum component difference 64872 integer levels.

- **`dirt_5.sbs/dirt_5` - Too hard for this pass:** All 3 cases cook. Exact pixels by case/output: 2514/16384, 9716/65536, 2587/16384; maximum component difference 65535 integer levels.

- **`dirt_6.sbs/dirt_6` - Too hard for this pass:** All 3 cases cook. Exact pixels by case/output: 51/16384, 188/65536, 60/16384; maximum component difference 64915 integer levels.

- **`dirt_gradient.sbs/dirt_gradient` - Too hard for this pass:** All 3 cases cook. Exact pixels by case/output: 0/16384, 1/65536, 1/16384; maximum component difference 65399 integer levels.

- **`fibers_1.sbs/fibers_1` - Too hard for this pass:** All four cases cook. Exact pixels by case/output: 0/16384, 1/65536, 0/16384, 0/16384; maximum component difference 48873 integer levels.

- **`flood_fill.sbs/flood_fill_to_random_color` - Too hard for this pass:** All four cases cook. Exact pixels by case/output: 0/16384, 0/65536, 0/16384, 0/16384; maximum component difference 65071 integer levels.

- **`flood_fill.sbs/flood_fill_to_random_grayscale` - Too hard for this pass:** All four cases cook. Exact pixels by case/output: 0/16384, 8/65536, 0/16384, 0/16384; maximum component difference 64844 integer levels.

- **`flood_fill_2.sbs/flood_fill_to_random_color` - Too hard for this pass:** All four cases cook. Exact pixels by case/output: 0/16384, 0/65536, 0/16384, 0/16384; maximum component difference 65071 integer levels.

- **`flood_fill_2.sbs/flood_fill_to_random_grayscale` - Too hard for this pass:** All four cases cook. Exact pixels by case/output: 0/16384, 0/65536, 0/16384, 0/16384; maximum component difference 65169 integer levels.

- **`flood_fill_2.sbs/flood_fill_to_color` - Too hard for this pass:** All four cases cook. Exact pixels by case/output: 16384/16384, 65536/65536, 16384/16384, 0/16384; maximum component difference 61956 integer levels.

- **`flood_fill_2.sbs/flood_fill_to_grayscale` - Too hard for this pass:** All four cases cook. Exact pixels by case/output: 16384/16384, 65536/65536, 16384/16384, 2/16384; maximum component difference 5503 integer levels.

- **`fluid.sbs/fluid` - Too hard for this pass:** All four cases cook. Exact pixels by case/output: 0/16384, 0/65536, 0/16384, 0/16384; maximum component difference 65454 integer levels.

- **`fractal_sum_base.sbs/fractal_sum_base` - Too hard for this pass:** All four cases cook. Exact pixels by case/output: 3/16384, 8/65536, 5/16384, 5/16384; maximum component difference 10237 integer levels.

- **`fur_1.sbs/fur_1` - Too hard for this pass:** All four cases cook. Exact pixels by case/output: 23/16384, 193/65536, 30/16384, 25/16384; maximum component difference 65535 integer levels.

- **`fur_2.sbs/fur_2` - Too hard for this pass:** All four cases cook. Exact pixels by case/output: 0/16384, 0/65536, 0/16384, 0/16384; maximum component difference 65523 integer levels.

- **`fur_3.sbs/fur_3` - Too hard for this pass:** All four cases cook. Exact pixels by case/output: 0/16384, 1/65536, 0/16384, 0/16384; maximum component difference 65535 integer levels.

- **`fur_4.sbs/fur_4` - Too hard for this pass:** All four cases cook. Exact pixels by case/output: 1134/16384, 4416/65536, 1205/16384, 1200/16384; maximum component difference 65535 integer levels.

- **`gaussian_1.sbs/gaussian_1` - Too hard for this pass:** All four cases cook. Exact pixels by case/output: 0/16384, 0/65536, 0/16384, 0/16384; maximum component difference 40180 integer levels.

- **`gaussian_2.sbs/gaussian_2` - Too hard for this pass:** All four cases cook. Exact pixels by case/output: 0/16384, 0/65536, 0/16384, 0/16384; maximum component difference 23969 integer levels.

- **`gaussian_spots_1.sbs/gaussian_spots_1` - Too hard for this pass:** All four cases cook. Exact pixels by case/output: 4/16384, 15/65536, 7/16384, 4/16384; maximum component difference 63216 integer levels.

- **`gaussian_spots_2.sbs/gaussian_spots_2` - Too hard for this pass:** All four cases cook. Exact pixels by case/output: 3/16384, 16/65536, 2/16384, 3/16384; maximum component difference 65535 integer levels.

- **`gradient_axial.sbs/gradient_axial` - Too hard for this pass:** All four cases cook. Exact pixels by case/output: 16384/16384, 65536/65536, 16384/16384, 16379/16384; maximum component difference 1 integer levels.

- **`gradient_axial.sbs/gradient_axial_reflected` - Too hard for this pass:** All four cases cook. Exact pixels by case/output: 16384/16384, 65536/65536, 16384/16384, 16366/16384; maximum component difference 1 integer levels.

- **`gradient_circular.sbs/gradient_circular` - Too hard for this pass:** All four cases cook. Exact pixels by case/output: 5531/16384, 21951/65536, 5531/16384, 5591/16384; maximum component difference 4 integer levels.

- **`gradient_linear_1.sbs/gradient_linear_1` - Too hard for this pass:** All four cases cook. Exact pixels by case/output: 16384/16384, 65536/65536, 16384/16384, 15456/16384; maximum component difference 24576 integer levels.

- **`gradient_linear_2.sbs/gradient_linear_2` - Too hard for this pass:** All four cases cook. Exact pixels by case/output: 16384/16384, 65536/65536, 16384/16384, 0/16384; maximum component difference 1984 integer levels.

- **`gradient_linear_3.sbs/gradient_linear_3` - Too hard for this pass:** All four cases cook. Exact pixels by case/output: 15495/16384, 61975/65536, 15495/16384, 4096/16384; maximum component difference 1830 integer levels.

- **`gradient_radial.sbs/gradient_radial` - Too hard for this pass:** Corrected integer storage so the changed shape/points case cooks instead of failing on a non-finite pixel. All four cases now cook and match 114,624/114,688 sixteen-bit pixels in total; 64 one-level differences remain.

- **`hald_clut.sbs/hald_clut` - Too hard for this pass:** Exact pixels by case/output: 16118/16384, 64478/65536, 16118/16384, 15905/16384; maximum stored-component difference 1.

- **`hbao.sbs/hbao_mip` - Too hard for this pass:** Exact pixels by case/output: 16384/16384, 65536/65536, 16384/16384, 22/16384; maximum stored-component difference 24094.

- **`hbao_2.sbs/hbao_subsample` - Too hard for this pass:** Exact pixels by case/output: 16384/16384, 65536/65536, 16384/16384, 16377/16384; maximum stored-component difference 32.

- **`hbao_2.sbs/hbao_mip` - Too hard for this pass:** Exact pixels by case/output: 16384/16384, 65536/65536, 16384/16384, 143/16384; maximum stored-component difference 94.

- **`hdr_merge.sbs/average_luminance_merge` - Too hard for this pass:** Exact pixels by case/output: 0/16384, 0/65536, 0/16384, 1/16384; maximum stored-component difference 26228.

- **`hdr_range_viewer.sbs/hdr_range_viewer_grayscale` - Too hard for this pass:** Exact pixels by case/output: 14532/16384, 58173/65536, 14532/16384, 11670/16384; maximum stored-component difference 1.

- **`height_blend.sbs/height_blend` - Too hard for this pass:** Exact pixels by case/output: 16384/16384, 16384/16384, 65536/65536, 65536/65536, 16384/16384, 16384/16384, 7573/16384, 8768/16384; maximum stored-component difference 23426.

- **`height_to_normal_world_units.sbs/height_to_normal_world_units` - Too hard for this pass:** Exact pixels by case/output: 16384/16384, 65536/65536, 16384/16384, 10198/16384; maximum stored-component difference 50.

- **`height_to_normal_world_units.sbs/height_to_normal_world_units_2` - Too hard for this pass:** Exact pixels by case/output: 16384/16384, 65536/65536, 16384/16384, 10198/16384; maximum stored-component difference 49.

- **`highpass.sbs/highpass` - Too hard for this pass:** Exact pixels by case/output: 223/16384, 556/65536, 223/16384, 6967/16384; maximum stored-component difference 59.

- **`highpass.sbs/highpass_grayscale` - Too hard for this pass:** Exact pixels by case/output: 223/16384, 3712/65536, 223/16384, 9926/16384; maximum stored-component difference 66.

- **`histogram_scan.sbs/histogram_scan` - Too hard for this pass:** Exact pixels by case/output: 16319/16384, 65280/65536, 16319/16384, 16384/16384; maximum stored-component difference 255.

- **`histogram_shift.sbs/histogram_shift` - Too hard for this pass:** Exact pixels by case/output: 16319/16384, 65280/65536, 16319/16384, 15935/16384; maximum stored-component difference 1.

- **`histogram_tools.sbs/histogram_compute` - Too hard for this pass:** Exact pixels by case/output: 4/256, 54/256, 1/256, 256/256, 4/256, 54/256, 1/128, 0/128; maximum stored-component difference 7.

- **`material_transforms.sbs/material_transforms` - Too hard for this pass:** Exact pixels by case/output: 16384/16384, 16384/16384, 16384/16384, 16384/16384, 16384/16384, 65536/65536, 65536/65536, 65536/65536, 65536/65536, 65536/65536, 16384/16384, 16384/16384, 16384/16384, 16384/16384, 16384/16384, 14559/16384, 13711/16384, 15603/16384, 15493/16384, 12192/16384; maximum stored-component difference 1.

- **`me_water_level.sbs/scratches_2` - Too hard for this pass:** Exact pixels by case/output: 3/16384, 29/65536, 2/16384, 9/16384; maximum stored-component difference 103.

- **`mean_of_least_variance.sbs/mlv_color_iteration` - Too hard for this pass:** Exact pixels by case/output: 4/16384, 0/65536, 4/16384, 658/16384; maximum stored-component difference 167.

- **`mean_of_least_variance.sbs/mlv_grayscale_iteration` - Too hard for this pass:** Exact pixels by case/output: 6292/16384, 33605/65536, 6292/16384, 1762/16384; maximum stored-component difference 88.

- **`median_filter.sbs/median_filter_color` - Too hard for this pass:** Exact pixels by case/output: 16384/16384, 65536/65536, 16384/16384, 12077/16384; maximum stored-component difference 236.

- **`mesh_1.sbs/mesh_1` - Too hard for this pass:** Exact pixels by case/output: 0/16384, 0/65536, 0/16384, 16/16384; maximum stored-component difference 65535.

- **`mesh_2.sbs/mesh_2` - Too hard for this pass:** Exact pixels by case/output: 0/16384, 0/65536, 0/16384, 160/16384; maximum stored-component difference 65535.

- **`mesh_adaptive_material_blender.sbs/mesh_material_blender_8_inputs` - Too hard for this pass:** Exact pixels by case/output: 16384/16384, 16384/16384, 89/16384, 65536/65536, 65536/65536, 345/65536, 16384/16384, 16384/16384, 89/16384, 3157/16384, 14385/16384, 76/16384; maximum stored-component difference 255.

- **`mesh_adaptive_material_blender.sbs/mesh_material_blender_4_inputs` - Too hard for this pass:** Exact pixels by case/output: 16384/16384, 16384/16384, 86/16384, 65536/65536, 65536/65536, 358/65536, 16384/16384, 16384/16384, 86/16384, 1595/16384, 14388/16384, 74/16384; maximum stored-component difference 255.

- **`mesh_adaptive_material_blender.sbs/mesh_material_blender_2_inputs` - Too hard for this pass:** Exact pixels by case/output: 16384/16384, 16384/16384, 20/16384, 65536/65536, 65536/65536, 81/65536, 16384/16384, 16384/16384, 20/16384, 231/16384, 2761/16384, 12/16384; maximum stored-component difference 255.

- **`messy_fibers_1.sbs/messy_fibers_1` - Too hard for this pass:** Exact pixels by case/output: 173/16384, 1893/65536, 145/16384, 104/16384; maximum stored-component difference 65535.

- **`messy_fibers_2.sbs/messy_fibers_2` - Too hard for this pass:** Exact pixels by case/output: 34/16384, 165/65536, 53/16384, 46/16384; maximum stored-component difference 65535.

- **`messy_fibers_3.sbs/messy_fibers_3` - Too hard for this pass:** Exact pixels by case/output: 6383/16384, 25656/65536, 6157/16384, 6471/16384; maximum stored-component difference 255.

- **`messy_fibers_4.sbs/messy_fibers_4` - Too hard for this pass:** Exact pixels by case/output: 0/16384, 0/65536, 0/16384; maximum stored-component difference 64037.

- **`microscope_view.sbs/microscope_view` - Too hard for this pass:** Exact pixels by case/output: 1/16384, 23/65536, 0/16384, 1/16384; maximum stored-component difference 65535.

- **`moisture_noise.sbs/moisture_noise` - Too hard for this pass:** Exact pixels by case/output: 7/16384, 34/65536, 3/16384, 10/16384; maximum stored-component difference 45639.

- **`mosaic.sbs/mosaic` - Too hard for this pass:** Exact pixels by case/output: 0/16384, 0/65536, 0/16384, 0/16384; maximum stored-component difference 190.

- **`mosaic.sbs/mosaic_grayscale` - Too hard for this pass:** Exact pixels by case/output: 198/16384, 630/65536, 198/16384, 258/16384; maximum stored-component difference 143.

- **`multi_directional_warp.sbs/multi_directional_warp_grayscale` - Too hard for this pass:** Exact pixels by case/output: 121/16384, 3395/65536, 121/16384, 9394/16384; maximum stored-component difference 158.

- **`multi_directional_warp.sbs/multi_directional_warp_color` - Too hard for this pass:** Exact pixels by case/output: 66/16384, 286/65536, 66/16384, 1215/16384; maximum stored-component difference 250.

- **`multiangle_to_albedo.sbs/multiangle_to_albedo` - Too hard for this pass:** Exact pixels by case/output: 16384/16384, 65534/65536, 16384/16384, 16384/16384; maximum stored-component difference 1.

- **`noise_anisotropic_noise.sbs/anisotropic_noise` - Too hard for this pass:** Exact pixels by case/output: 5/16384, 25/65536, 8/16384, 3612/16384; maximum stored-component difference 65214.

- **`noise_anisotropic_noise_v2.sbs/anisotropic_noise_v2` - Too hard for this pass:** Exact pixels by case/output: 5422/16384, 14285/65536, 5547/16384, 13676/16384; maximum stored-component difference 15.

- **`min_max.sbs/min_max` - Too hard for this pass:** All four cases cook and seven of eight scalar outputs match exactly. The alternate-input minimum differs by 5.9604645e-8 (0.1254901885986328 versus 0.12549012899398804).

- **`make_it_tile_patch.sbs/make_it_tile_patch` - Too hard for this pass:** Exact pixels by case/output: 596/16384, 2423/65536, 596/16384, 41/16384; maximum stored-component difference 255.

- **`make_it_tile_patch_grayscale.sbs/make_it_tile_patch_grayscale` - Too hard for this pass:** Exact pixels by case/output: 937/16384, 5840/65536, 937/16384, 90/16384; maximum stored-component difference 252.

- **`non_square_transform.sbs/non_square_transform` - Too hard for this pass:** Exact pixels by case/output: 16384/16384, 65536/65536, 16384/16384, 13939/16384; maximum stored-component difference 2.

- **`non_square_transform.sbs/non_square_transform_greyscale` - Too hard for this pass:** Exact pixels by case/output: 16384/16384, 65536/65536, 16384/16384, 15078/16384; maximum stored-component difference 2.

- **`non_uniform_blur.sbs/non_uniform_blur_sample` - Too hard for this pass:** Exact pixels by case/output: 8397/16384, 25549/65536, 8397/16384, 3980/16384; maximum stored-component difference 4.

- **`non_uniform_blur.sbs/non_uniform_blur_grayscale_sample` - Too hard for this pass:** Exact pixels by case/output: 13146/16384, 52479/65536, 13146/16384, 8120/16384; maximum stored-component difference 4.

- **`non_uniform_rotation.sbs/non_uniform_rotation_grayscale` - Too hard for this pass:** Exact pixels by case/output: 15829/16384, 63348/65536, 15829/16384, 15865/16384; maximum stored-component difference 1.

- **`non_uniform_rotation.sbs/non_uniform_rotation_color` - Too hard for this pass:** Exact pixels by case/output: 15888/16384, 63498/65536, 15888/16384, 15885/16384; maximum stored-component difference 2.

- **`normal_normalise.sbs/normal_normalise` - Too hard for this pass:** Exact pixels by case/output: 16356/16384, 65420/65536, 16356/16384, 16356/16384; maximum stored-component difference 1.

- **`normal_sobel.sbs/normal_sobel` - Too hard for this pass:** Exact pixels by case/output: 10198/16384, 40960/65536, 10198/16384, 10198/16384; maximum stored-component difference 2399.

- **`normal_sobel.sbs/normal_sobel_2` - Too hard for this pass:** Exact pixels by case/output: 10198/16384, 40960/65536, 10198/16384, 10198/16384; maximum stored-component difference 9.

- **`normal_to_height_2.sbs/normal_to_height` - Too hard for this pass:** Exact pixels by case/output: 10956/16384, 44049/65536, 10956/16384, 11389/16384; maximum stored-component difference 19614.

- **`normal_to_height_hq.sbs/base_algorithm_height_distance` - Too hard for this pass:** Exact pixels by case/output: 16384/16384, 65536/65536, 16384/16384, 14374/16384; maximum stored-component difference 347.

- **`normal_transform.sbs/normal_transform` - Too hard for this pass:** Exact pixels by case/output: 16356/16384, 65420/65536, 16356/16384, 16356/16384; maximum stored-component difference 1.

- **`panorama_gradients.sbs/panorama_gradient_linear` - Too hard for this pass:** Exact pixels by case/output: 0/16384, 0/65536, 0/16384, 0/16384; maximum stored-component difference 2.384186e-07.

- **`panorama_gradients.sbs/panorama_gradient_2_points` - Too hard for this pass:** Exact pixels by case/output: 445/16384, 1667/65536, 445/16384, 137/16384; maximum stored-component difference 4.172325e-07.

- **`panorama_nadir_extract.sbs/nadir_extract` - Too hard for this pass:** Exact pixels by case/output: 354/16384, 337/65536, 354/16384, 3312/16384; maximum stored-component difference 0.07189722.

- **`panorama_nadir_patch.sbs/panorama_nadir_patch` - Too hard for this pass:** Exact pixels by case/output: 15109/16384, 59874/65536, 15109/16384, 15569/16384; maximum stored-component difference 0.01915002.

- **`panorama_position.sbs/panorama_3d_position` - Too hard for this pass:** Exact pixels by case/output: 16183/16384, 64379/65536, 16183/16384, 16183/16384; maximum stored-component difference 1.

- **`panorama_rotation.sbs/panorama_rotation` - Too hard for this pass:** Exact pixels by case/output: 13885/16384, 48478/65536, 13885/16384, 11046/16384; maximum stored-component difference 7.

- **`panorama_shape.sbs/panorama_shape` - Too hard for this pass:** Exact pixels by case/output: 15802/16384, 63195/65536, 15802/16384, 15177/16384; maximum stored-component difference 9.202957e-05.

- **`panorama_shape.sbs/panorama_shape_2` - Too hard for this pass:** Exact pixels by case/output: 15374/16384, 61494/65536, 15374/16384, 10926/16384; maximum stored-component difference 6.747246e-05.

- **`panorama_shape.sbs/panorama_sphere_light` - Too hard for this pass:** Exact pixels by case/output: 16356/16384, 65425/65536, 16356/16384, 16269/16384; maximum stored-component difference 0.001916409.

- **`panorama_shape.sbs/panorama_plane_light` - Too hard for this pass:** Exact pixels by case/output: 15918/16384, 63690/65536, 15918/16384, 16384/16384; maximum stored-component difference 4.464388e-05.

- **`panorama_shape.sbs/panorama_spot` - Too hard for this pass:** Exact pixels by case/output: 16378/16384, 65508/65536, 16378/16384, 11303/16384; maximum stored-component difference 0.1226807.

- **`panorama_straighten_horizon.sbs/panorama_straighten_horizon` - Too hard for this pass:** Exact pixels by case/output: 16384/16384, 65536/65536, 16384/16384, 0/16384; maximum stored-component difference 250.

- **`paper_path.sbs/paper_path` - Too hard for this pass:** Exact pixels by case/output: 7664/16384, 27377/65536, 7766/16384, 6530/16384; maximum stored-component difference 65535.

- **`parquet_1.sbs/parquet_1` - Too hard for this pass:** Exact pixels by case/output: 16240/16384, 64996/65536, 16288/16384, 16096/16384; maximum stored-component difference 255.

- **`panorama_physical_sun_sky.sbs/Impl/GetOneParam` - Too hard for this pass:** Typed outputs match in 0/4 comparisons. output: [3.0352942943573, 3.1215686798095703, 3.1450982093811035] versus [3.0352942943573, 3.1215689182281494, 3.1450982093811035]; output: [2.8705883026123047, 3.3490195274353027, 2.462745428085327] versus [2.870588541030884, 3.349020004272461, 2.462745428085327]; output: [3.0352942943573, 3.1215686798095703, 3.1450982093811035] versus [3.0352942943573, 3.1215689182281494, 3.1450982093811035]; output: [0.9883579015731812, 0.8544117212295532, 1.0778186321258545] versus [0.9883579015731812, 0.8544118404388428, 1.0778186321258545].

- **`panorama_shape.sbs/panorama_line_light` - Too hard for this pass:** Exact pixels by case/output: 15172/16384, 60708/65536, 15172/16384, 14428/16384; maximum stored-component difference 0.0001544952.

- **`noise_bnw_spots_1.sbs/bnw_spots_1` - Too hard for this pass:** Cooks report: pattern graph exceeded its execution budget. Exact pixels by case/output: 1/16384, 0/65536, 1/16384; maximum stored-component difference 64390.

- **`noise_bnw_spots_1_v2.sbs/bnw_spots_1` - Too hard for this pass:** Cooks report: pattern graph exceeded its execution budget. Exact pixels by case/output: 131/16384, 541/65536, 141/16384; maximum stored-component difference 205.

- **`noise_bnw_spots_3.sbs/bnw_spots_3` - Too hard for this pass:** Cooks report: pattern graph exceeded its execution budget. Exact pixels by case/output: 0/16384, 1/65536, 0/16384; maximum stored-component difference 62911.

- **`noise_bnw_spots_3_v2.sbs/bnw_spots_3` - Too hard for this pass:** Cooks report: pattern graph exceeded its execution budget. Exact pixels by case/output: 49/16384, 162/65536, 48/16384; maximum stored-component difference 227.

- **`noise_cells_1.sbs/cells_1` - Too hard for this pass:** Exact pixels by case/output: 2/16384, 5/65536, 0/16384, 1/16384; maximum stored-component difference 56592.

- **`noise_cells_1_v2.sbs/cells_1` - Too hard for this pass:** Exact pixels by case/output: 236/16384, 1029/65536, 261/16384, 0/16384; maximum stored-component difference 223.

- **`noise_cells_4.sbs/cells_4` - Too hard for this pass:** Exact pixels by case/output: 0/16384, 0/65536, 0/16384, 23/16384; maximum stored-component difference 65462.

- **`noise_cells_4_v2.sbs/cells_4` - Too hard for this pass:** Exact pixels by case/output: 98/16384, 379/65536, 33/16384, 1970/16384; maximum stored-component difference 253.

- **`noise_clouds_1.sbs/clouds_1` - Too hard for this pass:** Exact pixels by case/output: 2/16384, 5/65536, 0/16384, 0/16384; maximum stored-component difference 48017.

- **`noise_clouds_1_v2.sbs/clouds_1` - Too hard for this pass:** Exact pixels by case/output: 229/16384, 792/65536, 186/16384, 242/16384; maximum stored-component difference 156.

- **`noise_clouds_2.sbs/clouds_2` - Too hard for this pass:** Exact pixels by case/output: 0/16384, 4/65536, 2/16384, 0/16384; maximum stored-component difference 44350.

- **`noise_clouds_2_v2.sbs/clouds_2` - Too hard for this pass:** Exact pixels by case/output: 201/16384, 832/65536, 211/16384, 245/16384; maximum stored-component difference 123.

- **`noise_creased.sbs/creased` - Too hard for this pass:** Exact pixels by case/output: 1/16384, 0/65536, 2/16384, 1/16384; maximum stored-component difference 42493.

- **`noise_crystal_1.sbs/crystal_1` - Too hard for this pass:** Exact pixels by case/output: 1/16384, 2/65536, 2/16384, 1/16384; maximum stored-component difference 61334.

- **`noise_crystal_2.sbs/crystal_2` - Too hard for this pass:** Exact pixels by case/output: 0/16384, 3/65536, 0/16384, 1/16384; maximum stored-component difference 51894.

- **`noise_directional_noise_1.sbs/directional_noise_1` - Too hard for this pass:** Exact pixels by case/output: 1/16384, 0/65536, 0/16384, 0/16384; maximum stored-component difference 64802.

- **`noise_directional_noise_1_v2.sbs/directional_noise_1` - Too hard for this pass:** Exact pixels by case/output: 8/16384, 54/65536, 0/16384, 2/16384; maximum stored-component difference 240.

- **`noise_directional_noise_2.sbs/directional_noise_2` - Too hard for this pass:** Cooks report: pattern graph exceeded its execution budget. Exact pixels by case/output: 0/16384, 0/65536, 0/16384; maximum stored-component difference 65535.

- **`noise_directional_noise_3.sbs/directional_noise_3` - Too hard for this pass:** Cooks report: pattern graph exceeded its execution budget.

- **`noise_directional_noise_4.sbs/directional_noise_4` - Too hard for this pass:** Exact pixels by case/output: 0/16384, 1/65536, 0/16384, 0/16384; maximum stored-component difference 47884.

- **`noise_directional_noise_4_v2.sbs/directional_noise_4` - Too hard for this pass:** Exact pixels by case/output: 161/16384, 713/65536, 161/16384, 148/16384; maximum stored-component difference 207.

- **`noise_directional_scratches.sbs/directional_scratches` - Too hard for this pass:** Exact pixels by case/output: 0/16384, 0/65536, 0/16384, 5334/16384; maximum stored-component difference 62981.

- **`noise_directional_scratches_v2.sbs/directional_scratches` - Too hard for this pass:** Exact pixels by case/output: 62/16384, 250/65536, 74/16384, 4096/16384; maximum stored-component difference 239.

- **`noise_dirt_1.sbs/dirt_1` - Too hard for this pass:** Cooks report: pattern graph exceeded its execution budget. Exact pixels by case/output: 1037/16384, 4038/65536, 992/16384; maximum stored-component difference 65535.

- **`noise_fluid.sbs/fluid` - Too hard for this pass:** Exact pixels by case/output: 0/16384, 0/65536, 0/16384, 0/16384; maximum stored-component difference 63791.

- **`noise_fractal_sum_base.sbs/fractal_sum_base` - Too hard for this pass:** Exact pixels by case/output: 2/16384, 6/65536, 1/16384, 0/16384; maximum stored-component difference 21118.

- **`noise_fractal_sum_base.sbs/fractal_sum_base_2` - Too hard for this pass:** Exact pixels by case/output: 2/16384, 6/65536, 1/16384, 0/16384; maximum stored-component difference 20137.

- **`noise_fractal_sum_base_v2.sbs/noise_upscale` - Too hard for this pass:** Exact pixels by case/output: 389/16384, 1536/65536, 389/16384, 279/16384; maximum stored-component difference 15.

- **`noise_fractal_sum_base_v2.sbs/noise_blend` - Too hard for this pass:** Exact pixels by case/output: 134/16384, 668/65536, 134/16384, 75/16384; maximum stored-component difference 100.

- **`noise_fur_1.sbs/fur_1` - Too hard for this pass:** Exact pixels by case/output: 13/16384, 75/65536, 16/16384, 10/16384; maximum stored-component difference 65535.

- **`noise_fur_2.sbs/fur_2` - Too hard for this pass:** Exact pixels by case/output: 0/16384, 0/65536, 0/16384, 0/16384; maximum stored-component difference 65530.

- **`noise_fur_3.sbs/fur_3` - Too hard for this pass:** Exact pixels by case/output: 0/16384, 0/65536, 0/16384, 0/16384; maximum stored-component difference 65529.

- **`noise_gaussian_noise.sbs/gaussian_noise` - Too hard for this pass:** Exact pixels by case/output: 0/16384, 0/65536, 0/16384, 0/16384; maximum stored-component difference 64187.

- **`noise_gaussian_noise_v2.sbs/gaussian_noise` - Too hard for this pass:** Exact pixels by case/output: 4/16384, 16/65536, 6/16384, 0/16384; maximum stored-component difference 251.

- **`noise_gaussian_spots_1.sbs/gaussian_spots_1` - Too hard for this pass:** Exact pixels by case/output: 0/16384, 1/65536, 1/16384, 0/16384; maximum stored-component difference 58938.

- **`noise_gaussian_spots_1_v2.sbs/gaussian_spots_1` - Too hard for this pass:** Exact pixels by case/output: 99/16384, 434/65536, 118/16384, 121/16384; maximum stored-component difference 235.

- **`noise_gaussian_spots_2.sbs/gaussian_spots_2` - Too hard for this pass:** Exact pixels by case/output: 0/16384, 1/65536, 0/16384, 0/16384; maximum stored-component difference 45111.

- **`noise_gaussian_spots_2_v2.sbs/gaussian_spots_2` - Too hard for this pass:** Exact pixels by case/output: 21/16384, 257/65536, 33/16384, 62/16384; maximum stored-component difference 255.

- **`noise_liquid.sbs/liquid` - Too hard for this pass:** Exact pixels by case/output: 0/16384, 0/65536, 0/16384, 0/16384; maximum stored-component difference 61102.

- **`noise_messy_fibers_1.sbs/messy_fibers_1` - Too hard for this pass:** Exact pixels by case/output: 0/16384, 1/65536, 0/16384, 0/16384; maximum stored-component difference 65295.

- **`noise_messy_fibers_1_v2.sbs/messy_fibers_1` - Too hard for this pass:** Exact pixels by case/output: 37/16384, 153/65536, 58/16384, 46/16384; maximum stored-component difference 255.

- **`noise_messy_fibers_2.sbs/messy_fibers_2` - Too hard for this pass:** Exact pixels by case/output: 55/16384, 282/65536, 116/16384, 87/16384; maximum stored-component difference 65535.

- **`noise_messy_fibers_2_v2.sbs/messy_fibers_2` - Too hard for this pass:** Exact pixels by case/output: 163/16384, 686/65536, 95/16384, 213/16384; maximum stored-component difference 255.

- **`noise_messy_fibers_3.sbs/messy_fibers_3` - Too hard for this pass:** Exact pixels by case/output: 78/16384, 304/65536, 77/16384, 63/16384; maximum stored-component difference 65535.

- **`noise_messy_fibers_3_v2.sbs/messy_fibers_3` - Too hard for this pass:** Exact pixels by case/output: 128/16384, 515/65536, 138/16384, 3/16384; maximum stored-component difference 255.

- **`noise_microscope_view.sbs/microscope_view` - Too hard for this pass:** Exact pixels by case/output: 0/16384, 0/65536, 0/16384, 0/16384; maximum stored-component difference 65528.

- **`noise_perlin_noise.sbs/perlin_noise` - Too hard for this pass:** Exact pixels by case/output: 2/16384, 6/65536, 4/16384, 4/16384; maximum stored-component difference 44958.

- **`noise_perlin_noise_v2.sbs/perlin_noise` - Too hard for this pass:** Exact pixels by case/output: 107/16384, 448/65536, 130/16384, 168/16384; maximum stored-component difference 215.

- **`noise_plasma.sbs/plasma` - Too hard for this pass:** Exact pixels by case/output: 0/16384, 3/65536, 1/16384, 0/16384; maximum stored-component difference 41315.

- **`noise_upscale_1.sbs/noise_upscale_1` - Too hard for this pass:** Exact pixels by case/output: 47496/65536, 189800/262144, 47496/65536, 46630/65536; maximum stored-component difference 42.

- **`noise_upscale_2.sbs/noise_upscale_2` - Too hard for this pass:** Exact pixels by case/output: 44458/65536, 177124/262144, 44458/65536, 44586/65536; maximum stored-component difference 23.

- **`noise_upscale_3.sbs/noise_upscale_3` - Too hard for this pass:** Exact pixels by case/output: 54720/65536, 223780/262144, 54720/65536, 55992/65536; maximum stored-component difference 2.

- **`pattern_alveolus.sbs/alveolus` - Too hard for this pass:** Exact pixels by case/output: 0/16384, 0/65536, 0/16384, 13568/16384; maximum stored-component difference 65535.

- **`pattern_arc_pavement.sbs/arc_pavement` - Too hard for this pass:** Exact pixels by case/output: 16384/16384, 65532/65536, 16384/16384, 12457/16384; maximum stored-component difference 65535.

- **`pattern_brick_1.sbs/brick_1` - Too hard for this pass:** Exact pixels by case/output: 14592/16384, 58368/65536, 14592/16384, 6528/16384; maximum stored-component difference 65535.

- **`pattern_brick_2.sbs/brick_2` - Too hard for this pass:** Exact pixels by case/output: 15040/16384, 61056/65536, 15040/16384, 6656/16384; maximum stored-component difference 65535.

- **`pattern_cube_3d.sbs/cube_3d` - Too hard for this pass:** Exact pixels by case/output: 16375/16384, 65369/65536, 16375/16384, 16328/16384; maximum stored-component difference 1.

- **`pattern_cube_3d.sbs/cube_3d_gbuffers` - Too hard for this pass:** Exact pixels by case/output: 16350/16384, 16384/16384, 16375/16384, 16352/16384, 65244/65536, 65536/65536, 65369/65536, 65322/65536, 16350/16384, 16384/16384, 16375/16384, 16352/16384, 16107/16384, 16384/16384, 16328/16384, 16253/16384; maximum stored-component difference 1.

- **`pattern_fibers_1.sbs/fibers_1` - Too hard for this pass:** Exact pixels by case/output: 0/16384, 1/65536, 0/16384, 0/16384; maximum stored-component difference 48873.

- **`pattern_fibers_2.sbs/fibers_2` - Too hard for this pass:** Exact pixels by case/output: 0/16384, 2/65536, 0/16384, 0/16384; maximum stored-component difference 62612.

- **`pattern_gaussian_1.sbs/gaussian_1` - Too hard for this pass:** Exact pixels by case/output: 0/16384, 0/65536, 0/16384, 0/16384; maximum stored-component difference 40180.

- **`pattern_gaussian_2.sbs/gaussian_2` - Too hard for this pass:** Exact pixels by case/output: 0/16384, 0/65536, 0/16384, 0/16384; maximum stored-component difference 23969.

- **`pattern_mesh_1.sbs/mesh_1` - Too hard for this pass:** Exact pixels by case/output: 0/16384, 0/65536, 0/16384, 16/16384; maximum stored-component difference 65535.

- **`pattern_mesh_2.sbs/mesh_2` - Too hard for this pass:** Exact pixels by case/output: 0/16384, 0/65536, 0/16384, 160/16384; maximum stored-component difference 65535.

- **`pattern_polygon_2.sbs/polygon_2` - Too hard for this pass:** Exact pixels by case/output: 14203/16384, 56174/65536, 14203/16384, 13032/16384; maximum stored-component difference 4.

- **`pattern_shape.sbs/shape_base` - Too hard for this pass:** Exact pixels by case/output: 16384/16384, 65536/65536, 16384/16384, 16217/16384; maximum stored-component difference 1.

- **`pattern_shape.sbs/hemisphere` - Too hard for this pass:** Exact pixels by case/output: 16368/16384, 65536/65536, 16368/16384, 16361/16384; maximum stored-component difference 1.

- **`pattern_splatter_circular.sbs/splatter_circular` - Too hard for this pass:** Exact pixels by case/output: 15941/16384, 63805/65536, 15941/16384, 15401/16384; maximum stored-component difference 57311.

- **`pattern_splatter_circular.sbs/splatter_circular_color` - Too hard for this pass:** Exact pixels by case/output: 15529/16384, 62023/65536, 15529/16384, 15401/16384; maximum stored-component difference 65535.

- **`pattern_stripes.sbs/stripes` - Too hard for this pass:** Exact pixels by case/output: 16384/16384, 45056/65536, 16384/16384, 16384/16384; maximum stored-component difference 9557.

- **`pattern_tile_generator.sbs/tile_generator` - Too hard for this pass:** Exact pixels by case/output: 13097/16384, 52436/65536, 13097/16384, 16384/16384; maximum stored-component difference 18088.

- **`pattern_tile_generator.sbs/tile_generator_color` - Too hard for this pass:** Exact pixels by case/output: 10819/16384, 41641/65536, 10819/16384, 16384/16384; maximum stored-component difference 23978.

- **`pattern_tile_random.sbs/tile_random` - Too hard for this pass:** Exact pixels by case/output: 13532/16384, 53254/65536, 13610/16384, 11762/16384; maximum stored-component difference 57311.

- **`pattern_tile_random.sbs/tile_random_color` - Too hard for this pass:** Exact pixels by case/output: 13532/16384, 53254/65536, 13610/16384, 11762/16384; maximum stored-component difference 65535.

- **`pattern_tile_random_2.sbs/tile_random_2` - Too hard for this pass:** Exact pixels by case/output: 16378/16384, 4788/16384, 65497/65536, 18310/65536, 16368/16384, 5261/16384, 16353/16384, 12087/16384; maximum stored-component difference 1.

- **`pattern_weave_1.sbs/weave_1` - Too hard for this pass:** Exact pixels by case/output: 928/16384, 3872/65536, 928/16384, 848/16384; maximum stored-component difference 65535.

- **`pattern_weave_2.sbs/weave_2` - Too hard for this pass:** Exact pixels by case/output: 256/16384, 2304/65536, 256/16384, 39/16384; maximum stored-component difference 65535.

- **`pbr_render.sbs/PBR_render_mapping_color` - Too hard for this pass:** Exact pixels by case/output: 16223/16384, 65081/65536, 16223/16384, 16123/16384; maximum stored-component difference 1.

- **`pbr_render.sbs/PBR_render_mapping_grayscale` - Too hard for this pass:** Exact pixels by case/output: 15835/16384, 63262/65536, 15835/16384, 16170/16384; maximum stored-component difference 1.

- **`pbr_render.sbs/environment_map_rescale` - Too hard for this pass:** Exact pixels by case/output: 1/16384, 2/65536, 1/16384, 0/16384; maximum stored-component difference 2.551079e-05.

- **`pbr_render.sbs/position_to_normal` - Too hard for this pass:** Exact pixels by case/output: 3119/16384, 4234/65536, 3119/16384, 1153/16384; maximum stored-component difference 0.1367188.

- **`pbr_render.sbs/chroma_shift` - Too hard for this pass:** Exact pixels by case/output: 16083/16384, 64304/65536, 16083/16384, 15570/16384; maximum stored-component difference 1.

- **`pbr_render.sbs/get_max_coords/get_max_coord` - Too hard for this pass:** Cooks report: Get Float2 (1379828382): undefined variable `$pos` in this context.

- **`pbr_render.sbs/ibl_specular_contrib_anisotropic/panoramic_lod` - Too hard for this pass:** Exact pixels by case/output: 0/16384, 0/65536, 0/16384, 0/16384; maximum stored-component difference 0.98973.

- **`pbr_render2.sbs/PBR_render_mapping_color` - Too hard for this pass:** Exact pixels by case/output: 16223/16384, 65081/65536, 16223/16384, 16123/16384; maximum stored-component difference 1.

- **`pbr_render2.sbs/PBR_render_mapping_grayscale` - Too hard for this pass:** Exact pixels by case/output: 15835/16384, 63262/65536, 15835/16384, 16170/16384; maximum stored-component difference 1.

- **`pbr_render2.sbs/environment_map_rescale` - Too hard for this pass:** Exact pixels by case/output: 1/16384, 2/65536, 1/16384, 0/16384; maximum stored-component difference 2.551079e-05.

- **`pbr_render2.sbs/position_to_normal` - Too hard for this pass:** Exact pixels by case/output: 8615/16384, 31600/65536, 8615/16384, 6737/16384; maximum stored-component difference 0.0004882812.

- **`pbr_render2.sbs/chroma_shift` - Too hard for this pass:** Exact pixels by case/output: 16083/16384, 64304/65536, 16083/16384, 15570/16384; maximum stored-component difference 1.

- **`pbr_render2.sbs/get_max_coords/get_max_coord` - Too hard for this pass:** Cooks report: Get Float2 (1379828382): undefined variable `$pos` in this context.

- **`pbr_render2.sbs/ibl_specular_contrib_anisotropic/panoramic_lod` - Too hard for this pass:** Exact pixels by case/output: 42/16384, 1690/65536, 42/16384, 741/16384; maximum stored-component difference 155.

- **`pbr_render2.sbs/lod` - Too hard for this pass:** Exact pixels by case/output: 12328/16384, 49408/65536, 12328/16384, 12632/16384; maximum stored-component difference 127.

- **`pbr_render_ortho.sbs/mipmap_generator` - Too hard for this pass:** Exact pixels by case/output: 0/16384, 0/65536, 0/16384, 0/16384; maximum stored-component difference 2.687117.

- **`pbr_render_ortho.sbs/rotate_environment` - Too hard for this pass:** Exact pixels by case/output: 13885/16384, 48478/65536, 13885/16384, 11791/16384; maximum stored-component difference 7.

- **`pbr_render_ortho.sbs/blue_noise` - Too hard for this pass:** Exact pixels by case/output: 133/16384, 489/65536, 133/16384, 144/16384; maximum stored-component difference 3.874302e-07.

- **`pbr_render_ortho.sbs/environment_map_rescale` - Too hard for this pass:** Exact pixels by case/output: 1/16384, 2/65536, 1/16384, 0/16384; maximum stored-component difference 2.551079e-05.

- **`pbr_render_ortho.sbs/vector_shadow_pass` - Too hard for this pass:** Exact pixels by case/output: 14758/16384, 60408/65536, 14758/16384, 14762/16384; maximum stored-component difference 1.149848.

- **`noise_directional_noise_2_v2.sbs/directional_noise_2` - Too hard for this pass:** The release CPU comparison did not finish its four-case batch within the 180-second per-graph allowance. Reference captures completed; no pixel-parity claim is made for this timed-out run.

- **`noise_directional_noise_3_v2.sbs/directional_noise_3` - Too hard for this pass:** The release CPU comparison did not finish its four-case batch within the 180-second per-graph allowance. Reference captures completed; no pixel-parity claim is made for this timed-out run.

- **`noise_dirt_1_v2.sbs/dirt_1` - Too hard for this pass:** The release CPU comparison did not finish its four-case batch within the 180-second per-graph allowance. Reference captures completed; no pixel-parity claim is made for this timed-out run.

- **`noise_moisture_noise.sbs/moisture_noise` - Too hard for this pass:** The release CPU comparison did not finish its four-case batch within the 180-second per-graph allowance. Reference captures completed; no pixel-parity claim is made for this timed-out run.

- **`noise_moisture_noise_1_v2.sbs/moisture_noise_1` - Too hard for this pass:** The release CPU comparison did not finish its four-case batch within the 180-second per-graph allowance. Reference captures completed; no pixel-parity claim is made for this timed-out run.

- **`noise_moisture_noise_2.sbs/moisture_noise_2` - Too hard for this pass:** The release CPU comparison did not finish its four-case batch within the 180-second per-graph allowance. Reference captures completed; no pixel-parity claim is made for this timed-out run.

- **`pattern_triangle_grid.sbs/triangle_grid_base` - Too hard for this pass:** The release CPU comparison did not finish its four-case batch within the 180-second per-graph allowance. Reference captures completed; no pixel-parity claim is made for this timed-out run.

- **`noise_voronoi.sbs/3d_voronoi_fractal` - Too hard for this pass:** All four 16x16/32x32 cases now cook after the unassigned-counter fix. Exact pixels by case/output: 232/256, 950/1024, 245/256, 253/256; maximum stored-component difference 75.

- **`noise_voronoi.sbs/3d_voronoi` - Too hard for this pass:** All four 16x16/32x32 cases now cook after the unassigned-counter fix. Exact pixels by case/output: 238/256, 968/1024, 240/256, 255/256; maximum stored-component difference 1.

- **`noise_voronoi.sbs/voronoi` - Too hard for this pass:** All four 16x16/32x32 cases now cook after the unassigned-counter fix. Exact pixels by case/output: 225/256, 888/1024, 175/256, 185/256; maximum stored-component difference 178.

- **`noise_voronoi.sbs/voronoi_fractal` - Too hard for this pass:** All four 16x16/32x32 cases now cook after the unassigned-counter fix. Exact pixels by case/output: 124/256, 502/1024, 87/256, 179/256; maximum stored-component difference 36916.

- **`pattern_tile_sampler.sbs/arrow` - Too hard for this pass:** The importer drops the explicit 512x512 graph-level output size. Manually aligning both graph sizes still leaves rendering differences. These three captures use 512x512 output. Exact pixels by case/output: 259850/262144, 259850/262144, 259850/262144; maximum stored-component difference 155.

- **`pbr_render_ortho.sbs/ibl_shadow_pass` - Too hard for this pass:** Exact pixels by case/output: 0/16384, 0/65536, 0/16384, 0/16384; maximum stored-component difference 31.45882.

- **`composite.sbs/composite/blends/blend_matte` - Too hard for this pass:** Exact pixels by case/output: 90/16384, 359/65536, 90/16384, 118/16384; maximum stored-component difference 196.

- **`noise_dirt_2.sbs/dirt_2` - Too hard for this pass:** Cooks report: pattern graph exceeded its execution budget. Exact pixels by case/output: 14450/16384, 57939/65536, 14517/16384; maximum stored-component difference 65535.

- **`noise_dirt_3.sbs/dirt_3` - Too hard for this pass:** Exact pixels by case/output: 375/16384, 0/65536, 326/16384, 259/16384; maximum stored-component difference 65535.

- **`noise_dirt_3_v2.sbs/dirt_3` - Too hard for this pass:** Exact pixels by case/output: 363/16384, 0/65536, 427/16384, 94/16384; maximum stored-component difference 255.

- **`noise_dirt_5.sbs/dirt_5` - Too hard for this pass:** Cooks report: pattern graph exceeded its execution budget. Exact pixels by case/output: 2184/16384, 6519/65536, 2184/16384; maximum stored-component difference 61798.

- **`noise_dirt_5_v2.sbs/dirt_5` - Too hard for this pass:** Cooks report: pattern graph exceeded its execution budget. Exact pixels by case/output: 1988/16384, 5879/65536, 1997/16384; maximum stored-component difference 218.

- **`noise_dirt_gradient.sbs/dirt_gradient` - Too hard for this pass:** Exact pixels by case/output: 0/16384, 2/65536, 0/16384, 0/16384; maximum stored-component difference 62840.

- **`perlin_noise_zoom.sbs/perlin_noise_zoom` - Too hard for this pass:** Exact pixels by case/output: 0/16384, 0/65536, 0/16384, 0/16384; maximum stored-component difference 62227.

- **`pre_multiply_straight.sbs/premult_to_straight` - Too hard for this pass:** Exact pixels by case/output: 16376/16384, 65504/65536, 16376/16384, 16381/16384; maximum stored-component difference 255.

- **`quad_transform.sbs/quad_transform` - Too hard for this pass:** Exact pixels by case/output: 16384/16384, 65536/65536, 16384/16384, 15477/16384; maximum stored-component difference 1.

- **`quad_transform.sbs/quad_transform_grayscale` - Too hard for this pass:** Exact pixels by case/output: 16384/16384, 65536/65536, 16384/16384, 15846/16384; maximum stored-component difference 1.

- **`quantize_color.sbs/modify_color_palette_sub` - Too hard for this pass:** Exact pixels by case/output: 240/256, 240/256, 240/256, 240/256; maximum stored-component difference 0.7294117.

- **`quantize_color.sbs/kmeans_iteration` - Too hard for this pass:** Exact pixels by case/output: 4080/4096, 4080/4096, 4080/4096, 4080/4096; maximum stored-component difference 6.023682.

- **`quantize_color.sbs/get_min_max/GetMaxFromAlpha_v2` - Too hard for this pass:** Exact pixels by case/output: 0/1, 1/1, 0/1, 0/1; maximum stored-component difference 128.

- **`quantize_color.sbs/color_conversions/convert_RGB_to_Lab` - Too hard for this pass:** Exact pixels by case/output: 16163/16384, 64702/65536, 16163/16384, 16163/16384; maximum stored-component difference 1.

- **`quantize_color.sbs/color_conversions/convert_Lab_to_RGB` - Too hard for this pass:** Exact pixels by case/output: 16377/16384, 65504/65536, 16377/16384, 16383/16384; maximum stored-component difference 255.

- **`radial_blur.sbs/radial_blur_grayscale` - Too hard for this pass:** Exact pixels by case/output: 48/16384, 200/65536, 48/16384, 19/16384; maximum stored-component difference 2355.

- **`replace_color.sbs/replace_color` - Too hard for this pass:** Exact pixels by case/output: 16384/16384, 65536/65536, 16384/16384, 1/16384; maximum stored-component difference 64.

- **`rt_caustics.sbs/rt_caustics_base` - Too hard for this pass:** Exact pixels by case/output: 282/16384, 1219/65536, 282/16384, 185/16384; maximum stored-component difference 242.

- **`rt_irradiance.sbs/panoramic_lod` - Too hard for this pass:** Exact pixels by case/output: 4979/16384, 20210/65536, 4979/16384, 5976/16384; maximum stored-component difference 155.

- **`safe_transform.sbs/safe_transform` - Too hard for this pass:** Exact pixels by case/output: 16384/16384, 65536/65536, 16384/16384, 3649/16384; maximum stored-component difference 145.

- **`safe_transform.sbs/safe_transform_grayscale` - Too hard for this pass:** Exact pixels by case/output: 16384/16384, 65536/65536, 16384/16384, 6679/16384; maximum stored-component difference 132.

- **`scratches_1.sbs/scratches_1` - Too hard for this pass:** Exact pixels by case/output: 3025/16384, 12465/65536, 2995/16384, 2178/16384; maximum stored-component difference 64929.

- **`scratches_2.sbs/scratches_2` - Too hard for this pass:** Exact pixels by case/output: 431/16384, 567/65536, 461/16384, 291/16384; maximum stored-component difference 65534.

- **`scratches_4.sbs/scratches_4` - Too hard for this pass:** Exact pixels by case/output: 97/16384, 1946/65536, 96/16384; maximum stored-component difference 65535.

- **`shape.sbs/shape_base` - Too hard for this pass:** Exact pixels by case/output: 16384/16384, 65536/65536, 16384/16384, 16217/16384; maximum stored-component difference 1.

- **`shape_mapper.sbs/shape_mapper` - Too hard for this pass:** Exact pixels by case/output: 7688/16384, 28865/65536, 7688/16384, 5078/16384; maximum stored-component difference 30712.

- **`shape_splatter.sbs/shape_splatter_blend_color` - Too hard for this pass:** Exact pixels by case/output: 16384/16384, 65536/65536, 16384/16384, 16383/16384; maximum stored-component difference 1.

- **`shape_splatter.sbs/shape_splatter_blend` - Too hard for this pass:** Exact pixels by case/output: 16379/16384, 65532/65536, 16379/16384, 16319/16384; maximum stored-component difference 36.

- **`shape_splatter.sbs/shape_splatter_data_extract` - Too hard for this pass:** Exact pixels by case/output: 12131/16384, 4638/16384, 16384/16384, 16384/16384, 16384/16384, 16384/16384, 16384/16384, 48527/65536, 19321/65536, 65536/65536, 65536/65536, 65536/65536, 65536/65536, 65536/65536, 12131/16384, 4638/16384, 16384/16384, 16384/16384, 16384/16384, 16384/16384, 16384/16384, 12151/16384, 4862/16384, 16384/16384, 16384/16384, 16384/16384, 16384/16384, 16384/16384; maximum stored-component difference 1.192093e-07.

- **`shape_splatter_v2.sbs/mipmap_tools/Mipmap_N_from_O_color` - Too hard for this pass:** Exact pixels by case/output: 28672/32768, 114688/131072, 28672/32768, 15872/16384; maximum stored-component difference 255.

- **`shape_splatter_v2.sbs/shape_splatter_compute_normal` - Too hard for this pass:** Exact pixels by case/output: 128/16384, 256/65536, 128/16384, 128/16384; maximum stored-component difference 255.

- **`skew.sbs/skew` - Too hard for this pass:** Exact pixels by case/output: 16384/16384, 65536/65536, 16384/16384, 14625/16384; maximum stored-component difference 1.

- **`skew.sbs/skew_grayscale` - Too hard for this pass:** Exact pixels by case/output: 16384/16384, 65536/65536, 16384/16384, 15263/16384; maximum stored-component difference 1.

- **`slope_blur.sbs/slope_blur` - Too hard for this pass:** Exact pixels by case/output: 0/16384, 0/65536, 0/16384, 0/16384; maximum stored-component difference 160.

- **`slope_blur.sbs/slope_blur_grayscale` - Too hard for this pass:** Exact pixels by case/output: 296/16384, 2588/65536, 296/16384, 217/16384; maximum stored-component difference 124.

- **`slope_blur.sbs/slope_blur_grayscale_2` - Too hard for this pass:** Exact pixels by case/output: 296/16384, 2588/65536, 296/16384, 217/16384; maximum stored-component difference 124.

- **`smart_auto_tile.sbs/pattern_find_edges_step` - Too hard for this pass:** Exact pixels by case/output: 1048574/1048576, 1048575/1048576, 1048574/1048576, 1024/1024; maximum stored-component difference 255.

- **`splatter_circular.sbs/splatter_circular` - Too hard for this pass:** Exact pixels by case/output: 16384/16384, 65536/65536, 16384/16384, 15750/16384; maximum stored-component difference 57311.

- **`splatter_circular.sbs/splatter_circular_color` - Too hard for this pass:** Exact pixels by case/output: 14564/16384, 57648/65536, 14564/16384, 13974/16384; maximum stored-component difference 65535.

- **`spot_emboss.sbs/spot_emboss` - Too hard for this pass:** Exact pixels by case/output: 460/16384, 2680/65536, 460/16384, 163/16384; maximum stored-component difference 255.

- **`starburst.sbs/starburst` - Too hard for this pass:** Exact pixels by case/output: 8952/16384, 35696/65536, 8952/16384, 12412/16384; maximum stored-component difference 58561.

- **`stripes.sbs/stripes` - Too hard for this pass:** Exact pixels by case/output: 16384/16384, 45056/65536, 16384/16384, 16350/16384; maximum stored-component difference 65535.

- **`swirl.sbs/swirl` - Too hard for this pass:** Exact pixels by case/output: 16186/16384, 64854/65536, 16186/16384, 15722/16384; maximum stored-component difference 2.

- **`swirl.sbs/swirl_grayscale` - Too hard for this pass:** Exact pixels by case/output: 16274/16384, 65132/65536, 16274/16384, 16007/16384; maximum stored-component difference 2.

- **`symmetry_slice.sbs/symmetry_slice` - Too hard for this pass:** Exact pixels by case/output: 16384/16384, 65536/65536, 16384/16384, 11619/16384; maximum stored-component difference 7.

- **`tile_generator.sbs/tile_generator` - Too hard for this pass:** Exact pixels by case/output: 12821/16384, 51231/65536, 12821/16384, 14592/16384; maximum stored-component difference 18088.

- **`tile_generator_color.sbs/tile_generator_color` - Too hard for this pass:** Exact pixels by case/output: 10819/16384, 41641/65536, 10819/16384, 13363/16384; maximum stored-component difference 23978.

- **`tile_random.sbs/tile_random` - Too hard for this pass:** Exact pixels by case/output: 13932/16384, 53844/65536, 13189/16384, 11424/16384; maximum stored-component difference 57311.

- **`tile_random.sbs/tile_random_color` - Too hard for this pass:** Exact pixels by case/output: 13932/16384, 53844/65536, 13189/16384, 11424/16384; maximum stored-component difference 65535.

- **`tile_sampler.sbs/arrow` - Too hard for this pass:** Exact pixels by case/output: 259850/262144, 259850/262144, 259850/262144; maximum stored-component difference 155.

- **`tileable_material_to_mesh_material.sbs/tileable_material_to_mesh_material` - Too hard for this pass:** Exact pixels by case/output: 16384/16384, 16384/16384, 65536/65536, 65536/65536, 16384/16384, 16384/16384, 16384/16384, 4460/16384; maximum stored-component difference 4.

- **`trapezoid_transform.sbs/trapezoid_transform` - Too hard for this pass:** Exact pixels by case/output: 16384/16384, 65536/65536, 16384/16384, 16382/16384; maximum stored-component difference 1.

- **`trapezoid_transform.sbs/trapezoid_transform_grayscale` - Too hard for this pass:** Exact pixels by case/output: 16384/16384, 65536/65536, 16384/16384, 16382/16384; maximum stored-component difference 1.

- **`uv_mapper.sbs/uv_mapper_grayscale_sub` - Too hard for this pass:** Exact pixels by case/output: 15835/16384, 63262/65536, 15835/16384, 16384/16384; maximum stored-component difference 1.

- **`uv_mapper.sbs/uv_mapper_color_sub` - Too hard for this pass:** Exact pixels by case/output: 16223/16384, 65081/65536, 16223/16384, 16384/16384; maximum stored-component difference 1.

- **`vector_morph.sbs/vector_morph` - Too hard for this pass:** Exact pixels by case/output: 2840/16384, 15375/65536, 2840/16384, 3262/16384; maximum stored-component difference 250.

- **`vector_morph.sbs/vector_morph_grayscale` - Too hard for this pass:** Exact pixels by case/output: 1277/16384, 4578/65536, 1277/16384, 1540/16384; maximum stored-component difference 146.

- **`vertical_noise.sbs/vertical_noise` - Too hard for this pass:** Exact pixels by case/output: 0/16384, 1/65536, 0/16384, 0/16384; maximum stored-component difference 51100.

- **`waveform_01_v2.sbs/waveform_01_v2` - Too hard for this pass:** Exact pixels by case/output: 12757/16384, 57538/65536, 12257/16384, 15190/16384; maximum stored-component difference 255.

- **`waveform_1.sbs/waveform_1` - Too hard for this pass:** Exact pixels by case/output: 10166/16384, 44500/65536, 10112/16384, 16046/16384; maximum stored-component difference 65535.

- **`weave_1.sbs/weave_1` - Too hard for this pass:** Exact pixels by case/output: 928/16384, 3872/65536, 928/16384, 848/16384; maximum stored-component difference 65535.

- **`weave_2.sbs/weave_2` - Too hard for this pass:** Exact pixels by case/output: 0/16384, 0/65536, 0/16384, 2/16384; maximum stored-component difference 64614.

- **`weave_3.sbs/weave_3` - Too hard for this pass:** Exact pixels by case/output: 15760/16384, 63040/65536, 15760/16384, 14816/16384; maximum stored-component difference 64255.

- **`weave_4.sbs/weave_4` - Too hard for this pass:** Exact pixels by case/output: 10240/16384, 40960/65536, 10240/16384, 9728/16384; maximum stored-component difference 64511.

- **`white_noise.sbs/white_noise_fast` - Too hard for this pass:** Exact pixels by case/output: 0/16384, 2/65536, 1/16384; maximum stored-component difference 65348.

- **`white_noise_v2.sbs/white_noise_v2` - Too hard for this pass:** Exact pixels by case/output: 68/16384, 298/65536, 70/16384, 153/16384; maximum stored-component difference 255.

- **`wood_fibers_1.sbs/wood_fibers_1` - Too hard for this pass:** Exact pixels by case/output: 3/16384, 10/65536, 0/16384, 4/16384; maximum stored-component difference 65495.

- **`wood_fibers_2.sbs/wood_fibers_2` - Too hard for this pass:** Cooks report: pattern graph exceeded its execution budget.

- **`paths_tools.sbs/paths_polygon` - Too hard for this pass:** Exact pixels by case/output: 252/256, 252/256, 252/256, 250/256; maximum stored-component difference 5.602837e-06.

- **`paths_tools.sbs/paths_trace` - Too hard for this pass:** Exact pixels by case/output: 184/256, 182/256, 184/256, 138/256; maximum stored-component difference 145.

- **`paths_tools.sbs/vertex_processor` - Too hard for this pass:** Exact pixels by case/output: 250/256, 248/256, 250/256, 248/256; maximum stored-component difference 1.

- **`paths_tools.sbs/vertex_processor_simple` - Too hard for this pass:** Exact pixels by case/output: 250/256, 248/256, 250/256, 248/256; maximum stored-component difference 1.

- **`paths_tools.sbs/Impl/compute_paths_lengths` - Too hard for this pass:** Cooks report: function returned bool, expected native type 256.

- **`paths_tools.sbs/Impl/sort_by_blue` - Too hard for this pass:** Cooks report: function returned bool, expected native type 256. Exact pixels by case/output: 256/256, 256/256, 256/256; maximum stored-component difference 0.

- **`paths_tools.sbs/Impl/mask_to_facets` - Too hard for this pass:** Exact pixels by case/output: 256/256, 240/256, 256/256, 247/256; maximum stored-component difference 2.980232e-08.

- **`spline_tools.sbs/internal_tools/2dtransform_spline_sub` - Too hard for this pass:** Its connectable spline-count input defaults to zero. Nonempty image buffers are available, but a nonzero value-pin case is not verified through the reference API in this pass.

- **`spline_tools.sbs/internal_tools/compute_bridge_distance` - Too hard for this pass:** Its connectable spline-count input defaults to zero. A meaningful nonempty case needs a configured value input; that case is not verified in this pass.

- **`spline_tools.sbs/internal_tools/points_preview` - Too hard for this pass:** Its connectable point-count input defaults to zero. Empty-preview agreement cannot establish parity for drawing a nonempty point list.

- **`spline_tools.sbs/internal_tools/scatter_splines_on_splines_preprocess` - Too hard for this pass:** The reference compute did not finish after more than six minutes with the default zero input-spline count and original curve data. The comparison session was recovered; nonzero value-input behavior is not verified.

- **`spline_tools.sbs/internal_tools/compute_spline_tangents` - Too hard for this pass:** Exact pixels by case/output: 247/256, 249/256, 247/256, 248/256; maximum stored-component difference 0.1875924.

- **`spline_tools.sbs/internal_tools/normalize_spline_distance` - Too hard for this pass:** Exact pixels by case/output: 193/256, 193/256, 193/256, 193/256; maximum stored-component difference 1.907349e-06.

- **`spline_tools.sbs/internal_tools/compute_spline_length_init` - Too hard for this pass:** Exact pixels by case/output: 251/256, 250/256, 251/256, 250/256; maximum stored-component difference 1.490116e-08.

- **`spline_tools.sbs/internal_tools/resample_spline_data` - Too hard for this pass:** Exact pixels by case/output: 220/256, 208/256, 220/256, 208/256, 220/256, 208/256, 220/256, 208/256; maximum stored-component difference 0.5.

- **`spline_tools.sbs/internal_tools/spline_warp_sub` - Too hard for this pass:** Exact pixels by case/output: 253/256, 253/256, 253/256, 240/256; maximum stored-component difference 0.1294118.

- **`spline_tools.sbs/internal_tools/select_spline_sub` - Too hard for this pass:** Exact pixels by case/output: 30720/32768, 30720/32768, 30720/32768, 30720/32768; maximum stored-component difference 0.5240468.

- **`spline_tools.sbs/internal_tools/modify_spline_property_sub` - Too hard for this pass:** Exact pixels by case/output: 15/256, 15/256, 15/256, 240/256; maximum stored-component difference 0.2784314.

- **`paths_tools.sbs/Impl/facets_to_paths_impl_link_template` - Too hard for this pass:** Exact pixels by case/output: 0/256, 0/256, 0/256, 256/256; maximum stored-component difference 1.

- **`paths_tools.sbs/Impl/facets_to_paths_impl_stitch_template` - Too hard for this pass:** Exact pixels by case/output: 0/256, 0/256, 0/256, 0/256; maximum stored-component difference 1.

- **`pavement_1.sbs/pavement_1` - Too hard for this pass:** Exact pixels by case/output: 2160/16384, 9712/65536, 2160/16384, 800/16384; maximum stored-component difference 65535.

- **`splatter.sbs/splatter` - Too hard for this pass:** Exact pixels by case/output: 2048/16384, 4096/65536, 2048/16384, 966/16384; maximum stored-component difference 44975.

- **`splatter_color.sbs/splatter_color` - Too hard for this pass:** Exact pixels by case/output: 0/16384, 0/65536, 0/16384, 924/16384; maximum stored-component difference 47536.

- **`noise_dirt_4.sbs/dirt_4` - Too hard for this pass:** The MaterialCraft measurement exceeded the 180-second limit. Reference captures exist; exact pixel parity remains unverified.

- **`noise_dirt_4_v2.sbs/dirt_4` - Too hard for this pass:** The MaterialCraft measurement exceeded the 180-second limit. Reference captures exist; exact pixel parity remains unverified.

- **`noise_dirt_gradient_v2.sbs/dirt_gradient` - Too hard for this pass:** The MaterialCraft measurement exceeded the 180-second limit. Reference captures exist; exact pixel parity remains unverified.

- **`pbr_render.sbs/raytrace_position` - Too hard for this pass:** The MaterialCraft measurement exceeded the 180-second limit. Reference captures exist; exact pixel parity remains unverified.

- **`pbr_render.sbs/raytraced_specular_pass_8` - Too hard for this pass:** The MaterialCraft measurement exceeded the 180-second limit. Reference captures exist; exact pixel parity remains unverified.

- **`pbr_render.sbs/raytraced_irradiance_pass_8` - Too hard for this pass:** The MaterialCraft measurement exceeded the 180-second limit. Reference captures exist; exact pixel parity remains unverified.

- **`pbr_render2.sbs/raytrace_position` - Too hard for this pass:** The MaterialCraft measurement exceeded the 180-second limit. Reference captures exist; exact pixel parity remains unverified.

- **`pbr_render2.sbs/raytraced_specular_pass_8` - Too hard for this pass:** The MaterialCraft measurement exceeded the 180-second limit. Reference captures exist; exact pixel parity remains unverified.

- **`pbr_render2.sbs/raytraced_irradiance_is_pass_8` - Too hard for this pass:** The MaterialCraft measurement exceeded the 180-second limit. Reference captures exist; exact pixel parity remains unverified.

- **`rt_irradiance.sbs/mip_max` - Too hard for this pass:** The reference output is 128x128 while MaterialCraft produces 64x64 for the same 128x128 case. Pixel parity cannot be claimed with different output dimensions.

- **`rt_shadows.sbs/mip_max` - Too hard for this pass:** The reference output is 128x128 while MaterialCraft produces 64x64 for the same 128x128 case. Pixel parity cannot be claimed with different output dimensions.

- **`pbr_render_ortho.sbs/sphere_displace` - Too hard for this pass:** The MaterialCraft measurement exceeded the 180-second limit. Reference captures exist; exact pixel parity remains unverified.

- **`rt_irradiance.sbs/compute_ray_direction` - Too hard for this pass:** Exact pixels by case/output: 16368/16384, 65414/65536, 16368/16384; maximum stored-component difference 0.0004882812. Connectable value pins retain their defaults; nondefault values require a wired reference fixture and remain unverified.

- **`rt_irradiance.sbs/compute_irradiance` - Too hard for this pass:** Exact pixels by case/output: 7010/16384, 27432/65536, 7010/16384; maximum stored-component difference 0.001953125. Connectable value pins retain their defaults; nondefault values require a wired reference fixture and remain unverified.

- **`rt_irradiance.sbs/debug_hit_uv` - Too hard for this pass:** Exact pixels by case/output: 67/16384, 260/65536, 67/16384; maximum stored-component difference 159. Connectable value pins retain their defaults; nondefault values require a wired reference fixture and remain unverified.

## Verified cases

- **`3d_perlin_noise.sbs/3D_perlin_noise_fractal` - Parity:** All 114,688 eight-bit output pixels match across four cases: defaults at 128x128 and 256x256, seed 42 at 128x128, and roughness 0.7 at 128x128. Generated linear XY position ramps use Z=128/255 and alpha=1. Parent precision is eight bits. Other parameters retain defaults. This validates these cases, not every parameter combination.

- **`white_noise.sbs/white_noise` - Parity:** All 98,304 sixteen-bit pixels match across defaults at 128x128/256x256 and seed 42. This preserves the original result after the shared importer, sampler, and function changes.

- **`3d_simplex_noise.sbs/3d_simplex_noise` - Parity:** All 114,688 eight-bit pixels match for defaults at 128x128 and 256x256, seed 42 at 128x128, and scale 4 at 128x128. Generated position maps match the other 3D-noise comparisons.

- **`3d_texture_jump_flood.sbs/3d_texture_jump_flood_iteration` - Parity:** All 131,072 eight-bit RGBA pixels match across ramp inputs at 128x128 and 256x256, iteration 1 at 128x128, and two randomized 128x128 candidate-point cases (iterations 0 and 1, with disabled seeds). Floored modulo fixes the previously incorrect negative-coordinate wrapping.

- **`3d_texture_position.sbs/3d_texture_position` - Parity:** Both native 4096x4096 RGBA16 atlases match exactly: default grid 16x16 and grid 8x4. All 33,554,432 pixels (134,217,728 components) were compared at the graph's actual fixed output size, with no resizing.

- **`3d_texture_transform.sbs/3d_texture_offset_color` - Parity:** All 131,072 RGBA8 pixels match across five cases: default 128x128, fractional signed offsets at 128x128, voxel offsets at 256x256, changed grid 8x4 with offsets at 128x128, and varying input alpha with fractional offsets. All four channels were compared.

- **`3d_texture_transform.sbs/3d_texture_offset_grayscale` - Parity:** All 114,688 eight-bit pixels match across four corresponding default, fractional-offset, voxel-offset, and changed-grid cases at 128x128 and 256x256.

- **`angle_to_vector.sbs/angle_to_vector` - Parity:** All 114,688 RGBA8 pixels match across defaults at 128x128/256x256, signed-vector format, and angle offset 0.25. Inputs are original scalar angle maps; all four output channels were compared.

- **`atlas_spliter.sbs/max_ba` - Parity:** All 98,304 eight-bit output pixels match across 128x128/256x256 color inputs and a varying-alpha 128x128 case. The varying-alpha output is nonconstant.

- **`auto_crop.sbs/min_max` - Parity:** All 24,576 RGBA8 output pixels match for original rectangular masks at parent sizes 128 and 256 and a varying-alpha color case. Actual outputs are 64x64, 128x128, and 64x64 respectively.

- **`auto_levels.sbs/auto_levels` - Parity:** All 86,016 eight-bit pixels match for restricted-range inputs at 128x128/256x256 and a constant 64x64 image. The nonconstant inputs span levels 32 through 191; the flat input is level 96.

- **`auto_levels.sbs/min_max` - Parity:** All 102,400 eight-bit pixels match across minimum/maximum selection at 128x128, minimum selection at 256x256, and a flat 64x64 input.

- **Exact blend family - Parity:** `blend.sbs/add`, `add_sub`, `copy`, `difference`, `exclusion`, `hard_light`, `linear_burn`, `max`, `min`, `multiply`, `overlay`, `screen`, `soft_light`, and `subtract` each match all 114,688 RGBA8 pixels across four cases: opaque 128x128, varying foreground/background alpha at 128x128, opacity 0.37 with alpha-channel blending disabled, and 256x256 varying-alpha inputs with opacity 0.67 and seed 42. Original masks and all four output channels were compared. Add and Copy additionally match their 16x16 RGBA32F HDR cases (512 pixels total).

- **`blend_switch.sbs/multi_switch` - Parity:** All 131,072 RGBA8 pixels match across default selection, twentieth input, eleventh of twelve inputs, out-of-range selection with three active inputs, and second input at 256x256. Original twenty-input maps include varying alpha.

- **`blend_switch.sbs/switch` - Parity:** Both boolean selections at 128x128 and 256x256 match all 163,840 RGBA8 pixels, including varying alpha.

- **Grayscale switches - Parity:** `blend_switch.sbs/multi_switch_grayscale` matches all 131,072 eight-bit pixels in the same five selection cases as its color counterpart. `switch_grayscale` matches both boolean selections at 128x128/256x256 (163,840 eight-bit pixels) and both selections at 128x128 in float storage (32,768 pixels). Original color resources exercise the corrected bridge conversion.

- **`brick_generator.sbs/resources/Tile` - Parity:** Defaults at 128x128/256x256 and seed 42 are exact but constant. A seven-by-three grid with scale 0.8 and offset 0.5 produces a nonconstant result and also matches exactly. All 114,688 sixteen-bit pixels across four cases agree.

- **`checker_2.sbs/checker_2` - Parity:** All 114,688 eight-bit pixels match for defaults at 128x128/256x256, seed 42, and changed tiling/luminance variation/disorder at 128x128.

- **`clamp.sbs/clamp_grayscale` - Parity:** All 114,688 eight-bit pixels match across defaults at 128x128/256x256, seed 42, and changed clamp limits.

- **`clone_patch.sbs/transform_to_center` - Parity:** All 114,688 RGBA8 pixels match across defaults at 128x128/256x256, seed 42, and changed transform settings. Default all-one matrix settings produce black; the changed nonsingular matrix/offset produces a nonconstant output and is also exact.

- **`clone_patch.sbs/transform_to_center_grayscale` - Parity:** All 114,688 eight-bit pixels match across defaults at 128x128/256x256, seed 42, and changed transform settings. Default all-one matrix settings produce black; the changed nonsingular matrix/offset produces a nonconstant output and is also exact.

- **`clone_patch.sbs/transform_from_center` - Parity:** All 114,688 RGBA8 pixels match across defaults at 128x128/256x256, seed 42, and changed transform settings. Default all-one matrix settings produce black; the changed nonsingular matrix/offset produces a nonconstant output and is also exact.

- **`color_space_conversion.sbs/linear_srgb_to_acescg` - Parity:** All 114,688 RGBA8 pixels match across defaults at 128x128/256x256, seed 42, and an alternate original input map.

- **`color_space_conversion.sbs/acescg_to_srgb` - Parity:** All 114,688 RGBA8 pixels match across defaults at 128x128/256x256, seed 42, and an alternate original input map.

- **`color_space_conversion.sbs/linear_srgb_to_srgb` - Parity:** All 114,688 RGBA8 pixels match across defaults at 128x128/256x256, seed 42, and an alternate original input map.

- **`color_space_conversion.sbs/srgb_to_linear_srgb` - Parity:** All 114,688 RGBA8 pixels match across defaults at 128x128/256x256, seed 42, and an alternate original input map.

- **`color_space_conversion.sbs/linear_srgb_to_srgb_grayscale` - Parity:** All 114,688 eight-bit pixels match across defaults at 128x128/256x256, seed 42, and an alternate original input map.

- **`color_space_conversion.sbs/srgb_to_linear_srgb_grayscale` - Parity:** All 114,688 eight-bit pixels match across defaults at 128x128/256x256, seed 42, and an alternate original input map.

- **Constant typed values - Parity:** `constant_values.sbs` graphs `constant_float`, `constant_float2`, `constant_float3`, `constant_float4`, `constant_integer`, `constant_integer2`, `constant_integer3`, `constant_boolean` match all three recorded values each (24 typed output comparisons). Cases include zero, positive/negative scalar and vector values, HDR floats, and both Boolean states. These graphs publish values, so image pixel comparisons do not apply. The four-component integer boundary exception is recorded above.

- **`clamp.sbs/clamp` - Parity:** All 114,688 RGBA8 pixels match for defaults at 128x128/256x256, seed 42, and limits 0.2/0.7 with alpha clamping disabled. Original varying-alpha input maps exercise both the corrected legacy ignore-alpha stages and the final alpha-channel selection.

- **`composite.sbs/composite/filters/filter_transformation_grayscale` - Parity:** All 114,688 output pixels match across defaults at 128x128/256x256, seed 42, and changed settings.

- **`composite.sbs/composite/filters/filter_mask` - Parity:** All 114,688 output pixels match across defaults at 128x128/256x256, seed 42, and an alternate original input map.

- **`composite.sbs/composite/filters/filter_split_rgb_a` - Parity:** All 229,376 output pixels match across defaults at 128x128/256x256, seed 42, and an alternate original input map. All output bridges were compared.

- **`composite.sbs/composite/blends/blend_content_grayscale` - Parity:** All 114,688 output pixels match across defaults at 128x128/256x256, seed 42, and changed settings.

- **`composite.sbs/composite/blends/blend_matte_fg_passthrough` - Parity:** All 114,688 output pixels match across defaults at 128x128/256x256, seed 42, and an alternate original input map.

- **`composite.sbs/composite/blends/blend_matte_bg_passthrough` - Parity:** All 114,688 output pixels match across defaults at 128x128/256x256, seed 42, and an alternate original input map.

- **`composite.sbs/composite/blends/blend_content_pre_post_composite_grayscale` - Parity:** All 114,688 output pixels match across defaults at 128x128/256x256, seed 42, and changed settings.

- **`composite.sbs/composite/background/uniform_color_float3` - Parity:** All 114,688 output pixels match across defaults at 128x128/256x256, seed 42, and changed settings.

- **`contrast_luminosity.sbs/contrast_luminosity` - Parity:** All 114,688 output pixels match across defaults at 128x128/256x256, seed 42, and changed contrast/luminosity.

- **`contrast_luminosity.sbs/contrast_luminosity_grayscale` - Parity:** All 114,688 output pixels match across defaults at 128x128/256x256, seed 42, and changed contrast/luminosity.

- **`exposure.sbs/exposure` - Parity:** All 114,688 output pixels match across defaults at 128x128/256x256, seed 42, and exposure 1.5.

- **`extend_shape.sbs/extend_shape_grayscale` - Parity:** All 114,688 output pixels match across defaults at 128x128/256x256, seed 42, and changed extension mode/endpoints with nearest sampling. Original varying-alpha color or nonconstant grayscale input maps were used.

- **`extend_shape.sbs/extend_shape_color` - Parity:** All 114,688 output pixels match across defaults at 128x128/256x256, seed 42, and changed extension mode/endpoints with nearest sampling. Original varying-alpha color or nonconstant grayscale input maps were used.

- **`facing_normal.sbs/facing_normal` - Parity:** All 114,688 output pixels match across defaults at 128x128/256x256, seed 42, and the additional input case. Inputs are original curved normal maps.

- **`flood_fill.sbs/flood_fill_step` - Parity:** All 114,688 output pixels match across defaults at 128x128/256x256, seed 42, and the additional input case. Inputs are original bounded RGBA coordinate/box fields; color and alpha were compared.

- **`flood_fill.sbs/flood_fill_to_bbox_size` - Parity:** All 114,688 output pixels match across defaults at 128x128/256x256, seed 42, and changed exposed parameters. Inputs are original bounded RGBA coordinate/box fields; color and alpha were compared.

- **`flood_fill.sbs/flood_fill_to_position` - Parity:** All 114,688 output pixels match across defaults at 128x128/256x256, seed 42, and the additional input case. Inputs are original bounded RGBA coordinate/box fields; color and alpha were compared.

- **`flood_fill_2.sbs/flood_fill_to_bbox_size` - Parity:** All 114,688 output pixels match across defaults at 128x128/256x256, seed 42, and changed exposed parameters. Inputs are original bounded RGBA coordinate/box fields; color and alpha were compared.

- **`flood_fill_2.sbs/flood_fill_to_position` - Parity:** All 114,688 output pixels match across defaults at 128x128/256x256, seed 42, and the additional input case. Inputs are original bounded RGBA coordinate/box fields; color and alpha were compared.

- **`flood_fill_2.sbs/flood_fill_impl/flood_fill_step_A` - Parity:** All 229,376 output pixels match across defaults at 128x128/256x256, seed 42, and the additional input case. Inputs are original bounded RGBA coordinate/box fields; color and alpha were compared. Every output bridge was compared.

- **`flood_fill_2.sbs/flood_fill_impl/flood_fill_step_A_switch` - Parity:** All 229,376 output pixels match across defaults at 128x128/256x256, seed 42, and changed exposed parameters. Inputs are original bounded RGBA coordinate/box fields; color and alpha were compared. Every output bridge was compared.

- **`flood_fill_2.sbs/flood_fill_impl/flood_fill_step_B` - Parity:** All 114,688 output pixels match across defaults at 128x128/256x256, seed 42, and the additional input case. Inputs are original bounded RGBA coordinate/box fields; color and alpha were compared.

- **`floodfill_to_index.sbs/pass` - Parity:** All 114,688 output pixels match across defaults at 128x128/256x256, seed 42, and changed exposed parameters.

- **`fxaa.sbs/fxaa_grayscale` - Parity:** All 114,688 output pixels match across defaults at 128x128/256x256, seed 42, and the additional input case.

- **`glow.sbs/clamp` - Parity:** All 114,688 output pixels match across defaults at 128x128/256x256, seed 42, and changed exposed parameters.

- **`curvature_smooth_v2.sbs/compute_curvature_v2` - Parity:** All 16,668 RGBA32F pixels match in six cases after correcting internal size resolution. Actual outputs are three 2x2 images, one 4x4 image, one 16x16 image, and one 128x128 image. Larger cases change the level and both intensity components and use original nonconstant curved normal and previous-result maps.

- **`grid_atlas.sbs/grid_atlas_grayscale` - Parity:** All 114,688 output pixels (LUM8) match across 4 recorded cases: defaults at 128x128/256x256, seed 42, and changed exposed parameters.

- **`grid_atlas.sbs/grid_atlas_color` - Parity:** All 114,688 output pixels (RGBA8) match across 4 recorded cases: defaults at 128x128/256x256, seed 42, and changed exposed parameters.

- **`hdr_exposure_preview.sbs/hdr_exposure_preview` - Parity:** All 114,688 output pixels (RGBA32F) match across 4 recorded cases: defaults at 128x128/256x256, seed 42, and changed exposed parameters. The parent format is f32.

- **`height_extrude.sbs/impl/acceleration_struct_impl` - Parity:** All 114,688 output pixels (RGBA8) match across 4 recorded cases: defaults at 128x128/256x256, seed 42, and changed exposed parameters.

- **`histogram_range.sbs/histogram_range` - Parity:** All 114,688 output pixels (LUM8) match across 4 recorded cases: defaults at 128x128/256x256, seed 42, and changed exposed parameters.

- **`histogram_scan_non_uniform.sbs/histogram_scan_non_uniform` - Parity:** All 114,688 output pixels (LUM8) match across 4 recorded cases: defaults at 128x128/256x256, seed 42, and changed exposed parameters.

- **`histogram_select.sbs/histogram_select` - Parity:** All 114,688 output pixels (LUM16) match across 4 recorded cases: defaults at 128x128/256x256, seed 42, and changed exposed parameters.

- **`histogram_tools.sbs/horizontal_max` - Parity:** All 4 output pixels (LUM8) match across 4 recorded cases: defaults at 128x128/256x256, seed 42, and an alternate original input map.

- **`id_to_mask.sbs/id_to_mask_color` - Parity:** All 114,688 output pixels (LUM16) match across 4 recorded cases: defaults at 128x128/256x256, seed 42, and changed exposed parameters.

- **`id_to_mask.sbs/id_to_mask_grayscale` - Parity:** All 114,688 output pixels (LUM16) match across 4 recorded cases: defaults at 128x128/256x256, seed 42, and changed exposed parameters.

- **`invert.sbs/invert` - Parity:** All 114,688 output pixels (RGBA8) match across 4 recorded cases: defaults at 128x128/256x256, seed 42, and changed exposed parameters.

- **`invert.sbs/invert_grayscale` - Parity:** All 114,688 output pixels (LUM8) match across 4 recorded cases: defaults at 128x128/256x256, seed 42, and changed exposed parameters.

- **`linear_srgb_conversion.sbs/convert_to_linear` - Parity:** All 114,688 output pixels (RGBA8) match across 4 recorded cases: defaults at 128x128/256x256, seed 42, and an alternate original input map.

- **`linear_srgb_conversion.sbs/convert_to_linear_grayscale` - Parity:** All 114,688 output pixels (LUM8) match across 4 recorded cases: defaults at 128x128/256x256, seed 42, and an alternate original input map.

- **`linear_srgb_conversion.sbs/convert_to_srgb` - Parity:** All 114,688 output pixels (RGBA8) match across 4 recorded cases: defaults at 128x128/256x256, seed 42, and an alternate original input map.

- **`linear_srgb_conversion.sbs/convert_to_srgb_grayscale` - Parity:** All 114,688 output pixels (LUM8) match across 4 recorded cases: defaults at 128x128/256x256, seed 42, and an alternate original input map.

- **`material_switch.sbs/material_switch` - Parity:** All 1,376,256 output pixels (LUM8, RGBA8) match across 4 recorded cases: defaults at 128x128/256x256, seed 42, and changed exposed parameters. All output bridges were compared.

- **`median_filter.sbs/median_filter_grayscale` - Parity:** All 114,688 output pixels (LUM8) match across 4 recorded cases: defaults at 128x128/256x256, seed 42, and changed exposed parameters.

- **`mesh_adaptive_base_color.sbs/base_color_8_inputs` - Parity:** All 114,688 output pixels (RGBA8) match across 4 recorded cases: defaults at 128x128/256x256, seed 42, and changed exposed parameters.

- **`mesh_adaptive_base_color.sbs/base_color_4_inputs` - Parity:** All 114,688 output pixels (RGBA8) match across 4 recorded cases: defaults at 128x128/256x256, seed 42, and changed exposed parameters.

- **`mesh_adaptive_base_color.sbs/base_color_2_inputs` - Parity:** All 114,688 output pixels (RGBA8) match across 4 recorded cases: defaults at 128x128/256x256, seed 42, and changed exposed parameters.

- **`mesh_data_combiner.sbs/mesh_data_combiner` - Parity:** All 1,261,568 output pixels (LUM16, LUM8, RGBA8) match across 4 recorded cases: defaults at 128x128/256x256, seed 42, and changed exposed parameters. All output bridges were compared.

- **`multiangle_to_normal.sbs/angle_to_normal` - Parity:** All 114,688 output pixels (RGBA8) match across 4 recorded cases: defaults at 128x128/256x256, seed 42, and changed exposed parameters.

- **`multiangle_to_normal.sbs/angle_weight` - Parity:** All 114,688 output pixels (RGBA8) match across 4 recorded cases: defaults at 128x128/256x256, seed 42, and changed exposed parameters.

- **`normal_color.sbs/normal_color` - Parity:** All 114,688 output pixels (RGBA8) match across 4 recorded cases: defaults at 128x128/256x256, seed 42, and changed exposed parameters.

- **`normal_intensity.sbs/normal_intensity` - Parity:** All 114,688 output pixels (RGBA8) match across 4 recorded cases: defaults at 128x128/256x256, seed 42, and changed exposed parameters.

- **`normal_invert.sbs/normal_invert` - Parity:** All 114,688 output pixels (RGBA8) match across 4 recorded cases: defaults at 128x128/256x256, seed 42, and changed exposed parameters.

- **`normal_mapper.sbs/normal_mapper` - Parity:** All 114,688 output pixels (RGBA8) match across 4 recorded cases: defaults at 128x128/256x256, seed 42, and changed exposed parameters.

- **`normal_vector_rotation.sbs/normal_vector_rotation` - Parity:** All 114,688 output pixels (RGBA8) match across 4 recorded cases: defaults at 128x128/256x256, seed 42, and changed exposed parameters.

- **`normal_uncombine.sbs/get_intensity_iteration` - Parity:** All 4 typed outputs match exactly across 4 recorded cases, including an alternate original input or changed exposed parameters.

- **`noise_fractal_sum_base_v2.sbs/noise_disorder` - Parity:** All 114,688 output pixels (LUM8) match across 4 recorded cases: defaults at 128x128/256x256, seed 42, and changed exposed parameters.

- **`pattern_brick_generator.sbs/resources/Tile` - Parity:** All 114,688 output pixels (LUM16) match across 4 recorded cases: defaults at 128x128/256x256, seed 42, and changed exposed parameters.

- **`pattern_checker_1.sbs/checker_1` - Parity:** All 114,688 output pixels (LUM16) match across 4 recorded cases: defaults at 128x128/256x256, seed 42, and changed exposed parameters.

- **`pattern_tile_random_2.sbs/functions/debug_find_cell` - Parity:** All 114,688 output pixels (RGBA32F) match across 4 recorded cases: defaults at 128x128/256x256, seed 42, and changed exposed parameters.

- **`pbr_albedo_safe_color.sbs/pbr_albedo_safe_color` - Parity:** All 114,688 output pixels (RGBA8) match across 4 recorded cases: defaults at 128x128/256x256, seed 42, and changed exposed parameters.

- **`pbr_dielectric_F0.sbs/pbr_dielectric_F0` - Parity:** All 114,688 output pixels (RGBA8) match across 4 recorded cases: defaults at 128x128/256x256, seed 42, and changed exposed parameters.

- **`pbr_metal_reflectance.sbs/pbr_metal_reflectance` - Parity:** All 114,688 output pixels (RGBA8) match across 4 recorded cases: defaults at 128x128/256x256, seed 42, and changed exposed parameters.

- **`pbr_render2.sbs/sat/sat_iteration` - Parity:** All 114,688 output pixels (LUM32F) match across 4 recorded cases: defaults at 128x128/256x256, seed 42, and changed exposed parameters.

- **`pbr_render_ortho.sbs/get_max_coord` - Parity:** All 4 output pixels (RGBA8) match across 4 recorded cases: defaults at 128x128/256x256, seed 42, and an alternate original input map.

- **`rt_ao.sbs/pp_template/pp_template_64g` - Parity:** All 114,688 output pixels (LUM8) match across 4 recorded cases: defaults at 128x128/256x256, seed 42, and changed exposed parameters.

- **`rt_bent_normal.sbs/pp_template/pp_template_64c` - Parity:** All 114,688 output pixels (RGBA8) match across 4 recorded cases: defaults at 128x128/256x256, seed 42, and changed exposed parameters.

- **`rt_irradiance.sbs/template/pp_template` - Parity:** All 114,688 output pixels (LUM8) match across 4 recorded cases: defaults at 128x128/256x256, seed 42, and changed exposed parameters.

- **`rt_shadows.sbs/template/pp_template` - Parity:** All 114,688 output pixels (LUM8) match across 4 recorded cases: defaults at 128x128/256x256, seed 42, and changed exposed parameters.

- **`pow.sbs/pow_grayscale` - Parity:** All 114,688 output pixels (LUM8) match across 4 recorded cases: defaults at 128x128/256x256, seed 42, and changed exposed parameters.

- **`pow.sbs/pow` - Parity:** All 114,688 output pixels (RGBA8) match across 4 recorded cases: defaults at 128x128/256x256, seed 42, and changed exposed parameters.

- **`quantize.sbs/quantize_grayscale` - Parity:** All 114,688 output pixels (LUM8) match across 4 recorded cases: defaults at 128x128/256x256, seed 42, and changed exposed parameters.

- **`quantize.sbs/quantize` - Parity:** All 114,688 output pixels (RGBA8) match across 4 recorded cases: defaults at 128x128/256x256, seed 42, and changed exposed parameters.

- **`quantize_2.sbs/quantize_grayscale` - Parity:** All 114,688 output pixels (LUM8) match across 4 recorded cases: defaults at 128x128/256x256, seed 42, and changed exposed parameters.

- **`quantize_color.sbs/apply_color_palette_sub` - Parity:** All 114,688 output pixels (RGBA8) match across 4 recorded cases: defaults at 128x128/256x256, seed 42, and an alternate original input map.

- **`quantize_color.sbs/get_min_max/GetMaxFromAlpha_v3` - Parity:** All 4 output pixels (RGBA8) match across 4 recorded cases: defaults at 128x128/256x256, seed 42, and an alternate original input map.

- **`quantize_color.sbs/helpers/cdf_pass` - Parity:** All 114,688 output pixels (LUM32F) match across 4 recorded cases: defaults at 128x128/256x256, seed 42, and changed exposed parameters.

- **`quantize_color.sbs/bitonic_sort/graph/remap_sorting` - Parity:** All 114,688 output pixels (RGBA32F) match across 4 recorded cases: defaults at 128x128/256x256, seed 42, and changed exposed parameters.

- **`quantize_color.sbs/bitonic_sort/graph/remap_sorting_256x1` - Parity:** All 114,688 output pixels (RGBA32F) match across 4 recorded cases: defaults at 128x128/256x256, seed 42, and changed exposed parameters.

- **`quantize_color.sbs/bitonic_sort/graph/bitonic_sort_iteration` - Parity:** All 114,688 output pixels (RGBA8) match across 4 recorded cases: defaults at 128x128/256x256, seed 42, and changed exposed parameters.

- **`quantize_color.sbs/bitonic_sort/graph/initial_iteration` - Parity:** All 57,344 output pixels (RGBA32F) match across 4 recorded cases: defaults at 128x128/256x256, seed 42, and an alternate original input map.

- **`rgb-a_merge.sbs/rgb-a_merge` - Parity:** All 114,688 output pixels (RGBA8) match across 4 recorded cases: defaults at 128x128/256x256, seed 42, and an alternate original input map.

- **`rgb-a_split.sbs/rgb-a_split` - Parity:** All 229,376 output pixels (LUM8, RGBA8) match across 4 recorded cases: defaults at 128x128/256x256, seed 42, and an alternate original input map. All output bridges were compared.

- **`rgba_merge.sbs/rgba_merge` - Parity:** All 114,688 output pixels (RGBA8) match across 4 recorded cases: defaults at 128x128/256x256, seed 42, and an alternate original input map.

- **`rgba_split.sbs/rgba_split` - Parity:** All 458,752 output pixels (LUM8) match across 4 recorded cases: defaults at 128x128/256x256, seed 42, and an alternate original input map. All output bridges were compared.

- **`quantize_color.sbs/create_color_palette_16colors` - Parity:** All 1,024 output pixels (RGBA8) match across 4 recorded cases: defaults at 128x128/256x256, seed 42, and changed exposed parameters.

- **`quantize_color.sbs/view_color_palette_sub` - Parity:** All 114,688 output pixels (RGBA8) match across 4 recorded cases: defaults at 128x128/256x256, seed 42, and an alternate original input map.

- **`rt_ao_v2.sbs/mip_max_step` - Parity:** All 25,600 output pixels (LUM8) match across 4 recorded cases: defaults at 128x128/256x256, seed 42, and changed exposed parameters.

- **`rt_bent_normal_v2.sbs/mip_max_step` - Parity:** All 25,600 output pixels (LUM8) match across 4 recorded cases: defaults at 128x128/256x256, seed 42, and changed exposed parameters.

- **`rt_shadows_v2.sbs/mip_max_step` - Parity:** All 25,600 output pixels (LUM8) match across 4 recorded cases: defaults at 128x128/256x256, seed 42, and changed exposed parameters.

- **`shape_splatter.sbs/shape_splatter_to_mask` - Parity:** All 114,688 output pixels (LUM32F) match across 4 recorded cases: defaults at 128x128/256x256, seed 42, and changed exposed parameters.

- **`shape_splatter.sbs/min_max` - Parity:** All 2,048 output pixels (LUM8) match across 4 recorded cases: defaults at 128x128/256x256, seed 42, and an alternate original input map. All output bridges were compared.

- **`shape_splatter_v2.sbs/bitonic_sort/graph/remap_sorting` - Parity:** All 114,688 output pixels (RGBA8) match across 4 recorded cases: defaults at 128x128/256x256, seed 42, and changed exposed parameters.

- **`shape_splatter_v2.sbs/bitonic_sort/graph/bitonic_sort_iteration` - Parity:** All 114,688 output pixels (RGBA8) match across 4 recorded cases: defaults at 128x128/256x256, seed 42, and changed exposed parameters.

- **`shape_splatter_v2.sbs/bitonic_sort/graph/initial_iteration` - Parity:** All 57,344 output pixels (RGBA32F) match across 4 recorded cases: defaults at 128x128/256x256, seed 42, and an alternate original input map.

- **`shape_splatter_v2.sbs/poisson_disc/poisson_disc_iteration` - Parity:** All 800 output pixels (RGBA8) match across 4 recorded cases: defaults at 128x128/256x256, seed 42, and changed exposed parameters.

- **`shape_splatter_v2.sbs/scanline_to_z_order` - Parity:** All 114,688 output pixels (RGBA16) match across 4 recorded cases: defaults at 128x128/256x256, seed 42, and an alternate original input map.

- **`shape_splatter_v2.sbs/grid_to_scanline` - Parity:** All 114,688 output pixels (RGBA32F) match across 4 recorded cases: defaults at 128x128/256x256, seed 42, and changed exposed parameters.

- **`shape_splatter_v2_companion.sbs/shape_splatter_to_mask` - Parity:** All 114,688 output pixels (LUM8) match across 4 recorded cases: defaults at 128x128/256x256, seed 42, and changed exposed parameters.

- **`summed_area_table.sbs/summed_area_table_iteration` - Parity:** All 114,688 output pixels (LUM32F) match across 4 recorded cases: defaults at 128x128/256x256, seed 42, and changed exposed parameters.

- **`summed_area_table_2.sbs/summed_area_table_iteration` - Parity:** All 114,688 output pixels (LUM32F) match across 4 recorded cases: defaults at 128x128/256x256, seed 42, and changed exposed parameters.

- **`symmetry.sbs/symmetry` - Parity:** All 114,688 output pixels (RGBA8) match across 4 recorded cases: defaults at 128x128/256x256, seed 42, and changed exposed parameters.

- **`threshold.sbs/threshold` - Parity:** All 114,688 output pixels (LUM8) match across 4 recorded cases: defaults at 128x128/256x256, seed 42, and changed exposed parameters.

- **`value_get_size.sbs/get_size_color` - Parity:** All 4 typed outputs match exactly across 4 recorded cases, including an alternate original input or changed exposed parameters.

- **`value_get_size.sbs/get_size_grayscale` - Parity:** All 4 typed outputs match exactly across 4 recorded cases, including an alternate original input or changed exposed parameters.

- **`paths_tools.sbs/Impl/get_number_of_valid_paths` - Parity:** All 4 typed outputs match exactly across 4 recorded cases, including an alternate original input or changed exposed parameters.

- **`paths_tools.sbs/Utils/RelativeToInputPP` - Parity:** All 1,024 output pixels (RGBA32F) match across 4 recorded cases: original 16x16 curve/path buffers, seed 42, and changed exposed parameters. The parent format is f32.

- **`spline_tools.sbs/internal_tools/get_spline_point_data` - Parity:** All 4 typed outputs match exactly across 4 recorded cases, including an alternate original input or changed exposed parameters.

- **`spline_tools.sbs/internal_tools/compute_spline_distance_iteration` - Parity:** All 1,024 output pixels (LUM32F) match across 4 recorded cases: original 16x16 curve/path buffers, seed 42, and changed exposed parameters. The parent format is f32.

- **`spline_tools.sbs/internal_tools/append_splines_sub` - Parity:** All 262,144 output pixels (RGBA32F) match across 4 recorded cases: original 16x16 curve/path buffers, seed 42, and changed exposed parameters. The parent format is f32. All output bridges were compared. All four typed count outputs also match; connectable counts retain their default values.

- **`spline_tools.sbs/-----WIP-----/circular_array_wip` - Parity:** All 1,024 output pixels (RGBA32F) match across 4 recorded cases: original 16x16 curve/path buffers, seed 42, and changed exposed parameters. The parent format is f32. All four typed count outputs also match; connectable counts retain their default values.

- **`spline_tools.sbs/-----WIP-----/displace_spline_by_spline_wip` - Parity:** All 1,024 output pixels (RGBA32F) match across 4 recorded cases: original 16x16 curve/path buffers, seed 42, and changed exposed parameters. The parent format is f32. All four typed count outputs also match; connectable counts retain their default values.

- **`templates.sbs/Templates/Standard_Materials/Simple_Material` - Parity:** All 491,520 output pixels (RGBA8) match across 3 recorded cases: defaults at 128x128/256x256 and seed 42. All output bridges were compared.

- **`templates.sbs/Templates/Standard_Materials/Extended_Material` - Parity:** All 688,128 output pixels (RGBA8) match across 3 recorded cases: defaults at 128x128/256x256 and seed 42. All output bridges were compared.

- **`pbr_render2.sbs/height_adjust` - Parity:** All 98,304 output pixels (LUM8) match across 3 recorded cases: defaults at 128x128/256x256 and seed 42. Connectable value pins retain their defaults; nondefault values require a wired reference fixture and remain unverified.

## First-pass validation

- Final release CLI and native desktop builds pass, including the unassigned-variable, grayscale Pixel Processor, integer-conversion, and output-bridge changes. Formatting, dependency layers, and all ten WebAssembly crate checks pass. An independent inventory rescan agrees with all 546 roots, and all 320 package links resolve.
- Workspace Clippy is blocked by an existing `chunks_exact_to_as_chunks` lint in the text primitive. Clippy for the four changed library crates passes with that existing lint suppressed.
- Native output-format save/reload checks preserve all four formats: u8, u16, f16, and f32. No repository tests were added or test suites run. Pixel and save/reload measurements use disposable files outside the repository. Reference captures use version 16.0.7 with the Direct3D 11 backend; exact-match claims apply to the recorded cases.
