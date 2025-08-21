# Text Namespace

Controls typography: font, size, weight, color, alignment, and text effects.

## Properties

### Font Family

Fonts are configured in `reedstyle.fonts.yaml` using the pattern font-a through font-f:

```yaml
# reedstyle.fonts.yaml
fonts:
  font-a:  # Primary font (matches brand-a pattern)
    family: "'Inter', sans-serif"
  font-b:  # Secondary font
    family: "'Playfair Display', serif"
  font-c:  # Code font
    family: "'JetBrains Mono', monospace"
```

Usage:

```html
<!-- Named fonts (from config) -->
<r-s as="div" text="font:font-a">    <!-- Primary font -->
<r-s as="div" text="font:font-b">    <!-- Secondary font -->
<r-s as="div" text="font:font-c">    <!-- Code font -->

<!-- Semantic mappings -->
<r-s as="div" text="font:system">    <!-- Maps to font-a -->
<r-s as="div" text="font:heading">   <!-- Maps to font-b -->
<r-s as="div" text="font:body">      <!-- Maps to font-a -->
<r-s as="div" text="font:code">      <!-- Maps to font-c -->
```

### Font Size

```html
<!-- Dimension Scale -->
<r-s as="div" text="size:tiny">      <!-- 0.75x root -->
<r-s as="div" text="size:small">     <!-- 0.875x root -->
<r-s as="div" text="size:normal">    <!-- 1x root -->
<r-s as="div" text="size:large">     <!-- 1.25x root -->
<r-s as="div" text="size:huge">      <!-- 1.5x root -->
<r-s as="div" text="size:mega">      <!-- 2x root -->
<r-s as="div" text="size:ultra">     <!-- 3x root -->
```

### Font Weight

```html
<r-s as="div" text="weight:thin">     <!-- 100 -->
<r-s as="div" text="weight:light">    <!-- 300 -->
<r-s as="div" text="weight:normal">   <!-- 400 -->
<r-s as="div" text="weight:medium">   <!-- 500 -->
<r-s as="div" text="weight:semibold"> <!-- 600 -->
<r-s as="div" text="weight:bold">     <!-- 700 -->
<r-s as="div" text="weight:extrabold"> <!-- 800 -->
<r-s as="div" text="weight:black">    <!-- 900 -->
```

### Text Color

```html
<!-- Brand colors -->
<r-s as="div" text="color:brand-a">
<r-s as="div" text="color:brand-b">

<!-- Base colors -->
<r-s as="div" text="color:base-900">  <!-- Dark text -->
<r-s as="div" text="color:base-700">  <!-- Medium dark -->
<r-s as="div" text="color:base-500">  <!-- Medium -->
<r-s as="div" text="color:base-300">  <!-- Light -->
<r-s as="div" text="color:base-100">  <!-- Very light -->

<!-- Semantic colors -->
<r-s as="div" text="color:state-success">
<r-s as="div" text="color:state-warning">
<r-s as="div" text="color:state-error">
<r-s as="div" text="color:state-info">

<!-- Special -->
<r-s as="div" text="color:current">   <!-- currentColor -->
<r-s as="div" text="color:inherit">   <!-- inherit -->
```

### Text Alignment

```html
<r-s as="div" text="align:left">
<r-s as="div" text="align:center">
<r-s as="div" text="align:right">
<r-s as="div" text="align:justify">
<r-s as="div" text="align:start">     <!-- Locale-aware -->
<r-s as="div" text="align:end">       <!-- Locale-aware -->
```

### Line Height

```html
<r-s as="div" text="leading:none">    <!-- 1 -->
<r-s as="div" text="leading:tight">   <!-- 1.25 -->
<r-s as="div" text="leading:snug">    <!-- 1.375 -->
<r-s as="div" text="leading:normal">  <!-- 1.5 -->
<r-s as="div" text="leading:relaxed"> <!-- 1.625 -->
<r-s as="div" text="leading:loose">   <!-- 2 -->
```

### Letter Spacing

```html
<r-s as="div" text="tracking:tighter"> <!-- -0.05em -->
<r-s as="div" text="tracking:tight">   <!-- -0.025em -->
<r-s as="div" text="tracking:normal">  <!-- 0 -->
<r-s as="div" text="tracking:wide">    <!-- 0.025em -->
<r-s as="div" text="tracking:wider">   <!-- 0.05em -->
<r-s as="div" text="tracking:widest">  <!-- 0.1em -->
```

### Text Decoration

```html
<!-- Decoration line -->
<r-s as="div" text="decoration:none">
<r-s as="div" text="decoration:underline">
<r-s as="div" text="decoration:overline">
<r-s as="div" text="decoration:line-through">

<!-- Decoration style -->
<r-s as="div" text="decoration:solid">
<r-s as="div" text="decoration:double">
<r-s as="div" text="decoration:dotted">
<r-s as="div" text="decoration:dashed">
<r-s as="div" text="decoration:wavy">

<!-- Decoration color -->
<r-s as="div" text="decoration:brand-a">  <!-- Color -->

<!-- Combined -->
<r-s as="div" text="[decoration:underline, decoration:brand-a, decoration:wavy]">
```

### Text Transform

```html
<r-s as="div" text="transform:none">
<r-s as="div" text="transform:uppercase">
<r-s as="div" text="transform:lowercase">
<r-s as="div" text="transform:capitalize">
```

### Text Style

```html
<r-s as="div" text="style:normal">
<r-s as="div" text="style:italic">
<r-s as="div" text="style:oblique">
```

### Text Overflow

```html
<r-s as="div" text="overflow:clip">
<r-s as="div" text="overflow:ellipsis">
<r-s as="div" text="truncate">        <!-- Shorthand for ellipsis -->
```

### Whitespace

```html
<r-s as="div" text="whitespace:normal">
<r-s as="div" text="whitespace:nowrap">
<r-s as="div" text="whitespace:pre">
<r-s as="div" text="whitespace:pre-line">
<r-s as="div" text="whitespace:pre-wrap">
```

### Word Break

```html
<r-s as="div" text="break:normal">
<r-s as="div" text="break:words">     <!-- break-word -->
<r-s as="div" text="break:all">       <!-- break-all -->
<r-s as="div" text="break:keep">      <!-- keep-all -->
```

### List Style

```html
<r-s as="ul" text="list:none">
<r-s as="ul" text="list:disc">
<r-s as="ul" text="list:circle">
<r-s as="ul" text="list:square">
<r-s as="ol" text="list:decimal">
<r-s as="ol" text="list:roman">
<r-s as="ol" text="list:alpha">
```

## Common Patterns

### Heading Styles

```html
<!-- Large heading -->
<r-s as="h1" 
      text="[size:5xl, weight:bold, leading:tight, tracking:tight]">
  Main Heading
</r-s>

<!-- Subheading -->
<r-s as="h2" 
      text="[size:3xl, weight:semibold, color:base-700]">
  Subheading
</r-s>
```

### Body Text

```html
<r-s as="p" 
      text="[size:base, leading:relaxed, color:base-800]">
  Comfortable reading text with good line height.
</r-s>
```

### Truncated Text

```html
<r-s as="div" 
      text="[truncate, whitespace:nowrap]"
      box="overflow:hidden">
  This long text will be truncated with ellipsis...
</r-s>
```

### Code Block

```html
<r-s as="pre" 
      text="[font:mono, size:sm, whitespace:pre]"
      face="[bg:base-100, radius:md]"
      box="[padding:4, overflow-x:auto]">
  const code = "example";
</r-s>
```

### Link Styles

```html
<r-s as="a" 
      text="[color:brand-a, decoration:underline]"
      text-hover="[color:brand-a-strong, decoration:none]">
  Styled link
</r-s>
```

### Responsive Typography

```html
<r-s as="h1" 
      text="[size:3xl, weight:bold]"
      text-tablet="[size:4xl]"
      text-screen="[size:5xl, tracking:tight]">
  Responsive heading
</r-s>
```

### Gradient Text

```html
<r-s as="h1" 
      text="[size:6xl, weight:bold]"
      face="[bg:gradient-primary, bg-clip:text, text-fill:transparent]">
  Gradient Text
</r-s>
```

## Typography Scale

ReedSTYLE uses the Dimension Scale for consistent typography:

| Size | Multiplier | Use Case |
|------|------------|----------|
| tiny | 0.75x | Small labels, disclaimers |
| small | 0.875x | Secondary text, captions |
| normal | 1x | Body text (base) |
| large | 1.25x | Lead text, emphasis |
| huge | 1.5x | Subheadings |
| mega | 2x | Headings |
| ultra | 3x | Display, hero text |

## Smart Typography Engine

### Typography Filters

ReedSTYLE includes an advanced typography engine with automatic language detection and professional typography rules:

```html
<!-- Enable smart typography -->
<r-s as="article" text="[filter:smart]" lang="de">
  "Quotes" become „German quotes" automatically.
  Three dots... become ellipsis…
  Dashes -- become proper em dashes—like this.
</r-s>

<!-- Professional filter with all enhancements -->
<r-s as="article" text="[filter:professional]">
  Enhanced with hyphenation, optical margins, and more
</r-s>

<!-- Minimal filter for basic improvements -->
<r-s as="article" text="[filter:minimal]">
  Only essential corrections
</r-s>
```

### Language-Specific Rules

#### German (DIN 5008 Compliant)
```html
<r-s as="article" lang="de" text="[filter:professional]">
  <!-- Automatic DIN 5008 formatting: -->
  <!-- „German quotes" instead of "quotes" -->
  <!-- Non-breaking spaces: z. B., d. h., u. a. -->
  <!-- Number formatting: 10 000 (with thin space) -->
  <!-- Units: 10 kg, 25 °C (with narrow no-break space) -->
  <!-- Currency: 29,99 € (with non-breaking space) -->
  <!-- Date: 14. März 2024 (with non-breaking space) -->
</r-s>
```

#### English Variants
```html
<!-- British English -->
<r-s as="article" lang="en-GB" text="[filter:smart]">
  'Single quotes' are primary in British English.
  Nested "double quotes" for secondary.
</r-s>

<!-- American English -->
<r-s as="article" lang="en-US" text="[filter:smart]">
  "Double quotes" are primary in American English.
  Nested 'single quotes' for secondary.
</r-s>
```

#### French Typography
```html
<r-s as="article" lang="fr" text="[filter:smart]">
  Les "guillemets" become « guillemets français ».
  Proper spacing before : ; ! ? punctuation.
</r-s>
```

### OpenType Features

```html
<!-- Ligatures -->
<r-s as="p" text="[ligatures:true]">
  Efficient office typography (fi, ff, ffi ligatures)
</r-s>

<!-- Small capitals -->
<r-s as="p" text="[small-caps:true]">
  SMALL CAPITALS for emphasis
</r-s>

<!-- Numeric styles -->
<r-s as="table" text="[numbers:tabular]">
  <!-- Tabular numbers align in columns -->
  <tr><td>1,234.56</td></tr>
  <tr><td>9,876.54</td></tr>
</r-s>

<r-s as="p" text="[numbers:oldstyle]">
  Text with 1234567890 oldstyle figures
</r-s>

<!-- Fractions -->
<r-s as="p" text="[fractions:true]">
  Recipe: 1/2 cup, 3/4 teaspoon (automatic fractions)
</r-s>

<!-- Kerning -->
<r-s as="h1" text="[kerning:true]">
  AVAST - Proper letter spacing
</r-s>
```

### Hyphenation & Justification

```html
<!-- Enable hyphenation -->
<r-s as="p" text="[hyphenate:true, align:justify]" lang="en">
  Long paragraph with automatic hyphenation for better
  text justification and even spacing between words.
</r-s>

<!-- Custom hyphenation settings -->
<r-s as="p" text="[hyphenate:custom, min-length:6, zone:8]">
  Custom hyphenation with minimum word length
</r-s>

<!-- Optical margin alignment -->
<r-s as="article" text="[hanging-punctuation:true]">
  "Quotes" hang into the margin for optical alignment
</r-s>
```

### Measure & Rhythm

```html
<!-- Optimal line length (measure) -->
<r-s as="article" text="[measure:narrow]">  <!-- 45-55 chars -->
  Narrow measure for sidebars
</r-s>

<r-s as="article" text="[measure:normal]">  <!-- 65-75 chars -->
  Optimal reading measure for body text
</r-s>

<r-s as="article" text="[measure:wide]">    <!-- 85-95 chars -->
  Wide measure for large screens
</r-s>

<!-- Baseline grid -->
<r-s as="article" text="[baseline:true, grid:8]">
  All text aligns to 8px baseline grid
</r-s>
```

### Advanced Typography Examples

```html
<!-- Article with full typography enhancement -->
<r-s as="article" 
      lang="de"
      text="[filter:professional, hyphenate:true, measure:normal]">
  <r-s as="h1" text="[size:mega, weight:bold, kerning:true]">
    Überschrift mit perfekter Typografie
  </r-s>
  
  <r-s as="p" text="[size:normal, leading:relaxed, align:justify]">
    Dieser Text wird automatisch mit deutschen Anführungszeichen,
    korrekten Abständen nach DIN 5008 und professioneller
    Silbentrennung formatiert. Zahlen wie 10 000 und Einheiten
    wie 25 kg werden korrekt dargestellt.
  </r-s>
  
  <r-s as="blockquote" text="[size:large, style:italic, hanging-punctuation:true]">
    „Ein Zitat mit hängender Interpunktion für optische Ausrichtung."
  </r-s>
</r-s>

<!-- Typography for different content types -->
<r-s as="code" text="[font:font-c, numbers:tabular, ligatures:false]">
  const value = 123456; // Tabular numbers, no ligatures
</r-s>

<r-s as="table" text="[numbers:tabular, size:small]">
  <!-- Financial data with aligned numbers -->
  <tr><td>€ 1,234.56</td></tr>
  <tr><td>€ 9,876.54</td></tr>
</r-s>

<r-s as="recipe" text="[fractions:true, numbers:oldstyle]">
  Add 1/2 cup flour and 3/4 tsp salt
</r-s>
```

## Typography Scopes

### Weight Scale (Typography Scope)
- `thin` → `light` → `regular` → `medium` → `bold` → `black`

### Size Scale (Dimension Scope)  
- `tiny` → `small` → `normal` → `large` → `huge` → `mega` → `ultra`

### Line Height Scale
- `tight` (1.25) → `normal` (1.5) → `relaxed` (1.75) → `loose` (2.0)

## Performance

The typography engine uses:
- **Lazy processing** - Only processes visible text
- **Caching** - Stores processed text to avoid reprocessing  
- **Progressive enhancement** - CSS features work without JavaScript
- **Language detection** - Automatic from DOM tree

## Best Practices

1. **Always specify language** with `lang` attribute for proper typography
2. **Use semantic sizes** (sm, lg) over arbitrary values
3. **Enable smart typography** for professional content
4. **Test with real content** in target languages
5. **Respect user preferences** (prefers-reduced-motion, prefers-contrast)
6. **Validate spacing** with native speakers (especially for DIN 5008)
7. **Use appropriate filters** - professional for print-quality, smart for web
8. **Consider performance** - lazy load typography for large text blocks

## Next: [Architecture](201-architecture.md)