import m from "https://cdn.skypack.dev/pin/mithril@v2.0.4-e2Z23g0XpzLzrW68CBcO/min/mithril.js";
import htm from "https://cdn.skypack.dev/pin/htm@v3.0.4-eKPIliCVcHknqhs5clvp/min/htm.js";

import hljs from "https://cdn.skypack.dev/pin/highlight.js@v10.5.0-kKTJ0zlR1haHew9l5lm1/mode=raw,min/lib/core";
import ini from "https://cdn.skypack.dev/pin/highlight.js@v10.5.0-kKTJ0zlR1haHew9l5lm1/mode=raw,min/lib/languages/ini";
import javascript from "https://cdn.skypack.dev/pin/highlight.js@v10.5.0-kKTJ0zlR1haHew9l5lm1/mode=raw,min/lib/languages/javascript";
import rust from "https://cdn.skypack.dev/pin/highlight.js@v10.5.0-kKTJ0zlR1haHew9l5lm1/mode=raw,min/lib/languages/rust";

hljs.registerLanguage("javascript", javascript);
hljs.registerLanguage("rust", rust);
hljs.registerLanguage("toml", ini);

export { hljs, m };
export const html = htm.bind(m);
