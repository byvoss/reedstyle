# JavaScript API

Optional JavaScript enhancements for advanced functionality.

## Overview

ReedSTYLE works without JavaScript. The optional JavaScript API adds:
- Dynamic component loading
- Interactive effects
- State management
- Form enhancements
- Animation control

## Installation

### Include the Script

```html
<!-- After CSS -->
<script src="reedstyle.js" defer></script>

<!-- Or module -->
<script type="module">
  import ReedStyle from './reedstyle.js';
  ReedStyle.init();
</script>
```

### NPM Installation

```bash
npm install reedstyle
```

```javascript
import ReedStyle from 'reedstyle';

// Initialize with options
ReedStyle.init({
  components: './components.yaml',
  effects: true,
  lazy: true
});
```

## Core API

### Initialization

```javascript
// Default initialization
ReedStyle.init();

// With options
ReedStyle.init({
  // Configuration files (default: project root)
  config: {
    main: './reedstyle.config.yaml',
    components: './reedstyle.components.yaml',
    colors: './reedstyle.colors.yaml',
    fonts: './reedstyle.fonts.yaml'
  },
  
  // Enable/disable features
  effects: true,           // FX namespace animations
  responsive: true,        // Responsive attribute handling
  lazy: true,             // Lazy loading
  
  // Custom breakpoints
  breakpoints: {
    phone: 320,
    tablet: 560,
    screen: 960,
    wide: 1260
  },
  
  // Callbacks
  onReady: () => console.log('ReedSTYLE ready'),
  onError: (err) => console.error('ReedSTYLE error:', err)
});
```

### Component Registration

```javascript
// Register single component
ReedStyle.registerComponent('my-card', {
  element: 'div',
  box: '[padding:6]',
  face: '[bg:base-0, radius:lg, shadow:md]'
});

// Register multiple components
ReedStyle.registerComponents({
  'alert-box': {
    element: 'div',
    face: '[bg:state-warning-weak, border:2:state-warning]'
  },
  'info-box': {
    element: 'div',
    face: '[bg:state-info-weak, border:2:state-info]'
  }
});

// Load from YAML
ReedStyle.loadComponents('./custom-components.yaml')
  .then(() => console.log('Components loaded'));
```

### Element Creation

```javascript
// Create reed element programmatically
const card = ReedStyle.createElement('card', {
  box: '[padding:8]',
  face: '[bg:brand-a]'
});

// With content
const button = ReedStyle.createElement('button-primary', {
  text: 'Click me',
  onclick: () => alert('Clicked!')
});

// Append to DOM
document.body.appendChild(card);
```

### Element Manipulation

```javascript
// Get reed element
const element = document.querySelector('reed[as="card"]');

// Update attributes
ReedStyle.setAttributes(element, {
  face: '[bg:brand-b, shadow:xl]',
  fx: '[hover:lift]'
});

// Add responsive attributes
ReedStyle.setResponsive(element, 'tablet', {
  layout: '[grid:2]',
  box: '[padding:6]'
});

// Toggle states
ReedStyle.toggleState(element, 'active');
```

## Effects System

### Animation Control

```javascript
// Trigger animation
ReedStyle.animate(element, 'fade-in', {
  duration: 500,
  easing: 'ease-out',
  delay: 100
});

// Chain animations
ReedStyle.animateSequence([
  { element: el1, animation: 'slide-up', duration: 300 },
  { element: el2, animation: 'fade-in', duration: 300 },
  { element: el3, animation: 'scale', duration: 300 }
]);

// Parallel animations
ReedStyle.animateParallel([
  { element: el1, animation: 'fade-in' },
  { element: el2, animation: 'slide-up' },
  { element: el3, animation: 'rotate' }
]);
```

### Scroll Effects

```javascript
// Enable scroll animations
ReedStyle.scrollEffects.init({
  threshold: 0.3,
  rootMargin: '50px'
});

// Register scroll trigger
ReedStyle.scrollEffects.observe(element, {
  onEnter: (el) => ReedStyle.animate(el, 'fade-in'),
  onExit: (el) => ReedStyle.animate(el, 'fade-out'),
  once: true
});

// Parallax effect
ReedStyle.scrollEffects.parallax(element, {
  speed: 0.5,
  offset: 100
});
```

### Interactive Effects

```javascript
// Hover effects
ReedStyle.hover(element, {
  enter: { scale: 1.05, shadow: 'xl' },
  exit: { scale: 1, shadow: 'md' }
});

// Click effects
ReedStyle.click(element, {
  active: { scale: 0.95 },
  release: { scale: 1 }
});

// Focus effects
ReedStyle.focus(element, {
  in: { outline: '2:brand-a', outlineOffset: 2 },
  out: { outline: 'none' }
});
```

## State Management

### Component State

```javascript
// Create stateful component
const statefulCard = ReedStyle.createStateful('card', {
  // Initial state
  state: {
    expanded: false,
    selected: false
  },
  
  // State change handlers
  handlers: {
    toggle: function() {
      this.setState({ expanded: !this.state.expanded });
    },
    select: function() {
      this.setState({ selected: true });
    }
  },
  
  // Render based on state
  render: function() {
    return {
      box: this.state.expanded ? '[height:auto]' : '[height:200]',
      face: this.state.selected ? '[bg:brand-a]' : '[bg:base-0]'
    };
  }
});
```

### Global State

```javascript
// Create store
const store = ReedStyle.createStore({
  theme: 'light',
  user: null,
  cart: []
});

// Subscribe to changes
store.subscribe((state) => {
  console.log('State changed:', state);
});

// Update state
store.setState({
  theme: 'dark'
});

// Get current state
const currentState = store.getState();
```

### Data Binding

```javascript
// Two-way binding
ReedStyle.bind(inputElement, {
  model: 'user.name',
  on: 'input'
});

// Computed properties
ReedStyle.computed(element, {
  text: () => `Welcome, ${store.getState().user?.name || 'Guest'}`
});

// Reactive updates
ReedStyle.reactive(element, {
  watch: 'theme',
  update: (theme) => ({
    face: theme === 'dark' ? '[bg:base-900]' : '[bg:base-0]'
  })
});
```

## Form Enhancements

### Validation

```javascript
// Add validation to form
ReedStyle.validateForm('contact-form', {
  rules: {
    email: {
      required: true,
      email: true
    },
    phone: {
      required: false,
      pattern: /^\d{10}$/
    }
  },
  
  messages: {
    email: {
      required: 'Email is required',
      email: 'Invalid email format'
    }
  },
  
  onValid: (data) => {
    console.log('Form valid:', data);
  },
  
  onInvalid: (errors) => {
    console.log('Form errors:', errors);
  }
});
```

### Ajax Forms

```javascript
// Enable Ajax submission
ReedStyle.ajaxForm('newsletter-form', {
  url: '/api/subscribe',
  method: 'POST',
  
  beforeSubmit: (data) => {
    // Modify data before sending
    return data;
  },
  
  onSuccess: (response) => {
    ReedStyle.notify('Subscribed successfully!', 'success');
  },
  
  onError: (error) => {
    ReedStyle.notify('Subscription failed', 'error');
  }
});
```

## Utility Functions

### DOM Utilities

```javascript
// Query reed elements
const cards = ReedStyle.query('card');
const buttons = ReedStyle.queryAll('button-primary');

// Check if element is reed
if (ReedStyle.isReed(element)) {
  // ...
}

// Convert HTML element to reed
ReedStyle.upgrade(document.querySelector('.my-div'), 'card');
```

### Style Utilities

```javascript
// Parse array syntax
const styles = ReedStyle.parseArray('[padding:4, margin:2]');
// { padding: '4', margin: '2' }

// Build array syntax
const array = ReedStyle.buildArray({
  padding: '4',
  margin: '2'
});
// '[padding:4, margin:2]'

// Apply styles
ReedStyle.applyStyles(element, {
  box: '[padding:4]',
  face: '[bg:brand-a]'
});
```

### Event Utilities

```javascript
// Delegate events
ReedStyle.on('click', 'button-primary', (e) => {
  console.log('Button clicked:', e.target);
});

// Once
ReedStyle.once('load', element, () => {
  console.log('Loaded once');
});

// Throttle
const throttled = ReedStyle.throttle(() => {
  console.log('Throttled function');
}, 200);

// Debounce
const debounced = ReedStyle.debounce(() => {
  console.log('Debounced function');
}, 300);
```

## Plugins

### Creating Plugins

```javascript
// Define plugin
const MyPlugin = {
  name: 'my-plugin',
  version: '1.0.0',
  
  install(ReedStyle, options) {
    // Add methods
    ReedStyle.myMethod = function() {
      // ...
    };
    
    // Register components
    ReedStyle.registerComponent('my-component', {
      // ...
    });
    
    // Hook into lifecycle
    ReedStyle.on('ready', () => {
      // ...
    });
  }
};

// Use plugin
ReedStyle.use(MyPlugin, {
  // Plugin options
});
```

### Available Plugins

```javascript
// Router plugin
ReedStyle.use(ReedRouter, {
  routes: {
    '/': 'home',
    '/about': 'about',
    '/contact': 'contact'
  }
});

// i18n plugin
ReedStyle.use(ReedI18n, {
  locale: 'en',
  messages: {
    en: { welcome: 'Welcome' },
    de: { welcome: 'Willkommen' }
  }
});

// Analytics plugin
ReedStyle.use(ReedAnalytics, {
  trackingId: 'UA-XXXXX-Y'
});
```

## Performance

### Lazy Loading

```javascript
// Enable lazy loading
ReedStyle.lazy.init({
  threshold: 0.1,
  rootMargin: '50px'
});

// Lazy load images
ReedStyle.lazy.images('img[data-src]');

// Lazy load components
ReedStyle.lazy.components('reed[data-lazy]');
```

### Virtual Scrolling

```javascript
// Create virtual list
ReedStyle.virtualList(container, {
  items: largeDataArray,
  itemHeight: 50,
  buffer: 5,
  
  renderItem: (item) => {
    return ReedStyle.createElement('list-item', {
      text: item.name
    });
  }
});
```

## Browser Support

### Feature Detection

```javascript
// Check feature support
if (ReedStyle.supports('webcomponents')) {
  // Use web components
}

if (ReedStyle.supports('intersectionobserver')) {
  // Use scroll effects
}

// Polyfills
ReedStyle.polyfill('customelements');
```

### Fallbacks

```javascript
// Provide fallbacks
ReedStyle.fallback({
  'css-layers': () => {
    // Load alternative styles
  },
  'webcomponents': () => {
    // Use alternative implementation
  }
});
```

## Examples

### Complete Application

```javascript
// app.js
import ReedStyle from 'reedstyle';

// Initialize
ReedStyle.init({
  components: './components.yaml',
  effects: true
});

// Create app
class App {
  constructor() {
    this.store = ReedStyle.createStore({
      user: null,
      theme: 'light'
    });
    
    this.init();
  }
  
  init() {
    // Setup routes
    this.setupRoutes();
    
    // Load user
    this.loadUser();
    
    // Bind events
    this.bindEvents();
  }
  
  setupRoutes() {
    ReedStyle.on('click', '[data-route]', (e) => {
      e.preventDefault();
      const route = e.target.dataset.route;
      this.navigate(route);
    });
  }
  
  loadUser() {
    fetch('/api/user')
      .then(res => res.json())
      .then(user => {
        this.store.setState({ user });
      });
  }
  
  bindEvents() {
    // Theme toggle
    ReedStyle.on('click', '#theme-toggle', () => {
      const theme = this.store.getState().theme;
      this.store.setState({
        theme: theme === 'light' ? 'dark' : 'light'
      });
    });
  }
  
  navigate(route) {
    // Handle navigation
    history.pushState({}, '', route);
    this.render(route);
  }
  
  render(route) {
    const container = ReedStyle.query('main');
    
    // Clear container
    container.innerHTML = '';
    
    // Render route
    switch(route) {
      case '/':
        this.renderHome(container);
        break;
      case '/profile':
        this.renderProfile(container);
        break;
    }
  }
  
  renderHome(container) {
    const home = ReedStyle.createElement('hero', {
      children: [
        { tag: 'h1', text: 'Welcome' },
        { tag: 'p', text: 'Build amazing apps with ReedSTYLE' }
      ]
    });
    
    container.appendChild(home);
    
    // Animate entrance
    ReedStyle.animate(home, 'fade-in');
  }
  
  renderProfile(container) {
    const user = this.store.getState().user;
    
    const profile = ReedStyle.createElement('card', {
      children: [
        { tag: 'h2', text: user?.name || 'Guest' },
        { tag: 'p', text: user?.email || 'Not logged in' }
      ]
    });
    
    container.appendChild(profile);
  }
}

// Start app
new App();
```

## API Reference

Full API documentation: [https://reedstyle.dev/api](https://reedstyle.dev/api)

## Summary

The JavaScript API is optional but provides powerful enhancements:
- Component management
- State handling  
- Animation control
- Form validation
- Performance optimization

ReedSTYLE's core styling works without any JavaScript, making it resilient and performant.