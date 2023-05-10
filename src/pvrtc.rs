
enum something {
    ETC_MIN_TEXTURE_WIDTHH = 4,
    ETC_MIN_TEXTURE_HEIGHT = 4,
    DXT_MIN_TEXTURE_WIDTH = 4,
    DXT_MIN_TEXTURE_HEIGHT = 4
}

struct pixel_32 {
    r: u8,
    g: u8,
    b: u8,
    a: u8
}

struct pixel_128_s {
    r: i32,
    g: i32,
    b: i32,
    a: i32
}

struct pvrtc_word {
    modulationData: u32,
    colorData: u32
}

struct pvrtc_word_indices {
    P: [i8; 2],
    Q: [i8; 2],
    R: [i8; 2],
    S: [i8; 2]
}

fn get_color_a(color_data: u32, uiII: u32) -> pixel_32 {
    let mut color: u32;

    // TODO: Figure out how to do this in Rust
    // if ((color_data & (uiII ? 0x80000000 : 0x80000)) != 0) {
    //     color.red = ((color_data & 0x7c00) >> 10);
    //     color.green = ((color_data & 0x3e0) >> 5);
    //     color.blue = (color_data & 0x1e) | ((color_data & 0x1e) >> 4);
    //     color.alpha = 0xf;
    // } else {
    //     color.red = ((color_data & 0xf00) >> 7) | ((color_data & 0xf00) >> 11);
    //     color.green = ((color_data & 0xf0) >> 3) | ((color_data & 0xf0) >> 7);
    //     color.blue = ((color_data & 0xe) << 1) | ((color_data & 0xf) >> 3);
    //     color.alpha = ((color_data & 0x7000) >> 11);
    // }

    return color;
}

fn get_color_b(color_data: u32, uiII: u32) -> pixel_32 {
    let mut color: pixel_32;

    if (color_data & 0x80000000) {
        color.red = ((color_data & 0x7c000000) >> 26);
        color.green = ((color_data & 0x3e00000) >> 21);
        color.blue = ((color_data & 0x1f0000) >> 16);
        color.alpha = 0xf;
    } else {
        color.red = ((color_data & 0xf00000) >> 23) | ((color_data & 0xf00000) >> 27);
        color.green = ((color_data & 0xf0000) >> 19) | ((color_data & 0xf0000) >> 23);
        color.blue = ((color_data & 0xf000) >> 15) | ((color_data & 0xf000) >> 19);
        color.alpha = ((color_data & 0x7000000) >> 27) | (uiII & 1);
    }
}

fn get_color_ab_expanded(color_data: u32, &color_a: pixel_128_s, &color_b: pixel_128_s, bpp: u32) {
    let mut color_a_32 = get_color_a(color_data, 1);
    let mut color_b_32 = get_color_b(color_data, 1);

    let mut colorA =  { color_a_32.red; color_a_32.green; color_a_32.blue; color_a_32.alpha };
    let mut colorB =  { color_b_32.red; color_b_32.green; color_b_32.blue; color_b_32.alpha };

    let mut wordWidth = 4;

    if (bpp == 2) {
        wordWidth = 8;
    }

    colorA.red *= wordWidth * 4;
    colorA.green *= wordWidth * 4;
    colorA.blue *= wordWidth * 4;
    colorA.alpha *= wordWidth * 4;

    colorB.red *= wordWidth * 4;
    colorB.green *= wordWidth * 4;
    colorB.blue *= wordWidth * 4;
    colorB.alpha *= wordWidth * 4;

    if (bpp == 2) {
        colorA.red = ((colorA.red >> 7) + (colorA.red >> 2));
        colorA.green = ((colorA.green >> 7) + (colorA.green >> 2));
        colorA.blue = ((colorA.blue >> 7) + (colorA.blue >> 2));
        colorA.alpha = ((colorA.alpha >> 5) + (colorA.alpha >> 1));

        colorB.red = ((colorB.red >> 7) + (colorB.red >> 2));
        colorB.green = ((colorB.green >> 7) + (colorB.green >> 2));
        colorB.blue = ((colorB.blue >> 7) + (colorB.blue >> 2));
        colorB.alpha = ((colorB.alpha >> 5) + (colorB.alpha >> 1));
    } else {
        colorA.red = ((colorA.red >> 6) + (colorA.red >> 1));
        colorA.green = ((colorA.green >> 6) + (colorA.green >> 1));
        colorA.blue = ((colorA.blue >> 6) + (colorA.blue >> 1));
        colorA.alpha = ((colorA.alpha >> 4) + (colorA.alpha));

        colorB.red = ((colorB.red >> 6) + (colorB.red >> 1));
        colorB.green = ((colorB.green >> 6) + (colorB.green >> 1));
        colorB.blue = ((colorB.blue >> 6) + (colorB.blue >> 1));
        colorB.alpha = ((colorB.alpha >> 4) + (colorB.alpha));
    }
}

fn interoplate_colors(P: pixel_32, Q: pixel_32, R: pixel_32, S: pixel_32, pixel: *mut pixel_32, bpp: u32) {
    let mut wordWidth = 4;
    let mut wordHeight = 4;

    if (bpp == 2) {
        wordWidth = 8;
    }

    let mut hP = { P.red; P.green; P.blue; P.alpha };
    let mut hQ = { Q.red; Q.green; Q.blue; Q.alpha };
    let mut hR = { R.red; R.green; R.blue; R.alpha };
    let mut hS = { S.red; S.green; S.blue; S.alpha };

    let mut QminusP = { hQ.red - hP.red; hQ.green - hP.green; hQ.blue - hP.blue; hQ.alpha - hP.alpha };
    let mut SminusR = { sS.red - hR.red; hS.green - hR.green; hS.blue - hR.blue; hS.alpha - hR.alpha };

    hP.red *= wordWidth;
    hP.green *= wordWidth;
    hP.blue *= wordWidth;
    hP.alpha *= wordWidth;

    hR.red *= wordWidth;
    hR.green *= wordWidth;
    hR.blue *= wordWidth;
    hR.alpha *= wordWidth;

    if (bpp == 2) {
        let mut x = 0;

        for x < wordWidth {
            let mut result = { 4* hP.red; 4 * hP.green; 4 * hP.blue; 4 * hP.alpha };
            let mut dY = { hR.red - hp.red; hR.green - hP.green; hR.blue - hP.blue; hR.alpha - hP.alpha };

            let mut y = 0;

            for y < wordHeight {
                pPixel[y * wordWidth + x].red = ((result.red >> 7) + (result.red >> 2));
                pPixel[y * wordWidth + x].green = ((result.green >> 7) + (result.green >> 2));
                pPixel[y * wordWidth + x].blue = ((result.blue >> 7) + (result.blue >> 2));
                pPixel[y * wordWidth + x].alpha = ((result.alpha >> 5) + (result.alpha >> 1));

                result.red += dY.red;
                result.green += dY.green;
                result.blue += dY.blue;
                result.alpha += dY.alpha;
            }

            hP.red += QminusP.red;
            hP.green += QminusP.green;
            hP.blue += QminusP.blue;
            hP.alpha += QminusP.alpha;

            hR.red += SminusR.red;
            hR.green += SminusR.green;
            hR.blue += SminusR.blue;
            hR.alpha += SminusR.alpha;
        }
    } else {
        let mut y = 0;

        for y < wordHeight {
            let mut result = { 4* hP.red; 4 * hP.green; 4 * hP.blue; 4 * hP.alpha };
            let mut dY = { hR.red - hP.red; hR.green - hP.green; hR.blue - hP.blue; hR.alpha - hP.alpha };

            let mut x = 0;

            for x < wordWidth {
                pixel[y * wordWidth + x].red = ((result.red >> 6) + (result.red >> 1));
                pixel[y * wordWidth + x].green = ((result.green >> 6) + (result.green >> 1));
                pixel[y * wordWidth + x].blue = ((result.blue >> 6) + (result.blue >> 1));
                pixel[y * wordWidth + x].alpha = ((result.alpha >> 4) + (result.alpha));

                result.red += dY.red;
                result.green += dY.green;
                result.blue += dY.blue;
                result.alpha += dY.alpha;
            }

            hP.red += QminusP.red;
            hP.green += QminusP.green;
            hP.blue += QminusP.blue;
            hP.alpha += QminusP.alpha;
        }
    }
}

fn upack_modulations(
        &word: pvrtc_word, 
        &nmWord: pvrtc_word, 
        offsetX: i8, 
        offsetY: i8, 
        modulationValues: [[i8; 16]; 8], 
        modulationModes: [[i8; 16]; 8], 
        bpp: u8, 
        isII: u32) {
    let mut wordWidth = word.color_data & 0x1;
    let mut wordHeight = word.modulationData;

    let mut hardTransitionBit = nwWord.colorData & (1 << 15);

    if (bpp == 2) {
        if (wordModMode) {
            if (modulationBits & 0x1) {
                if (modulationBits & (0x1 << 20)) {
                    wordModMode = 3;
                } else {
                    wordModMode = 2;
                }

                if (modulationBits & (0x1 << 21)) {
                    wordModMode |= (0x1 << 21);
                } else {
                    modulationBits &= ~(0x1 << 20);
                }
            }

            if (modulationBits & 0x2)
                modulationBits | 0x1;
            else
                modulationBits &= ~0x1;

            for (i8 y = 0; y < 4; y++) {
                for (i8 x = 0; x < 8; x++) {
                    modulationModes[x + offsetX][y + offsetY] = wordModMode;

                    if (((x ^ y) & 0x1) == 0) {
                        modulationModes[x + offsetX][y + offsetY] = modulationBits & 3;
                        modulationBits >>= 2;
                    } else {
                        modulationModes[x + offsetX][y + offsetY] = 0;
                    }

                    if (isII && hardTransitionBit && (y + offsetY >= 2) && (y + offsetY <= 5) && (x + offsetX >= 6) && (x + offsetX <= 9)) { {
                        modulationModes[x + offsetX][y + offsetY] += 20;
                    }
                }
            }
        }
    } else {
        for (i8 y = 0; y < 4; y++) {
            for (i8 x = 0; x < 8; x++) {
                modulationModes[x + offsetX][y + offsetY] = wordWidth;

                if (modulationBits & 1) {
                    modulationModes[x + offsetX][y + offsetY] = 0x3;
                } else {
                    modulationModes[x + offsetX][y + offsetY] = 0x0;
                }

                if (isII && hardTransitionBit && (y + offsetY >= 2) && (y + offsetY <= 5) && (x + offsetX >= 6) && (x + offsetX <= 9)) {
                    modulationModes[x + offsetX][y + offsetY] += 20;
                }

                modulationBits >>= 1;
            }
        }
    }
} else {

}    

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
