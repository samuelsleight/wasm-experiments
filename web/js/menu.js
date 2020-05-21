let current_menu = null;

function set_current_menu(element) {
    if(current_menu !== null) {
        current_menu.style.display = "none";
    }

    current_menu = element;

    if(current_menu !== null) {
        current_menu.style.display = "block";
    }
}

export default function(context) {
    function generate_world(e) {
        e.preventDefault();

        const seed = document.getElementById("settings_seed").value;
        context.generate_world(seed);
    }

    function init_menu() {
        const settings_button = document.getElementById("menu_settings");
        const settings_section = document.getElementById("settings");
        settings_button.onmouseover = () => set_current_menu(settings_section);

        const reset_button = document.getElementById("menu_reset");
        reset_button.onmouseover = () => set_current_menu(null);
        reset_button.onclick = () => context.generate_world("default");

        const settings_form = document.getElementById("form_settings");
        settings_form.onsubmit = generate_world

        const overlay = document.getElementById("overlay");
        overlay.onmouseleave = () => set_current_menu(null);
    }

    init_menu();
}
