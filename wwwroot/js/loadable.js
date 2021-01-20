import { m, html } from "./deps.js";

export default class Loadable {
    view({ attrs, children }) {
        return html`
            <div class="loadable ${attrs.loading && "loading"}">
                <div class="loadable-inner">
                    ${children}
                </div>

                ${attrs.loading && html`
                    <div class="loading-modal">
                        <div class="progress"></div>
                    </div>
                `}
            </div>
        `;
    }
}
