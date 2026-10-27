const colors = ["bg", "panel", "line", "acid_green", "cyan", "purple", "hot_pink"];
const grid = document.querySelector("#grid"), count = document.querySelector("#count"), search = document.querySelector("#search"), family = document.querySelector("#family"), preview = document.querySelector("#preview");
let themes = [];

const esc = value => String(value).replace(/[&<>"']/g, c => ({ "&": "&amp;", "<": "&lt;", ">": "&gt;", "\"": "&quot;", "'": "&#39;" }[c]));

function swatches(theme, className = "swatches") {
  return `<div class="${className}">${colors.map(key => theme.colors[key] ? `<i style="--c:#${esc(theme.colors[key])}" title="${key}"></i>` : "").join("")}</div>`;
}

function render() {
  const query = search.value.trim().toLowerCase(), selected = family.value;
  const visible = themes.filter(theme => (!selected || theme.family === selected) && (!query || `${theme.name} ${theme.family} ${theme.id}`.toLowerCase().includes(query)));
  count.textContent = `${visible.length} THEMES · ${new Set(visible.map(theme => theme.family)).size} FAMILIES`;
  grid.innerHTML = visible.map(theme => `<article class="theme" data-id="${esc(theme.id)}"><div class="theme-head"><span class="theme-family">${esc(theme.family)}</span><span class="theme-id">${esc(theme.id)}</span></div><h2>${esc(theme.name)}</h2>${swatches(theme)}</article>`).join("") || "<p>No public themes match that search.</p>";
  grid.querySelectorAll(".theme").forEach(card => card.addEventListener("click", () => select(card.dataset.id)));
}

function select(id) {
  const theme = themes.find(item => item.id === id);
  if (!theme) return;
  grid.querySelectorAll(".theme").forEach(card => card.classList.toggle("selected", card.dataset.id === id));
  preview.hidden = false;
  preview.innerHTML = `<h2>${esc(theme.name)}</h2><p>${esc(theme.family)} · <span class="theme-id">${esc(theme.id)}</span></p>${swatches(theme, "preview-swatches")}`;
}

fetch("../docs/theme-chooser-data.json")
  .then(response => { if (!response.ok) throw new Error("Theme catalog unavailable"); return response.json(); })
  .then(data => {
    themes = data.themes;
    [...new Set(themes.map(theme => theme.family))].sort().forEach(name => family.insertAdjacentHTML("beforeend", `<option value="${esc(name)}">${esc(name)}</option>`));
    render();
  })
  .catch(error => { count.textContent = error.message; });

search.addEventListener("input", render);
family.addEventListener("change", render);
