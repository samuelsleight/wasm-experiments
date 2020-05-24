import { Context } from './context.js'

class Menu {
    constructor(container: HTMLElement) {
        this.current_page = null;

        container.onmouseleave = this.set_current_page.bind(this, null);
    }

    add_page(button: HTMLElement, page: HTMLElement | null, callback: (() => void) | null) {
        button.onmouseover = this.set_current_page.bind(this, page);
        button.onclick = callback;
    }

    private set_current_page(page: HTMLElement | null) {
        if(this.current_page !== null) {
            this.current_page.style.display = "none";
        }

        this.current_page = page;

        if(this.current_page !== null) {
            this.current_page.style.display = "block";
        }
    }

    private current_page: HTMLElement | null;
}

function generate_world(event: Event, context: Context) {
    event.preventDefault();

    const seed = (<HTMLInputElement>document.getElementById("settings_seed")).value;
    context.generate_world(seed);
}

export default (context: Context) => {
    const menu = new Menu(document.getElementById("overlay"));
    menu.add_page(document.getElementById("menu_settings"), document.getElementById("settings"), null);
    menu.add_page(document.getElementById("menu_reset"), null, context.generate_world.bind(context, "default"));

    const settings_form = document.getElementById("form_settings");
    settings_form.onsubmit = (event: Event) => generate_world(event, context);
}
