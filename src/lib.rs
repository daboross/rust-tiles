pub fn iter_tiles<'a, T, I>(tiles: T, display_size: f64, tile_size: f64, scroll: f64)
        -> Box<Iterator<Item=(I, f64)> + 'a> where T: IntoIterator<Item=I>, T::IntoIter: 'a {

    let first_tile_offset = scroll % tile_size;

    let (first_tile, underflow) = to_usize_zeroing(scroll / tile_size);

    // how much of the first tile is going to be displayed
    let first_tile_display_size = tile_size - first_tile_offset;
    // `+ 1` because we subtract the number of pixels the first tile takes up before dividing.
    // `.ceil()` because the last tile may be partially off the display.
    let num_tiles = (((display_size - first_tile_display_size) / tile_size)).ceil() as usize + 1;

    Box::new(
        tiles.into_iter()
        .skip(first_tile)
        .take(num_tiles)
        .enumerate()
        .map(move |(tile_num, tile)| {
            // Notes:
            //
            // `position == absolute_tile_num * tile_size - scroll`
            //
            // if `scroll >= 0`, `scroll == first_tile * tile_size + first_tile_offset`
            //
            // thus, if `scroll >= 0`,
            // `position =
            //      absolute_tile_num * tile_size - (first_tile * tile_size + first_tile_offset)`
            //
            // because `absolute_tile_num - first_tile == tile_num`:
            //
            // if `scroll >= 0`, `position = tile_num * tile_size - first_tile_offset`
            //
            // if `scroll < 0`, `first_tile == 0` and `underflow = abs(scroll / tile_size)`
            //
            // thus, if `scroll < 0`, `tile_num == absolute_tile_num`, and:
            // `position = (tile_size + underflow) * tile_size - first_tile_offset`
            //
            // if `scroll >= 0`, `underflow = 0` and `position = tile_num * tile_size - first_tile_offset`.
            //
            // thus, if `scroll >= 0`,
            // `position = (tile_num + underflow) * tile_size - first_tile_offset`
            // since underflow = 0 anyways
            //
            // if `scroll < 0`, we already know that
            // `position = (tile_size + underflow) * tile_size - first_tile_offset`
            //
            // Thus, the position no matter what scroll is, is:
            let position = tile_size * (tile_num as f64 + underflow as f64) - first_tile_offset;
            // return an iterator over (tile, position_on_display)
            (tile, position)
        })
    )
}

pub fn iter_infinite_tiles(display_size: f64, tile_size: f64, scroll: f64)
        -> Box<Iterator<Item=(f64)>> {
    let first_tile_offset = if scroll < 0.0 {
        scroll % tile_size + tile_size
    } else {
        scroll % tile_size
    };

    // how much of the first tile is going to be displayed
    let first_tile_display_size = tile_size - first_tile_offset;
    // `+ 1` because we subtract the number of pixels the first tile takes up before dividing.
    // `.ceil()` because the last tile may be partially off the display.
    let num_tiles = (((display_size - first_tile_display_size) / tile_size)).ceil() as usize + 1;

    Box::new(
        (0..)
        .take(num_tiles)
        .map(move |tile_num| {
            tile_size * (tile_num as f64) - first_tile_offset
        })
    )
}

/// Returns `(input as usize, 0)` if `input > 0` and `(0, -input as usize)` if `input <= 0`
#[inline]
fn to_usize_zeroing(input: f64) -> (usize, usize) {
    if input <= 0.0 {
        (0, (-input) as usize)
    } else {
        (input as usize, 0)
    }
}
