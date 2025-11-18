import lemonade from "lemonadejs";
import { SSE } from "shared/lib";

function VeinMenu() {
    const status = (ev: MouseEvent) => {
        ev.preventDefault();
        ev.stopPropagation();
        SSE.emit('page', 'status');
    };
    const logs = (ev: MouseEvent) => {
        ev.preventDefault();
        ev.stopPropagation();
        SSE.emit('page', 'logs');
    };
    return (render: any) => render`
        <nav class="bg-gray-800 p-6">
            <ul class="flex gap-4">
                <li>
                    <a href="#status" class="text-gray-400 hover:text-white" onclick=${status}>
                        Status
                    </a>
                </li>
                <li>
                    <a href="#logs" class="text-gray-400 hover:text-white" onclick=${logs}>
                        Logs
                    </a>
                </li>
            </ul>
        </nav>
        `;
}

lemonade.createWebComponent('menu', VeinMenu, { prefix: 'vein' });
