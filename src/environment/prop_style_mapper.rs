use crate::arq::arq_item::ArqItem;

use super::prop_style::PropStyle;


pub fn prop_style_mapper(arq_item: &ArqItem, space: &str) -> PropStyle {
    
    let prop_type:usize = match arq_item.prop_type_place {
        Some(tp) => tp,
        _ => 0
    };
    let prop_place = match arq_item.prop_prop_place {
        Some(pp) => pp,
        _ => 0
    };
    let result = PropStyle {
        type_separator: arq_item.prop_type_separator.clone(),
        prop_place: prop_place,
        type_place: prop_type,
        prefix: arq_item.prop_prefix.clone(),
        prefix_space: Some(space.to_string())
    };
    return result
}