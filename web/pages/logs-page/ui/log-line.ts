import lemonade from "lemonadejs";

function LogLineView() {
    return (render: any) => render`
    <div class="${this.level == "error" ? 'text-red-800' : this.level == 'warn' ? 'text-yellow-600' : ''}">
    ${this.line}
    </div>
    `;
}
lemonade.createWebComponent('log-line', LogLineView, { shadowRoot: false, prefix: 'vein' });
