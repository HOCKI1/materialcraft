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

For the exposed scalar controls use .5, .625, .7, 1, 1.125, 1.25, 2, 4 and 1.375 for either logarithm. For the exponential use −2, −1, −.625, −.5, −.125, 0, .5, 1, 1.375 and 2. Cycle seeds 71, 137 and 313 at 16². In a separate Value Processor, the same arguments match 5/9 typed base-two logarithms, 5/9 natural logarithms and 9/10 powers of two. Maximum errors are 5.96046448e−8, 2.98023224e−8 and 2.38418579e−7 respectively. These uniform operations remain open.

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
| Plane light | World position (−.75,.4375,1.25), UV (.3125,.28125), rotation (.0625k,−.125,.1875), scale .6875, size (.75,1.25). Use world position mode in B and alternate up vector in C. Set color mode and pattern to k mod 3, color (.75,.34375,.625), temperature 4375 and exposure 1.25. Disable background input in C and enable ground clipping in B. |
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

### Volume-mask shapes and axis sweeps

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

The line-light differences begin in its pixel processor. On the signed field, pixel (6,2) stores 1.98653376 in the three color channels in the reference and 1.98652911 in MaterialCraft. For the horizon, the input matches. Its pixel processor differs at two texels in the 64² A case. Pixel (39,49) has reference red .756740689 and MaterialCraft red .753627837. Both roots remain in progress.

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

Four additional original graphs replicate a single draw to RGBA, reuse two draws in alternating channels, double a reused draw, or apply ranges 1, 1, −2 and .375 to successive draws. At 16², 32² and 64² with seed zero and 32² with seed 42, all **25,600/25,600 reference pixels** agree with the corresponding channel expressions, maximum error zero. The MaterialCraft evaluator now matches the progression given the recovered initial state. Its mapping from pixel position and graph seed to that state remains open. For example, the native and CPU four-draw images at 32² still match 0/1,024 entire pixels at seeds zero and 42, with maximum component errors .995849967 and .989675045.

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

**Worley.** With a constant input position of `(0,0,0,255)`, no LUM32F pixel matches in the 32² default case at seed 0 (maximum .02120566) or seed 42 (.07068205). The graph's internal random table uses four draws. A standalone Pixel Processor writing four successive `rand(1)` values to RGBA16 reproduces a discrepancy before any cell-distance calculation. It gives 0/1,024 exact pixels at 32², seed 0 (maximum 65,122), 0/16,384 at 128², seed 0 (65,136), and 0/16,384 at 128², seed 42 (65,348). At seed 0 the first reference pixel changes with resolution, from `(47238,35719,5497,27913)` to `(37730,56747,64327,4184)`. MaterialCraft returns `(42015,40251,56435,61318)` at both sizes. The random sequence and any residual differences in the distance calculation remain open.

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
