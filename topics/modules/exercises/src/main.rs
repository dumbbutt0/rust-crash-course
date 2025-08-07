use my_modules::util;
use my_modules::math;
fn main() {
    util::log::debug(&format!("min: {}", math::min(1, 2)));
    util::log::debug(&format!("max: {}", math::max(1, 2)));
}
