// interprets a scancode with the set 1 or xt set
pub fn interpret(raw_scancode: u8) -> Result<(bool, char), u8> {
    let is_pressed = if raw_scancode >> 7 == 0 { true } else { false };
    let scancode = raw_scancode & 0b0111_1111;

    // digits 1-0 (so 1, 2, 3 ... 8, 9, 0) are 2-11 in set 1
    if scancode >= 2 && scancode <= 11 {
        // in unicode, 0-9 are 0x30 - 0x39
        if scancode == 11 {
            return Ok((is_pressed, '0'));
        } else {
            return Ok((
                is_pressed,
                char::from_u32(scancode as u32 + 0x2F).expect("failed to build digit"),
            ));
        }
    }

    match scancode {
        12 => Ok((is_pressed, '-')),
        13 => Ok((is_pressed, '=')),
        15 => Ok((is_pressed, '\t')),

        16 => Ok((is_pressed, 'q')),
        17 => Ok((is_pressed, 'w')),
        18 => Ok((is_pressed, 'e')),
        19 => Ok((is_pressed, 'r')),
        20 => Ok((is_pressed, 't')),
        21 => Ok((is_pressed, 'y')),
        22 => Ok((is_pressed, 'u')),
        23 => Ok((is_pressed, 'i')),
        24 => Ok((is_pressed, 'o')),
        25 => Ok((is_pressed, 'p')),
        26 => Ok((is_pressed, '[')),
        27 => Ok((is_pressed, ']')),
        28 => Ok((is_pressed, '\n')),

        30 => Ok((is_pressed, 'a')),
        31 => Ok((is_pressed, 's')),
        32 => Ok((is_pressed, 'd')),
        33 => Ok((is_pressed, 'f')),
        34 => Ok((is_pressed, 'g')),
        35 => Ok((is_pressed, 'h')),
        36 => Ok((is_pressed, 'j')),
        37 => Ok((is_pressed, 'k')),
        38 => Ok((is_pressed, 'l')),

        44 => Ok((is_pressed, 'z')),
        45 => Ok((is_pressed, 'x')),
        46 => Ok((is_pressed, 'c')),
        47 => Ok((is_pressed, 'v')),
        48 => Ok((is_pressed, 'b')),
        49 => Ok((is_pressed, 'n')),
        50 => Ok((is_pressed, 'm')),
        51 => Ok((is_pressed, ',')),
        52 => Ok((is_pressed, '.')),
        53 => Ok((is_pressed, '/')),

        57 => Ok((is_pressed, ' ')),

        _ => Err(scancode),
    }
}
