const STORAGE_KEY = "lambdavim-theme";
const THEMES = ["light", "dark"];

function getSystemTheme() {
    return window.matchMedia && window.matchMedia("(prefers-color-scheme: dark)").matches
        ? "dark"
        : "light";
}

function getStoredTheme() {
    try {
        const stored = window.localStorage.getItem(STORAGE_KEY);
        return THEMES.includes(stored) ? stored : null;
    } catch {
        return null;
    }
}

function applyTheme(theme, persist = false) {
    document.documentElement.setAttribute('data-theme', theme);
    document.documentElement.style.colorScheme = theme;
    
    if (persist) {
        try {
            window.localStorage.setItem(STORAGE_KEY, theme);
        } catch {
            // ignore storage failures
        }
    }

    // Update aria labels on theme toggle buttons
    document.querySelectorAll("[data-theme-toggle]").forEach((button) => {
        button.setAttribute("aria-pressed", String(theme === "dark"));
        button.setAttribute("aria-label", theme === "dark" ? "Switch to light mode" : "Switch to dark mode");
    });
}

function initTheme() {
    const storedTheme = getStoredTheme();
    const initialTheme = storedTheme ?? getSystemTheme();
    applyTheme(initialTheme);

    // Listen for system theme changes
    const media = window.matchMedia("(prefers-color-scheme: dark)");
    if (typeof media.addEventListener === "function") {
        media.addEventListener("change", (event) => {
            if (getStoredTheme() === null) {
                applyTheme(event.matches ? "dark" : "light");
            }
        });
    }

    // Listen for custom theme change events from sidebar
    window.addEventListener('themechange', (event) => {
        applyTheme(event.detail.theme);
    });
}

if (document.readyState === "loading") {
    document.addEventListener("DOMContentLoaded", initTheme, { once: true });
} else {
    initTheme();
}
