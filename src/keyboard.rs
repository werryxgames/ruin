use lazy_static::lazy_static;
use crate::{println, task::keyboard::ScancodeStream};
use spin::{Mutex, MutexGuard};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct KeyboardState {
    pub lctrl_pressed: bool,
    pub lshift_pressed: bool,
    pub lalt_pressed: bool,
    pub rctrl_pressed: bool,
    pub rshift_pressed: bool,
    pub ralt_pressed: bool,
    pub super_pressed: bool,
    pub caps_lock_active: bool,
    pub caps_lock_pressed: bool,
    pub scroll_lock_active: bool,
    pub scroll_lock_pressed: bool,
    pub num_lock_active: bool,
    pub num_lock_pressed: bool,
    pub tab_pressed: bool,
    pub esc_pressed: bool,
    pub f1_pressed: bool,
    pub f2_pressed: bool,
    pub f3_pressed: bool,
    pub f4_pressed: bool,
    pub f5_pressed: bool,
    pub f6_pressed: bool,
    pub f7_pressed: bool,
    pub f8_pressed: bool,
    pub f9_pressed: bool,
    pub f10_pressed: bool,
    pub f11_pressed: bool,
    pub f12_pressed: bool,
    pub arrow_up_pressed: bool,
    pub arrow_left_pressed: bool,
    pub arrow_right_pressed: bool,
    pub arrow_down_pressed: bool,
    pub insert_pressed: bool,
    pub delete_pressed: bool,
    pub backspace_pressed: bool,
    pub end_pressed: bool,
    pub d1_pressed: bool,
    pub d2_pressed: bool,
    pub d3_pressed: bool,
    pub d4_pressed: bool,
    pub d5_pressed: bool,
    pub d6_pressed: bool,
    pub d7_pressed: bool,
    pub d8_pressed: bool,
    pub d9_pressed: bool,
    pub d0_pressed: bool,
    pub pause_pressed: bool,
    pub prtsc_pressed: bool,
    pub home_pressed: bool,
    pub pgup_pressed: bool,
    pub pgdown_pressed: bool,
    pub context_menu_pressed: bool,
    pub eq_pressed: bool,
    pub hyphen_pressed: bool,
    pub open_sqbracket_pressed: bool,
    pub close_sqbracket_pressed: bool,
    pub enter_pressed: bool,
    pub semicolon_pressed: bool,
    pub single_quote_pressed: bool,
    pub back_tick_pressed: bool,
    pub backslash_pressed: bool,
    pub comma_pressed: bool,
    pub dot_pressed: bool,
    pub slash_pressed: bool,
    pub space_pressed: bool,
    pub numpad_asterisk_pressed: bool,
    pub numpad_7_pressed: bool,
    pub numpad_8_pressed: bool,
    pub numpad_9_pressed: bool,
    pub numpad_minus_pressed: bool,
    pub numpad_4_pressed: bool,
    pub numpad_5_pressed: bool,
    pub numpad_6_pressed: bool,
    pub numpad_plus_pressed: bool,
    pub numpad_1_pressed: bool,
    pub numpad_2_pressed: bool,
    pub numpad_3_pressed: bool,
    pub numpad_0_pressed: bool,
    pub numpad_dot_pressed: bool,
    pub numpad_enter_pressed: bool
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct EnglishQwertySet {
    pub q_pressed: bool,
    pub w_pressed: bool,
    pub e_pressed: bool,
    pub r_pressed: bool,
    pub t_pressed: bool,
    pub y_pressed: bool,
    pub u_pressed: bool,
    pub i_pressed: bool,
    pub o_pressed: bool,
    pub p_pressed: bool,
    pub a_pressed: bool,
    pub s_pressed: bool,
    pub d_pressed: bool,
    pub f_pressed: bool,
    pub g_pressed: bool,
    pub h_pressed: bool,
    pub j_pressed: bool,
    pub k_pressed: bool,
    pub l_pressed: bool,
    pub z_pressed: bool,
    pub x_pressed: bool,
    pub c_pressed: bool,
    pub v_pressed: bool,
    pub b_pressed: bool,
    pub n_pressed: bool,
    pub m_pressed: bool
}

lazy_static! {
    pub static ref KEYBOARD: Mutex<KeyboardState> = Mutex::new(KeyboardState {
        lctrl_pressed: false,
        lshift_pressed: false,
        lalt_pressed: false,
        rctrl_pressed: false,
        rshift_pressed: false,
        ralt_pressed: false,
        super_pressed: false,
        caps_lock_active: false,
        caps_lock_pressed: false,
        scroll_lock_active: false,
        scroll_lock_pressed: false,
        num_lock_active: false,
        num_lock_pressed: false,
        tab_pressed: false,
        esc_pressed: false,
        f1_pressed: false,
        f2_pressed: false,
        f3_pressed: false,
        f4_pressed: false,
        f5_pressed: false,
        f6_pressed: false,
        f7_pressed: false,
        f8_pressed: false,
        f9_pressed: false,
        f10_pressed: false,
        f11_pressed: false,
        f12_pressed: false,
        arrow_up_pressed: false,
        arrow_left_pressed: false,
        arrow_right_pressed: false,
        arrow_down_pressed: false,
        insert_pressed: false,
        delete_pressed: false,
        backspace_pressed: false,
        end_pressed: false,
        d1_pressed: false,
        d2_pressed: false,
        d3_pressed: false,
        d4_pressed: false,
        d5_pressed: false,
        d6_pressed: false,
        d7_pressed: false,
        d8_pressed: false,
        d9_pressed: false,
        d0_pressed: false,
        pause_pressed: false,
        prtsc_pressed: false,
        home_pressed: false,
        pgup_pressed: false,
        pgdown_pressed: false,
        context_menu_pressed: false,
        eq_pressed: false,
        hyphen_pressed: false,
        open_sqbracket_pressed: false,
        close_sqbracket_pressed: false,
        enter_pressed: false,
        semicolon_pressed: false,
        single_quote_pressed: false,
        back_tick_pressed: false,
        backslash_pressed: false,
        comma_pressed: false,
        dot_pressed: false,
        slash_pressed: false,
        numpad_asterisk_pressed: false,
        space_pressed: false,
        numpad_7_pressed: false,
        numpad_8_pressed: false,
        numpad_9_pressed: false,
        numpad_minus_pressed: false,
        numpad_4_pressed: false,
        numpad_5_pressed: false,
        numpad_6_pressed: false,
        numpad_plus_pressed: false,
        numpad_1_pressed: false,
        numpad_2_pressed: false,
        numpad_3_pressed: false,
        numpad_0_pressed: false,
        numpad_dot_pressed: false,
        numpad_enter_pressed: false
    });

    pub static ref ENG_QWERTY: Mutex<EnglishQwertySet> = Mutex::new(EnglishQwertySet {
        q_pressed: false,
        w_pressed: false,
        e_pressed: false,
        r_pressed: false,
        t_pressed: false,
        y_pressed: false,
        u_pressed: false,
        i_pressed: false,
        o_pressed: false,
        p_pressed: false,
        a_pressed: false,
        s_pressed: false,
        d_pressed: false,
        f_pressed: false,
        g_pressed: false,
        h_pressed: false,
        j_pressed: false,
        k_pressed: false,
        l_pressed: false,
        z_pressed: false,
        x_pressed: false,
        c_pressed: false,
        v_pressed: false,
        b_pressed: false,
        n_pressed: false,
        m_pressed: false
    });
}

fn print_keyboard() {
    use crate::vga;

    fn print_sc(pressed: bool, text: &str) {
        let mut writer: MutexGuard<vga::VgaWriter> = vga::WRITER.lock();

        if pressed {
            writer.set_color(vga::VgaColor::Black, vga::VgaColor::White);
        } else {
            writer.set_color(vga::VgaColor::White, vga::VgaColor::Black);
        }

        writer.write_string(text);
    }

    let kbd: MutexGuard<KeyboardState> = KEYBOARD.lock();
    let eng: MutexGuard<EnglishQwertySet> = ENG_QWERTY.lock();
    vga::WRITER.lock().clear();
    print_sc(kbd.esc_pressed, "esc");
    print_sc(kbd.f1_pressed, "f1");
    print_sc(kbd.f2_pressed, "f2");
    print_sc(kbd.f3_pressed, "f3");
    print_sc(kbd.f4_pressed, "f4");
    print_sc(kbd.f5_pressed, "f5");
    print_sc(kbd.f6_pressed, "f6");
    print_sc(kbd.f7_pressed, "f7");
    print_sc(kbd.f8_pressed, "f8");
    print_sc(kbd.f9_pressed, "f9");
    print_sc(kbd.f10_pressed, "f10");
    print_sc(kbd.f11_pressed, "f11");
    print_sc(kbd.f12_pressed, "f12");
    print_sc(kbd.pause_pressed, "<Pause>");
    print_sc(kbd.prtsc_pressed, "<PrtSc>");
    print_sc(kbd.delete_pressed, "del");
    print_sc(kbd.home_pressed, "home");
    print_sc(kbd.pgup_pressed, "pgup");
    print_sc(kbd.pgdown_pressed, "pgdn");
    println!();
    print_sc(kbd.back_tick_pressed, "`");
    print_sc(kbd.d1_pressed, "1");
    print_sc(kbd.d2_pressed, "2");
    print_sc(kbd.d3_pressed, "3");
    print_sc(kbd.d4_pressed, "4");
    print_sc(kbd.d5_pressed, "5");
    print_sc(kbd.d6_pressed, "6");
    print_sc(kbd.d7_pressed, "7");
    print_sc(kbd.d8_pressed, "8");
    print_sc(kbd.d9_pressed, "9");
    print_sc(kbd.d0_pressed, "0");
    print_sc(kbd.hyphen_pressed, "-");
    print_sc(kbd.eq_pressed, "=");
    print_sc(kbd.backspace_pressed, "<BS>");
    print_sc(kbd.num_lock_pressed, "<NL>");
    print_sc(kbd.numpad_asterisk_pressed, "*");
    print_sc(kbd.numpad_minus_pressed, "-");
    println!();
    print_sc(kbd.tab_pressed, "Tab");
    print_sc(eng.q_pressed, "q");
    print_sc(eng.w_pressed, "w");
    print_sc(eng.e_pressed, "e");
    print_sc(eng.r_pressed, "r");
    print_sc(eng.t_pressed, "t");
    print_sc(eng.y_pressed, "y");
    print_sc(eng.u_pressed, "u");
    print_sc(eng.i_pressed, "i");
    print_sc(eng.o_pressed, "o");
    print_sc(eng.p_pressed, "p");
    print_sc(kbd.open_sqbracket_pressed, "[");
    print_sc(kbd.close_sqbracket_pressed, "]");
    print_sc(kbd.backslash_pressed, "\\");
    print_sc(kbd.numpad_7_pressed, "7");
    print_sc(kbd.numpad_8_pressed, "8");
    print_sc(kbd.numpad_9_pressed, "9");
    print_sc(kbd.numpad_plus_pressed, "+");
    println!();
    print_sc(kbd.caps_lock_pressed, "<CS>");
    print_sc(eng.a_pressed, "a");
    print_sc(eng.s_pressed, "s");
    print_sc(eng.d_pressed, "d");
    print_sc(eng.f_pressed, "f");
    print_sc(eng.g_pressed, "g");
    print_sc(eng.h_pressed, "h");
    print_sc(eng.j_pressed, "j");
    print_sc(eng.k_pressed, "k");
    print_sc(eng.l_pressed, "l");
    print_sc(kbd.semicolon_pressed, ";");
    print_sc(kbd.single_quote_pressed, "'");
    print_sc(kbd.enter_pressed, "<Enter>");
    print_sc(kbd.numpad_4_pressed, "4");
    print_sc(kbd.numpad_5_pressed, "5");
    print_sc(kbd.numpad_6_pressed, "6");
    println!();
    print_sc(kbd.lshift_pressed, "<LS>");
    print_sc(eng.z_pressed, "z");
    print_sc(eng.x_pressed, "x");
    print_sc(eng.c_pressed, "c");
    print_sc(eng.v_pressed, "v");
    print_sc(eng.b_pressed, "b");
    print_sc(eng.n_pressed, "n");
    print_sc(eng.m_pressed, "m");
    print_sc(kbd.comma_pressed, ",");
    print_sc(kbd.dot_pressed, ".");
    print_sc(kbd.slash_pressed, "/");
    print_sc(kbd.rshift_pressed, "<RS>");
    print_sc(kbd.arrow_up_pressed, "<A>");
    print_sc(kbd.numpad_1_pressed, "1");
    print_sc(kbd.numpad_2_pressed, "2");
    print_sc(kbd.numpad_3_pressed, "3");
    print_sc(kbd.numpad_enter_pressed, "<Enter>");
    println!();
    print_sc(kbd.lctrl_pressed, "<LC>");
    print_sc(kbd.super_pressed, "<S>");
    print_sc(kbd.lalt_pressed, "<LA>");
    print_sc(kbd.space_pressed, "<SPC>");
    print_sc(kbd.ralt_pressed, "<RA>");
    print_sc(kbd.context_menu_pressed, "<CTX>");
    print_sc(kbd.rctrl_pressed, "<RC>");
    print_sc(kbd.arrow_left_pressed, "<D>");
    print_sc(kbd.arrow_down_pressed, "<B>");
    print_sc(kbd.arrow_right_pressed, "<C>");
    print_sc(kbd.numpad_0_pressed, "0");
    print_sc(kbd.numpad_dot_pressed, ".");
    println!();
}

pub async fn handle_key_press2(stream: &mut ScancodeStream) {
    let scancode: u8 = stream.get_next().await;

    match scancode {
        0x1C => KEYBOARD.lock().numpad_enter_pressed = true,
        0x1D => KEYBOARD.lock().rctrl_pressed = true,
        0x2A => KEYBOARD.lock().prtsc_pressed = true,
        0x37 => KEYBOARD.lock().prtsc_pressed = true,
        0x38 => KEYBOARD.lock().ralt_pressed = true,
        0x47 => KEYBOARD.lock().home_pressed = true,
        0x48 => KEYBOARD.lock().arrow_up_pressed = true,
        0x49 => KEYBOARD.lock().pgup_pressed = true,
        0x4B => KEYBOARD.lock().arrow_left_pressed = true,
        0x4D => KEYBOARD.lock().arrow_right_pressed = true,
        0x4F => KEYBOARD.lock().end_pressed = true,
        0x50 => KEYBOARD.lock().arrow_down_pressed = true,
        0x51 => KEYBOARD.lock().pgdown_pressed = true,
        0x52 => KEYBOARD.lock().insert_pressed = true,
        0x53 => KEYBOARD.lock().delete_pressed = true,
        0x5B => KEYBOARD.lock().super_pressed = true,
        0x5D => KEYBOARD.lock().context_menu_pressed = true,
        0x9C => KEYBOARD.lock().numpad_enter_pressed = false,
        0x9D => KEYBOARD.lock().rctrl_pressed = false,
        0xAA => KEYBOARD.lock().prtsc_pressed = false,
        0xB7 => KEYBOARD.lock().prtsc_pressed = false,
        0xB8 => KEYBOARD.lock().ralt_pressed = false,
        0xC7 => KEYBOARD.lock().home_pressed = false,
        0xC8 => KEYBOARD.lock().arrow_up_pressed = false,
        0xC9 => KEYBOARD.lock().pgup_pressed = false,
        0xCB => KEYBOARD.lock().arrow_left_pressed = false,
        0xCD => KEYBOARD.lock().arrow_right_pressed = false,
        0xCF => KEYBOARD.lock().end_pressed = false,
        0xD0 => KEYBOARD.lock().arrow_down_pressed = false,
        0xD1 => KEYBOARD.lock().pgdown_pressed = false,
        0xD2 => KEYBOARD.lock().insert_pressed = false,
        0xD3 => KEYBOARD.lock().delete_pressed = false,
        0xDB => KEYBOARD.lock().super_pressed = false,
        0xDD => KEYBOARD.lock().context_menu_pressed = false,
        _ => println!("E0: {}", scancode)
    }
}

pub async fn handle_key_press3(stream: &mut ScancodeStream) {
    let scancode: u16 = ((stream.get_next().await as u16) << 8) | stream.get_next().await as u16;

    match scancode {
        0x1D45 => KEYBOARD.lock().pause_pressed = true,
        0x9DC5 => KEYBOARD.lock().pause_pressed = false,
        _ => println!("E1: {}", scancode)
    }
}

pub async fn handle_key_press(stream: &mut ScancodeStream) {
    let scancode: u8 = stream.get_next().await;

    match scancode {
        0x01 => KEYBOARD.lock().esc_pressed = true,
        0x02 => KEYBOARD.lock().d1_pressed = true,
        0x03 => KEYBOARD.lock().d2_pressed = true,
        0x04 => KEYBOARD.lock().d3_pressed = true,
        0x05 => KEYBOARD.lock().d4_pressed = true,
        0x06 => KEYBOARD.lock().d5_pressed = true,
        0x07 => KEYBOARD.lock().d6_pressed = true,
        0x08 => KEYBOARD.lock().d7_pressed = true,
        0x09 => KEYBOARD.lock().d8_pressed = true,
        0x0A => KEYBOARD.lock().d9_pressed = true,
        0x0B => KEYBOARD.lock().d0_pressed = true,
        0x0C => KEYBOARD.lock().hyphen_pressed = true,
        0x0D => KEYBOARD.lock().eq_pressed = true,
        0x0E => KEYBOARD.lock().backspace_pressed = true,
        0x0F => KEYBOARD.lock().tab_pressed = true,
        0x10 => ENG_QWERTY.lock().q_pressed = true,
        0x11 => ENG_QWERTY.lock().w_pressed = true,
        0x12 => ENG_QWERTY.lock().e_pressed = true,
        0x13 => ENG_QWERTY.lock().r_pressed = true,
        0x14 => ENG_QWERTY.lock().t_pressed = true,
        0x15 => ENG_QWERTY.lock().y_pressed = true,
        0x16 => ENG_QWERTY.lock().u_pressed = true,
        0x17 => ENG_QWERTY.lock().i_pressed = true,
        0x18 => ENG_QWERTY.lock().o_pressed = true,
        0x19 => ENG_QWERTY.lock().p_pressed = true,
        0x1A => KEYBOARD.lock().open_sqbracket_pressed = true,
        0x1B => KEYBOARD.lock().close_sqbracket_pressed = true,
        0x1C => KEYBOARD.lock().enter_pressed = true,
        0x1D => KEYBOARD.lock().lctrl_pressed = true,
        0x1E => ENG_QWERTY.lock().a_pressed = true,
        0x1F => ENG_QWERTY.lock().s_pressed = true,
        0x20 => ENG_QWERTY.lock().d_pressed = true,
        0x21 => ENG_QWERTY.lock().f_pressed = true,
        0x22 => ENG_QWERTY.lock().g_pressed = true,
        0x23 => ENG_QWERTY.lock().h_pressed = true,
        0x24 => ENG_QWERTY.lock().j_pressed = true,
        0x25 => ENG_QWERTY.lock().k_pressed = true,
        0x26 => ENG_QWERTY.lock().l_pressed = true,
        0x27 => KEYBOARD.lock().semicolon_pressed = true,
        0x28 => KEYBOARD.lock().single_quote_pressed = true,
        0x29 => KEYBOARD.lock().back_tick_pressed = true,
        0x2A => KEYBOARD.lock().lshift_pressed = true,
        0x2B => KEYBOARD.lock().backslash_pressed = true,
        0x2C => ENG_QWERTY.lock().z_pressed = true,
        0x2D => ENG_QWERTY.lock().x_pressed = true,
        0x2E => ENG_QWERTY.lock().c_pressed = true,
        0x2F => ENG_QWERTY.lock().v_pressed = true,
        0x30 => ENG_QWERTY.lock().b_pressed = true,
        0x31 => ENG_QWERTY.lock().n_pressed = true,
        0x32 => ENG_QWERTY.lock().m_pressed = true,
        0x33 => KEYBOARD.lock().comma_pressed = true,
        0x34 => KEYBOARD.lock().dot_pressed = true,
        0x35 => KEYBOARD.lock().slash_pressed = true,
        0x36 => KEYBOARD.lock().rshift_pressed = true,
        0x37 => KEYBOARD.lock().numpad_asterisk_pressed = true,
        0x38 => KEYBOARD.lock().lalt_pressed = true,
        0x39 => KEYBOARD.lock().space_pressed = true,
        0x3A => {
            let mut kbd: MutexGuard<KeyboardState> = KEYBOARD.lock();
            kbd.caps_lock_pressed = true;
            kbd.caps_lock_active = !kbd.caps_lock_active;
        },
        0x3B => KEYBOARD.lock().f1_pressed = true,
        0x3C => KEYBOARD.lock().f2_pressed = true,
        0x3D => KEYBOARD.lock().f3_pressed = true,
        0x3E => KEYBOARD.lock().f4_pressed = true,
        0x3F => KEYBOARD.lock().f5_pressed = true,
        0x40 => KEYBOARD.lock().f6_pressed = true,
        0x41 => KEYBOARD.lock().f7_pressed = true,
        0x42 => KEYBOARD.lock().f8_pressed = true,
        0x43 => KEYBOARD.lock().f9_pressed = true,
        0x44 => KEYBOARD.lock().f10_pressed = true,
        0x45 => {
            let mut kbd: MutexGuard<KeyboardState> = KEYBOARD.lock();
            kbd.num_lock_pressed = true;
            kbd.num_lock_active = !kbd.num_lock_active;
        },
        0x46 => {
            let mut kbd: MutexGuard<KeyboardState> = KEYBOARD.lock();
            kbd.scroll_lock_pressed = true;
            kbd.scroll_lock_active = !kbd.scroll_lock_active;
        },
        0x47 => KEYBOARD.lock().numpad_7_pressed = true,
        0x48 => KEYBOARD.lock().numpad_8_pressed = true,
        0x49 => KEYBOARD.lock().numpad_9_pressed = true,
        0x4A => KEYBOARD.lock().numpad_minus_pressed = true,
        0x4B => KEYBOARD.lock().numpad_4_pressed = true,
        0x4C => KEYBOARD.lock().numpad_5_pressed = true,
        0x4D => KEYBOARD.lock().numpad_6_pressed = true,
        0x4E => KEYBOARD.lock().numpad_plus_pressed = true,
        0x4F => KEYBOARD.lock().numpad_1_pressed = true,
        0x50 => KEYBOARD.lock().numpad_2_pressed = true,
        0x51 => KEYBOARD.lock().numpad_3_pressed = true,
        0x52 => KEYBOARD.lock().numpad_0_pressed = true,
        0x53 => KEYBOARD.lock().numpad_dot_pressed = true,
        0x57 => KEYBOARD.lock().f11_pressed = true,
        0x58 => KEYBOARD.lock().f12_pressed = true,
        0x5D => KEYBOARD.lock().context_menu_pressed = true,
        0x81 => KEYBOARD.lock().esc_pressed = false,
        0x82 => KEYBOARD.lock().d1_pressed = false,
        0x83 => KEYBOARD.lock().d2_pressed = false,
        0x84 => KEYBOARD.lock().d3_pressed = false,
        0x85 => KEYBOARD.lock().d4_pressed = false,
        0x86 => KEYBOARD.lock().d5_pressed = false,
        0x87 => KEYBOARD.lock().d6_pressed = false,
        0x88 => KEYBOARD.lock().d7_pressed = false,
        0x89 => KEYBOARD.lock().d8_pressed = false,
        0x8A => KEYBOARD.lock().d9_pressed = false,
        0x8B => KEYBOARD.lock().d0_pressed = false,
        0x8C => KEYBOARD.lock().hyphen_pressed = false,
        0x8D => KEYBOARD.lock().eq_pressed = false,
        0x8E => KEYBOARD.lock().backspace_pressed = false,
        0x8F => KEYBOARD.lock().tab_pressed = false,
        0x90 => ENG_QWERTY.lock().q_pressed = false,
        0x91 => ENG_QWERTY.lock().w_pressed = false,
        0x92 => ENG_QWERTY.lock().e_pressed = false,
        0x93 => ENG_QWERTY.lock().r_pressed = false,
        0x94 => ENG_QWERTY.lock().t_pressed = false,
        0x95 => ENG_QWERTY.lock().y_pressed = false,
        0x96 => ENG_QWERTY.lock().u_pressed = false,
        0x97 => ENG_QWERTY.lock().i_pressed = false,
        0x98 => ENG_QWERTY.lock().o_pressed = false,
        0x99 => ENG_QWERTY.lock().p_pressed = false,
        0x9A => KEYBOARD.lock().open_sqbracket_pressed = false,
        0x9B => KEYBOARD.lock().close_sqbracket_pressed = false,
        0x9C => KEYBOARD.lock().enter_pressed = false,
        0x9D => KEYBOARD.lock().lctrl_pressed = false,
        0x9E => ENG_QWERTY.lock().a_pressed = false,
        0x9F => ENG_QWERTY.lock().s_pressed = false,
        0xA0 => ENG_QWERTY.lock().d_pressed = false,
        0xA1 => ENG_QWERTY.lock().f_pressed = false,
        0xA2 => ENG_QWERTY.lock().g_pressed = false,
        0xA3 => ENG_QWERTY.lock().h_pressed = false,
        0xA4 => ENG_QWERTY.lock().j_pressed = false,
        0xA5 => ENG_QWERTY.lock().k_pressed = false,
        0xA6 => ENG_QWERTY.lock().l_pressed = false,
        0xA7 => KEYBOARD.lock().semicolon_pressed = false,
        0xA8 => KEYBOARD.lock().single_quote_pressed = false,
        0xA9 => KEYBOARD.lock().back_tick_pressed = false,
        0xAA => KEYBOARD.lock().lshift_pressed = false,
        0xAB => KEYBOARD.lock().backslash_pressed = false,
        0xAC => ENG_QWERTY.lock().z_pressed = false,
        0xAD => ENG_QWERTY.lock().x_pressed = false,
        0xAE => ENG_QWERTY.lock().c_pressed = false,
        0xAF => ENG_QWERTY.lock().v_pressed = false,
        0xB0 => ENG_QWERTY.lock().b_pressed = false,
        0xB1 => ENG_QWERTY.lock().n_pressed = false,
        0xB2 => ENG_QWERTY.lock().m_pressed = false,
        0xB3 => KEYBOARD.lock().comma_pressed = false,
        0xB4 => KEYBOARD.lock().dot_pressed = false,
        0xB5 => KEYBOARD.lock().slash_pressed = false,
        0xB6 => KEYBOARD.lock().rshift_pressed = false,
        0xB7 => KEYBOARD.lock().numpad_asterisk_pressed = false,
        0xB8 => KEYBOARD.lock().lalt_pressed = false,
        0xB9 => KEYBOARD.lock().space_pressed = false,
        0xBA => KEYBOARD.lock().caps_lock_pressed = false,
        0xBB => KEYBOARD.lock().f1_pressed = false,
        0xBC => KEYBOARD.lock().f2_pressed = false,
        0xBD => KEYBOARD.lock().f3_pressed = false,
        0xBE => KEYBOARD.lock().f4_pressed = false,
        0xBF => KEYBOARD.lock().f5_pressed = false,
        0xC0 => KEYBOARD.lock().f6_pressed = false,
        0xC1 => KEYBOARD.lock().f7_pressed = false,
        0xC2 => KEYBOARD.lock().f8_pressed = false,
        0xC3 => KEYBOARD.lock().f9_pressed = false,
        0xC4 => KEYBOARD.lock().f10_pressed = false,
        0xC5 => KEYBOARD.lock().num_lock_pressed = false,
        0xC6 => KEYBOARD.lock().scroll_lock_pressed = false,
        0xC7 => KEYBOARD.lock().numpad_7_pressed = false,
        0xC8 => KEYBOARD.lock().numpad_8_pressed = false,
        0xC9 => KEYBOARD.lock().numpad_9_pressed = false,
        0xCA => KEYBOARD.lock().numpad_minus_pressed = false,
        0xCB => KEYBOARD.lock().numpad_4_pressed = false,
        0xCC => KEYBOARD.lock().numpad_5_pressed = false,
        0xCD => KEYBOARD.lock().numpad_6_pressed = false,
        0xCE => KEYBOARD.lock().numpad_plus_pressed = false,
        0xCF => KEYBOARD.lock().numpad_1_pressed = false,
        0xD0 => KEYBOARD.lock().numpad_2_pressed = false,
        0xD1 => KEYBOARD.lock().numpad_3_pressed = false,
        0xD2 => KEYBOARD.lock().numpad_0_pressed = false,
        0xD3 => KEYBOARD.lock().numpad_dot_pressed = false,
        0xD7 => KEYBOARD.lock().f11_pressed = false,
        0xD8 => KEYBOARD.lock().f12_pressed = false,
        0xDD => KEYBOARD.lock().context_menu_pressed = false,
        0xE0 => handle_key_press2(stream).await,
        0xE1 => handle_key_press3(stream).await,
        _ => println!("{}", scancode)
    }

    print_keyboard();
}
