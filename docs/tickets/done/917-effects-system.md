# Ticket #917: Effects System (FX Namespace)

## Status: ✅ Done

## Decision Log References
- DEC002 - Color Conversion (effects use CSS variables)

## Description
Implement comprehensive visual effects system using the FX namespace for hover, click, scroll, and animation effects.

## Requirements

### Translation to Reed Element System

#### Old System (data-r-fx)
```html
<!-- Old attribute system -->
<button data-r-fx-hover="[effect:lift, shadow:strong]">
<div data-r-fx-scroll="[effect:fade-in]">
<button data-r-fx-click="[effect:ripple]">
```

#### New System (reed element)
```html
<!-- New reed element with fx namespace -->
<reed as="button" fx="[hover:lift, shadow:strong]">
<reed as="div" fx="[scroll:fade-in, threshold:0.3]">
<reed as="button" fx="[click:ripple, color:white]">
```

### Effect Categories

1. **Hover Effects**
   - lift, sink, grow, shrink
   - glow, blur, rotate, skew, flip

2. **Click Effects**
   - ripple, pulse, bounce
   - scale, flash

3. **Scroll Effects**
   - fade-in, slide-up, slide-down
   - zoom-in, zoom-out
   - parallax, reveal

4. **Focus Effects**
   - highlight, glow, outline

5. **Loading Effects**
   - spinner, progress, skeleton, pulse

### CSS Generation (fx.rs)

```rust
// src/namespaces/fx.rs
pub fn generate(config: &Config) -> String {
    let mut css = String::new();
    
    // Transitions
    css.push_str(r#"
/* Base transitions */
reed[fx*="transition:none"] { transition: none; }
reed[fx*="transition:fast"] { transition: all 150ms ease; }
reed[fx*="transition:smooth"] { transition: all 300ms ease; }
reed[fx*="transition:slow"] { transition: all 500ms ease; }

/* Hover effects */
reed[fx*="hover:lift"] {
  transition: transform 200ms ease-out, box-shadow 200ms ease-out;
  will-change: transform;
}

reed[fx*="hover:lift"]:hover {
  transform: translateY(-4px) translateZ(0);
  box-shadow: 0 8px 16px rgba(0,0,0,0.15);
}

reed[fx*="hover:grow"]:hover {
  transform: scale(1.05);
}

reed[fx*="hover:shrink"]:hover {
  transform: scale(0.95);
}

reed[fx*="hover:glow"]:hover {
  box-shadow: 0 0 20px rgba(var(--glow-color, 66, 153, 225), 0.5);
}

reed[fx*="hover:rotate"]:hover {
  transform: rotate(3deg);
}

/* Click effects */
reed[fx*="click:scale"]:active {
  transform: scale(0.95);
}

reed[fx*="click:brightness"]:active {
  filter: brightness(0.9);
}

/* Scroll animations */
@keyframes fade-in {
  from {
    opacity: 0;
  }
  to {
    opacity: 1;
  }
}

@keyframes slide-up {
  from {
    opacity: 0;
    transform: translateY(20px);
  }
  to {
    opacity: 1;
    transform: translateY(0);
  }
}

@keyframes zoom-in {
  from {
    opacity: 0;
    transform: scale(0.9);
  }
  to {
    opacity: 1;
    transform: scale(1);
  }
}

reed[fx*="animate:fade-in"] {
  animation: fade-in 300ms ease-out forwards;
}

reed[fx*="animate:slide-up"] {
  animation: slide-up 300ms ease-out forwards;
}

reed[fx*="animate:zoom-in"] {
  animation: zoom-in 300ms ease-out forwards;
}

/* Duration control (Dimension scope) */
reed[fx*="duration:tiny"] { animation-duration: 50ms; }
reed[fx*="duration:small"] { animation-duration: 100ms; }
reed[fx*="duration:normal"] { animation-duration: 200ms; }
reed[fx*="duration:large"] { animation-duration: 300ms; }
reed[fx*="duration:huge"] { animation-duration: 500ms; }
reed[fx*="duration:mega"] { animation-duration: 750ms; }
reed[fx*="duration:ultra"] { animation-duration: 1000ms; }

/* Shadow levels (Visual scope) */
reed[fx*="shadow:weak"] { box-shadow: 0 1px 2px rgba(0,0,0,0.05); }
reed[fx*="shadow:light"] { box-shadow: 0 2px 4px rgba(0,0,0,0.1); }
reed[fx*="shadow:normal"] { box-shadow: 0 4px 8px rgba(0,0,0,0.15); }
reed[fx*="shadow:intense"] { box-shadow: 0 8px 16px rgba(0,0,0,0.2); }
reed[fx*="shadow:bright"] { box-shadow: 0 12px 24px rgba(0,0,0,0.25); }
reed[fx*="shadow:strong"] { box-shadow: 0 16px 32px rgba(0,0,0,0.3); }

/* Blur effects */
reed[fx*="blur:glass"] {
  background: rgba(255, 255, 255, 0.1);
  backdrop-filter: blur(10px);
  -webkit-backdrop-filter: blur(10px);
  border: 1px solid rgba(255, 255, 255, 0.2);
}

reed[fx*="blur:frost"] {
  background: rgba(255, 255, 255, 0.5);
  backdrop-filter: blur(20px) saturate(180%);
  -webkit-backdrop-filter: blur(20px) saturate(180%);
}

/* Respect reduced motion */
@media (prefers-reduced-motion: reduce) {
  reed[fx*="duration"],
  reed[fx*="animate"] {
    animation-duration: 0.01ms !important;
    transition-duration: 0.01ms !important;
  }
  
  reed[fx*="scroll"] {
    opacity: 1 !important;
    transform: none !important;
  }
}
"#);
    
    css
}
```

### JavaScript Implementation

```javascript
// dist/reedstyle.js - Effects module
class EffectsEngine {
  constructor() {
    this.observers = new Map();
    this.init();
  }
  
  init() {
    // Process click effects
    this.initClickEffects();
    
    // Process scroll effects
    this.initScrollEffects();
    
    // Process hover enhancements
    this.initHoverEffects();
  }
  
  initClickEffects() {
    document.querySelectorAll('reed[fx*="click:"]').forEach(element => {
      const fx = element.getAttribute('fx');
      
      if (fx.includes('click:ripple')) {
        element.addEventListener('click', (e) => this.createRipple(e));
      }
      
      if (fx.includes('click:pulse')) {
        element.addEventListener('click', () => this.createPulse(element));
      }
    });
  }
  
  createRipple(event) {
    const element = event.currentTarget;
    const ripple = document.createElement('span');
    ripple.className = 'reed-ripple';
    
    // Position at click point
    const rect = element.getBoundingClientRect();
    const x = event.clientX - rect.left;
    const y = event.clientY - rect.top;
    
    ripple.style.cssText = `
      position: absolute;
      left: ${x}px;
      top: ${y}px;
      transform: translate(-50%, -50%);
      width: 0;
      height: 0;
      border-radius: 50%;
      background: rgba(255, 255, 255, 0.5);
      animation: reed-ripple 600ms ease-out;
    `;
    
    element.style.position = 'relative';
    element.style.overflow = 'hidden';
    element.appendChild(ripple);
    
    setTimeout(() => ripple.remove(), 600);
  }
  
  initScrollEffects() {
    const elements = document.querySelectorAll('reed[fx*="scroll:"]');
    
    const observer = new IntersectionObserver(
      (entries) => {
        entries.forEach(entry => {
          if (entry.isIntersecting) {
            const fx = entry.target.getAttribute('fx');
            const effect = this.extractScrollEffect(fx);
            
            // Add animation class
            entry.target.classList.add(`reed-scroll-${effect}`);
            
            // Handle stagger
            if (fx.includes('stagger:')) {
              this.applyStagger(entry.target, fx);
            }
            
            // Unobserve if not repeating
            if (!fx.includes('repeat:true')) {
              observer.unobserve(entry.target);
            }
          }
        });
      },
      {
        threshold: this.getThreshold(element.getAttribute('fx')),
        rootMargin: '0px'
      }
    );
    
    elements.forEach(el => observer.observe(el));
  }
  
  extractScrollEffect(fx) {
    const match = fx.match(/scroll:([a-z-]+)/);
    return match ? match[1] : 'fade-in';
  }
  
  getThreshold(fx) {
    const match = fx.match(/threshold:([\d.]+)/);
    return match ? parseFloat(match[1]) : 0.1;
  }
  
  applyStagger(element, fx) {
    const siblings = [...element.parentElement.children];
    const index = siblings.indexOf(element);
    const stagger = this.getStaggerDelay(fx);
    
    element.style.animationDelay = `${index * stagger}ms`;
  }
  
  getStaggerDelay(fx) {
    const delays = {
      'tiny': 50,
      'small': 100,
      'normal': 200,
      'large': 300
    };
    
    const match = fx.match(/stagger:(\w+)/);
    return match ? delays[match[1]] || 100 : 100;
  }
}

// Auto-init with ReedStyle
if (window.ReedStyle) {
  window.ReedStyle.effects = new EffectsEngine();
}

// CSS for ripple animation
const style = document.createElement('style');
style.textContent = `
@keyframes reed-ripple {
  to {
    width: 200px;
    height: 200px;
    opacity: 0;
  }
}

.reed-scroll-fade-in {
  animation: fade-in 300ms ease-out forwards;
}

.reed-scroll-slide-up {
  animation: slide-up 300ms ease-out forwards;
}

.reed-scroll-zoom-in {
  animation: zoom-in 300ms ease-out forwards;
}
`;
document.head.appendChild(style);
```

### Usage Examples

```html
<!-- Button with multiple effects -->
<reed as="button-primary" 
      fx="[hover:lift, click:ripple, shadow:normal]">
  Interactive Button
</reed>

<!-- Card with scroll animation -->
<reed as="card" 
      fx="[scroll:fade-in, threshold:0.3, duration:normal]">
  <h3>Animated Card</h3>
  <p>Fades in when scrolled into view</p>
</reed>

<!-- Gallery with staggered animations -->
<reed as="gallery" layout="[grid:3]">
  <reed as="img" fx="[scroll:zoom-in, stagger:tiny]" src="1.jpg">
  <reed as="img" fx="[scroll:zoom-in, stagger:tiny]" src="2.jpg">
  <reed as="img" fx="[scroll:zoom-in, stagger:tiny]" src="3.jpg">
</reed>

<!-- Glass morphism card -->
<reed as="card" fx="[blur:glass, shadow:light]">
  <h3>Glass Effect</h3>
  <p>Modern glassmorphism design</p>
</reed>

<!-- Hero with parallax -->
<reed as="hero" fx="[scroll:parallax, speed:slow]">
  <reed as="h1" fx="[scroll:slide-up, delay:small]">
    Welcome
  </reed>
  <reed as="p" fx="[scroll:slide-up, delay:normal]">
    Beautiful effects
  </reed>
</reed>
```

## Acceptance Criteria

- [x] All hover effects work without JavaScript
- [x] Click effects enhance with JavaScript
- [x] Scroll animations trigger at correct threshold
- [x] Stagger delays work for grouped elements
- [x] Blur effects have vendor prefixes
- [x] Shadow levels follow Visual scope
- [x] Duration follows Dimension scope
- [x] Respects prefers-reduced-motion
- [x] GPU acceleration for transforms
- [x] Ripple effect positioned correctly

## Testing

```html
<!-- Test hover effects -->
<reed as="button" fx="[hover:lift]">Hover me</reed>
<reed as="button" fx="[hover:grow]">Hover me</reed>
<reed as="button" fx="[hover:glow]">Hover me</reed>

<!-- Test click effects -->
<reed as="button" fx="[click:ripple]">Click for ripple</reed>
<reed as="button" fx="[click:pulse]">Click for pulse</reed>

<!-- Test scroll effects -->
<reed as="div" fx="[scroll:fade-in]" style="margin-top: 200vh;">
  Scroll down to see me fade in
</reed>

<!-- Test combined effects -->
<reed as="card" 
      fx="[hover:lift, click:ripple, scroll:fade-in, shadow:normal]">
  All effects combined
</reed>
```

## Dependencies

- Ticket #907 (Namespace CSS Generation) - FX namespace
- Ticket #913 (JavaScript Optional API) - Enhancement features

## Blocks

None

## Performance Considerations

1. **Use transform/opacity** for animations (GPU accelerated)
2. **will-change** for frequently animated properties
3. **Intersection Observer** for scroll effects
4. **Lazy initialization** of effects
5. **Debounce/throttle** for performance
6. **CSS-only fallbacks** for core effects

## Notes

- Effects are progressive enhancement
- CSS provides base functionality
- JavaScript adds advanced features
- Always respect user motion preferences
- Keep animations under 300ms for responsiveness