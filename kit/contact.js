// <cybercore-contact>: the shared Cybercore Tech contact block.
//
// One source of truth: ../data/contact.json (next to this script on the org
// site). Every Cybercore page loads the same file, so changing an address or
// a link there updates every site at once.
//
// Theme-aware: the shadow styles read the host page's CYBERGRID tokens
// (--bg, --panel, --line, --muted, --white or --fg, --cyan, --pink, --acid,
// --orange, --purple). Custom properties inherit into shadow DOM, so the
// block re-themes live whenever the page swaps palettes.
//
// Usage:
//   <script type="module" src="https://cybercoretech.github.io/kit/contact.js"></script>
//   <cybercore-contact></cybercore-contact>                    full footer block
//   <cybercore-contact variant="compact"></cybercore-contact>  one-line strip
//   <cybercore-contact repo="argus"></cybercore-contact>       adds that repo's issue link
//   <cybercore-contact bare></cybercore-contact>               no sign-off line, for pages
//                                                              that already have their own
//
// The website (data.home, falling back to brand.site) is featured as an
// animated "home base" card at the top of the channels column, and as a
// highlighted first link in the compact strip. Both are skipped on the
// website itself (cybercoretech.net), where they'd only link to the same page.
//
// Anything inside the tag is the no-JS / offline fallback and is shown only
// if the data can't be loaded, e.g.
//   <cybercore-contact><a href="mailto:dev@cybercoretech.net">dev@cybercoretech.net</a></cybercore-contact>

const DATA_URL = new URL('../data/contact.json', import.meta.url).href;

// The home-base card's border spins via an animated custom property, which
// has to be registered at document level (@property doesn't work inside a
// shadow root). Harmless if the browser lacks it: the border just stays put.
try {
  CSS.registerProperty({ name: '--cc-spin', syntax: '<angle>', inherits: false, initialValue: '0deg' });
} catch { /* already registered, or unsupported */ }
let dataPromise;

function loadData(src) {
  if (src) return fetch(src).then((r) => (r.ok ? r.json() : Promise.reject(r.status)));
  dataPromise ??= fetch(DATA_URL).then((r) => (r.ok ? r.json() : Promise.reject(r.status)));
  return dataPromise;
}

// True when the page is already on the site `url` points to (www. or not):
// the home-base link would only point back at itself there.
function isCurrentSite(url) {
  try {
    const bare = (h) => h.replace(/^www\./, '').toLowerCase();
    return bare(new URL(url).host) === bare(location.host);
  } catch {
    return false;
  }
}

// Only https: and mailto: links ever make it into the page.
function safeUrl(url) {
  try {
    const u = new URL(url);
    return u.protocol === 'https:' || u.protocol === 'mailto:' ? u.href : null;
  } catch {
    return null;
  }
}

function el(tag, props = {}, ...children) {
  const node = document.createElement(tag);
  for (const [k, v] of Object.entries(props)) {
    if (v == null) continue;
    if (k === 'class') node.className = v;
    else if (k === 'text') node.textContent = v;
    else node.setAttribute(k, v);
  }
  node.append(...children.filter(Boolean));
  return node;
}

function link(url, ...children) {
  const href = safeUrl(url);
  if (!href) return null;
  const external = href.startsWith('https:');
  return el('a', { href, target: external ? '_blank' : null, rel: external ? 'noopener noreferrer' : null }, ...children);
}

function mailLink(address) {
  const a = link(`mailto:${address}`);
  if (!a) return null;
  a.className = 'mail';
  a.textContent = address;
  return a;
}

const STYLE = `
  :host {
    --cc-bg: var(--panel, #16151f);
    --cc-fg: var(--white, var(--fg, #f0f0f2));
    --cc-line: var(--line, #32303e);
    --cc-muted: var(--muted, #807f8b);
    --cc-mono: var(--mono, var(--font-mono, 'Share Tech Mono', ui-monospace, monospace));
    --cc-display: var(--display, var(--font-display, var(--cc-mono)));
    display: block;
    container-type: inline-size;
    color: var(--cc-fg);
    font: 12px/1.5 var(--cc-mono);
  }
  :host([hidden]) { display: none; }
  * { box-sizing: border-box; }
  a { color: inherit; text-decoration: none; }
  a:focus-visible { outline: 1px solid var(--cyan, #00e5ff); outline-offset: 3px; }

  .grid { display: grid; grid-template-columns: 1.15fr 1fr 1fr; }
  .col { padding: 4px 26px 6px; border-left: 1px solid var(--col, var(--cc-line)); }
  .col:first-child { padding-left: 0; border-left: 0; }
  .label { display: block; margin-bottom: 14px; color: var(--col); font: 700 10px var(--cc-mono); letter-spacing: .16em; text-transform: uppercase; }
  .c-contact { --col: var(--cyan, #00e5ff); }
  .c-channels { --col: var(--pink, #ff147f); }
  .c-support { --col: var(--acid, #39ff33); }

  .mail { display: inline-block; color: var(--cc-fg); font: 15px var(--cc-mono); letter-spacing: .02em; transition: color .2s, text-shadow .2s; }
  .mail:hover { color: var(--cyan, #00e5ff); text-shadow: 0 0 14px color-mix(in srgb, var(--cyan, #00e5ff) 60%, transparent); }
  small, .note { display: block; margin-top: 6px; color: var(--cc-muted); font-size: 9px; letter-spacing: .12em; text-transform: uppercase; }
  .sec { margin-top: 18px; }
  .sec .mail { font-size: 13px; }
  .sec .mail:hover { color: var(--orange, #ff9000); text-shadow: 0 0 14px color-mix(in srgb, var(--orange, #ff9000) 60%, transparent); }
  .sec .tag { color: var(--orange, #ff9000); }
  .contributors { margin-top: 18px; }
  .contributors .tag { color: var(--pink, #ff147f); }
  .contributors .mail { font-size: 13px; }
  .team-link { display: inline-block; margin-top: 5px; color: var(--pink, #ff147f); font-size: 9px; letter-spacing: .08em; text-transform: uppercase; }
  .team-link:hover { text-decoration: underline; text-underline-offset: 3px; }

  ul { margin: 0; padding: 0; list-style: none; }
  .channel-list { display: grid; grid-template-columns: repeat(2, minmax(0, 1fr)); gap: 7px 10px; }
  .channel-list li { min-width: 0; }
  .chan { display: flex; align-items: center; min-width: 0; gap: 7px; font-size: 10px; letter-spacing: .08em; text-transform: uppercase; transition: transform .2s, color .2s; }
  .chan:hover { color: var(--col); transform: translateX(3px); }
  .glyph { display: grid; place-items: center; flex: none; width: 24px; height: 24px; border: 1px solid color-mix(in srgb, var(--col) 55%, transparent); border-radius: 50%; color: var(--col); background: color-mix(in srgb, var(--col) 10%, transparent); font: 700 9px var(--cc-mono); letter-spacing: 0; }
  .handle { min-width: 0; overflow: hidden; color: var(--cc-muted); font-size: 8px; text-overflow: ellipsis; white-space: nowrap; }
  .arrow { color: var(--col); }

  .support-list { display: grid; justify-items: start; gap: 9px; }
  .coffee { display: inline-flex; align-items: center; width: min(100%, 220px); min-height: 42px; gap: 10px; padding: 7px 12px 7px 9px; border: 1px solid var(--acid, #39ff33); color: var(--acid, #39ff33); background: color-mix(in srgb, var(--acid, #39ff33) 8%, transparent); font: 700 10px var(--cc-mono); letter-spacing: .08em; text-transform: uppercase; transition: transform .2s, box-shadow .2s, background .2s; }
  .coffee:hover { transform: translateY(-2px); background: color-mix(in srgb, var(--acid, #39ff33) 16%, transparent); box-shadow: 0 0 22px color-mix(in srgb, var(--acid, #39ff33) 30%, transparent); }
  .coffee .glyph { --col: var(--acid, #39ff33); }
  .legal { margin-top: 18px; color: var(--cc-muted); font-size: 10px; letter-spacing: .06em; }
  .legal a { color: var(--acid, #39ff33); }
  .legal a:hover { text-decoration: underline; }

  .foot { display: flex; justify-content: space-between; gap: 18px; flex-wrap: wrap; margin-top: 26px; padding-top: 16px; border-top: 1px solid var(--cc-line); color: var(--cc-muted); font-size: 9px; letter-spacing: .12em; text-transform: uppercase; }
  .foot a:hover { color: var(--cyan, #00e5ff); }

  /* home base: cybercoretech.net, the one link that should stand out */
  .home {
    --h1: var(--cyan, #00e5ff); --h2: var(--pink, #ff147f); --h3: var(--acid, #39ff33); --h4: var(--purple, #8a22e2);
    position: relative; display: flex; align-items: center; gap: 12px;
    margin: 0 0 16px; padding: 12px 14px; border-radius: 10px; isolation: isolate; overflow: hidden;
    background: color-mix(in srgb, var(--cc-bg) 80%, black);
    box-shadow: 0 0 0 1px color-mix(in srgb, var(--h1) 25%, transparent), 0 0 24px -6px color-mix(in srgb, var(--h1) 45%, transparent);
    transition: transform .25s cubic-bezier(.2,.8,.2,1), box-shadow .25s;
  }
  /* spinning neon border: a conic gradient masked down to a 1.5px ring */
  .home::before {
    content: ""; position: absolute; inset: 0; z-index: -1; border-radius: inherit; padding: 1.5px;
    background: conic-gradient(from var(--cc-spin), var(--h1), var(--h2), var(--h3), var(--h4), var(--h1));
    -webkit-mask: linear-gradient(#000 0 0) content-box, linear-gradient(#000 0 0);
    -webkit-mask-composite: xor; mask-composite: exclude;
    animation: cc-spin 6s linear infinite;
  }
  /* light sweep on hover */
  .home::after {
    content: ""; position: absolute; top: 0; bottom: 0; left: -60%; width: 45%; z-index: 0; pointer-events: none;
    background: linear-gradient(100deg, transparent, color-mix(in srgb, white 22%, transparent), transparent);
    transform: skewX(-18deg); opacity: 0;
  }
  .home:hover, .home:focus-visible {
    transform: translateY(-3px) scale(1.015);
    box-shadow: 0 0 0 1px color-mix(in srgb, var(--h2) 45%, transparent), 0 10px 34px -6px color-mix(in srgb, var(--h2) 55%, transparent), 0 0 46px -10px color-mix(in srgb, var(--h1) 70%, transparent);
  }
  .home:hover::before, .home:focus-visible::before { animation-duration: 1.6s; }
  .home:hover::after, .home:focus-visible::after { opacity: 1; animation: cc-sweep .9s ease-out; left: 120%; }
  .home > * { position: relative; z-index: 1; }
  .globe { flex: none; width: 34px; height: 34px; color: var(--h1); filter: drop-shadow(0 0 6px color-mix(in srgb, var(--h1) 70%, transparent)); }
  .globe .orbit { transform-origin: 17px 17px; animation: cc-orbit 4s linear infinite; }
  .home:hover .globe { color: var(--h3); }
  .home-text { display: grid; gap: 2px; min-width: 0; }
  .home-kicker { display: flex; align-items: center; gap: 6px; color: var(--h3); font: 700 9px var(--cc-mono); letter-spacing: .18em; text-transform: uppercase; }
  .live { width: 6px; height: 6px; border-radius: 50%; background: var(--h3); box-shadow: 0 0 8px var(--h3); animation: cc-pulse 1.6s ease-in-out infinite; }
  .home-title {
    position: relative; color: var(--cc-fg); font: 700 15px/1.2 var(--cc-display); letter-spacing: .02em;
    text-shadow: 0 0 12px color-mix(in srgb, var(--h1) 55%, transparent);
    overflow-wrap: break-word;
  }
  .home-title::before, .home-title::after { content: attr(data-text); position: absolute; inset: 0; opacity: 0; pointer-events: none; }
  .home-title::before { color: var(--h1); }
  .home-title::after { color: var(--h2); }
  .home:hover .home-title::before { opacity: .8; animation: cc-glitch-a .5s steps(2) 2; }
  .home:hover .home-title::after { opacity: .8; animation: cc-glitch-b .5s steps(2) 2; }
  .home-note { color: var(--cc-muted); font-size: 9px; letter-spacing: .1em; text-transform: uppercase; }
  .home-go {
    margin-left: auto; flex: none; display: grid; place-items: center; width: 28px; height: 28px; border-radius: 50%;
    color: var(--h1); border: 1px solid color-mix(in srgb, var(--h1) 50%, transparent); font-size: 13px;
    transition: transform .25s, color .25s, border-color .25s, background .25s;
  }
  .home:hover .home-go { transform: translateX(3px) rotate(-45deg); color: var(--cc-bg); background: var(--h1); border-color: var(--h1); }
  @keyframes cc-spin { to { --cc-spin: 360deg; } }
  @keyframes cc-sweep { from { left: -60%; } to { left: 120%; } }
  @keyframes cc-orbit { to { transform: rotate(360deg); } }
  @keyframes cc-pulse { 50% { opacity: .3; } }
  @keyframes cc-glitch-a { 0% { transform: translate(-2px, 1px); clip-path: inset(0 0 55% 0); } 50% { transform: translate(2px, -1px); clip-path: inset(40% 0 0 0); } 100% { transform: none; clip-path: inset(0 0 100% 0); } }
  @keyframes cc-glitch-b { 0% { transform: translate(2px, -1px); clip-path: inset(50% 0 0 0); } 50% { transform: translate(-2px, 1px); clip-path: inset(0 0 60% 0); } 100% { transform: none; clip-path: inset(100% 0 0 0); } }

  /* compact strip: the site link gets the same gradient treatment, in small */
  .strip a.s.home-s {
    color: var(--cc-fg); padding: 2px 9px; border-radius: 999px; font-weight: 700;
    background: linear-gradient(var(--cc-bg), var(--cc-bg)) padding-box,
                conic-gradient(from var(--cc-spin), var(--cyan, #00e5ff), var(--pink, #ff147f), var(--acid, #39ff33), var(--cyan, #00e5ff)) border-box;
    border: 1px solid transparent; animation: cc-spin 6s linear infinite;
    text-shadow: 0 0 10px color-mix(in srgb, var(--cyan, #00e5ff) 50%, transparent);
    transition: box-shadow .2s, color .2s;
  }
  .strip a.s.home-s:hover { color: var(--cyan, #00e5ff); box-shadow: 0 0 16px color-mix(in srgb, var(--pink, #ff147f) 45%, transparent); }

  /* compact: one line for project pages */
  .strip { display: flex; flex-wrap: wrap; align-items: center; gap: 8px 18px; font-size: 10px; letter-spacing: .1em; text-transform: uppercase; }
  .strip .mail { font-size: 11px; text-transform: none; letter-spacing: .03em; }
  .strip .sep { color: var(--cc-line); }
  .strip a.s { color: var(--cc-muted); transition: color .2s; }
  .strip a.s:hover { color: var(--pink, #ff147f); }
  .strip a.s.coffee-s { color: var(--acid, #39ff33); }
  .strip a.s.sec-s:hover { color: var(--orange, #ff9000); }

  /* Sized by the space the block is given, not the window, so it never
     cramps when a page drops it into a narrow column. */
  @container (max-width: 860px) {
    .grid { grid-template-columns: 1fr 1fr; gap: 26px 0; }
    /* two columns: the channels column can be ~300px; keep the domain on one line */
    .home { padding: 10px 11px; gap: 9px; }
    .home-title { font-size: 13px; letter-spacing: 0; }
    .globe { width: 28px; height: 28px; }
    .c-support { grid-column: 1 / -1; padding-left: 0; border-left: 0; padding-top: 20px; border-top: 1px solid var(--cc-line); }
  }
  @container (max-width: 560px) {
    .grid { grid-template-columns: 1fr; gap: 24px; }
    .col, .col:first-child, .c-support { padding: 0 0 0 14px; border: 0; border-left: 1px solid var(--col); }
    .foot { flex-direction: column; gap: 8px; }
  }
  @media (prefers-reduced-motion: reduce) {
    *, *::before, *::after { transition: none !important; animation: none !important; }
    .home:hover, .home:focus-visible { transform: none; }
  }
`;

class CybercoreContact extends HTMLElement {
  static observedAttributes = ['variant', 'repo', 'src', 'bare', 'hide-home'];

  connectedCallback() {
    if (!this.shadowRoot) this.attachShadow({ mode: 'open' });
    this.render();
  }

  attributeChangedCallback() {
    if (this.shadowRoot) this.render();
  }

  async render() {
    const root = this.shadowRoot;
    let data;
    try {
      data = await loadData(this.getAttribute('src'));
    } catch {
      // No data: show the page's own fallback content (the light DOM).
      root.replaceChildren(el('slot'));
      return;
    }
    const body = this.getAttribute('variant') === 'compact' ? this.compact(data) : this.full(data);
    root.replaceChildren(el('style', { text: STYLE }), body);
  }

  repoIssues() {
    const repo = this.getAttribute('repo');
    return repo && /^[A-Za-z0-9._-]+$/.test(repo) ? `https://github.com/cybercoretech/${repo}/issues` : null;
  }

  full(d) {
    const { contact, security, contributors } = d.email;
    const channels = [...d.channels];
    const issues = this.repoIssues();
    if (issues) channels.push({ glyph: '!?', label: 'Issues', handle: this.getAttribute('repo'), url: issues });

    const contactCol = el('div', { class: 'col c-contact' },
      el('span', { class: 'label', text: contact.label }), mailLink(contact.address), el('small', { text: contact.note }));
    if (security) {
      contactCol.append(el('div', { class: 'sec' },
        el('small', { class: 'tag', text: security.label }), mailLink(security.address), el('small', { text: security.note })));
    }
    if (contributors) {
      const contributorBlock = el('div', { class: 'contributors' },
        el('small', { class: 'tag', text: contributors.label }),
        mailLink(contributors.address),
        el('small', { text: contributors.note }));
      const teamLink = link(d.team?.url);
      if (teamLink) { teamLink.className = 'team-link'; teamLink.textContent = 'VIEW ROLES / APPLY ↗'; contributorBlock.append(teamLink); }
      contactCol.append(contributorBlock);
    }

    const list = el('ul', { class: 'channel-list' });
    for (const c of channels) {
      const a = link(c.url, el('span', { class: 'glyph', text: c.glyph, 'aria-hidden': 'true' }),
        el('span', { text: c.label }), el('span', { class: 'handle', text: c.handle ? `/ ${c.handle}` : '' }), el('span', { class: 'arrow', text: '↗', 'aria-hidden': 'true' }));
      if (a) { a.className = 'chan'; list.append(el('li', {}, a)); }
    }
    const channelsCol = el('div', { class: 'col c-channels' }, el('span', { class: 'label', text: 'Signal channels' }), this.homeCard(d), list);

    const supportCol = el('div', { class: 'col c-support' }, el('span', { class: 'label', text: 'Support the work' }));
    const supportList = el('div', { class: 'support-list' });
    for (const s of d.support || []) {
      const a = link(s.url, el('span', { class: 'glyph', text: s.glyph, 'aria-hidden': 'true' }), el('span', { text: `${s.label} ↗` }));
      if (a) { a.className = 'coffee'; supportList.append(a); }
    }
    supportCol.append(supportList);
    const lic = link(d.brand.site); if (lic) lic.textContent = d.brand.site.replace(/^https:\/\/|\/$/g, '');
    supportCol.append(el('p', { class: 'legal' }, document.createTextNode(`${d.legal.license} licensed · `), lic));

    const site = link(d.brand.site); if (site) site.textContent = d.brand.signoff;
    const foot = el('div', { class: 'foot' }, site || el('span', { text: d.brand.signoff }), el('span', { text: `${d.legal.copyright} · ${d.legal.notice}` }));

    const grid = el('div', { class: 'grid' }, contactCol, channelsCol, supportCol);
    return el('nav', { 'aria-label': `${d.brand.name} contact` }, grid, this.hasAttribute('bare') ? null : foot);
  }

  /** The website, featured: the one link on the block meant to stand out. */
  homeCard(d) {
    if (this.hasAttribute('hide-home')) return null;
    const home = d.home || { label: 'Home base', url: d.brand.site, title: (d.brand.site || '').replace(/^https:\/\/|\/$/g, '') };
    const href = safeUrl(home.url);
    if (!href || isCurrentSite(href)) return null;
    const svgNS = 'http://www.w3.org/2000/svg';
    const globe = document.createElementNS(svgNS, 'svg');
    globe.setAttribute('viewBox', '0 0 34 34');
    globe.setAttribute('class', 'globe');
    globe.setAttribute('aria-hidden', 'true');
    for (const [tag, attrs] of [
      ['circle', { cx: 17, cy: 17, r: 11, fill: 'none', stroke: 'currentColor', 'stroke-width': 1.6 }],
      ['ellipse', { cx: 17, cy: 17, rx: 5, ry: 11, fill: 'none', stroke: 'currentColor', 'stroke-width': 1.2 }],
      ['path', { d: 'M6 17h22M8 11h18M8 23h18', fill: 'none', stroke: 'currentColor', 'stroke-width': 1.1 }],
    ]) {
      const n = document.createElementNS(svgNS, tag);
      for (const [k, v] of Object.entries(attrs)) n.setAttribute(k, v);
      globe.append(n);
    }
    const orbit = document.createElementNS(svgNS, 'g');
    orbit.setAttribute('class', 'orbit');
    const sat = document.createElementNS(svgNS, 'circle');
    for (const [k, v] of Object.entries({ cx: 17, cy: 2.5, r: 2.2, fill: 'currentColor' })) sat.setAttribute(k, v);
    orbit.append(sat);
    globe.append(orbit);

    const title = home.title || href.replace(/^https:\/\/|\/$/g, '');
    const a = link(href,
      globe,
      el('span', { class: 'home-text' },
        el('span', { class: 'home-kicker' }, el('span', { class: 'live', 'aria-hidden': 'true' }), document.createTextNode(home.label || 'Home base')),
        el('span', { class: 'home-title', 'data-text': title, text: title }),
        home.note ? el('span', { class: 'home-note', text: home.note }) : null),
      el('span', { class: 'home-go', text: '→', 'aria-hidden': 'true' }));
    if (!a) return null;
    a.className = 'home';
    a.setAttribute('aria-label', `${home.label || 'Home base'}: ${title} (opens in a new tab)`);
    return a;
  }

  compact(d) {
    const { contact, security, contributors } = d.email;
    const strip = el('nav', { class: 'strip', 'aria-label': `${d.brand.name} contact` }, mailLink(contact.address));
    const add = (url, text, cls = '') => {
      const a = link(url); if (!a) return;
      a.className = `s ${cls}`.trim(); a.textContent = text;
      strip.append(el('span', { class: 'sep', text: '/', 'aria-hidden': 'true' }), a);
    };
    const home = d.home || { url: d.brand.site };
    if (!this.hasAttribute('hide-home') && home.url && !isCurrentSite(home.url)) add(home.url, `${(home.title || home.url.replace(/^https:\/\/|\/$/g, ''))} ↗`, 'home-s');
    for (const c of d.channels) add(c.url, c.label);
    const issues = this.repoIssues(); if (issues) add(issues, 'Issues');
    if (security) add(`mailto:${security.address}`, 'Security', 'sec-s');
    if (contributors) add(`mailto:${contributors.address}`, 'Contributors');
    if (d.team?.url) add(d.team.url, 'Dev Team / Apply');
    for (const s of d.support || []) add(s.url, `${s.label} ↗`, 'coffee-s');
    return strip;
  }
}

if (!customElements.get('cybercore-contact')) customElements.define('cybercore-contact', CybercoreContact);
