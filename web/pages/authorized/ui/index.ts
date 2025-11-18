import lemonade from "lemonadejs";

function AuthorizedPage() {
    return (render: any) => render`
        <article>
            <vein-menu />

            <vein-status-page />
            <vein-logs-page />
        </article>
    `;
}
lemonade.createWebComponent('authorized', AuthorizedPage, { prefix: 'vein' })
